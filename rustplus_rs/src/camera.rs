use crate::client::RustPlus;
use crate::error::{RustPlusError, Result};
use crate::events::{RustPlusEvent, EventBroadcaster};
use crate::proto::*;
use image::{ImageBuffer, Rgb, RgbImage};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};

/// Camera control for CCTV, PTZ cameras, and auto turrets
#[derive(Debug)]
pub struct Camera {
    rustplus: Arc<RustPlus>,
    identifier: String,
    is_subscribed: Arc<Mutex<bool>>,
    camera_rays: Arc<Mutex<Vec<AppCameraRays>>>,
    camera_subscribe_info: Arc<Mutex<Option<AppCameraInfo>>>,
    event_broadcaster: EventBroadcaster,
}

/// Camera button constants
pub mod buttons {
    pub const NONE: i32 = 0;
    pub const FORWARD: i32 = 2;
    pub const BACKWARD: i32 = 4;
    pub const LEFT: i32 = 8;
    pub const RIGHT: i32 = 16;
    pub const JUMP: i32 = 32;
    pub const DUCK: i32 = 64;
    pub const SPRINT: i32 = 128;
    pub const USE: i32 = 256;
    pub const FIRE_PRIMARY: i32 = 1024;
    pub const FIRE_SECONDARY: i32 = 2048;
    pub const RELOAD: i32 = 8192;
    pub const FIRE_THIRD: i32 = 134217728;
}

/// Camera control flags
pub mod control_flags {
    pub const NONE: i32 = 0;
    pub const MOVEMENT: i32 = 1;
    pub const MOUSE: i32 = 2;
    pub const SPRINT_AND_DUCK: i32 = 4;
    pub const FIRE: i32 = 8;
    pub const RELOAD: i32 = 16;
    pub const CROSSHAIR: i32 = 32;
}

impl Camera {
    /// Create a new camera instance
    pub fn new(rustplus: Arc<RustPlus>, identifier: &str) -> Self {
        Self {
            rustplus,
            identifier: identifier.to_string(),
            is_subscribed: Arc::new(Mutex::new(false)),
            camera_rays: Arc::new(Mutex::new(Vec::new())),
            camera_subscribe_info: Arc::new(Mutex::new(None)),
            event_broadcaster: EventBroadcaster::new(100),
        }
    }
    
    /// Get the camera identifier
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
    
    /// Check if the camera is subscribed
    pub async fn is_subscribed(&self) -> bool {
        *self.is_subscribed.lock().await
    }
    
    /// Get event broadcaster for camera events
    pub fn event_broadcaster(&self) -> EventBroadcaster {
        self.event_broadcaster.clone()
    }
    
    /// Subscribe to the camera
    pub async fn subscribe(&self) -> Result<()> {
        if self.is_subscribed().await {
            return Ok(());
        }
        
        self.event_broadcaster.broadcast(RustPlusEvent::Connecting)?;
        
        // Subscribe to camera
        let camera_info = self.rustplus.subscribe_to_camera(&self.identifier).await?;
        
        // Update state
        *self.camera_subscribe_info.lock().await = Some(camera_info);
        *self.is_subscribed.lock().await = true;
        
        self.event_broadcaster.broadcast(RustPlusEvent::Connected)?;
        
        // Start auto-resubscribe task
        self.start_auto_resubscribe().await;
        
        Ok(())
    }
    
    /// Unsubscribe from the camera
    pub async fn unsubscribe(&self) -> Result<()> {
        if !self.is_subscribed().await {
            return Ok(());
        }
        
        self.event_broadcaster.broadcast(RustPlusEvent::Disconnected)?;
        
        // Update state
        *self.is_subscribed.lock().await = false;
        *self.camera_rays.lock().await = Vec::new();
        *self.camera_subscribe_info.lock().await = None;
        
        // Unsubscribe from server if connected
        if self.rustplus.is_connected().await {
            let _ = self.rustplus.unsubscribe_from_camera().await;
        }
        
        Ok(())
    }
    
    /// Move the camera (mouse movement)
    pub async fn move_camera(&self, buttons: i32, x: f32, y: f32) -> Result<()> {
        if !self.is_subscribed().await {
            return Err(RustPlusError::CameraNotSubscribed);
        }
        
        let mouse_delta = Vector2 { x: Some(x), y: Some(y) };
        self.rustplus.send_camera_input(buttons, mouse_delta).await
    }
    
    /// Zoom the camera (for PTZ cameras)
    pub async fn zoom(&self) -> Result<()> {
        // Press left mouse button to zoom in
        self.move_camera(buttons::FIRE_PRIMARY, 0.0, 0.0).await?;
        
        // Release all buttons
        self.move_camera(buttons::NONE, 0.0, 0.0).await
    }
    
    /// Shoot (for auto turrets)
    pub async fn shoot(&self) -> Result<()> {
        // Press left mouse button to shoot
        self.move_camera(buttons::FIRE_PRIMARY, 0.0, 0.0).await?;
        
        // Release all buttons
        self.move_camera(buttons::NONE, 0.0, 0.0).await
    }
    
    /// Reload (for auto turrets)
    pub async fn reload(&self) -> Result<()> {
        // Press reload button
        self.move_camera(buttons::RELOAD, 0.0, 0.0).await?;
        
        // Release all buttons
        self.move_camera(buttons::NONE, 0.0, 0.0).await
    }
    
    /// Check if this camera is an auto turret
    pub async fn is_auto_turret(&self) -> bool {
        if let Some(info) = self.camera_subscribe_info.lock().await.as_ref() {
            (info.control_flags & control_flags::CROSSHAIR) == control_flags::CROSSHAIR
        } else {
            false
        }
    }
    
    /// Process camera rays and render frame
    pub async fn process_camera_rays(&self, camera_rays: AppCameraRays) -> Result<()> {
        if !self.is_subscribed().await {
            return Ok(());
        }
        
        let mut rays = self.camera_rays.lock().await;
        rays.push(camera_rays);
        
        // Keep only the last 10 frames
        if rays.len() > 10 {
            rays.remove(0);
        }
        
        // Render frame if we have enough data
        if rays.len() > 5 {
            if let Some(info) = self.camera_subscribe_info.lock().await.as_ref() {
                let _frame = self.render_camera_frame(&rays, info.width, info.height).await?;
                
                // Emit render event
                self.event_broadcaster.broadcast(RustPlusEvent::Message(AppMessage {
                    response: None,
                    broadcast: None,
                }))?;
            }
        }
        
        Ok(())
    }
    
    /// Render camera frame to PNG
    async fn render_camera_frame(&self, _frames: &[AppCameraRays], width: i32, height: i32) -> Result<Vec<u8>> {
        // This is a simplified version of the camera rendering logic
        // The full implementation would need to decode the ray data similar to the JS version
        
        let mut image: RgbImage = ImageBuffer::new(width as u32, height as u32);
        
        // Fill with a placeholder color for now
        for pixel in image.pixels_mut() {
            *pixel = Rgb([128, 128, 128]); // Gray placeholder
        }
        
        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut png_bytes);
            image.write_to(&mut cursor, image::ImageOutputFormat::Png)
                .map_err(|e| RustPlusError::ImageProcessing(e.to_string()))?;
        }
        
        Ok(png_bytes)
    }
    
    /// Start auto-resubscribe task
    async fn start_auto_resubscribe(&self) {
        let rustplus = Arc::clone(&self.rustplus);
        let identifier = self.identifier.clone();
        let is_subscribed = Arc::clone(&self.is_subscribed);
        let camera_subscribe_info = Arc::clone(&self.camera_subscribe_info);
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            
            loop {
                interval.tick().await;
                
                if *is_subscribed.lock().await {
                    // Resubscribe to keep connection alive
                    if let Ok(camera_info) = rustplus.subscribe_to_camera(&identifier).await {
                        *camera_subscribe_info.lock().await = Some(camera_info);
                    }
                } else {
                    break;
                }
            }
        });
    }
}
