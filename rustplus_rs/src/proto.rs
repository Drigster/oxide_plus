//! Protobuf definitions for RustPlus protocol

use prost::Message;
use serde::{Deserialize, Serialize};

// Vector types
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Vector2 {
    #[prost(float, optional, tag = "1")]
    pub x: Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: Option<f32>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Vector3 {
    #[prost(float, optional, tag = "1")]
    pub x: Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: Option<f32>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Vector4 {
    #[prost(float, optional, tag = "1")]
    pub x: Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub y: Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub z: Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub w: Option<f32>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct Color {
    #[prost(float, optional, tag = "1")]
    pub r: Option<f32>,
    #[prost(float, optional, tag = "2")]
    pub g: Option<f32>,
    #[prost(float, optional, tag = "3")]
    pub b: Option<f32>,
    #[prost(float, optional, tag = "4")]
    pub a: Option<f32>,
}

// Enums
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AppEntityType {
    Switch = 1,
    Alarm = 2,
    StorageMonitor = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AppMarkerType {
    Undefined = 0,
    Player = 1,
    Explosion = 2,
    VendingMachine = 3,
    Ch47 = 4,
    CargoShip = 5,
    Crate = 6,
    GenericRadius = 7,
    PatrolHelicopter = 8,
}

// Empty message
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppEmpty {}

// Request messages
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppSendMessage {
    #[prost(string, required, tag = "1")]
    pub message: String,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppSetEntityValue {
    #[prost(bool, required, tag = "1")]
    pub value: bool,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppPromoteToLeader {
    #[prost(uint64, required, tag = "1")]
    pub steam_id: u64,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppGetNexusAuth {
    #[prost(string, required, tag = "1")]
    pub app_key: String,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppFlag {
    #[prost(bool, required, tag = "1")]
    pub value: bool,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppCameraSubscribe {
    #[prost(string, required, tag = "1")]
    pub camera_id: String,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppCameraInput {
    #[prost(int32, required, tag = "1")]
    pub buttons: i32,
    #[prost(message, required, tag = "2")]
    pub mouse_delta: Vector2,
}

// Main request message
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppRequest {
    #[prost(uint32, required, tag = "1")]
    pub seq: u32,
    #[prost(uint64, required, tag = "2")]
    pub player_id: u64,
    #[prost(int32, required, tag = "3")]
    pub player_token: i32,
    #[prost(uint32, optional, tag = "4")]
    pub entity_id: Option<u32>,
    #[prost(message, optional, tag = "8")]
    pub get_info: Option<AppEmpty>,
    #[prost(message, optional, tag = "9")]
    pub get_time: Option<AppEmpty>,
    #[prost(message, optional, tag = "10")]
    pub get_map: Option<AppEmpty>,
    #[prost(message, optional, tag = "11")]
    pub get_team_info: Option<AppEmpty>,
    #[prost(message, optional, tag = "12")]
    pub get_team_chat: Option<AppEmpty>,
    #[prost(message, optional, tag = "13")]
    pub send_team_message: Option<AppSendMessage>,
    #[prost(message, optional, tag = "14")]
    pub get_entity_info: Option<AppEmpty>,
    #[prost(message, optional, tag = "15")]
    pub set_entity_value: Option<AppSetEntityValue>,
    #[prost(message, optional, tag = "16")]
    pub check_subscription: Option<AppEmpty>,
    #[prost(message, optional, tag = "17")]
    pub set_subscription: Option<AppFlag>,
    #[prost(message, optional, tag = "18")]
    pub get_map_markers: Option<AppEmpty>,
    #[prost(message, optional, tag = "20")]
    pub promote_to_leader: Option<AppPromoteToLeader>,
    #[prost(message, optional, tag = "21")]
    pub get_clan_info: Option<AppEmpty>,
    #[prost(message, optional, tag = "22")]
    pub set_clan_motd: Option<AppSendMessage>,
    #[prost(message, optional, tag = "23")]
    pub get_clan_chat: Option<AppEmpty>,
    #[prost(message, optional, tag = "24")]
    pub send_clan_message: Option<AppSendMessage>,
    #[prost(message, optional, tag = "25")]
    pub get_nexus_auth: Option<AppGetNexusAuth>,
    #[prost(message, optional, tag = "30")]
    pub camera_subscribe: Option<AppCameraSubscribe>,
    #[prost(message, optional, tag = "31")]
    pub camera_unsubscribe: Option<AppEmpty>,
    #[prost(message, optional, tag = "32")]
    pub camera_input: Option<AppCameraInput>,
}

// Response messages
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppSuccess {}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppError {
    #[prost(string, required, tag = "1")]
    pub error: String,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppInfo {
    #[prost(string, required, tag = "1")]
    pub name: String,
    #[prost(string, required, tag = "2")]
    pub header_image: String,
    #[prost(string, required, tag = "3")]
    pub url: String,
    #[prost(string, required, tag = "4")]
    pub map: String,
    #[prost(uint32, required, tag = "5")]
    pub map_size: u32,
    #[prost(uint32, required, tag = "6")]
    pub wipe_time: u32,
    #[prost(uint32, required, tag = "7")]
    pub players: u32,
    #[prost(uint32, required, tag = "8")]
    pub max_players: u32,
    #[prost(uint32, required, tag = "9")]
    pub queued_players: u32,
    #[prost(uint32, optional, tag = "10")]
    pub seed: Option<u32>,
    #[prost(uint32, optional, tag = "11")]
    pub salt: Option<u32>,
    #[prost(string, optional, tag = "12")]
    pub logo_image: Option<String>,
    #[prost(string, optional, tag = "13")]
    pub nexus: Option<String>,
    #[prost(int32, optional, tag = "14")]
    pub nexus_id: Option<i32>,
    #[prost(string, optional, tag = "15")]
    pub nexus_zone: Option<String>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTime {
    #[prost(float, required, tag = "1")]
    pub day_length_minutes: f32,
    #[prost(float, required, tag = "2")]
    pub time_scale: f32,
    #[prost(float, required, tag = "3")]
    pub sunrise: f32,
    #[prost(float, required, tag = "4")]
    pub sunset: f32,
    #[prost(float, required, tag = "5")]
    pub time: f32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppMap {
    #[prost(uint32, required, tag = "1")]
    pub width: u32,
    #[prost(uint32, required, tag = "2")]
    pub height: u32,
    #[prost(bytes, required, tag = "3")]
    pub jpg_image: Vec<u8>,
    #[prost(int32, required, tag = "4")]
    pub ocean_margin: i32,
    #[prost(message, repeated, tag = "5")]
    pub monuments: Vec<AppMapMonument>,
    #[prost(string, optional, tag = "6")]
    pub background: Option<String>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppMapMonument {
    #[prost(string, required, tag = "1")]
    pub token: String,
    #[prost(float, required, tag = "2")]
    pub x: f32,
    #[prost(float, required, tag = "3")]
    pub y: f32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppEntityInfo {
    #[prost(enumeration = "AppEntityType", required, tag = "1")]
    pub entity_type: i32,
    #[prost(message, required, tag = "3")]
    pub payload: AppEntityPayload,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppEntityPayload {
    #[prost(bool, optional, tag = "1")]
    pub value: Option<bool>,
    #[prost(message, repeated, tag = "2")]
    pub items: Vec<AppEntityPayloadItem>,
    #[prost(int32, optional, tag = "3")]
    pub capacity: Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub has_protection: Option<bool>,
    #[prost(uint32, optional, tag = "5")]
    pub protection_expiry: Option<u32>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppEntityPayloadItem {
    #[prost(int32, required, tag = "1")]
    pub item_id: i32,
    #[prost(int32, required, tag = "2")]
    pub quantity: i32,
    #[prost(bool, required, tag = "3")]
    pub item_is_blueprint: bool,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTeamInfo {
    #[prost(uint64, required, tag = "1")]
    pub leader_steam_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub members: Vec<AppTeamInfoMember>,
    #[prost(message, repeated, tag = "3")]
    pub map_notes: Vec<AppTeamInfoNote>,
    #[prost(message, repeated, tag = "4")]
    pub leader_map_notes: Vec<AppTeamInfoNote>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTeamInfoMember {
    #[prost(uint64, required, tag = "1")]
    pub steam_id: u64,
    #[prost(string, required, tag = "2")]
    pub name: String,
    #[prost(float, required, tag = "3")]
    pub x: f32,
    #[prost(float, required, tag = "4")]
    pub y: f32,
    #[prost(bool, required, tag = "5")]
    pub is_online: bool,
    #[prost(uint32, required, tag = "6")]
    pub spawn_time: u32,
    #[prost(bool, required, tag = "7")]
    pub is_alive: bool,
    #[prost(uint32, required, tag = "8")]
    pub death_time: u32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTeamInfoNote {
    #[prost(int32, required, tag = "2")]
    pub note_type: i32,
    #[prost(float, required, tag = "3")]
    pub x: f32,
    #[prost(float, required, tag = "4")]
    pub y: f32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTeamMessage {
    #[prost(uint64, required, tag = "1")]
    pub steam_id: u64,
    #[prost(string, required, tag = "2")]
    pub name: String,
    #[prost(string, required, tag = "3")]
    pub message: String,
    #[prost(string, required, tag = "4")]
    pub color: String,
    #[prost(uint32, required, tag = "5")]
    pub time: u32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTeamChat {
    #[prost(message, repeated, tag = "1")]
    pub messages: Vec<AppTeamMessage>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppMarker {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(enumeration = "AppMarkerType", required, tag = "2")]
    pub marker_type: i32,
    #[prost(float, required, tag = "3")]
    pub x: f32,
    #[prost(float, required, tag = "4")]
    pub y: f32,
    #[prost(uint64, optional, tag = "5")]
    pub steam_id: Option<u64>,
    #[prost(float, optional, tag = "6")]
    pub rotation: Option<f32>,
    #[prost(float, optional, tag = "7")]
    pub radius: Option<f32>,
    #[prost(message, optional, tag = "8")]
    pub color1: Option<Vector4>,
    #[prost(message, optional, tag = "9")]
    pub color2: Option<Vector4>,
    #[prost(float, optional, tag = "10")]
    pub alpha: Option<f32>,
    #[prost(string, optional, tag = "11")]
    pub name: Option<String>,
    #[prost(bool, optional, tag = "12")]
    pub out_of_stock: Option<bool>,
    #[prost(message, repeated, tag = "13")]
    pub sell_orders: Vec<AppMarkerSellOrder>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppMarkerSellOrder {
    #[prost(int32, required, tag = "1")]
    pub item_id: i32,
    #[prost(int32, required, tag = "2")]
    pub quantity: i32,
    #[prost(int32, required, tag = "3")]
    pub currency_id: i32,
    #[prost(int32, required, tag = "4")]
    pub cost_per_item: i32,
    #[prost(int32, required, tag = "5")]
    pub amount_in_stock: i32,
    #[prost(bool, required, tag = "6")]
    pub item_is_blueprint: bool,
    #[prost(bool, required, tag = "7")]
    pub currency_is_blueprint: bool,
    #[prost(float, optional, tag = "8")]
    pub item_condition: Option<f32>,
    #[prost(float, optional, tag = "9")]
    pub item_condition_max: Option<f32>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppMapMarkers {
    #[prost(message, repeated, tag = "1")]
    pub markers: Vec<AppMarker>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppCameraInfo {
    #[prost(int32, required, tag = "1")]
    pub width: i32,
    #[prost(int32, required, tag = "2")]
    pub height: i32,
    #[prost(float, required, tag = "3")]
    pub near_plane: f32,
    #[prost(float, required, tag = "4")]
    pub far_plane: f32,
    #[prost(int32, required, tag = "5")]
    pub control_flags: i32,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppCameraRays {
    #[prost(float, required, tag = "1")]
    pub vertical_fov: f32,
    #[prost(int32, required, tag = "2")]
    pub sample_offset: i32,
    #[prost(bytes, required, tag = "3")]
    pub ray_data: Vec<u8>,
    #[prost(float, required, tag = "4")]
    pub distance: f32,
    #[prost(message, repeated, tag = "5")]
    pub entities: Vec<AppCameraRaysEntity>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppCameraRaysEntity {
    #[prost(uint32, required, tag = "1")]
    pub entity_id: u32,
    #[prost(enumeration = "AppCameraRaysEntityType", required, tag = "2")]
    pub entity_type: i32,
    #[prost(message, required, tag = "3")]
    pub position: Vector3,
    #[prost(message, required, tag = "4")]
    pub rotation: Vector3,
    #[prost(message, required, tag = "5")]
    pub size: Vector3,
    #[prost(string, optional, tag = "6")]
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AppCameraRaysEntityType {
    Tree = 1,
    Player = 2,
}

// Main response message
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppResponse {
    #[prost(uint32, required, tag = "1")]
    pub seq: u32,
    #[prost(message, optional, tag = "4")]
    pub success: Option<AppSuccess>,
    #[prost(message, optional, tag = "5")]
    pub error: Option<AppError>,
    #[prost(message, optional, tag = "6")]
    pub info: Option<AppInfo>,
    #[prost(message, optional, tag = "7")]
    pub time: Option<AppTime>,
    #[prost(message, optional, tag = "8")]
    pub map: Option<AppMap>,
    #[prost(message, optional, tag = "9")]
    pub team_info: Option<AppTeamInfo>,
    #[prost(message, optional, tag = "10")]
    pub team_chat: Option<AppTeamChat>,
    #[prost(message, optional, tag = "11")]
    pub entity_info: Option<AppEntityInfo>,
    #[prost(message, optional, tag = "12")]
    pub flag: Option<AppFlag>,
    #[prost(message, optional, tag = "13")]
    pub map_markers: Option<AppMapMarkers>,
    #[prost(message, optional, tag = "20")]
    pub camera_subscribe_info: Option<AppCameraInfo>,
}

// Broadcast messages
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppEntityChanged {
    #[prost(uint32, required, tag = "1")]
    pub entity_id: u32,
    #[prost(message, required, tag = "2")]
    pub payload: AppEntityPayload,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppNewTeamMessage {
    #[prost(message, required, tag = "1")]
    pub message: AppTeamMessage,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppBroadcast {
    #[prost(message, optional, tag = "4")]
    pub team_changed: Option<AppTeamChanged>,
    #[prost(message, optional, tag = "5")]
    pub team_message: Option<AppNewTeamMessage>,
    #[prost(message, optional, tag = "6")]
    pub entity_changed: Option<AppEntityChanged>,
    #[prost(message, optional, tag = "10")]
    pub camera_rays: Option<AppCameraRays>,
}

#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppTeamChanged {
    #[prost(uint64, required, tag = "1")]
    pub player_id: u64,
    #[prost(message, required, tag = "2")]
    pub team_info: AppTeamInfo,
}

// Main message wrapper
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct AppMessage {
    #[prost(message, optional, tag = "1")]
    pub response: Option<AppResponse>,
    #[prost(message, optional, tag = "2")]
    pub broadcast: Option<AppBroadcast>,
}
