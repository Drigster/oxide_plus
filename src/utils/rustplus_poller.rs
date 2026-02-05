// Code written by Claude Opus 4.5

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use futures_channel::mpsc::UnboundedSender;
use rustplus_rs::RustPlus;

use crate::ChannelSend;
use crate::utils::ServerData;

pub struct Poller {
    details: Arc<Mutex<Option<ServerData>>>,
    stop_flag: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    state_tx: UnboundedSender<ChannelSend>,
}

const POLL_INTERVAL_MS: u64 = 1000;

impl Poller {
    pub fn new(details: Option<ServerData>, state_tx: UnboundedSender<ChannelSend>) -> Self {
        Self {
            details: Arc::new(Mutex::new(details)),
            stop_flag: Arc::new(AtomicBool::new(false)),
            handle: None,
            state_tx,
        }
    }

    pub fn start(&mut self) {
        self.stop();

        self.stop_flag.store(false, Ordering::SeqCst);

        let details = Arc::clone(&self.details);
        let stop_flag = Arc::clone(&self.stop_flag);

        let state_tx = self.state_tx.clone();
        let handle = thread::spawn(move || {
            smol::block_on(async move {
                println!("Polling thread started");
                let server_data = {
                    let d = details.lock().unwrap();
                    d.clone()
                };

                if server_data.is_none() {
                    println!("No server data found");
                    return;
                }

                let server_data = server_data.clone().unwrap();

                let rustplus = Arc::new(
                    match RustPlus::new(
                        &server_data.ip.clone(),
                        server_data.port.parse::<u16>().unwrap(),
                        server_data.player_id.parse::<u64>().unwrap(),
                        server_data.player_token.parse::<i32>().unwrap(),
                        false,
                    )
                    .await
                    {
                        Ok(client) => client,
                        Err(e) => {
                            let err_msg = format!("Failed to create RustPlus client: {}", e);
                            println!("Error: {}", err_msg);
                            // radio
                            //     .write_channel(DataChannel::ErrorStateUpdate)
                            //     .error_state = err_msg;
                            return;
                        }
                    },
                );

                if let Err(e) = rustplus.connect().await {
                    let err_msg = format!("Failed to connect to server: {}", e);
                    println!("Error: {}", err_msg);
                    // radio
                    //     .write_channel(DataChannel::ErrorStateUpdate)
                    //     .error_state = err_msg;
                    return;
                }
                println!("Connected to server");

                let get_info = rustplus.get_info();
                let get_map = rustplus.get_map();
                let get_map_markers = rustplus.get_map_markers();
                let get_team_info = rustplus.get_team_info();

                match get_info.await {
                    Ok(info) => {
                        std::fs::write("info.json", serde_json::to_string_pretty(&info).unwrap())
                            .unwrap();
                        state_tx
                            .unbounded_send(ChannelSend::InfoStateUpdate(Some(info.clone())))
                            .unwrap();
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to get map data: {}", e);
                        println!("Error: {}", err_msg);
                        // radio
                        //     .write_channel(DataChannel::ErrorStateUpdate)
                        //     .error_state = err_msg;
                    }
                }

                match get_map.await {
                    Ok(map) => {
                        std::fs::write("map.json", serde_json::to_string_pretty(&map).unwrap())
                            .unwrap();
                        state_tx
                            .unbounded_send(ChannelSend::MapStateUpdate(Some(map.clone())))
                            .unwrap();
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to get map data: {}", e);
                        println!("Error: {}", err_msg);
                        // radio
                        //     .write_channel(DataChannel::ErrorStateUpdate)
                        //     .error_state = err_msg;
                    }
                }

                match get_map_markers.await {
                    Ok(markers) => {
                        std::fs::write(
                            "markers.json",
                            serde_json::to_string_pretty(&markers).unwrap(),
                        )
                        .unwrap();
                        state_tx
                            .unbounded_send(ChannelSend::MapMarkersUpdate(Some(markers)))
                            .unwrap();
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to get map data: {}", e);
                        println!("Error: {}", err_msg);
                        // radio
                        //     .write_channel(DataChannel::ErrorStateUpdate)
                        //     .error_state = err_msg;
                    }
                }

                match get_team_info.await {
                    Ok(team_info) => {
                        std::fs::write(
                            "team_info.json",
                            serde_json::to_string_pretty(&team_info).unwrap(),
                        )
                        .unwrap();
                        state_tx
                            .unbounded_send(ChannelSend::TeamInfoUpdate(Some(team_info.clone())))
                            .unwrap();
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to get map data: {}", e);
                        println!("Error: {}", err_msg);
                        // radio
                        //     .write_channel(DataChannel::ErrorStateUpdate)
                        //     .error_state = err_msg;
                    }
                }

                while !stop_flag.load(Ordering::SeqCst) {
                    let get_map_markers = rustplus.get_map_markers();
                    let get_team_info = rustplus.get_team_info();

                    match get_map_markers.await {
                        Ok(markers) => {
                            state_tx
                                .unbounded_send(ChannelSend::MapMarkersUpdate(Some(markers)))
                                .unwrap();
                        }
                        Err(e) => {
                            let err_msg = format!("Failed to get map data: {}", e);
                            println!("Error: {}", err_msg);
                            // radio
                            //     .write_channel(DataChannel::ErrorStateUpdate)
                            //     .error_state = err_msg;
                        }
                    }
                    match get_team_info.await {
                        Ok(team) => {
                            state_tx
                                .unbounded_send(ChannelSend::TeamInfoUpdate(Some(team)))
                                .unwrap();
                        }
                        Err(e) => {
                            let err_msg = format!("Failed to get map data: {}", e);
                            println!("Error: {}", err_msg);
                            // radio
                            //     .write_channel(DataChannel::ErrorStateUpdate)
                            //     .error_state = err_msg;
                        }
                    }

                    // Sleep in small increments to check stop flag more frequently
                    let sleep_chunk = 100;
                    let mut slept = 0;
                    while slept < POLL_INTERVAL_MS && !stop_flag.load(Ordering::SeqCst) {
                        thread::sleep(Duration::from_millis(
                            sleep_chunk.min(POLL_INTERVAL_MS - slept),
                        ));
                        slept += sleep_chunk;
                    }
                }

                println!("Polling thread stopped");
            })
        });

        self.handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    /// Update connection details and restart polling
    pub fn update_details(&mut self, new_details: Option<ServerData>) {
        {
            let mut details = self.details.lock().unwrap();
            if *details == new_details {
                return; // No change, don't restart
            }
            *details = new_details;
        }

        println!("Connection details changed, restarting poller...");
        self.start(); // This stops the old thread and starts a new one
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        self.stop();
    }
}
