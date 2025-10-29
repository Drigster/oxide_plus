use crate::proto::*;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Event types emitted by the RustPlus client
#[derive(Debug, Clone)]
pub enum RustPlusEvent {
    /// Fired when connecting to the Rust server
    Connecting,
    /// Fired when connected to the Rust server
    Connected,
    /// Fired when disconnected from the Rust server
    Disconnected,
    /// Fired when an error occurs
    Error(String),
    /// Fired when a message is received from the server
    Message(AppMessage),
    /// Fired when a request is sent to the server
    Request(AppRequest),
}

/// Trait for handling RustPlus events
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: RustPlusEvent);
}

/// Event broadcaster for RustPlus events
#[derive(Debug)]
pub struct EventBroadcaster {
    sender: broadcast::Sender<RustPlusEvent>,
}

impl EventBroadcaster {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<RustPlusEvent> {
        self.sender.subscribe()
    }
    
    pub fn broadcast(&self, event: RustPlusEvent) -> Result<usize, broadcast::error::SendError<RustPlusEvent>> {
        self.sender.send(event)
    }
    
    pub fn receiver_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Clone for EventBroadcaster {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}
