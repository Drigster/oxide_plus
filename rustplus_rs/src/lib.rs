//! # RustPlus RS
//! 
//! An unofficial Rust library for interacting with Smart Switches, Smart Alarms and various other things in the PC game Rust.
//! 
//! This library communicates with the Rust Game Server via WebSocket using the same protocol as the official Rust+ app.
//! 
//! ## Features
//! 
//! - Smart Switch and Smart Alarm control
//! - Team chat messaging
//! - Map and server information retrieval
//! - Camera subscription and control (CCTV, PTZ, Auto Turrets)
//! - Real-time entity state monitoring
//! - Async/await support with Tokio
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use rustplus_rs::{RustPlus, RustPlusError};
//! use tokio;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), RustPlusError> {
//!     let mut rustplus = RustPlus::new("127.0.0.1", 28082, 76561198000000000, 12345, false).await?;
//!     
//!     // Connect to the server
//!     rustplus.connect().await?;
//!     
//!     // Send a team message
//!     rustplus.send_team_message("Hello from RustPlus RS!").await?;
//!     
//!     // Turn on a smart switch
//!     rustplus.turn_smart_switch_on(1234567).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod camera;
pub mod events;
pub mod error;
pub mod proto;

// Re-export main types
pub use client::RustPlus;
pub use camera::Camera;
pub use error::{RustPlusError, Result};
pub use events::{RustPlusEvent, EventHandler};

// Re-export protobuf types
pub use proto::*;