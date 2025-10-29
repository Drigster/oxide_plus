use crate::error::{RustPlusError, Result};
use crate::events::{RustPlusEvent, EventBroadcaster};
use crate::proto::*;
use futures_util::{SinkExt, StreamExt};
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use url::Url;

/// Main RustPlus client for interacting with Rust servers
#[derive(Debug)]
pub struct RustPlus {
    server: String,
    port: u16,
    player_id: u64,
    player_token: i32,
    use_facepunch_proxy: bool,
    
    // Connection state
    websocket: Arc<Mutex<Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    is_connected: Arc<Mutex<bool>>,
    
    // Request/Response handling
    seq: Arc<Mutex<u32>>,
    pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<AppResponse>>>>,
    
    // Event system
    event_broadcaster: EventBroadcaster,
}

impl RustPlus {
    /// Create a new RustPlus client instance
    pub async fn new(
        server: &str,
        port: u16,
        player_id: u64,
        player_token: i32,
        use_facepunch_proxy: bool,
    ) -> Result<Self> {
        Ok(Self {
            server: server.to_string(),
            port,
            player_id,
            player_token,
            use_facepunch_proxy,
            websocket: Arc::new(Mutex::new(None)),
            is_connected: Arc::new(Mutex::new(false)),
            seq: Arc::new(Mutex::new(0)),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            event_broadcaster: EventBroadcaster::new(1000),
        })
    }
    
    /// Connect to the Rust server
    pub async fn connect(&self) -> Result<()> {
        // Disconnect if already connected
        if *self.is_connected.lock().await {
            self.disconnect().await?;
        }
        
        self.event_broadcaster.broadcast(RustPlusEvent::Connecting)?;
        
        // Build WebSocket URL
        let url = if self.use_facepunch_proxy {
            format!("wss://companion-rust.facepunch.com/game/{}/{}", self.server, self.port)
        } else {
            format!("ws://{}:{}", self.server, self.port)
        };
        
        //println!("Connecting to: {}", url);
        let url = Url::parse(&url)?;
        let (ws_stream, _) = connect_async(url).await?;
        
        *self.websocket.lock().await = Some(ws_stream);
        *self.is_connected.lock().await = true;
        
        self.event_broadcaster.broadcast(RustPlusEvent::Connected)?;
        
        // Start message handling task
        self.start_message_handler().await;
        
        // Give the connection a moment to establish
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        Ok(())
    }
    
    /// Disconnect from the Rust server
    pub async fn disconnect(&self) -> Result<()> {
        if let Some(ws) = self.websocket.lock().await.take() {
            let (mut write, _) = ws.split();
            let _ = write.close().await;
        }
        
        *self.is_connected.lock().await = false;
        
        // Cancel all pending requests
        let mut pending = self.pending_requests.lock().await;
        pending.clear();
        
        self.event_broadcaster.broadcast(RustPlusEvent::Disconnected)?;
        Ok(())
    }
    
    /// Check if connected to the server
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.lock().await
    }
    
    /// Get event broadcaster for subscribing to events
    pub fn event_broadcaster(&self) -> EventBroadcaster {
        self.event_broadcaster.clone()
    }
    
    /// Send a request to the server and wait for response
    pub async fn send_request(&self, data: AppRequestData) -> Result<AppResponse> {
        if !self.is_connected().await {
            return Err(RustPlusError::NotConnected);
        }
        
        let seq = {
            let mut seq_guard = self.seq.lock().await;
            *seq_guard += 1;
            *seq_guard
        };
        
        // Create response channel
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(seq, tx);
        }
        
        // Create and send request
        let request = AppRequest {
            seq,
            player_id: self.player_id,
            player_token: self.player_token,
            entity_id: data.entity_id,
            get_info: data.get_info,
            get_time: data.get_time,
            get_map: data.get_map,
            get_team_info: data.get_team_info,
            get_team_chat: data.get_team_chat,
            send_team_message: data.send_team_message,
            get_entity_info: data.get_entity_info,
            set_entity_value: data.set_entity_value,
            check_subscription: data.check_subscription,
            set_subscription: data.set_subscription,
            get_map_markers: data.get_map_markers,
            promote_to_leader: data.promote_to_leader,
            get_clan_info: data.get_clan_info,
            set_clan_motd: data.set_clan_motd,
            get_clan_chat: data.get_clan_chat,
            send_clan_message: data.send_clan_message,
            get_nexus_auth: data.get_nexus_auth,
            camera_subscribe: data.camera_subscribe,
            camera_unsubscribe: data.camera_unsubscribe,
            camera_input: data.camera_input,
        };
        
        self.send_raw_request(request).await?;
        
        // Read the response message
        let response = self.read_response_message(seq).await?;
        
        if let Some(error) = response.error {
            Err(RustPlusError::ServerError(error.error))
        } else {
            Ok(response)
        }
    }
    
    /// Send a raw AppRequest to the server
    async fn send_raw_request(&self, request: AppRequest) -> Result<()> {
        let mut ws_guard = self.websocket.lock().await;
        let ws = ws_guard.as_mut()
            .ok_or(RustPlusError::NotConnected)?;
        
        let encoded = prost::Message::encode_to_vec(&request);
        let message = WsMessage::Binary(encoded);
        
        let (mut write, _) = ws.split();
        write.send(message).await?;
        
        self.event_broadcaster.broadcast(RustPlusEvent::Request(request))?;
        Ok(())
    }
    
    /// Start the message handler task
    async fn start_message_handler(&self) {
        //println!("Message handler started - will process messages when requests are made");
    }
    
    /// Read a response message for a specific sequence number
    async fn read_response_message(&self, expected_seq: u32) -> Result<AppResponse> {
        //println!("Reading response for seq: {}", expected_seq);
        
        // Try to read messages for up to 10 seconds
        let start_time = std::time::Instant::now();
        let timeout_duration = Duration::from_secs(10);
        
        while start_time.elapsed() < timeout_duration {
            // Try to read a message
            if let Some(message) = self.try_read_message().await? {
                //println!("📨 Received message: seq={:?}", message.response.as_ref().map(|r| r.seq));
                
                if let Some(ref response) = message.response {
                    let seq = response.seq;
                    //println!("📥 Response for seq {}: success={:?}", seq, response.success.is_some());
                    
                    if seq == expected_seq {
                        // Broadcast the message for general event handling
                        let _ = self.event_broadcaster.broadcast(RustPlusEvent::Message(message.clone()));
                        return Ok(response.clone());
                    }
                }
                
                // Broadcast the message for general event handling
                let _ = self.event_broadcaster.broadcast(RustPlusEvent::Message(message));
            }
            
            // Wait a bit before trying again
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        
        Err(RustPlusError::Timeout)
    }
    
    /// Try to read a single message from the WebSocket
    async fn try_read_message(&self) -> Result<Option<AppMessage>> {
        let mut ws_guard = self.websocket.lock().await;
        if let Some(ws) = ws_guard.as_mut() {
            let (_, mut read) = ws.split();
            
            // Try to read a message with a short timeout
            match tokio::time::timeout(Duration::from_millis(50), read.next()).await {
                Ok(Some(Ok(WsMessage::Binary(data)))) => {
                    match AppMessage::decode(&data[..]) {
                        Ok(message) => Ok(Some(message)),
                        Err(e) => {
                            println!("Failed to decode message: {}", e);
                            Ok(None)
                        }
                    }
                }
                Ok(Some(Ok(WsMessage::Close(_)))) => {
                    println!("WebSocket connection closed by server");
                    Err(RustPlusError::ConnectionClosed)
                }
                Ok(Some(Err(e))) => {
                    println!("WebSocket error: {}", e);
                    Err(RustPlusError::ConnectionError(e.to_string()))
                }
                Ok(None) => {
                    println!("WebSocket connection ended");
                    Err(RustPlusError::ConnectionClosed)
                }
                Ok(Some(_)) => {
                    // Ignore other message types
                    Ok(None)
                }
                Err(_) => {
                    // Timeout - no message available
                    Ok(None)
                }
            }
        } else {
            Err(RustPlusError::NotConnected)
        }
    }
    
    
    // Convenience methods
    
    /// Turn a smart switch on
    pub async fn turn_smart_switch_on(&self, entity_id: u32) -> Result<()> {
        self.set_entity_value(entity_id, true).await
    }
    
    /// Turn a smart switch off
    pub async fn turn_smart_switch_off(&self, entity_id: u32) -> Result<()> {
        self.set_entity_value(entity_id, false).await
    }
    
    /// Set entity value
    pub async fn set_entity_value(&self, entity_id: u32, value: bool) -> Result<()> {
        let data = AppRequestData {
            entity_id: Some(entity_id),
            set_entity_value: Some(AppSetEntityValue { value }),
            ..Default::default()
        };
        
        self.send_request(data).await?;
        Ok(())
    }
    
    /// Send a team message
    pub async fn send_team_message(&self, message: &str) -> Result<()> {
        let data = AppRequestData {
            send_team_message: Some(AppSendMessage {
                message: message.to_string(),
            }),
            ..Default::default()
        };
        
        self.send_request(data).await?;
        Ok(())
    }
    
    /// Get entity info
    pub async fn get_entity_info(&self, entity_id: u32) -> Result<AppEntityInfo> {
        let data = AppRequestData {
            entity_id: Some(entity_id),
            get_entity_info: Some(AppEmpty {}),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.entity_info
            .ok_or_else(|| RustPlusError::Generic("No entity info in response".to_string()))
    }
    
    /// Get server info
    pub async fn get_info(&self) -> Result<AppInfo> {
        let data = AppRequestData {
            get_info: Some(AppEmpty {}),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.info
            .ok_or_else(|| RustPlusError::Generic("No info in response".to_string()))
    }
    
    /// Get map data
    pub async fn get_map(&self) -> Result<AppMap> {
        let data = AppRequestData {
            get_map: Some(AppEmpty {}),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.map
            .ok_or_else(|| RustPlusError::Generic("No map in response".to_string()))
    }
    
    /// Get time info
    pub async fn get_time(&self) -> Result<AppTime> {
        let data = AppRequestData {
            get_time: Some(AppEmpty {}),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.time
            .ok_or_else(|| RustPlusError::Generic("No time in response".to_string()))
    }
    
    /// Get map markers
    pub async fn get_map_markers(&self) -> Result<AppMapMarkers> {
        let data = AppRequestData {
            get_map_markers: Some(AppEmpty {}),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.map_markers
            .ok_or_else(|| RustPlusError::Generic("No map markers in response".to_string()))
    }
    
    /// Get team info
    pub async fn get_team_info(&self) -> Result<AppTeamInfo> {
        let data = AppRequestData {
            get_team_info: Some(AppEmpty {}),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.team_info
            .ok_or_else(|| RustPlusError::Generic("No team info in response".to_string()))
    }
    
    /// Subscribe to a camera
    pub async fn subscribe_to_camera(&self, camera_id: &str) -> Result<AppCameraInfo> {
        let data = AppRequestData {
            camera_subscribe: Some(AppCameraSubscribe {
                camera_id: camera_id.to_string(),
            }),
            ..Default::default()
        };
        
        let response = self.send_request(data).await?;
        response.camera_subscribe_info
            .ok_or_else(|| RustPlusError::Generic("No camera info in response".to_string()))
    }
    
    /// Unsubscribe from camera
    pub async fn unsubscribe_from_camera(&self) -> Result<()> {
        let data = AppRequestData {
            camera_unsubscribe: Some(AppEmpty {}),
            ..Default::default()
        };
        
        self.send_request(data).await?;
        Ok(())
    }
    
    /// Send camera input
    pub async fn send_camera_input(&self, buttons: i32, mouse_delta: Vector2) -> Result<()> {
        let data = AppRequestData {
            camera_input: Some(AppCameraInput {
                buttons,
                mouse_delta,
            }),
            ..Default::default()
        };
        
        self.send_request(data).await?;
        Ok(())
    }
}

/// Helper struct for building AppRequest data
#[derive(Debug, Default)]
pub struct AppRequestData {
    pub entity_id: Option<u32>,
    pub get_info: Option<AppEmpty>,
    pub get_time: Option<AppEmpty>,
    pub get_map: Option<AppEmpty>,
    pub get_team_info: Option<AppEmpty>,
    pub get_team_chat: Option<AppEmpty>,
    pub send_team_message: Option<AppSendMessage>,
    pub get_entity_info: Option<AppEmpty>,
    pub set_entity_value: Option<AppSetEntityValue>,
    pub check_subscription: Option<AppEmpty>,
    pub set_subscription: Option<AppFlag>,
    pub get_map_markers: Option<AppEmpty>,
    pub promote_to_leader: Option<AppPromoteToLeader>,
    pub get_clan_info: Option<AppEmpty>,
    pub set_clan_motd: Option<AppSendMessage>,
    pub get_clan_chat: Option<AppEmpty>,
    pub send_clan_message: Option<AppSendMessage>,
    pub get_nexus_auth: Option<AppGetNexusAuth>,
    pub camera_subscribe: Option<AppCameraSubscribe>,
    pub camera_unsubscribe: Option<AppEmpty>,
    pub camera_input: Option<AppCameraInput>,
}
