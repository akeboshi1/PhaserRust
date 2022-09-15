// pyr

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSynchroShop {
    #[prost(message, required, tag="1")]
    pub shop: super::op_gameconfig::Shop,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientLuaLog {
    #[prost(string, required, tag="1")]
    pub log_context: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientUiBindAttribute {
    #[prost(message, repeated, tag="1")]
    pub slot: ::prost::alloc::vec::Vec<super::op_gameconfig::Slot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpClientPing {
    #[prost(string, required, tag="1")]
    pub msg: ::prost::alloc::string::String,
    #[prost(enumeration="super::op_def::ResponseStatus", required, tag="2")]
    pub rs: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayResClientStartNewWorld {
    #[prost(string, optional, tag="1")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayResClientError {
    #[prost(string, required, tag="1")]
    pub msg: ::prost::alloc::string::String,
    #[prost(enumeration="super::op_def::ResponseStatus", required, tag="2")]
    pub response_status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientGameOver {
    #[prost(string, required, tag="1")]
    pub msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientChangeScene {
    #[prost(message, required, tag="1")]
    pub scene: Scene,
    #[prost(message, required, tag="2")]
    pub actor: Actor,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSynchroPackage {
    #[prost(message, required, tag="1")]
    pub package: super::op_gameconfig::Package,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientAddItem {
    #[prost(message, repeated, tag="1")]
    pub item: ::prost::alloc::vec::Vec<super::op_gameconfig::Item>,
    #[prost(int32, required, tag="2")]
    pub packageid: i32,
    #[prost(int32, required, tag="3")]
    pub id: i32,
    #[prost(enumeration="super::op_def::NodeType", required, tag="4")]
    pub nodetype: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientRemoveItem {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(int32, required, tag="2")]
    pub packageid: i32,
    #[prost(int32, repeated, packed="false", tag="3")]
    pub item_id: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::op_def::NodeType", required, tag="4")]
    pub nodetype: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientExchangeItemPos {
    #[prost(int32, optional, tag="1")]
    pub first_pos: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub first_item: ::core::option::Option<super::op_gameconfig::Item>,
    #[prost(int32, optional, tag="3")]
    pub second_pos: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub second_item: ::core::option::Option<super::op_gameconfig::Item>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientTargetUi {
    #[prost(int32, required, tag="1")]
    pub ui_id: i32,
    #[prost(int32, required, tag="2")]
    pub component_id: i32,
    #[prost(int32, repeated, packed="false", tag="3")]
    pub data: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePairs {
    #[prost(string, required, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientShowUi {
    #[prost(message, repeated, tag="1")]
    pub text: ::prost::alloc::vec::Vec<super::op_gameconfig_01::Text>,
    #[prost(message, repeated, tag="3")]
    pub button: ::prost::alloc::vec::Vec<super::op_gameconfig_01::Button>,
    #[prost(int32, required, tag="4")]
    pub id: i32,
    #[prost(int32, repeated, packed="false", tag="5")]
    pub time: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, required, tag="6")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, repeated, packed="false", tag="7")]
    pub data: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="8")]
    pub actors: ::prost::alloc::vec::Vec<Actor>,
    #[prost(message, repeated, tag="9")]
    pub kvps: ::prost::alloc::vec::Vec<KeyValuePairs>,
    #[prost(message, repeated, tag="10")]
    pub menu_item: ::prost::alloc::vec::Vec<super::op_gameconfig_01::MenuItem>,
    #[prost(message, repeated, tag="11")]
    pub display: ::prost::alloc::vec::Vec<super::op_gameconfig_01::Display>,
    #[prost(message, repeated, tag="12")]
    pub input_text: ::prost::alloc::vec::Vec<super::op_gameconfig_01::InputText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientUpdateUi {
    #[prost(message, repeated, tag="1")]
    pub text: ::prost::alloc::vec::Vec<super::op_gameconfig_01::Text>,
    #[prost(message, repeated, tag="3")]
    pub button: ::prost::alloc::vec::Vec<super::op_gameconfig_01::Button>,
    #[prost(int32, required, tag="4")]
    pub id: i32,
    #[prost(int32, repeated, packed="false", tag="5")]
    pub time: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, required, tag="6")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, repeated, packed="false", tag="7")]
    pub data: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="8")]
    pub actors: ::prost::alloc::vec::Vec<Actor>,
    #[prost(message, repeated, tag="9")]
    pub kvps: ::prost::alloc::vec::Vec<KeyValuePairs>,
    #[prost(message, repeated, tag="10")]
    pub menu_item: ::prost::alloc::vec::Vec<super::op_gameconfig_01::MenuItem>,
    #[prost(message, repeated, tag="11")]
    pub display: ::prost::alloc::vec::Vec<super::op_gameconfig_01::Display>,
    #[prost(message, repeated, tag="12")]
    pub input_text: ::prost::alloc::vec::Vec<super::op_gameconfig_01::InputText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientCloseUi {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
// ///////////////////////////////////////////////
// virtual world message 0
// //////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveData {
    #[prost(int32, required, tag="1")]
    pub move_object_id: i32,
    #[prost(enumeration="super::op_def::MoveType", required, tag="2")]
    pub move_type: i32,
    #[prost(enumeration="super::op_def::Direction", required, tag="3")]
    pub direction: i32,
    /// 终点坐标
    #[prost(message, required, tag="4")]
    pub destination_point3f: super::op_def::PbPoint3f,
    /// 时间间隔，根据发包间隔外加预计网络延时决定
    #[prost(int32, required, tag="5")]
    pub time_span: i32,
    #[prost(double, optional, tag="6")]
    pub timestemp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayReqClientMoveCharacter {
    #[prost(message, repeated, tag="1")]
    pub move_data: ::prost::alloc::vec::Vec<MoveData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayReqClientMoveElement {
    #[prost(message, repeated, tag="1")]
    pub move_data: ::prost::alloc::vec::Vec<MoveData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MovePosition {
    #[prost(int32, required, tag="1")]
    pub move_object_id: i32,
    #[prost(enumeration="super::op_def::MoveType", required, tag="2")]
    pub move_type: i32,
    #[prost(enumeration="super::op_def::Direction", required, tag="3")]
    pub direction: i32,
    /// 终点坐标
    #[prost(message, required, tag="4")]
    pub destination_point3f: super::op_def::PbPoint3f,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayReqClientSetCharacterPosition {
    #[prost(message, repeated, tag="1")]
    pub move_postion: ::prost::alloc::vec::Vec<MovePosition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayReqClientSetElementPosition {
    #[prost(message, repeated, tag="1")]
    pub move_postion: ::prost::alloc::vec::Vec<MovePosition>,
}
// message OP_CLIENT_REQ_GATEWAY_SYNC_PLAYER_LOCATION
// {
//     required Point3f point3f = 1;
// }
//
// message OP_GATEWAY_RES_CLIENT_SYNC_PLAYER_LOCATION
// {
//     required Point3f point3f = 1;
// }

/// 聊天文本设置
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatSetting {
    #[prost(string, optional, tag="1")]
    pub text_color: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub text_fontsize: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub text_pattern: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub duration: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub bubble_resource: ::core::option::Option<::prost::alloc::string::String>,
}
/// 聊天气泡
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatBubble {
    #[prost(int32, required, tag="1")]
    pub pattern: i32,
    #[prost(message, optional, tag="2")]
    pub chatsetting: ::core::option::Option<ChatSetting>,
}
/// 聊天消息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientChat {
    #[prost(enumeration="super::op_def::ChatChannel", required, tag="1")]
    pub chat_channel: i32,
    #[prost(int32, optional, tag="2")]
    pub chat_otheractor: ::core::option::Option<i32>,
    #[prost(string, required, tag="3")]
    pub chat_context: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="4")]
    pub chat_senderid: ::core::option::Option<i32>,
    #[prost(message, optional, tag="5")]
    pub chat_bubble: ::core::option::Option<ChatBubble>,
    #[prost(message, optional, tag="6")]
    pub chat_setting: ::core::option::Option<ChatSetting>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientNotice {
    #[prost(string, required, tag="1")]
    pub notice_context: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub chatsetting: ::core::option::Option<ChatSetting>,
}
/// /只有气泡的消息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientOnlyBubble {
    #[prost(string, required, tag="1")]
    pub context: ::prost::alloc::string::String,
    #[prost(int32, required, tag="2")]
    pub receiverid: i32,
    #[prost(message, optional, tag="3")]
    pub chatsetting: ::core::option::Option<ChatSetting>,
}
/// /只有气泡的清除消息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientOnlyBubbleClean {
    #[prost(int32, required, tag="1")]
    pub receiverid: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortcutKey {
    #[prost(int32, optional, tag="1")]
    pub keycode: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub bindtype: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actor {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    ///   类型最大人数
    #[prost(int32, optional, tag="3", default="20")]
    pub max_num: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub camp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    #[prost(message, repeated, tag="6")]
    pub attributes: ::prost::alloc::vec::Vec<super::op_gameconfig::Attribute>,
    /// Item Container, N item in package
    #[prost(message, optional, tag="7")]
    pub package: ::core::option::Option<super::op_gameconfig::Package>,
    #[prost(int32, optional, tag="8")]
    pub scene_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub z: ::core::option::Option<i32>,
    /// 美术资源方向 与人物初始方向不同， 人物一般固定为\[1,3,5,7\] 四个方向中的一个
    #[prost(int32, optional, tag="13")]
    pub avatar_dir: ::core::option::Option<i32>,
    #[prost(string, optional, tag="14")]
    pub walkable_area: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub collision_area: ::core::option::Option<::prost::alloc::string::String>,
    /// default 0,0, set position on collision and walkable area
    #[prost(int32, repeated, packed="false", tag="15")]
    pub origin_point: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed="false", tag="16")]
    pub walk_origin_point: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag="17")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="18")]
    pub slot: ::prost::alloc::vec::Vec<super::op_gameconfig::Slot>,
    #[prost(int32, required, tag="19")]
    pub uuid: i32,
    #[prost(bool, optional, tag="20")]
    pub is_in_current_scene_voice_room: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="21")]
    pub display_badge_cards: ::prost::alloc::vec::Vec<super::op_def::BadgeCard>,
    #[prost(string, optional, tag="22")]
    pub platform_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag="23")]
    pub speed: ::core::option::Option<f32>,
    /// op_def.UserInput 0x00000000 表示不监听任何input
    #[prost(uint32, optional, tag="24", default="4294967295")]
    pub input_mask: ::core::option::Option<u32>,
    #[prost(bool, optional, tag="25", default="false")]
    pub immovable: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scene {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(float, required, tag="2")]
    pub cols: f32,
    #[prost(float, required, tag="3")]
    pub rows: f32,
    #[prost(int32, required, tag="4")]
    pub tile_width: i32,
    #[prost(int32, required, tag="5")]
    pub tile_height: i32,
    #[prost(float, optional, tag="6", default="0")]
    pub z_start: ::core::option::Option<f32>,
    #[prost(float, optional, tag="7", default="0")]
    pub z_end: ::core::option::Option<f32>,
    #[prost(message, repeated, tag="8")]
    pub elements: ::prost::alloc::vec::Vec<Element>,
    #[prost(message, repeated, tag="9")]
    pub terrains: ::prost::alloc::vec::Vec<Terrain>,
    #[prost(int32, optional, tag="10")]
    pub voice_chat_room_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="11")]
    pub shops: ::prost::alloc::vec::Vec<super::op_gameconfig::Shop>,
    #[prost(enumeration="super::op_def::SceneTypeEnum", optional, tag="12")]
    pub scene_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Element {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(string, required, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// 方向
    #[prost(int32, required, tag="3")]
    pub dir: i32,
    #[prost(float, required, tag="4")]
    pub x: f32,
    #[prost(float, required, tag="5")]
    pub y: f32,
    #[prost(float, required, tag="6", default="0")]
    pub z: f32,
    #[prost(string, optional, tag="7")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="15")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(message, repeated, tag="16")]
    pub attributes: ::prost::alloc::vec::Vec<super::op_gameconfig::Attribute>,
    /// 当前播放的animation
    #[prost(string, optional, tag="21")]
    pub animation_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="22")]
    pub scale: ::core::option::Option<bool>,
    #[prost(message, optional, tag="23")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(message, optional, tag="24")]
    pub package: ::core::option::Option<super::op_gameconfig::Package>,
    #[prost(message, repeated, tag="25")]
    pub shops: ::prost::alloc::vec::Vec<super::op_gameconfig::Shop>,
}
/// 选择角色
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSelectCharacter {
    #[prost(int32, required, tag="2")]
    pub select_character_id: i32,
}
/// scene init data to client
/// actorId is uuid
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEnterScene {
    #[prost(message, required, tag="1")]
    pub actor: Actor,
    #[prost(message, required, tag="2")]
    pub scene: Scene,
    #[prost(message, repeated, tag="3")]
    pub terrain_types: ::prost::alloc::vec::Vec<super::op_gameconfig::TerrainType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientSceneTransitions {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeAnimation {
    #[prost(string, required, tag="1")]
    pub animation_name: ::prost::alloc::string::String,
    /// 是否翻转
    #[prost(bool, optional, tag="2")]
    pub scale: ::core::option::Option<bool>,
    /// 播放次数 可以不填 按编辑器内设置播放
    #[prost(int32, optional, tag="3")]
    pub times: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpGatewayResClientVirtualWorldInit {
    #[prost(message, repeated, tag="1")]
    pub terrain_types: ::prost::alloc::vec::Vec<super::op_gameconfig::TerrainType>,
    #[prost(message, repeated, tag="2")]
    pub element_types: ::prost::alloc::vec::Vec<super::op_gameconfig::ElementType>,
    #[prost(message, repeated, tag="3")]
    pub item_types: ::prost::alloc::vec::Vec<super::op_gameconfig::ItemType>,
    #[prost(message, repeated, tag="4")]
    pub avatar_backbone: ::prost::alloc::vec::Vec<super::op_gameconfig::AvatarBackbone>,
    #[prost(string, repeated, tag="5")]
    pub resource_root: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub shops: ::prost::alloc::vec::Vec<super::op_gameconfig::Shop>,
    #[prost(string, repeated, tag="7")]
    pub config_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="8")]
    pub key_events: ::prost::alloc::vec::Vec<super::op_def::KeyCodeEvent>,
    #[prost(string, optional, tag="9")]
    pub virtual_world_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="super::op_def::MoveStyle", optional, tag="10", default="DirectionMoveStyle")]
    pub move_style: ::core::option::Option<i32>,
    /// 平台游戏app key
    #[prost(string, optional, tag="11")]
    pub app_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientCharacterTalking {
    #[prost(int32, required, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientCharacterShutUp {
    #[prost(int32, required, tag="1")]
    pub id: i32,
}
/// add element
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientAddElement {
    #[prost(int32, required, tag="1")]
    pub sceneid: i32,
    #[prost(message, repeated, tag="2")]
    pub elements: ::prost::alloc::vec::Vec<Element>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientRemoveElement {
    #[prost(int32, required, tag="1")]
    pub sceneid: i32,
    #[prost(int32, required, tag="2")]
    pub elementid: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSynchroCharacter {
    #[prost(message, repeated, tag="1")]
    pub actors: ::prost::alloc::vec::Vec<Actor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientAddCharacter {
    #[prost(message, repeated, tag="1")]
    pub actors: ::prost::alloc::vec::Vec<Actor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientRemoveCharacter {
    #[prost(int32, required, tag="1")]
    pub uuid: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientAddTerrain {
    #[prost(int32, required, tag="1")]
    pub sceneid: i32,
    #[prost(message, repeated, tag="2")]
    pub terrain: ::prost::alloc::vec::Vec<Terrain>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVitualWorldResClientAfterCollision {
    #[prost(int32, required, tag="1")]
    pub collision_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientQcloudGmeAuthbuffer {
    /// qcloud sig with tea encrpy than encode base64
    #[prost(string, required, tag="1")]
    pub signature: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientQueryPackage {
    /// shop id or package id
    #[prost(int32, required, tag="1")]
    pub id: i32,
    /// if gt than max_page return empty items
    #[prost(int32, optional, tag="2")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3", default="10")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub max_page: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="5")]
    pub items: ::prost::alloc::vec::Vec<super::op_gameconfig::Item>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientShowEffect {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub id: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSyncTime {
    #[prost(double, required, tag="1")]
    pub client_start_ts: f64,
    #[prost(double, required, tag="2")]
    pub server_receive_ts: f64,
    #[prost(double, required, tag="3")]
    pub server_send_ts: f64,
}
// pyr end

// editor

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientSetEditorMode {
    #[prost(string, required, tag="1")]
    pub mode: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientAddElement {
    #[prost(message, required, tag="1")]
    pub element: Element,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terrain {
    #[prost(int32, optional, tag="11")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, required, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="5")]
    pub z: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="6")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(string, required, tag="7")]
    pub animation_name: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="9")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub y: ::core::option::Option<i32>,
    #[prost(message, optional, tag="1")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientAddTerrain {
    #[prost(message, required, tag="1")]
    pub terrain: Terrain,
    #[prost(bool, optional, tag="3")]
    pub all: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientDeleteElement {
    #[prost(int32, required, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientDeleteTerrain {
    #[prost(int32, repeated, packed="false", tag="4")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, required, tag="1")]
    pub x: i32,
    #[prost(int32, required, tag="2")]
    pub y: i32,
    #[prost(bool, optional, tag="3")]
    pub all: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientRemoveTerrain {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag="2")]
    pub position: ::prost::alloc::vec::Vec<super::op_def::PbPoint3f>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientMouseFollow {
    #[prost(message, optional, tag="1")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(message, optional, tag="2")]
    pub animation: ::core::option::Option<super::op_gameconfig_01::AnimationData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientSelectElement {
    #[prost(int32, required, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientFixedToElement {
    #[prost(int32, required, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientSyncElement {
    #[prost(message, repeated, tag="1")]
    pub element: ::prost::alloc::vec::Vec<Element>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientAlignGrid {
    #[prost(bool, required, tag="1")]
    pub align: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientVisibleGrid {
    #[prost(bool, required, tag="1")]
    pub visible: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSyncUserBalance {
    #[prost(double, optional, tag="1")]
    pub tu_ding: ::core::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub qing_song_tang: ::core::option::Option<f64>,
    #[prost(double, optional, tag="3")]
    pub gold: ::core::option::Option<f64>,
}
// editor end

// new Proto 2019.08.26

// 流程
// 客户端游戏创建完成后会同步新增物件/人与旧id之间的注册信息
// 并且同步有变更的物件当前状态
// 之后运行时，是添加ADD_OBJECT 同步SYNC_OBJECT 删除逻辑DELETE_OBJECT
// 运行时 也会动态添加 BIND_ID信息

/// 结构体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindId {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    /// repeated?
    #[prost(int32, optional, tag="2")]
    pub bind_id: ::core::option::Option<i32>,
}
/// 关于客户端无法从pi内获取的资源，获取规则
/// 当Sprite第一次被添加时，会校验是否已经加载过
/// 若已经加载过，则不会在发送动画资源(包含avatar，frames等）
/// 如果客户端因为某些原因（缓存资源被清除等），可以通过OP_REQ_VIRTUAL_WORLD_QUERY_SPRITE_RESOURCE协议强制获取sprite资源
/// 请求后将服务端返回带有资源的OP_VIRTUAL_WORLD_REQ_CLIENT_SYNC_SPRITE消息作为回应
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sprite {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(message, optional, tag="2")]
    pub point3f: ::core::option::Option<super::op_def::PbPoint3f>,
    #[prost(message, optional, tag="3")]
    pub avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    #[prost(string, optional, tag="4")]
    pub current_animation_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 动画朝向, 默认3,即面向玩家. 动画
    #[prost(enumeration="super::op_def::Direction", optional, tag="5")]
    pub direction: ::core::option::Option<i32>,
    #[prost(string, optional, tag="6")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="7")]
    pub display_badge_cards: ::prost::alloc::vec::Vec<super::op_def::BadgeCard>,
    /// \[0-100\] 0为透明
    #[prost(int32, optional, tag="8", default="100")]
    pub opacity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub bind_id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="10")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(message, optional, tag="11")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(string, optional, tag="12")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="14")]
    pub animation_registration_map: ::prost::alloc::vec::Vec<super::op_def::StrPair>,
    #[prost(message, repeated, tag="15")]
    pub attrs: ::prost::alloc::vec::Vec<super::op_def::StrPair>,
    #[prost(bool, optional, tag="16")]
    pub is_moss: ::core::option::Option<bool>,
    /// 如果该位置不存在sprite将用0补全
    #[prost(int32, repeated, packed="false", tag="17")]
    pub mount_sprites: ::prost::alloc::vec::Vec<i32>,
    #[prost(float, optional, tag="18")]
    pub speed: ::core::option::Option<f32>,
    /// op_def.TitleMask.TQ_NickName << 16; 后16位由用户定义
    #[prost(uint32, optional, tag="19")]
    pub title_mask: ::core::option::Option<u32>,
    #[prost(int32, optional, tag="20")]
    pub layer: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteModifyResult {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(message, optional, tag="2")]
    pub point3f: ::core::option::Option<super::op_def::PbPoint3f>,
    #[prost(string, optional, tag="4")]
    pub current_animation_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 动画朝向, 默认3,即面向玩家. 动画
    #[prost(enumeration="super::op_def::Direction", optional, tag="5")]
    pub direction: ::core::option::Option<i32>,
    #[prost(string, optional, tag="6")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
    /// 0xffff代表删除, 0x0001代表新增, 0x0002代表修改,  删除操作高于所有, 即假定已经收回的物件 重新摆放出来是新的id
    #[prost(int32, optional, tag="7")]
    pub command_mask: ::core::option::Option<i32>,
}
/// 消息
/// BIND_ID，由BIND_ID注册Object之间的关系， （暂无卸载注册关系需求）
/// Object后续变动由sync消息同步
/// 添加ADD_SPRITE 单纯同步id 以及位置信息
/// 在镜头内删除DELETE_SPRITE 单纯删除id，但是注册关系保留，下次直接由ADD_SPRITE添加
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientSyncSprite {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_def::OpCommand", optional, tag="3", default="Patch")]
    pub command: ::core::option::Option<i32>,
    #[prost(string, repeated, tag="4")]
    pub patch_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientAddSprite {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub packet: ::core::option::Option<super::op_def::Packet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientCurrentSceneAllSprite {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub packet: ::core::option::Option<super::op_def::Packet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientNoticeReloadScene {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub packet: ::core::option::Option<super::op_def::Packet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientSyncActor {
    #[prost(message, optional, tag="1")]
    pub actor: ::core::option::Option<Actor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientDeleteSprite {
    /// 消息只删除场景中的物件，并不清除物件的bind关系， 在镜头中移除后下次不会再发送bind消息，直接由ADD_OBJECT添加
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientAdjustPosition {
    #[prost(message, repeated, tag="1")]
    pub sprite_positions: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientGotoAnotherGame {
    #[prost(string, required, tag="1")]
    pub game_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub virtual_world_id: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="3")]
    pub scene_id: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub loc: ::core::option::Option<super::op_def::PbPoint3f>,
    #[prost(int32, optional, tag="5")]
    pub spawn_point_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag="6")]
    pub world_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientSetSpritePosition {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(message, required, tag="2")]
    pub position: super::op_def::PbPoint3f,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="3")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientChangeToEditorMode {
    #[prost(message, required, tag="1")]
    pub actor: Actor,
    #[prost(message, required, tag="2")]
    pub scene: Scene,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientChangeToEditorMode {
    #[prost(message, required, tag="1")]
    pub actor: Actor,
    #[prost(message, required, tag="2")]
    pub scene: Scene,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientMouseSelectedSprite {
    #[prost(message, required, tag="1")]
    pub sprite: Sprite,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="3")]
    pub is_moss: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="4")]
    pub key: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientFetchSprite {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientMouseSelectedSprite {
    #[prost(message, required, tag="1")]
    pub sprite: Sprite,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="3")]
    pub is_moss: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="4")]
    pub key: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientCreateSprite {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientDeleteSprite {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientSyncSprite {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientAddSpritesWithLocs {
    #[prost(message, repeated, tag="1")]
    pub locs: ::prost::alloc::vec::Vec<super::op_def::MossMetaData>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub packet: ::core::option::Option<super::op_def::Packet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientDeleteSpritesWithLocs {
    #[prost(message, repeated, tag="1")]
    pub locs: ::prost::alloc::vec::Vec<super::op_def::MossMetaData>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientAddMosses {
    #[prost(message, repeated, tag="1")]
    pub locs: ::prost::alloc::vec::Vec<super::op_def::MossMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientDeleteMosses {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientSyncMosses {
    #[prost(message, repeated, tag="1")]
    pub locs: ::prost::alloc::vec::Vec<super::op_def::MossMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientAddScenery {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(message, required, tag="2")]
    pub uris: super::op_def::StrArrays,
    #[prost(int32, required, tag="3")]
    pub depth: i32,
    #[prost(message, optional, tag="4")]
    pub offset: ::core::option::Option<super::op_def::PbPoint2f>,
    #[prost(float, optional, tag="5")]
    pub speed: ::core::option::Option<f32>,
    #[prost(int32, optional, tag="6")]
    pub fit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub height: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub width: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientDeleteScenery {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientUpdateScenery {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(message, required, tag="2")]
    pub uris: super::op_def::StrArrays,
    #[prost(int32, required, tag="3")]
    pub depth: i32,
    #[prost(message, optional, tag="4")]
    pub offset: ::core::option::Option<super::op_def::PbPoint2f>,
    #[prost(float, optional, tag="5")]
    pub speed: ::core::option::Option<f32>,
    #[prost(int32, optional, tag="6")]
    pub fit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub height: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub width: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEditorReqClientFetchScenery {
    #[prost(int32, required, tag="1")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientSetCameraFollow {
    #[prost(int32, optional, tag="1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub effect: ::core::option::Option<::prost::alloc::string::String>,
    /// 有id就没pos有pos就没id,hasOwnProperty判断有没有
    #[prost(message, optional, tag="3")]
    pub pos: ::core::option::Option<super::op_def::PbPoint2f>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientChangeSpriteAnimation {
    /// 需要修改动画组的sprite的id
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    /// 包含多个动作, 动作按序列顺序执行,当所有动作执行完毕后,默认回到idle, loop为播放次数,默认为1,-1代表无限播放直到下一个动作,打断该动作.动画组播放完毕之后进入idle
    #[prost(message, repeated, tag="2")]
    pub change_animation: ::prost::alloc::vec::Vec<ChangeAnimation>,
    /// 是否加入队列顺序执行, true将加入顺序加入队列中执行,false将清空之前队列内容,使用当前动作组覆盖执行
    #[prost(bool, optional, tag="3", default="false")]
    pub add_to_queue: ::core::option::Option<bool>,
    /// optional bool need_report = 4\[false\]; // 在动画结束时是否需要通知服务端
    /// optional int32 id = 5;  // 通知id
    #[prost(enumeration="super::op_def::NodeType", optional, tag="4")]
    pub node_type: ::core::option::Option<i32>,
}
// // 客户端通知服务端某一动作执行完毕
// message OP_CLIENT_RES_VIRTUAL_WORLD_FINISH_SPRITE_ANIMATION
// {
//     required string animation_name = 1; // mining, 若animation触发时为播放3次则只在3次都播放完毕时发送该协议
// }

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientMoveSprite {
    #[prost(message, repeated, tag="1")]
    pub move_data: ::prost::alloc::vec::Vec<MoveData>,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeSelectedSprite {
    #[prost(message, required, tag="1")]
    pub sprite: Sprite,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktSuitGroup {
    #[prost(message, repeated, tag="1")]
    pub avatar_suit: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientShowCreateRoleUi {
    #[prost(message, required, tag="1")]
    pub text: super::op_gameconfig_01::Text,
    #[prost(message, repeated, tag="2")]
    pub avatars: ::prost::alloc::vec::Vec<super::op_gameconfig::Avatar>,
    #[prost(message, required, tag="3")]
    pub button: super::op_gameconfig_01::Button,
    #[prost(message, repeated, tag="4")]
    pub avatar_suits: ::prost::alloc::vec::Vec<PktSuitGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientCreateRoleGenerateNewName {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientCreateRoleErrorMessage {
    #[prost(string, required, tag="1")]
    pub error_msg: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientGetMarketCategories {
    #[prost(message, repeated, tag="1")]
    pub market_category: ::prost::alloc::vec::Vec<super::op_def::MarketCategory>,
    #[prost(string, optional, tag="2")]
    pub market_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCommodity {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub icon: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub price: ::prost::alloc::vec::Vec<super::op_gameconfig::Price>,
    #[prost(string, optional, tag="6")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="7")]
    pub remain_number: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8", default="1")]
    pub each_purchase_number: ::core::option::Option<i32>,
    #[prost(string, optional, tag="9")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub short_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="11")]
    pub require_values: ::prost::alloc::vec::Vec<super::op_pkt_def::PktCompareValue>,
    #[prost(message, repeated, tag="12")]
    pub affect_values: ::prost::alloc::vec::Vec<super::op_pkt_def::PktCompareValue>,
    #[prost(int32, optional, tag="13")]
    pub limit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="14")]
    pub remain: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_pkt_def::PktManorCommodityState", optional, tag="15")]
    pub manor_state: ::core::option::Option<i32>,
    #[prost(string, optional, tag="16")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="17")]
    pub suit_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="18")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="19")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="20")]
    pub slot: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMarketQuery {
    #[prost(int32, optional, tag="1")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub max_page: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub subcategory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub commodities: ::prost::alloc::vec::Vec<MarketCommodity>,
    #[prost(string, optional, tag="7")]
    pub market_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMarketQueryCommodityResource {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(message, optional, tag="4")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(message, optional, tag="5")]
    pub avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    #[prost(string, optional, tag="6")]
    pub market_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMarketQueryPackageItemResource {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(message, optional, tag="3")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(message, optional, tag="4")]
    pub avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeAddSpriteError {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MovePoint {
    #[prost(message, required, tag="1")]
    pub point3f: super::op_def::PbPoint3f,
    #[prost(double, optional, tag="2")]
    pub timestemp: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientMoveSpriteByPath {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="3")]
    pub path: ::prost::alloc::vec::Vec<MovePoint>,
    /// 时间间隔，根据发包间隔外加预计网络延时决定
    #[prost(int32, optional, tag="4")]
    pub time_span: ::core::option::Option<i32>,
    #[prost(double, optional, tag="5")]
    pub timestemp: ::core::option::Option<f64>,
    #[prost(enumeration="super::op_def::PathReachableStatus", optional, tag="6")]
    pub path_status: ::core::option::Option<i32>,
    #[prost(message, optional, tag="7")]
    pub target_pos: ::core::option::Option<super::op_def::PbPoint3f>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientUnwalkableBitMap {
    #[prost(message, repeated, tag="1")]
    pub int_array: ::prost::alloc::vec::Vec<super::op_def::IntArray>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeGetSpawnPoint {
    #[prost(message, required, tag="1")]
    pub spawn_point: super::op_def::PbPoint3f,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeUpdateRoomInfo {
    #[prost(string, required, tag="1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="super::op_def::EditModeRoomPrivacy", optional, tag="4")]
    pub privacy: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub owner_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub player_count: ::core::option::Option<i32>,
    #[prost(message, optional, tag="7")]
    pub room_level: ::core::option::Option<super::op_pkt_def::PktLevel>,
    #[prost(string, optional, tag="8")]
    pub room_type: ::core::option::Option<::prost::alloc::string::String>,
    /// 点赞数量
    #[prost(int32, optional, tag="9")]
    pub praise: ::core::option::Option<i32>,
    /// 是否已经点赞
    #[prost(bool, optional, tag="10")]
    pub has_praised: ::core::option::Option<bool>,
    #[prost(double, optional, tag="11")]
    pub competitiveness: ::core::option::Option<f64>,
    ///     optional double turnover = 12;   // 每日营业额
    ///
    /// 热度
    #[prost(double, optional, tag="13")]
    pub popularity: ::core::option::Option<f64>,
    #[prost(message, optional, tag="14")]
    pub turnover_prop: ::core::option::Option<super::op_pkt_def::PktProperty>,
    #[prost(double, optional, tag="15")]
    pub undepreciated: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="16")]
    pub opening_party: ::core::option::Option<bool>,
    /// 用于判断是否是自己的房间
    #[prost(string, optional, tag="17")]
    pub owner_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 派对的描述
    #[prost(string, optional, tag="18")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeRoomInfo {
    #[prost(string, required, tag="1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="super::op_def::EditModeRoomPrivacy", optional, tag="4")]
    pub privacy: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub owner_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub player_count: ::core::option::Option<i32>,
    #[prost(message, optional, tag="7")]
    pub room_level: ::core::option::Option<super::op_pkt_def::PktLevel>,
    #[prost(string, optional, tag="8")]
    pub room_type: ::core::option::Option<::prost::alloc::string::String>,
    /// 点赞数量
    #[prost(int32, optional, tag="9")]
    pub praise: ::core::option::Option<i32>,
    /// 是否已经点赞
    #[prost(bool, optional, tag="10")]
    pub has_praised: ::core::option::Option<bool>,
    #[prost(double, optional, tag="11")]
    pub competitiveness: ::core::option::Option<f64>,
    ///     optional double turnover = 12;   // 每日营业额
    ///
    /// 热度
    #[prost(double, optional, tag="13")]
    pub popularity: ::core::option::Option<f64>,
    #[prost(message, optional, tag="14")]
    pub turnover_prop: ::core::option::Option<super::op_pkt_def::PktProperty>,
    #[prost(double, optional, tag="15")]
    pub undepreciated: ::core::option::Option<f64>,
    #[prost(bool, optional, tag="16")]
    pub opening_party: ::core::option::Option<bool>,
    /// 用于判断是否是自己的房间
    #[prost(string, optional, tag="17")]
    pub owner_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 派对的描述
    #[prost(string, optional, tag="18")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditModeRoom {
    #[prost(string, required, tag="1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub game_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="4")]
    pub player_count: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_def::EditModeRoomPrivacy", optional, tag="5")]
    pub privacy: ::core::option::Option<i32>,
    #[prost(message, optional, tag="6")]
    pub room_level: ::core::option::Option<super::op_pkt_def::PktLevel>,
    /// 存储额
    #[prost(double, optional, tag="7")]
    pub savings: ::core::option::Option<f64>,
    #[prost(double, optional, tag="8")]
    pub competitiveness: ::core::option::Option<f64>,
    /// 每日营业额
    #[prost(double, optional, tag="9")]
    pub turnover: ::core::option::Option<f64>,
    #[prost(string, optional, tag="10")]
    pub industry: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub store_type: ::core::option::Option<::prost::alloc::string::String>,
    /// 热度
    #[prost(double, optional, tag="12")]
    pub popularity: ::core::option::Option<f64>,
    /// 好评
    #[prost(double, optional, tag="13")]
    pub praise: ::core::option::Option<f64>,
    #[prost(string, optional, tag="14")]
    pub owner_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="15")]
    pub focus: ::core::option::Option<i32>,
    #[prost(message, optional, tag="16")]
    pub topic: ::core::option::Option<super::op_pkt_def::PktProperty>,
    #[prost(int32, optional, tag="17")]
    pub manor_count: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="18")]
    pub manor_limit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="19")]
    pub street_id: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_def::RoomTypeEnum", optional, tag="20")]
    pub room_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeRoomList {
    #[prost(message, repeated, tag="1")]
    pub popular_rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
    #[prost(message, repeated, tag="2")]
    pub player_rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
    #[prost(int32, optional, tag="3")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub max_page: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktStreetList {
    #[prost(int32, optional, tag="1")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub max_page: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="4")]
    pub street: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeRotateSprite {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, required, tag="2")]
    pub animation: ChangeAnimation,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeEnterRoom {
    #[prost(string, required, tag="1")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::op_def::EditModeEnterRoomResult", optional, tag="2")]
    pub result: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub game_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeAddSpriteByType {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub remain_count: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeAddSingleSpriteByType {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(enumeration="super::op_def::NodeType", optional, tag="2")]
    pub node_type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub remain_count: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="4")]
    pub added_sprites: ::prost::alloc::vec::Vec<Sprite>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeGetPlayerEnterRoomHistory {
    #[prost(message, repeated, tag="1")]
    pub self_rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
    #[prost(message, repeated, tag="2")]
    pub history_rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeGetPackageCategories {
    #[prost(enumeration="super::op_pkt_def::PktPackageType", required, tag="1")]
    pub category: i32,
    #[prost(message, repeated, tag="2")]
    pub subcategory: ::prost::alloc::vec::Vec<super::op_def::StrPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountablePackageItem {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, required, tag="2")]
    pub count: i32,
    /// 道具名称
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    /// 背包物件贴图
    #[prost(message, optional, tag="5")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(string, optional, tag="6")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub short_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub category: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub subcategory: ::core::option::Option<::prost::alloc::string::String>,
    /// 售价
    #[prost(message, optional, tag="10")]
    pub selling_price: ::core::option::Option<super::op_gameconfig::Price>,
    /// 是否可以与玩家交易
    #[prost(bool, optional, tag="11", default="false")]
    pub tradable: ::core::option::Option<bool>,
    /// 是否可被系统回收
    #[prost(bool, optional, tag="12", default="false")]
    pub recyclable: ::core::option::Option<bool>,
    /// 是否可以使用
    #[prost(bool, optional, tag="13", default="false")]
    pub executable: ::core::option::Option<bool>,
    /// 品质
    #[prost(string, optional, tag="14")]
    pub quality: ::core::option::Option<::prost::alloc::string::String>,
    /// 所需数量, 例如 配方中需要的数量
    #[prost(int32, optional, tag="15")]
    pub needed_count: ::core::option::Option<i32>,
    #[prost(string, optional, tag="16")]
    pub index_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="17")]
    pub avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    /// 右角标
    #[prost(enumeration="super::op_pkt_def::PktSubscript", optional, tag="18", default="Unset")]
    pub right_subscript: ::core::option::Option<i32>,
    #[prost(string, optional, tag="19", default="\u{4f7f}\u{7528}")]
    pub use_button_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 最后使用时间戳
    #[prost(int32, optional, tag="20")]
    pub latest_use: ::core::option::Option<i32>,
    /// 推荐系数 由策划提供，推荐值越高，排位越靠前，具体以相应ui文档为准
    #[prost(int32, optional, tag="21")]
    pub recommended: ::core::option::Option<i32>,
    /// 单次使用上限 -1表示无上限
    #[prost(int32, optional, tag="22", default="-1")]
    pub once_use_limit: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="23")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    /// 动画图集
    #[prost(message, optional, tag="24")]
    pub animation_display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(message, optional, tag="25")]
    pub count_range: ::core::option::Option<super::op_def::Range>,
    #[prost(int32, optional, tag="26")]
    pub expired_time: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="27")]
    pub require_values: ::prost::alloc::vec::Vec<super::op_pkt_def::PktCompareValue>,
    #[prost(message, repeated, tag="28")]
    pub affect_values: ::prost::alloc::vec::Vec<super::op_pkt_def::PktCompareValue>,
    #[prost(string, optional, tag="29")]
    pub suit_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="30")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="31")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="32")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="33")]
    pub slot: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="34")]
    pub rarity: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="35")]
    pub grade: ::core::option::Option<i32>,
    #[prost(string, optional, tag="36")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    /// 最后加入背包的时间
    #[prost(int32, optional, tag="37")]
    pub added_timestamp: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModeQueryEditPackage {
    #[prost(int32, optional, tag="1")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub max_page: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_pkt_def::PktPackageType", optional, tag="4")]
    pub category: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub subcategory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub items: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMarketQueryPackage {
    #[prost(int32, optional, tag="1")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub max_page: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_pkt_def::PktPackageType", optional, tag="4")]
    pub category: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub subcategory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="6")]
    pub items: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientShowInteractiveBubble {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(int32, required, tag="2")]
    pub receiver_id: i32,
    /// 存续时间,单位毫秒,-1代表无限长
    #[prost(int32, optional, tag="3", default="-1")]
    pub duration: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWordlReqClientRemoveInteractiveBubble {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientMiningModeShowRewardPackage {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
/// 游戏音效控制
/// 收到播放音效的消息后，前端程序会查找 source_id 对应的对象是否存在一个类型为media的Attribute，并播放此Attribute对应的音频资源
/// 注意：前端需要预加载mediaAttr，不然在收到消息后再加载会有延迟。
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSoundCtl {
    /// 音效层级：背景音（场景级别），舞台音（物件级别），IO音（UI级别），应当在游戏前后端统一制定层级枚举。
    #[prost(int32, required, tag="1")]
    pub scope: i32,
    /// 声音源, 场景id、物件id、ui的id等等游戏对象的id
    #[prost(int32, optional, tag="2")]
    pub source_id: ::core::option::Option<i32>,
    /// 音效资源的key值, 请参阅 op_gameconfig_01.Attribute.media
    #[prost(string, required, tag="3")]
    pub sound_key: ::prost::alloc::string::String,
    /// 循环播放信标
    #[prost(bool, optional, tag="4")]
    pub r#loop: ::core::option::Option<bool>,
    /// 延迟播放，单位ms
    #[prost(int32, optional, tag="5")]
    pub delay: ::core::option::Option<i32>,
    /// 操作模式 1: 播放，2: 停止
    #[prost(int32, optional, tag="6", default="1")]
    pub command: ::core::option::Option<i32>,
}
/// 音效设定
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSoundSetting {
    /// 一个bool数组，每个值对应一个值对应一个层级的音效是否静音。顺序应当和OP_VIRTUAL_WORLD_RES_CLIENT_SOUND_CTL.scope枚举相同
    #[prost(bool, repeated, packed="false", tag="1")]
    pub scope_mute: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiningEquipment {
    ///
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// 名称
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 说明
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// buff的说明展示,可能会有多个
    #[prost(string, repeated, tag="4")]
    pub buff_display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 可能存在需要条件不能购买的情况
    #[prost(string, repeated, tag="5")]
    pub condition_display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 价格
    #[prost(message, optional, tag="6")]
    pub price: ::core::option::Option<super::op_gameconfig::Price>,
    /// 是否已经拥有
    #[prost(bool, optional, tag="7")]
    pub owned: ::core::option::Option<bool>,
    /// 当前装备
    #[prost(bool, optional, tag="8")]
    pub selected: ::core::option::Option<bool>,
    /// 是否可以购买
    #[prost(bool, optional, tag="9")]
    pub qualified: ::core::option::Option<bool>,
    /// 展示资源
    #[prost(message, optional, tag="10")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MiningEquipmenetArray {
    /// 矿镐 or 矿石车
    #[prost(string, required, tag="1")]
    pub equipment_type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub mine_equipments: ::prost::alloc::vec::Vec<MiningEquipment>,
}
/// 服务端通知客户端 展示矿工装备选择面板
/// 装备存在owned属性以及condition_display_names属性,若装备owned属性为false则需要展示condition_display_names否者展示勾选按钮即可
/// 装备不一定需要金币购买可能需要等级或其他限制解锁
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientMiningModeShowSelectEquipmentPanel {
    #[prost(message, repeated, tag="1")]
    pub mine_equipments: ::prost::alloc::vec::Vec<MiningEquipmenetArray>,
}
/// 服务端返回激活的装备
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMiningModeActiveEquipment {
    #[prost(message, optional, tag="1")]
    pub mine_equipment: ::core::option::Option<MiningEquipment>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMiningModeQueryMinePackage {
    #[prost(message, repeated, tag="1")]
    pub subcategories: ::prost::alloc::vec::Vec<super::op_def::StrPair>,
    #[prost(string, optional, tag="2")]
    pub subcategory: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub items: ::prost::alloc::vec::Vec<CountablePackageItem>,
    #[prost(int32, optional, tag="4")]
    pub limit: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientShowRewardTips {
    #[prost(message, required, tag="1")]
    pub display: super::op_gameconfig::Display,
    #[prost(string, required, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub item_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientShowHighQualityRewardTips {
    #[prost(message, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<CountablePackageItem>,
    #[prost(string, optional, tag="2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientShowBlingPanel {
    /// 面板 ID (目前就一个)
    #[prost(int32, required, tag="1")]
    pub panel_id: i32,
    /// 第一行字
    #[prost(string, required, tag="2")]
    pub line1: ::prost::alloc::string::String,
    /// 第二行字
    #[prost(string, optional, tag="3")]
    pub line2: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientPktPlayerInfo {
    #[prost(int32, optional, tag="1")]
    pub coin: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub diamond: ::core::option::Option<i32>,
    #[prost(message, optional, tag="3")]
    pub level: ::core::option::Option<super::op_pkt_def::PktLevel>,
    #[prost(message, optional, tag="4")]
    pub energy: ::core::option::Option<super::op_def::ValueBar>,
    #[prost(message, optional, tag="9")]
    pub work_chance: ::core::option::Option<super::op_def::ValueBar>,
    #[prost(enumeration="super::op_def::OpCommand", optional, tag="5", default="Patch")]
    pub command: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="6")]
    pub rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
    #[prost(message, optional, tag="7")]
    pub handheld: ::core::option::Option<CountablePackageItem>,
    #[prost(message, repeated, tag="8")]
    pub properties: ::prost::alloc::vec::Vec<super::op_pkt_def::PktProperty>,
    /// 昵称
    #[prost(string, optional, tag="10")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
    /// 皮卡星
    #[prost(int32, optional, tag="12")]
    pub pica_star: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="13")]
    pub investigate_chance: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientPktGuideData {
    /// 引导状态
    #[prost(int32, repeated, packed="false", tag="1")]
    pub finished_guide: ::prost::alloc::vec::Vec<i32>,
}
/// 服务端返回客户端玩家自身信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktSelfPlayerInfo {
    /// sprite id
    #[prost(int32, optional, tag="1")]
    pub id: ::core::option::Option<i32>,
    /// 玩家角色id, 展示
    #[prost(string, optional, tag="2")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
    /// 点赞
    #[prost(int32, optional, tag="3", default="0")]
    pub like: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub current_avatar: ::core::option::Option<super::op_pkt_def::PktAvatar>,
    /// 昵称
    #[prost(string, optional, tag="5")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    /// 等级
    #[prost(message, optional, tag="6")]
    pub level: ::core::option::Option<super::op_pkt_def::PktLevel>,
    /// 当前称号
    #[prost(string, optional, tag="7")]
    pub current_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="8")]
    pub life_skills: ::prost::alloc::vec::Vec<super::op_pkt_def::PktSkill>,
    /// 徽章
    #[prost(message, repeated, tag="9")]
    pub badges: ::prost::alloc::vec::Vec<super::op_pkt_def::PktBadge>,
    /// 称号
    #[prost(message, repeated, tag="10")]
    pub titles: ::prost::alloc::vec::Vec<super::op_pkt_def::PktTitle>,
    #[prost(message, repeated, tag="11")]
    pub properties: ::prost::alloc::vec::Vec<super::op_pkt_def::PktProperty>,
    #[prost(message, optional, tag="12")]
    pub handheld: ::core::option::Option<CountablePackageItem>,
    #[prost(message, repeated, tag="14")]
    pub avatar_suit: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
/// 服务端返回客户端其他玩家信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktAnotherPlayerInfo {
    /// sprite id
    #[prost(int32, optional, tag="1")]
    pub id: ::core::option::Option<i32>,
    /// 玩家角色id, 展示
    #[prost(string, optional, tag="2")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
    /// 点赞
    #[prost(int32, optional, tag="3", default="0")]
    pub like: ::core::option::Option<i32>,
    #[prost(message, optional, tag="4")]
    pub current_avatar: ::core::option::Option<super::op_pkt_def::PktAvatar>,
    /// 昵称
    #[prost(string, optional, tag="5")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    /// 等级
    #[prost(message, optional, tag="6")]
    pub level: ::core::option::Option<super::op_pkt_def::PktLevel>,
    /// 当前称号
    #[prost(string, optional, tag="7")]
    pub current_title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="8")]
    pub life_skills: ::prost::alloc::vec::Vec<super::op_pkt_def::PktSkill>,
    /// 徽章
    #[prost(message, repeated, tag="9")]
    pub badges: ::prost::alloc::vec::Vec<super::op_pkt_def::PktBadge>,
    /// 称号
    #[prost(message, repeated, tag="11")]
    pub titles: ::prost::alloc::vec::Vec<super::op_pkt_def::PktTitle>,
    /// 房间列表 功能待定 字段预留
    #[prost(string, repeated, tag="12")]
    pub room_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 备注名
    #[prost(string, optional, tag="13")]
    pub remark: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否为好友
    #[prost(bool, optional, tag="14")]
    pub friend: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="15")]
    pub properties: ::prost::alloc::vec::Vec<super::op_pkt_def::PktProperty>,
    #[prost(message, repeated, tag="16")]
    pub avatar_suit: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktCraftSkill {
    #[prost(message, optional, tag="1")]
    pub skill: ::core::option::Option<super::op_pkt_def::PktSkill>,
    /// 合成技能产物名称
    #[prost(string, optional, tag="2")]
    pub product_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 合成技能产物描述
    #[prost(string, optional, tag="3")]
    pub product_des: ::core::option::Option<::prost::alloc::string::String>,
    /// 合成技能产物来源
    #[prost(string, optional, tag="4")]
    pub product_source: ::core::option::Option<::prost::alloc::string::String>,
    /// 合成技能产物动画
    #[prost(message, repeated, tag="5")]
    pub product_animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    /// 合成技能产物资源
    #[prost(message, optional, tag="6")]
    pub product_display: ::core::option::Option<super::op_gameconfig::Display>,
    /// 合成技能产物Avatar
    #[prost(message, optional, tag="7")]
    pub product_avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    /// 配方原料
    #[prost(message, repeated, tag="8")]
    pub materials: ::prost::alloc::vec::Vec<CountablePackageItem>,
    /// 合成产物
    #[prost(message, optional, tag="9")]
    pub product: ::core::option::Option<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCraftSkills {
    #[prost(message, repeated, tag="1")]
    pub skills: ::prost::alloc::vec::Vec<PktCraftSkill>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCraftQueryFormula {
    /// 合成技能ID
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// 合成技能产物名称
    #[prost(string, optional, tag="2")]
    pub product_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 合成技能产物描述
    #[prost(string, optional, tag="3")]
    pub product_des: ::core::option::Option<::prost::alloc::string::String>,
    /// 合成技能产物来源
    #[prost(string, optional, tag="4")]
    pub product_source: ::core::option::Option<::prost::alloc::string::String>,
    /// 合成技能产物动画
    #[prost(message, repeated, tag="5")]
    pub product_animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    /// 合成技能产物资源
    #[prost(message, optional, tag="6")]
    pub product_display: ::core::option::Option<super::op_gameconfig::Display>,
    /// 合成技能产物Avatar
    #[prost(message, optional, tag="7")]
    pub product_avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    /// 配方原料
    #[prost(message, repeated, tag="8")]
    pub materials: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktResetAvatar {
    #[prost(message, required, tag="1")]
    pub avatar: super::op_gameconfig::Avatar,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCurrentDressAvatarItemId {
    #[prost(string, repeated, tag="1")]
    pub avatar_item_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktQuest {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 任务目标描述
    #[prost(string, optional, tag="3")]
    pub detail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="4")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    /// 任务状态
    #[prost(enumeration="super::op_pkt_def::PktQuestStage", optional, tag="5", default="NotAccepted")]
    pub stage: ::core::option::Option<i32>,
    /// 任务类型
    #[prost(enumeration="super::op_pkt_def::PktQuestType", optional, tag="6")]
    pub quest_type: ::core::option::Option<i32>,
    /// 任务目标 展示 可能为空
    #[prost(message, repeated, tag="7")]
    pub targets: ::prost::alloc::vec::Vec<CountablePackageItem>,
    /// 任务奖励
    #[prost(message, repeated, tag="8")]
    pub rewards: ::prost::alloc::vec::Vec<CountablePackageItem>,
    /// 刷新结束时间
    #[prost(int32, optional, tag="9")]
    pub refresh_deadline: ::core::option::Option<i32>,
    /// 订单任务送货结束时间
    #[prost(int32, optional, tag="10")]
    pub delivery_deadline: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub cabin_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryQuestList {
    ///
    #[prost(message, repeated, tag="1")]
    pub quests: ::prost::alloc::vec::Vec<PktQuest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryQuestDetail {
    #[prost(message, required, tag="1")]
    pub quest: PktQuest,
}
/// 新增服务端打开客户端商店面板协议
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientMarketShowMarketByName {
    #[prost(string, optional, tag="1")]
    pub market_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// 新增默认UI设定
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientPktRefreshActiveUi {
    /// PICaMain.RoomInfo PlayerInfo Navigate Quest Map Market Package GoHome
    #[prost(message, repeated, tag="1")]
    pub ui: ::prost::alloc::vec::Vec<super::op_pkt_def::PktUi>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateGroup {
    #[prost(message, required, tag="1")]
    pub owner: super::op_gameconfig_01::Node,
    #[prost(message, repeated, tag="2")]
    pub state: ::prost::alloc::vec::Vec<super::op_def::State>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientSyncState {
    #[prost(message, repeated, tag="1")]
    pub state_group: ::prost::alloc::vec::Vec<StateGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktUnlockElementRequirement {
    /// unlock 对象
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    /// 原料
    #[prost(message, repeated, tag="2")]
    pub materials: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktTeambuildElementRequirement {
    /// unlock 对象
    #[prost(int32, repeated, packed="false", tag="1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    /// 原料
    #[prost(message, repeated, tag="2")]
    pub materials: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktHandheld {
    #[prost(string, optional, tag="1")]
    pub current_handheld_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub handheld: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktIndustryModels {
    #[prost(message, repeated, tag="1")]
    pub industry: ::prost::alloc::vec::Vec<super::op_pkt_def::PktIndustry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktMyStore {
    #[prost(int32, optional, tag="1")]
    pub store_limit: ::core::option::Option<i32>,
    ///
    #[prost(message, repeated, tag="2")]
    pub store_list: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCommercialStreet {
    #[prost(message, repeated, tag="1")]
    pub commercial_street: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPlan {
    /// 计划id
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// 所需物品
    #[prost(message, repeated, tag="2")]
    pub requirements: ::prost::alloc::vec::Vec<CountablePackageItem>,
    /// 描述
    #[prost(string, optional, tag="3")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    ///   图标
    #[prost(string, optional, tag="4")]
    pub icon: ::core::option::Option<::prost::alloc::string::String>,
    /// ,名称
    #[prost(string, optional, tag="5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 描述2 对应effect
    #[prost(string, optional, tag="6")]
    pub buff_des: ::core::option::Option<::prost::alloc::string::String>,
    /// 类型
    #[prost(string, optional, tag="7")]
    pub market_plan_type: ::core::option::Option<::prost::alloc::string::String>,
    /// 总耗时 单位秒
    #[prost(int32, optional, tag="8")]
    pub total_time: ::core::option::Option<i32>,
    /// 结束时间 时间戳 单位秒
    #[prost(int32, optional, tag="9")]
    pub end_time: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPlanPair {
    /// 营销计划类型
    #[prost(string, optional, tag="1")]
    pub market_plan_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub market_plan: ::core::option::Option<MarketPlan>,
}
/// 已经选择的营销计划
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktMarketPlan {
    #[prost(message, repeated, tag="1")]
    pub market_plan_pairs: ::prost::alloc::vec::Vec<MarketPlanPair>,
    #[prost(string, optional, tag="2")]
    pub industry_background: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub industry_des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub industry_buff_des: ::core::option::Option<::prost::alloc::string::String>,
}
/// 可供选择的营销计划
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktMarketPlanModelsByType {
    #[prost(message, repeated, tag="1")]
    pub market_plan: ::prost::alloc::vec::Vec<MarketPlan>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryStoreRankingList {
    #[prost(message, repeated, tag="2")]
    pub rank_champions: ::prost::alloc::vec::Vec<super::op_pkt_def::PktRankChampion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryStoreRankingDetail {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="3")]
    pub stores: ::prost::alloc::vec::Vec<super::op_pkt_def::PktStoreRankItem>,
    #[prost(message, optional, tag="4")]
    pub player_store: ::core::option::Option<super::op_pkt_def::PktStoreRankItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktRewardStage {
    ///   表示阶段闭区间 [1, 2] 表示1~2
    #[prost(uint32, optional, tag="1")]
    pub start: ::core::option::Option<u32>,
    ///
    #[prost(uint32, optional, tag="2")]
    pub end: ::core::option::Option<u32>,
    #[prost(message, repeated, tag="3")]
    pub rewards: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryStoreRankingReward {
    #[prost(message, repeated, tag="1")]
    pub reward_stage: ::prost::alloc::vec::Vec<PktRewardStage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryStoreEnterHistory {
    #[prost(message, repeated, tag="1")]
    pub history: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktSyncPackage {
    #[prost(enumeration="super::op_pkt_def::PktPackageType", optional, tag="1")]
    pub package_name: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CountablePackageItem>,
    #[prost(message, optional, tag="3")]
    pub packet: ::core::option::Option<super::op_def::Packet>,
    #[prost(int32, optional, tag="4", default="-1")]
    pub limit: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktUpdatePackage {
    #[prost(enumeration="super::op_pkt_def::PktPackageType", optional, tag="1")]
    pub package_name: ::core::option::Option<i32>,
    /// 服务器推送更新将在第一次PKT_SYNC_PACKAGE之后开始, 应为proto不能使用undefined的定义 需要删除的项目id为0
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktSearchRoom {
    #[prost(int32, optional, tag="1")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="3")]
    pub rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktPlayerList {
    #[prost(message, repeated, tag="1")]
    pub player_infos: ::prost::alloc::vec::Vec<super::op_pkt_def::PktPlayerInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktSearchPlayer {
    #[prost(message, repeated, tag="1")]
    pub player_infos: ::prost::alloc::vec::Vec<super::op_pkt_def::PktPlayerInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktOrderList {
    ///
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<PktQuest>,
    #[prost(message, optional, tag="2")]
    pub royal_order_limit: ::core::option::Option<super::op_def::ValueBar>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktProgress {
    #[prost(int32, optional, tag="1")]
    pub target_value: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="2")]
    pub rewards: ::prost::alloc::vec::Vec<CountablePackageItem>,
    #[prost(bool, optional, tag="3")]
    pub received: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryPlayerProgress {
    #[prost(int32, optional, tag="1")]
    pub current_progress_value: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="2")]
    pub steps: ::prost::alloc::vec::Vec<PktProgress>,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktJobList {
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<PktQuest>,
    #[prost(int32, optional, tag="2")]
    pub times: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryRoomRefurbishRequirements {
    #[prost(message, repeated, tag="1")]
    pub requirements: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktPartyList {
    #[prost(message, optional, tag="1")]
    pub hotel: ::core::option::Option<EditModeRoom>,
    #[prost(message, optional, tag="2")]
    pub picatown: ::core::option::Option<EditModeRoom>,
    #[prost(message, repeated, tag="3")]
    pub party: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCreatePartyRequirements {
    #[prost(message, repeated, tag="1")]
    pub topics: ::prost::alloc::vec::Vec<super::op_pkt_def::PktProperty>,
    #[prost(int32, optional, tag="2")]
    pub tickets_count: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="3")]
    pub created: ::core::option::Option<bool>,
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub topic: ::core::option::Option<super::op_pkt_def::PktProperty>,
    #[prost(int32, optional, tag="7")]
    pub expired: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktPartySendGift {
    #[prost(string, required, tag="1")]
    pub sender_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub room_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub room_name: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(int32, required, tag="6")]
    pub count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktDrawResult {
    /// 抽卡奖励
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<CountablePackageItem>,
    /// 皮卡星奖励
    #[prost(int32, optional, tag="2")]
    pub star: ::core::option::Option<i32>,
    /// 卡池更新
    #[prost(message, optional, tag="3")]
    pub pool_update: ::core::option::Option<DrawPoolStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktDrawStatusResult {
    /// 卡池列表
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<DrawPoolStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrawPoolStatus {
    /// 单抽/十连视为不同的卡池，常驻卡池为写死的固定 ID，活动卡池规则待定
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// 货币 ID
    #[prost(string, required, tag="2")]
    pub token_id: ::prost::alloc::string::String,
    /// 使用货币抽卡的价格
    #[prost(int32, required, tag="3")]
    pub price: i32,
    /// 代币 ID，比如漫游票
    #[prost(string, optional, tag="4")]
    pub alter_token_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 需要 alterTokenId 的数量（同时也是抽卡的次数）
    #[prost(int32, optional, tag="5")]
    pub draw_time: ::core::option::Option<i32>,
    /// 下次免费时间 (没有则不显示)
    #[prost(int32, optional, tag="6")]
    pub next_free_time: ::core::option::Option<i32>,
    /// 累积奖励（只有部分奖池有）
    #[prost(message, repeated, tag="7")]
    pub progress_award: ::prost::alloc::vec::Vec<PktProgress>,
    /// 累计奖励进度
    #[prost(int32, optional, tag="8")]
    pub progress: ::core::option::Option<i32>,
    /// 累计进度清零时间
    #[prost(int32, optional, tag="9")]
    pub progress_expire_time: ::core::option::Option<i32>,
    /// 代币不够的话单独购买的价格
    #[prost(int32, optional, tag="10")]
    pub unit_price: ::core::option::Option<i32>,
    /// 获得皮卡星数量
    #[prost(int32, optional, tag="11")]
    pub pica_star_count: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCombineResult {
    /// 合成结果
    #[prost(message, required, tag="1")]
    pub reward: CountablePackageItem,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktForgeResult {
    /// 重铸结果
    #[prost(message, required, tag="1")]
    pub reward: CountablePackageItem,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktForgeListResult {
    #[prost(message, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktExploreLevelData {
    #[prost(int32, required, tag="1")]
    pub level_id: i32,
    /// 123 表示 1 颗星, 第二颗星 23% 的进度
    #[prost(int32, required, tag="2")]
    pub progress: i32,
    /// 下面的将来从配置读
    #[prost(int32, optional, tag="11")]
    pub chapter_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag="12")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub image_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="14")]
    pub level_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="15")]
    pub clue_items: ::prost::alloc::vec::Vec<CountablePackageItem>,
    #[prost(int32, optional, tag="16")]
    pub required_player_level: ::core::option::Option<i32>,
    /// 配置表中的关卡 sn
    #[prost(string, optional, tag="17")]
    pub room_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 消耗精力
    #[prost(int32, optional, tag="18")]
    pub energy_cost: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktExploreChapterProgress {
    #[prost(message, repeated, tag="1")]
    pub chapters: ::prost::alloc::vec::Vec<PktExploreChapterProgress>,
    /// 关卡进度
    #[prost(int32, optional, tag="11")]
    pub next_chapter_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="12")]
    pub next_level_id: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktExploreChapterProgress {
    #[prost(int32, required, tag="1")]
    pub chapter_id: i32,
    /// 
    #[prost(int32, optional, tag="2")]
    pub progress: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktExploreChapterData {
    #[prost(int32, required, tag="1")]
    pub chapter_id: i32,
    #[prost(int32, required, tag="2")]
    pub star_progress: i32,
    #[prost(bool, optional, tag="3")]
    pub award_taken: ::core::option::Option<bool>,
    /// 下面的将来从配置读
    #[prost(string, optional, tag="11")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub image_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="13")]
    pub total_star: ::core::option::Option<i32>,
    #[prost(message, optional, tag="14")]
    pub award: ::core::option::Option<CountablePackageItem>,
    #[prost(int32, optional, tag="15")]
    pub required_player_level: ::core::option::Option<i32>,
    #[prost(string, optional, tag="16")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktExploreShowCountdown {
    /// 连击数
    #[prost(int32, optional, tag="1")]
    pub combo: ::core::option::Option<i32>,
    /// 倒计时秒数
    #[prost(int32, optional, tag="2")]
    pub seconds: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryChapterResult {
    #[prost(message, required, tag="1")]
    pub chapter: PktExploreChapterData,
    #[prost(message, repeated, tag="2")]
    pub levels: ::prost::alloc::vec::Vec<PktExploreLevelData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktExploreRequireList {
    #[prost(string, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktExploreClueData {
    #[prost(int32, required, tag="1")]
    pub star: i32,
    #[prost(message, optional, tag="2")]
    pub item: ::core::option::Option<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktExploreSummary {
    #[prost(int32, required, tag="1")]
    pub level_id: i32,
    /// 之前的关卡进度百分比
    #[prost(int32, optional, tag="2")]
    pub previous_progress: ::core::option::Option<i32>,
    /// 现在的关卡进度百分比
    #[prost(int32, optional, tag="3")]
    pub latest_progress: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub point_base: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub point_combo: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub point_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="7")]
    pub point_accuracy: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8")]
    pub point_hint: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="9")]
    pub rewards: ::prost::alloc::vec::Vec<CountablePackageItem>,
    /// 以下的之后从配置读
    ///
    /// 可能获得的剧情道具
    #[prost(message, repeated, tag="21")]
    pub clue: ::prost::alloc::vec::Vec<OpVirtualWorldResClientPktExploreClueData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FurnitureRequirements {
    #[prost(string, optional, tag="1")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub requirements: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktRequireFurnitureUnfrozenRequirements {
    #[prost(message, repeated, tag="1")]
    pub furniture_requirements: ::prost::alloc::vec::Vec<FurnitureRequirements>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientTest {
    #[prost(int32, optional, tag="1")]
    pub should_be_null: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub should_be_zero: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub should_be_null_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub should_be_empty_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="5")]
    pub should_be_null_bool: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="6")]
    pub should_be_false: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="7", default="1")]
    pub default_one: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="8", default="1")]
    pub default_one_set_one: ::core::option::Option<i32>,
}
/// server 设置位置
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSetPosition {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
}
/// server 强制停止移动
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientStop {
    #[prost(message, repeated, tag="1")]
    pub sprites: ::prost::alloc::vec::Vec<Sprite>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteIntPair {
    #[prost(int32, required, tag="1")]
    pub key: i32,
    #[prost(message, optional, tag="2")]
    pub sprite: ::core::option::Option<Sprite>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientTerrainCollection {
    #[prost(message, optional, tag="1")]
    pub terrain: ::core::option::Option<super::op_gameconfig_01::TerrainCollection>,
    #[prost(message, repeated, tag="2")]
    pub sprite_sample: ::prost::alloc::vec::Vec<SpriteIntPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryConfigs {
    #[prost(enumeration="super::op_def::ResponseStatus", required, tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientActiveSprite {
    /// 由谁触发
    #[prost(int32, required, tag="1")]
    pub sprite_id: i32,
    /// 和什么交互
    #[prost(int32, optional, tag="2")]
    pub target_id: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub param: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktDressUpAvatar {
    #[prost(string, optional, tag="1")]
    pub suit_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientActiveSpriteEnd {
    /// 由谁触发
    #[prost(int32, optional, tag="1")]
    pub sprite_id: ::core::option::Option<i32>,
    /// 和什么交互
    #[prost(int32, optional, tag="2")]
    pub target_id: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktCurrentRoomPlayerList {
    #[prost(message, repeated, tag="1")]
    pub player_infos: ::prost::alloc::vec::Vec<super::op_pkt_def::PktPlayerInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientGameMode {
    #[prost(enumeration="super::op_def::AvatarStyle", optional, tag="1")]
    pub avatar_style: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktQueryQuestGroup {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub quests: ::prost::alloc::vec::Vec<PktQuest>,
    #[prost(message, optional, tag="5")]
    pub reward: ::core::option::Option<CountablePackageItem>,
    #[prost(bool, optional, tag="6")]
    pub rewards_received: ::core::option::Option<bool>,
    /// 任务类型
    #[prost(enumeration="super::op_pkt_def::PktQuestType", optional, tag="7")]
    pub quest_type: ::core::option::Option<i32>,
    /// 组进度
    #[prost(int32, optional, tag="8")]
    pub progress: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientPktShowCreateRoleUi {
    #[prost(message, repeated, tag="1")]
    pub avatars: ::prost::alloc::vec::Vec<CountablePackageItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientEditModelResult {
    /// 是否成功
    #[prost(bool, optional, tag="1")]
    pub status: ::core::option::Option<bool>,
    /// 描述
    #[prost(string, optional, tag="2")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientStartEditModel {
    /// 是否成功
    #[prost(bool, optional, tag="1")]
    pub status: ::core::option::Option<bool>,
    /// 描述
    #[prost(string, optional, tag="2")]
    pub msg: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktLevelUp {
    #[prost(enumeration="super::op_def::LevelUpType", required, tag="1")]
    pub r#type: i32,
    #[prost(int32, optional, tag="2")]
    pub value: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktRoomShowGuideText {
    #[prost(message, repeated, tag="1")]
    pub text: ::prost::alloc::vec::Vec<GuideText>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuideText {
    #[prost(string, required, tag="1")]
    pub text: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="2")]
    pub total_steps: ::core::option::Option<i32>,
    /// progress == total_steps 则为已经完成
    #[prost(int32, optional, tag="3")]
    pub progress: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientMoveSprite {
    #[prost(message, repeated, tag="1")]
    pub move_path: ::prost::alloc::vec::Vec<super::op_def::MovePath>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientSelfRoomList {
    #[prost(enumeration="super::op_def::RoomTypeEnum", optional, tag="1")]
    pub room_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="2")]
    pub rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientRoomList {
    #[prost(enumeration="super::op_def::RoomTypeEnum", optional, tag="1")]
    pub room_type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="2")]
    pub rooms: ::prost::alloc::vec::Vec<EditModeRoom>,
    #[prost(int32, optional, tag="3")]
    pub page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub per_page: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub max_page: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientPktShopData {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<super::op_gameconfig::ShopItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktGalleryItem {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// 1: 灰，2: 完整
    #[prost(int32, required, tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktMailData {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub attachments: ::prost::alloc::vec::Vec<CountablePackageItem>,
    #[prost(int32, optional, tag="5")]
    pub sent_time: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="6")]
    pub expire_time: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="7")]
    pub has_read: ::core::option::Option<bool>,
    #[prost(string, optional, tag="8")]
    pub sender_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="9")]
    pub attach_taken: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpClientReqVirtualWorldPktUpdateGallery {
    #[prost(message, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<PktGalleryItem>,
    #[prost(bool, optional, tag="2")]
    pub to_notify: ::core::option::Option<bool>,
    /// 灰色进度奖励-下次领取的进度等级
    #[prost(int32, optional, tag="3")]
    pub reward1_next_index: ::core::option::Option<i32>,
    /// 闪亮进度奖励-下次领取的进度等级
    #[prost(int32, optional, tag="4")]
    pub reward2_next_index: ::core::option::Option<i32>,
    /// 灰色进度
    #[prost(int32, optional, tag="5")]
    pub reward1_progress: ::core::option::Option<i32>,
    /// 闪亮进度
    #[prost(int32, optional, tag="6")]
    pub reward2_progress: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpClientReqVirtualWorldPktUpdateGalleryCollectionData {
    /// 完成的收藏任务id
    #[prost(int32, repeated, packed="false", tag="1")]
    pub done_mission_id_list: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpClientReqVirtualWorldPktUpdateMailsData {
    /// 所有邮件信息
    #[prost(message, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<PktMailData>,
    /// true: 所有邮件，false：新邮件
    #[prost(bool, optional, tag="2")]
    pub is_all: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpClientReqVirtualRedDotStatus {
    #[prost(enumeration="super::op_def::RedDotTypeEnum", repeated, packed="false", tag="1")]
    pub list: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientTriggerMoveSprite {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    /// 可为空,速度向量, 默认为物件速度的向量
    #[prost(message, optional, tag="2")]
    pub velocity: ::core::option::Option<super::op_def::PbPoint3f>,
    /// 可为空, 单位像素,默认为一秒内可移动距离?
    #[prost(double, optional, tag="3")]
    pub length: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientUnlockDone {
    /// 物件场景id
    #[prost(int32, required, tag="1")]
    pub eid: i32,
    #[prost(string, optional, tag="2")]
    pub config_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientInvestigateSuccess {
    /// 配置表id
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldReqClientShowPlacard {
    #[prost(string, required, tag="1")]
    pub message: ::prost::alloc::string::String,
    /// 客户端用参数，例如展示样式
    #[prost(string, optional, tag="3")]
    pub c_param: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpVirtualWorldResClientCustomProto {
    #[prost(string, required, tag="1")]
    pub msg_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub cmd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="3")]
    pub msg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
// edit by n 18/09/20
//   ########################################################
//   #   magic  #  uit16  #  16bit  #  zip by zlib or not  #
//   #######################################################
//   #   len   # uint16  #  16bit #  package body length  #
//   ######################################################
//   #   op   #  uint   #  32bit #  op code for protobuf  #
//   #####################################################
//   #  uuid # uint64 #  64bit #   uuid for connection   #
//   ######################################################
//   # param #  uint   # 32bit #   param for head        #
//   #####################################################
//
//   Total should be 96bit for head length

/// TODO auto gen Message by clinet.h
/// can not use the same name of message, because proto define unique error
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Opcode {
    OpMajor = 20,
    OpUnknown = 0,
    OpClientPing = 1376255,
    OpGalaxyResClientStartNewWorld = 1310722,
    OpGatewayResClientError = 1314816,
    OpGatewayReqClientMoveCharacter = 1314817,
    OpGatewayResClientStartNewWorld = 1314818,
    OpGatewayResClientVirtualWorldInit = 1314820,
    OpGatewayReqClientMoveElement = 1314819,
    OpGatewayReqClientSetCharacterPosition = 1314821,
    OpGatewayReqClientSetElementPosition = 1314823,
    OpGatewayReqClientChangeElementAnimation = 1314825,
    OpGatewayResClientPong = 1314826,
    OpVirtualWorldReqClientShowPlacard = 1314853,
    OpVirtualWorldReqClientSceneTransitions = 1314833,
    OpVirtualWorldReqClientChangeTerrainAnimation = 1314835,
    OpVirtualWorldReqClientChangeCharacterAnimation = 1314837,
    OpVirtualWorldReqClientChangeItemAnimation = 1314839,
    OpVirtualWorldReqClientForceOffline = 1314841,
    OpVirtualWorldReqClientCharacterTalking = 1314849,
    OpVirtualWorldReqClientCharacterShutUp = 1314851,
    /// pyr
    OpVirtualWorldResClientEnterScene = 1314822,
    OpVirtualWorldResClientSelectCharacter = 1314824,
    OpVirtualWorldResClientSynchroCharacter = 1314832,
    OpVirtualWorldResClientChat = 1314834,
    OpVirtualWorldResClientLuaLog = 1314836,
    OpVirtualWorldResClientAddElement = 1314838,
    OpVirtualWorldResClientRemoveElement = 1314840,
    OpVirtualWorldResClientUiBindAttribute = 1314848,
    OpVirtualWorldResClientAddCharacter = 1314850,
    OpVirtualWorldResClientRemoveCharacter = 1314852,
    OpVirtualWorldResClientAddTerrain = 1314854,
    OpVirtualWorldResClientGameOver = 1314856,
    OpVirtualWorldResClientChangeScene = 1314864,
    OpVirtualWorldResClientAddTerrainEnd = 1314866,
    OpVirtualWorldResClientSynchroPackage = 1314868,
    OpVirtualWorldResClientAddItem = 1314870,
    OpVirtualWorldResClientRemoveItem = 1314872,
    OpVirtualWorldResClientExchangeItemPos = 1314880,
    OpVirtualWorldResClientRemoveTerrain = 1314882,
    OpVirtualWorldResClientResetCameraSize = 1314884,
    /// _OP_VIRTUAL_WORLD_RES_CLIENT_TARGET_UI          = 0x00141046;
    OpVirtualWorldResClientShowUi = 1314888,
    OpVirtualWorldResClientPackageItemUse = 1314896,
    OpVirtualWorldResClientStoredItem = 1314898,
    OpVirtualWorldResClientCloseUi = 1314902,
    OpVirtualWorldResClientOpenPackage = 1314904,
    OpVirtualWorldResClientQcloudGmeAuthbuffer = 1314912,
    OpVirtualWorldResClientNotice = 1314914,
    OpVirtualWorldResClientOnlyBubble = 1314916,
    /// pyr end
    OpVirtualWorldResClientOnlyBubbleClean = 1314918,
    OpVirtualWorldResClientSynchroShop = 1314920,
    OpVirtualWorldResClientQueryPackage = 1314928,
    OpVirtualWorldResClientSyncUserBalance = 1314930,
    OpVirtualWorldResClientUpdateUi = 1314932,
    OpVirtualWorldResClientShowEffect = 1314934,
    OpVirtualWorldResClientSyncTime = 1314936,
    OpVirtualWorldReqClientMoveSprite = 1314937,
    OpVirtualWorldReqClientTriggerMoveSprite = 1314938,
    /// 服务器客制化协议
    OpVirtualWorldResClientCustomProto = 1314939,
    /// editor
    OpEditorReqClientSetEditorMode = 1318913,
    OpEditorReqClientSetEditorElement = 1318915,
    OpEditorReqClientAddElement = 1318917,
    OpEditorReqClientAddTerrain = 1318919,
    OpEditorReqClientDeleteElement = 1318921,
    OpEditorReqClientDeleteTerrain = 1318929,
    OpEditorReqClientMouseFollow = 1318931,
    OpEditorReqClientSelectElement = 1318933,
    OpEditorReqClientFixedToElement = 1318935,
    OpEditorReqClientSyncElement = 1318937,
    OpEditorReqClientAlignGrid = 1318945,
    OpEditorReqClientVisibleGrid = 1318947,
    OpEditorReqClientCreateSprite = 1318914,
    OpEditorReqClientDeleteSprite = 1318916,
    OpEditorReqClientSyncSprite = 1318918,
    // editor end

    OpVirtualWorldReqClientSyncSprite = 1318949,
    OpVirtualWorldReqClientAddSprite = 1318950,
    OpVirtualWorldReqClientDeleteSprite = 1318951,
    OpVirtualWorldReqClientAdjustPosition = 1318953,
    OpVirtualWorldReqClientCurrentSceneAllSprite = 1318954,
    OpVirtualWorldReqClientGotoAnotherGame = 1318960,
    OpVirtualWorldReqClientChangeToEditorMode = 1318961,
    OpEditorReqClientChangeToEditorMode = 1318962,
    OpVirtualWorldReqClientMouseSelectedSprite = 1318963,
    OpEditorReqClientMouseSelectedSprite = 1318964,
    OpEditorReqClientFetchSprite = 1318965,
    OpVirtualWorldReqClientSetCameraFollow = 1318966,
    OpVirtualWorldReqClientChangeSpriteAnimation = 1318967,
    OpVirtualWorldReqClientAddSpriteEnd = 1318968,
    OpVirtualWorldReqClientEnableEditMode = 1318969,
    OpVirtualWorldResClientEditModeQueryEditPackage = 1318970,
    OpVirtualWorldResClientEditModeReady = 1318971,
    OpVirtualWorldResClientEditModeSelectedSprite = 1318972,
    OpVirtualWorldReqClientSetSpritePosition = 1318973,
    OpVirtualWorldResClientCreateRoleGenerateNewName = 1318975,
    OpVirtualWorldResClientCreateRoleErrorMessage = 1318976,
    OpVirtualWorldReqClientShowCreateRoleUi = 1318977,
    OpVirtualWorldReqClientCloseCreateRoleUi = 1318978,
    OpVirtualWorldResClientGetMarketCategories = 1318979,
    OpVirtualWorldResClientMarketQuery = 1318980,
    OpVirtualWorldResClientMarketQueryCommodityResource = 1318981,
    OpVirtualWorldReqClientEnableMarket = 1318982,
    OpVirtualWorldResClientMarketQueryPackage = 1318983,
    OpVirtualWorldResClientEditModeAddSpriteError = 1318984,
    OpVirtualWorldReqClientMoveSpriteByPath = 1318985,
    OpVirtualWorldReqClientUnwalkableBitMap = 1318986,
    OpVirtualWorldResClientEditModeGetSpawnPoint = 1318987,
    OpVirtualWorldResClientEditModeRoomInfo = 1318988,
    OpVirtualWorldResClientEditModeUpdateRoomInfo = 1318991,
    OpVirtualWorldResClientEditModeRoomList = 1318989,
    OpVirtualWorldResClientEditModeRotateSprite = 1318990,
    OpVirtualWorldResClientEditModeEnterRoom = 1318992,
    OpVirtualWorldResClientEditModeAddSpriteByType = 1318993,
    OpVirtualWorldResClientEditModeAddSingleSpriteByType = 1318994,
    OpVirtualWorldResClientEditModeGetPlayerEnterRoomHistory = 1318995,
    OpVirtualWorldResClientEditModeGetPackageCategories = 1318996,
    OpVirtualWorldResClientMarketQueryPackageItemResource = 1318997,
    OpVirtualWorldReqClientShowInteractiveBubble = 1318998,
    OpVirtualWordlReqClientRemoveInteractiveBubble = 1318999,
    OpVirtualWorldResClientMoveSprite = 1319006,
    /// Mining
    OpVirtualWorldReqClientMiningModeShowRewardPackage = 1319000,
    OpVirtualWorldReqClientMiningModeShowSelectEquipmentPanel = 1319005,
    OpVirtualWorldResClientMiningModeActiveEquipment = 1319008,
    OpVirtualWorldResClientMiningModeQueryMinePackage = 1319009,
    OpVirtualWorldResClientSoundCtl = 1319003,
    OpVirtualWorldResClientSoundSetting = 1319004,
    OpEditorReqClientAddSpritesWithLocs = 1319001,
    OpEditorReqClientDeleteSpritesWithLocs = 1319002,
    OpEditorReqClientAddMosses = 1319010,
    OpEditorReqClientDeleteMosses = 1319011,
    OpEditorReqClientSyncMosses = 1319012,
    OpVirtualWorldReqClientShowRewardTips = 1319013,
    OpEditorReqClientAddScenery = 1319014,
    OpEditorReqClientDeleteScenery = 1319015,
    OpEditorReqClientUpdateScenery = 1319016,
    OpEditorReqClientFetchScenery = 1319017,
    OpVirtualWorldReqClientPktPlayerInfo = 1319024,
    OpVirtualWorldResClientPktSelfPlayerInfo = 1319025,
    OpVirtualWorldResClientPktAnotherPlayerInfo = 1319026,
    OpVirtualWorldResClientPktCraftSkills = 1319027,
    OpVirtualWorldResClientPktCraftQueryFormula = 1319028,
    OpVirtualWorldResClientPktResetAvatar = 1319029,
    OpVirtualWorldResClientPktCurrentDressAvatarItemId = 1319030,
    OpVirtualWorldResClientPktQueryQuestList = 1319031,
    OpVirtualWorldResClientPktQueryQuestDetail = 1319032,
    OpVirtualWorldReqClientMarketShowMarketByName = 1319033,
    OpVirtualWorldReqClientPktRefreshActiveUi = 1319034,
    OpVirtualWorldReqClientSyncState = 1319035,
    OpVirtualWorldResClientPktUnlockElementRequirement = 1319036,
    OpVirtualWorldResClientPktHandheld = 1319037,
    OpVirtualWorldResClientPktIndustryModels = 1319038,
    OpVirtualWorldResClientPktMyStore = 1319040,
    OpVirtualWorldResClientPktCommercialStreet = 1319041,
    OpVirtualWorldResClientPktMarketPlanModelsByType = 1319042,
    OpVirtualWorldResClientPktMarketPlan = 1319043,
    OpVirtualWorldResClientPktQueryStoreRankingList = 1319044,
    OpVirtualWorldResClientPktQueryStoreRankingDetail = 1319045,
    OpVirtualWorldResClientPktQueryStoreRankingReward = 1319046,
    OpVirtualWorldResClientPktQueryStoreEnterHistory = 1319047,
    OpVirtualWorldResClientPktSyncPackage = 1319048,
    OpVirtualWorldResClientPktUpdatePackage = 1319049,
    OpVirtualWorldResClientPktSearchRoom = 1319050,
    OpVirtualWorldResClientPktPlayerList = 1319051,
    OpVirtualWorldResClientPktSearchPlayer = 1319052,
    OpVirtualWorldResClientPktOrderList = 1319053,
    OpVirtualWorldResClientPktQueryPlayerProgress = 1319054,
    OpVirtualWorldResClientPktJobList = 1319056,
    OpVirtualWorldResClientPktQueryRoomRefurbishRequirements = 1319057,
    OpVirtualWorldResClientPktPartyList = 1319058,
    OpVirtualWorldResClientPktCreatePartyRequirements = 1319059,
    OpVirtualWorldResClientPktRequireFurnitureUnfrozenRequirements = 1319060,
    OpVirtualWorldResClientPktStreetList = 1319061,
    OpVirtualWorldResClientSetPosition = 1319062,
    OpVirtualWorldResClientStop = 1319063,
    OpVirtualWorldResClientTerrainCollection = 1319064,
    OpVirtualWorldResClientPktQueryConfigs = 1319065,
    /// 激活物件交互行为
    OpVirtualWorldResClientActiveSprite = 1319067,
    /// 交互行为停止后广播
    OpVirtualWorldResClientActiveSpriteEnd = 1319068,
    /// 派对中发送礼物的广播协议
    OpVirtualWorldResClientPktPartySendGift = 1319066,
    /// 团队共建协议
    OpVirtualWorldResClientPktTeambuildElementRequirement = 1319072,
    /// 玩家信息相关 20b0~20bf
    ///
    /// 当前场景内玩家信息
    OpVirtualWorldResClientPktCurrentRoomPlayerList = 1319088,
    /// 返回主线任务组
    OpVirtualWorldResClientPktQueryQuestGroup = 1319089,
    /// 新版创建角色UI 2020/12/22
    OpVirtualWorldReqClientPktShowCreateRoleUi = 1319091,
    /// 新版创建角色关闭UI
    OpVirtualWorldReqClientPktCloseCreateRoleUi = 1319092,
    /// 玩家引导信息
    OpVirtualWorldReqClientPktGuideData = 1319093,
    /// 编辑模式
    ///
    /// 返回客户端开启编辑结果
    OpVirtualWorldResClientStartEditModel = 1319168,
    /// 返回客户端编辑结果
    OpVirtualWorldResClientEditModelResult = 1319169,
    /// 重新加载场景
    OpVirtualWorldReqClientNoticeReloadScene = 1319170,
    /// 修复家具成功通知
    OpVirtualWorldReqClientUnlockDone = 1319171,
    /// 修复家具成功通知
    OpVirtualWorldReqClientInvestigateSuccess = 1319172,
    /// 游戏设置 20c0~20cf
    /// avatar模式 suit_type or avatar
    OpVirtualWorldReqClientGameMode = 1319104,
    /// 返回房间列表
    OpVirtualWorldResClientRoomList = 1319105,
    /// 返回自己的房间列表
    OpVirtualWorldResClientSelfRoomList = 1319106,
    /// 玩家本身信息同步
    OpVirtualWorldReqClientSyncActor = 1323008,
    /// 抽卡相关
    ///
    /// 抽卡结果
    OpVirtualWorldResClientPktDrawResult = 1323025,
    /// 抽卡状态返回
    OpVirtualWorldResClientPktDrawStatusResult = 1323026,
    /// 合成与重铸
    ///
    /// 合成结果
    OpVirtualWorldResClientPktCombineResult = 1323041,
    /// 重铸结果
    OpVirtualWorldResClientPktForgeResult = 1323042,
    /// 重铸目标列表结果
    OpVirtualWorldResClientPktForgeListResult = 1323043,
    /// 探索关卡
    ///
    /// 请求章节数据结果
    OpVirtualWorldResClientPktQueryChapterResult = 1323057,
    /// 通关结算数据
    OpVirtualWorldResClientPktExploreSummary = 1323058,
    /// 关卡中更新目标列表
    OpVirtualWorldResClientPktExploreRequireList = 1323059,
    /// 更新关卡进度
    OpVirtualWorldResClientPktExploreChapterProgress = 1323060,
    /// 显示倒计时
    OpVirtualWorldResClientPktExploreShowCountdown = 1323061,
    /// 显示下一步指引
    OpVirtualWorldResClientPktRoomShowGuideText = 1323073,
    /// 商店
    ///
    /// 商店商品列表
    OpVirtualWorldResClientPktShopData = 1323089,
    /// 图鉴
    ///
    /// 更新图鉴信息（注意这个可能是局部更新）
    OpClientReqVirtualWorldPktUpdateGallery = 1323094,
    /// 更新收藏图鉴信息（注意这个可能是局部更新）
    OpClientReqVirtualWorldPktUpdateGalleryCollectionData = 1323095,
    /// 邮件
    ///
    /// 发送邮件信息
    OpClientReqVirtualWorldPktUpdateMailsData = 1323105,
    /// 发送红点状态
    OpClientReqVirtualRedDotStatus = 1323121,
    /// 通用协议
    ///
    /// 获得高价值物品
    OpVirtualWorldReqClientShowHighQualityRewardTips = 1372161,
    /// 弹出闪亮面板
    OpVirtualWorldReqClientShowBlingPanel = 1372162,
    /// 2020.12.02 PKT协议 0x00145
    /// 人物相关   000~03f
    ///
    /// 装扮保存状态返回
    OpVirtualWorldResClientPktDressUpAvatar = 1331200,
    /// 升级
    OpVirtualWorldResClientPktLevelUp = 1331201,
    OpVirtualWorldResClientTest = 1376240,
}
impl Opcode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Opcode::OpMajor => "OP_MAJOR",
            Opcode::OpUnknown => "_OP_UNKNOWN",
            Opcode::OpClientPing => "_OP_CLIENT_PING",
            Opcode::OpGalaxyResClientStartNewWorld => "_OP_GALAXY_RES_CLIENT_START_NEW_WORLD",
            Opcode::OpGatewayResClientError => "_OP_GATEWAY_RES_CLIENT_ERROR",
            Opcode::OpGatewayReqClientMoveCharacter => "_OP_GATEWAY_REQ_CLIENT_MOVE_CHARACTER",
            Opcode::OpGatewayResClientStartNewWorld => "_OP_GATEWAY_RES_CLIENT_START_NEW_WORLD",
            Opcode::OpGatewayResClientVirtualWorldInit => "_OP_GATEWAY_RES_CLIENT_VIRTUAL_WORLD_INIT",
            Opcode::OpGatewayReqClientMoveElement => "_OP_GATEWAY_REQ_CLIENT_MOVE_ELEMENT",
            Opcode::OpGatewayReqClientSetCharacterPosition => "_OP_GATEWAY_REQ_CLIENT_SET_CHARACTER_POSITION",
            Opcode::OpGatewayReqClientSetElementPosition => "_OP_GATEWAY_REQ_CLIENT_SET_ELEMENT_POSITION",
            Opcode::OpGatewayReqClientChangeElementAnimation => "_OP_GATEWAY_REQ_CLIENT_CHANGE_ELEMENT_ANIMATION",
            Opcode::OpGatewayResClientPong => "_OP_GATEWAY_RES_CLIENT_PONG",
            Opcode::OpVirtualWorldReqClientShowPlacard => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SHOW_PLACARD",
            Opcode::OpVirtualWorldReqClientSceneTransitions => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SCENE_TRANSITIONS",
            Opcode::OpVirtualWorldReqClientChangeTerrainAnimation => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHANGE_TERRAIN_ANIMATION",
            Opcode::OpVirtualWorldReqClientChangeCharacterAnimation => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHANGE_CHARACTER_ANIMATION",
            Opcode::OpVirtualWorldReqClientChangeItemAnimation => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHANGE_ITEM_ANIMATION",
            Opcode::OpVirtualWorldReqClientForceOffline => "_OP_VIRTUAL_WORLD_REQ_CLIENT_FORCE_OFFLINE",
            Opcode::OpVirtualWorldReqClientCharacterTalking => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHARACTER_TALKING",
            Opcode::OpVirtualWorldReqClientCharacterShutUp => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHARACTER_SHUT_UP",
            Opcode::OpVirtualWorldResClientEnterScene => "_OP_VIRTUAL_WORLD_RES_CLIENT_ENTER_SCENE",
            Opcode::OpVirtualWorldResClientSelectCharacter => "_OP_VIRTUAL_WORLD_RES_CLIENT_SELECT_CHARACTER",
            Opcode::OpVirtualWorldResClientSynchroCharacter => "_OP_VIRTUAL_WORLD_RES_CLIENT_SYNCHRO_CHARACTER",
            Opcode::OpVirtualWorldResClientChat => "_OP_VIRTUAL_WORLD_RES_CLIENT_CHAT",
            Opcode::OpVirtualWorldResClientLuaLog => "_OP_VIRTUAL_WORLD_RES_CLIENT_LUA_LOG",
            Opcode::OpVirtualWorldResClientAddElement => "_OP_VIRTUAL_WORLD_RES_CLIENT_ADD_ELEMENT",
            Opcode::OpVirtualWorldResClientRemoveElement => "_OP_VIRTUAL_WORLD_RES_CLIENT_REMOVE_ELEMENT",
            Opcode::OpVirtualWorldResClientUiBindAttribute => "_OP_VIRTUAL_WORLD_RES_CLIENT_UI_BIND_ATTRIBUTE",
            Opcode::OpVirtualWorldResClientAddCharacter => "_OP_VIRTUAL_WORLD_RES_CLIENT_ADD_CHARACTER",
            Opcode::OpVirtualWorldResClientRemoveCharacter => "_OP_VIRTUAL_WORLD_RES_CLIENT_REMOVE_CHARACTER",
            Opcode::OpVirtualWorldResClientAddTerrain => "_OP_VIRTUAL_WORLD_RES_CLIENT_ADD_TERRAIN",
            Opcode::OpVirtualWorldResClientGameOver => "_OP_VIRTUAL_WORLD_RES_CLIENT_GAME_OVER",
            Opcode::OpVirtualWorldResClientChangeScene => "_OP_VIRTUAL_WORLD_RES_CLIENT_CHANGE_SCENE",
            Opcode::OpVirtualWorldResClientAddTerrainEnd => "_OP_VIRTUAL_WORLD_RES_CLIENT_ADD_TERRAIN_END",
            Opcode::OpVirtualWorldResClientSynchroPackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_SYNCHRO_PACKAGE",
            Opcode::OpVirtualWorldResClientAddItem => "_OP_VIRTUAL_WORLD_RES_CLIENT_ADD_ITEM",
            Opcode::OpVirtualWorldResClientRemoveItem => "_OP_VIRTUAL_WORLD_RES_CLIENT_REMOVE_ITEM",
            Opcode::OpVirtualWorldResClientExchangeItemPos => "_OP_VIRTUAL_WORLD_RES_CLIENT_EXCHANGE_ITEM_POS",
            Opcode::OpVirtualWorldResClientRemoveTerrain => "_OP_VIRTUAL_WORLD_RES_CLIENT_REMOVE_TERRAIN",
            Opcode::OpVirtualWorldResClientResetCameraSize => "_OP_VIRTUAL_WORLD_RES_CLIENT_RESET_CAMERA_SIZE",
            Opcode::OpVirtualWorldResClientShowUi => "_OP_VIRTUAL_WORLD_RES_CLIENT_SHOW_UI",
            Opcode::OpVirtualWorldResClientPackageItemUse => "_OP_VIRTUAL_WORLD_RES_CLIENT_PACKAGE_ITEM_USE",
            Opcode::OpVirtualWorldResClientStoredItem => "_OP_VIRTUAL_WORLD_RES_CLIENT_STORED_ITEM",
            Opcode::OpVirtualWorldResClientCloseUi => "_OP_VIRTUAL_WORLD_RES_CLIENT_CLOSE_UI",
            Opcode::OpVirtualWorldResClientOpenPackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_OPEN_PACKAGE",
            Opcode::OpVirtualWorldResClientQcloudGmeAuthbuffer => "_OP_VIRTUAL_WORLD_RES_CLIENT_QCLOUD_GME_AUTHBUFFER",
            Opcode::OpVirtualWorldResClientNotice => "_OP_VIRTUAL_WORLD_RES_CLIENT_NOTICE",
            Opcode::OpVirtualWorldResClientOnlyBubble => "_OP_VIRTUAL_WORLD_RES_CLIENT_ONLY_BUBBLE",
            Opcode::OpVirtualWorldResClientOnlyBubbleClean => "_OP_VIRTUAL_WORLD_RES_CLIENT_ONLY_BUBBLE_CLEAN",
            Opcode::OpVirtualWorldResClientSynchroShop => "_OP_VIRTUAL_WORLD_RES_CLIENT_SYNCHRO_SHOP",
            Opcode::OpVirtualWorldResClientQueryPackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_QUERY_PACKAGE",
            Opcode::OpVirtualWorldResClientSyncUserBalance => "_OP_VIRTUAL_WORLD_RES_CLIENT_SYNC_USER_BALANCE",
            Opcode::OpVirtualWorldResClientUpdateUi => "_OP_VIRTUAL_WORLD_RES_CLIENT_UPDATE_UI",
            Opcode::OpVirtualWorldResClientShowEffect => "_OP_VIRTUAL_WORLD_RES_CLIENT_SHOW_EFFECT",
            Opcode::OpVirtualWorldResClientSyncTime => "_OP_VIRTUAL_WORLD_RES_CLIENT_SYNC_TIME",
            Opcode::OpVirtualWorldReqClientMoveSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_MOVE_SPRITE",
            Opcode::OpVirtualWorldReqClientTriggerMoveSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_TRIGGER_MOVE_SPRITE",
            Opcode::OpVirtualWorldResClientCustomProto => "_OP_VIRTUAL_WORLD_RES_CLIENT_CUSTOM_PROTO",
            Opcode::OpEditorReqClientSetEditorMode => "_OP_EDITOR_REQ_CLIENT_SET_EDITOR_MODE",
            Opcode::OpEditorReqClientSetEditorElement => "_OP_EDITOR_REQ_CLIENT_SET_EDITOR_ELEMENT",
            Opcode::OpEditorReqClientAddElement => "_OP_EDITOR_REQ_CLIENT_ADD_ELEMENT",
            Opcode::OpEditorReqClientAddTerrain => "_OP_EDITOR_REQ_CLIENT_ADD_TERRAIN",
            Opcode::OpEditorReqClientDeleteElement => "_OP_EDITOR_REQ_CLIENT_DELETE_ELEMENT",
            Opcode::OpEditorReqClientDeleteTerrain => "_OP_EDITOR_REQ_CLIENT_DELETE_TERRAIN",
            Opcode::OpEditorReqClientMouseFollow => "_OP_EDITOR_REQ_CLIENT_MOUSE_FOLLOW",
            Opcode::OpEditorReqClientSelectElement => "_OP_EDITOR_REQ_CLIENT_SELECT_ELEMENT",
            Opcode::OpEditorReqClientFixedToElement => "_OP_EDITOR_REQ_CLIENT_FIXED_TO_ELEMENT",
            Opcode::OpEditorReqClientSyncElement => "_OP_EDITOR_REQ_CLIENT_SYNC_ELEMENT",
            Opcode::OpEditorReqClientAlignGrid => "_OP_EDITOR_REQ_CLIENT_ALIGN_GRID",
            Opcode::OpEditorReqClientVisibleGrid => "_OP_EDITOR_REQ_CLIENT_VISIBLE_GRID",
            Opcode::OpEditorReqClientCreateSprite => "_OP_EDITOR_REQ_CLIENT_CREATE_SPRITE",
            Opcode::OpEditorReqClientDeleteSprite => "_OP_EDITOR_REQ_CLIENT_DELETE_SPRITE",
            Opcode::OpEditorReqClientSyncSprite => "_OP_EDITOR_REQ_CLIENT_SYNC_SPRITE",
            Opcode::OpVirtualWorldReqClientSyncSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SYNC_SPRITE",
            Opcode::OpVirtualWorldReqClientAddSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_ADD_SPRITE",
            Opcode::OpVirtualWorldReqClientDeleteSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_DELETE_SPRITE",
            Opcode::OpVirtualWorldReqClientAdjustPosition => "_OP_VIRTUAL_WORLD_REQ_CLIENT_ADJUST_POSITION",
            Opcode::OpVirtualWorldReqClientCurrentSceneAllSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CURRENT_SCENE_ALL_SPRITE",
            Opcode::OpVirtualWorldReqClientGotoAnotherGame => "_OP_VIRTUAL_WORLD_REQ_CLIENT_GOTO_ANOTHER_GAME",
            Opcode::OpVirtualWorldReqClientChangeToEditorMode => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHANGE_TO_EDITOR_MODE",
            Opcode::OpEditorReqClientChangeToEditorMode => "_OP_EDITOR_REQ_CLIENT_CHANGE_TO_EDITOR_MODE",
            Opcode::OpVirtualWorldReqClientMouseSelectedSprite => "_OP_VIRTUAL_WORLD_REQ_CLIENT_MOUSE_SELECTED_SPRITE",
            Opcode::OpEditorReqClientMouseSelectedSprite => "_OP_EDITOR_REQ_CLIENT_MOUSE_SELECTED_SPRITE",
            Opcode::OpEditorReqClientFetchSprite => "_OP_EDITOR_REQ_CLIENT_FETCH_SPRITE",
            Opcode::OpVirtualWorldReqClientSetCameraFollow => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SET_CAMERA_FOLLOW",
            Opcode::OpVirtualWorldReqClientChangeSpriteAnimation => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CHANGE_SPRITE_ANIMATION",
            Opcode::OpVirtualWorldReqClientAddSpriteEnd => "_OP_VIRTUAL_WORLD_REQ_CLIENT_ADD_SPRITE_END",
            Opcode::OpVirtualWorldReqClientEnableEditMode => "_OP_VIRTUAL_WORLD_REQ_CLIENT_ENABLE_EDIT_MODE",
            Opcode::OpVirtualWorldResClientEditModeQueryEditPackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_QUERY_EDIT_PACKAGE",
            Opcode::OpVirtualWorldResClientEditModeReady => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_READY",
            Opcode::OpVirtualWorldResClientEditModeSelectedSprite => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_SELECTED_SPRITE",
            Opcode::OpVirtualWorldReqClientSetSpritePosition => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SET_SPRITE_POSITION",
            Opcode::OpVirtualWorldResClientCreateRoleGenerateNewName => "_OP_VIRTUAL_WORLD_RES_CLIENT_CREATE_ROLE_GENERATE_NEW_NAME",
            Opcode::OpVirtualWorldResClientCreateRoleErrorMessage => "_OP_VIRTUAL_WORLD_RES_CLIENT_CREATE_ROLE_ERROR_MESSAGE",
            Opcode::OpVirtualWorldReqClientShowCreateRoleUi => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SHOW_CREATE_ROLE_UI",
            Opcode::OpVirtualWorldReqClientCloseCreateRoleUi => "_OP_VIRTUAL_WORLD_REQ_CLIENT_CLOSE_CREATE_ROLE_UI",
            Opcode::OpVirtualWorldResClientGetMarketCategories => "_OP_VIRTUAL_WORLD_RES_CLIENT_GET_MARKET_CATEGORIES",
            Opcode::OpVirtualWorldResClientMarketQuery => "_OP_VIRTUAL_WORLD_RES_CLIENT_MARKET_QUERY",
            Opcode::OpVirtualWorldResClientMarketQueryCommodityResource => "_OP_VIRTUAL_WORLD_RES_CLIENT_MARKET_QUERY_COMMODITY_RESOURCE",
            Opcode::OpVirtualWorldReqClientEnableMarket => "_OP_VIRTUAL_WORLD_REQ_CLIENT_ENABLE_MARKET",
            Opcode::OpVirtualWorldResClientMarketQueryPackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_MARKET_QUERY_PACKAGE",
            Opcode::OpVirtualWorldResClientEditModeAddSpriteError => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ADD_SPRITE_ERROR",
            Opcode::OpVirtualWorldReqClientMoveSpriteByPath => "_OP_VIRTUAL_WORLD_REQ_CLIENT_MOVE_SPRITE_BY_PATH",
            Opcode::OpVirtualWorldReqClientUnwalkableBitMap => "_OP_VIRTUAL_WORLD_REQ_CLIENT_UNWALKABLE_BIT_MAP",
            Opcode::OpVirtualWorldResClientEditModeGetSpawnPoint => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_GET_SPAWN_POINT",
            Opcode::OpVirtualWorldResClientEditModeRoomInfo => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ROOM_INFO",
            Opcode::OpVirtualWorldResClientEditModeUpdateRoomInfo => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_UPDATE_ROOM_INFO",
            Opcode::OpVirtualWorldResClientEditModeRoomList => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ROOM_LIST",
            Opcode::OpVirtualWorldResClientEditModeRotateSprite => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ROTATE_SPRITE",
            Opcode::OpVirtualWorldResClientEditModeEnterRoom => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ENTER_ROOM",
            Opcode::OpVirtualWorldResClientEditModeAddSpriteByType => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ADD_SPRITE_BY_TYPE",
            Opcode::OpVirtualWorldResClientEditModeAddSingleSpriteByType => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_ADD_SINGLE_SPRITE_BY_TYPE",
            Opcode::OpVirtualWorldResClientEditModeGetPlayerEnterRoomHistory => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_GET_PLAYER_ENTER_ROOM_HISTORY",
            Opcode::OpVirtualWorldResClientEditModeGetPackageCategories => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODE_GET_PACKAGE_CATEGORIES",
            Opcode::OpVirtualWorldResClientMarketQueryPackageItemResource => "_OP_VIRTUAL_WORLD_RES_CLIENT_MARKET_QUERY_PACKAGE_ITEM_RESOURCE",
            Opcode::OpVirtualWorldReqClientShowInteractiveBubble => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SHOW_INTERACTIVE_BUBBLE",
            Opcode::OpVirtualWordlReqClientRemoveInteractiveBubble => "_OP_VIRTUAL_WORDL_REQ_CLIENT_REMOVE_INTERACTIVE_BUBBLE",
            Opcode::OpVirtualWorldResClientMoveSprite => "_OP_VIRTUAL_WORLD_RES_CLIENT_MOVE_SPRITE",
            Opcode::OpVirtualWorldReqClientMiningModeShowRewardPackage => "_OP_VIRTUAL_WORLD_REQ_CLIENT_MINING_MODE_SHOW_REWARD_PACKAGE",
            Opcode::OpVirtualWorldReqClientMiningModeShowSelectEquipmentPanel => "_OP_VIRTUAL_WORLD_REQ_CLIENT_MINING_MODE_SHOW_SELECT_EQUIPMENT_PANEL",
            Opcode::OpVirtualWorldResClientMiningModeActiveEquipment => "_OP_VIRTUAL_WORLD_RES_CLIENT_MINING_MODE_ACTIVE_EQUIPMENT",
            Opcode::OpVirtualWorldResClientMiningModeQueryMinePackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_MINING_MODE_QUERY_MINE_PACKAGE",
            Opcode::OpVirtualWorldResClientSoundCtl => "_OP_VIRTUAL_WORLD_RES_CLIENT_SOUND_CTL",
            Opcode::OpVirtualWorldResClientSoundSetting => "_OP_VIRTUAL_WORLD_RES_CLIENT_SOUND_SETTING",
            Opcode::OpEditorReqClientAddSpritesWithLocs => "_OP_EDITOR_REQ_CLIENT_ADD_SPRITES_WITH_LOCS",
            Opcode::OpEditorReqClientDeleteSpritesWithLocs => "_OP_EDITOR_REQ_CLIENT_DELETE_SPRITES_WITH_LOCS",
            Opcode::OpEditorReqClientAddMosses => "_OP_EDITOR_REQ_CLIENT_ADD_MOSSES",
            Opcode::OpEditorReqClientDeleteMosses => "_OP_EDITOR_REQ_CLIENT_DELETE_MOSSES",
            Opcode::OpEditorReqClientSyncMosses => "_OP_EDITOR_REQ_CLIENT_SYNC_MOSSES",
            Opcode::OpVirtualWorldReqClientShowRewardTips => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SHOW_REWARD_TIPS",
            Opcode::OpEditorReqClientAddScenery => "_OP_EDITOR_REQ_CLIENT_ADD_SCENERY",
            Opcode::OpEditorReqClientDeleteScenery => "_OP_EDITOR_REQ_CLIENT_DELETE_SCENERY",
            Opcode::OpEditorReqClientUpdateScenery => "_OP_EDITOR_REQ_CLIENT_UPDATE_SCENERY",
            Opcode::OpEditorReqClientFetchScenery => "_OP_EDITOR_REQ_CLIENT_FETCH_SCENERY",
            Opcode::OpVirtualWorldReqClientPktPlayerInfo => "_OP_VIRTUAL_WORLD_REQ_CLIENT_PKT_PLAYER_INFO",
            Opcode::OpVirtualWorldResClientPktSelfPlayerInfo => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_SELF_PLAYER_INFO",
            Opcode::OpVirtualWorldResClientPktAnotherPlayerInfo => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_ANOTHER_PLAYER_INFO",
            Opcode::OpVirtualWorldResClientPktCraftSkills => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_CRAFT_SKILLS",
            Opcode::OpVirtualWorldResClientPktCraftQueryFormula => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_CRAFT_QUERY_FORMULA",
            Opcode::OpVirtualWorldResClientPktResetAvatar => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_RESET_AVATAR",
            Opcode::OpVirtualWorldResClientPktCurrentDressAvatarItemId => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_CURRENT_DRESS_AVATAR_ITEM_ID",
            Opcode::OpVirtualWorldResClientPktQueryQuestList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_QUEST_LIST",
            Opcode::OpVirtualWorldResClientPktQueryQuestDetail => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_QUEST_DETAIL",
            Opcode::OpVirtualWorldReqClientMarketShowMarketByName => "_OP_VIRTUAL_WORLD_REQ_CLIENT_MARKET_SHOW_MARKET_BY_NAME",
            Opcode::OpVirtualWorldReqClientPktRefreshActiveUi => "_OP_VIRTUAL_WORLD_REQ_CLIENT_PKT_REFRESH_ACTIVE_UI",
            Opcode::OpVirtualWorldReqClientSyncState => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SYNC_STATE",
            Opcode::OpVirtualWorldResClientPktUnlockElementRequirement => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_UNLOCK_ELEMENT_REQUIREMENT",
            Opcode::OpVirtualWorldResClientPktHandheld => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_HANDHELD",
            Opcode::OpVirtualWorldResClientPktIndustryModels => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_INDUSTRY_MODELS",
            Opcode::OpVirtualWorldResClientPktMyStore => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_MY_STORE",
            Opcode::OpVirtualWorldResClientPktCommercialStreet => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_COMMERCIAL_STREET",
            Opcode::OpVirtualWorldResClientPktMarketPlanModelsByType => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_MARKET_PLAN_MODELS_BY_TYPE",
            Opcode::OpVirtualWorldResClientPktMarketPlan => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_MARKET_PLAN",
            Opcode::OpVirtualWorldResClientPktQueryStoreRankingList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_STORE_RANKING_LIST",
            Opcode::OpVirtualWorldResClientPktQueryStoreRankingDetail => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_STORE_RANKING_DETAIL",
            Opcode::OpVirtualWorldResClientPktQueryStoreRankingReward => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_STORE_RANKING_REWARD",
            Opcode::OpVirtualWorldResClientPktQueryStoreEnterHistory => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_STORE_ENTER_HISTORY",
            Opcode::OpVirtualWorldResClientPktSyncPackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_SYNC_PACKAGE",
            Opcode::OpVirtualWorldResClientPktUpdatePackage => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_UPDATE_PACKAGE",
            Opcode::OpVirtualWorldResClientPktSearchRoom => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_SEARCH_ROOM",
            Opcode::OpVirtualWorldResClientPktPlayerList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_PLAYER_LIST",
            Opcode::OpVirtualWorldResClientPktSearchPlayer => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_SEARCH_PLAYER",
            Opcode::OpVirtualWorldResClientPktOrderList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_ORDER_LIST",
            Opcode::OpVirtualWorldResClientPktQueryPlayerProgress => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_PLAYER_PROGRESS",
            Opcode::OpVirtualWorldResClientPktJobList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_JOB_LIST",
            Opcode::OpVirtualWorldResClientPktQueryRoomRefurbishRequirements => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_ROOM_REFURBISH_REQUIREMENTS",
            Opcode::OpVirtualWorldResClientPktPartyList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_PARTY_LIST",
            Opcode::OpVirtualWorldResClientPktCreatePartyRequirements => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_CREATE_PARTY_REQUIREMENTS",
            Opcode::OpVirtualWorldResClientPktRequireFurnitureUnfrozenRequirements => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_REQUIRE_FURNITURE_UNFROZEN_REQUIREMENTS",
            Opcode::OpVirtualWorldResClientPktStreetList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_STREET_LIST",
            Opcode::OpVirtualWorldResClientSetPosition => "_OP_VIRTUAL_WORLD_RES_CLIENT_SET_POSITION",
            Opcode::OpVirtualWorldResClientStop => "_OP_VIRTUAL_WORLD_RES_CLIENT_STOP",
            Opcode::OpVirtualWorldResClientTerrainCollection => "_OP_VIRTUAL_WORLD_RES_CLIENT_TERRAIN_COLLECTION",
            Opcode::OpVirtualWorldResClientPktQueryConfigs => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_CONFIGS",
            Opcode::OpVirtualWorldResClientActiveSprite => "_OP_VIRTUAL_WORLD_RES_CLIENT_ACTIVE_SPRITE",
            Opcode::OpVirtualWorldResClientActiveSpriteEnd => "_OP_VIRTUAL_WORLD_RES_CLIENT_ACTIVE_SPRITE_END",
            Opcode::OpVirtualWorldResClientPktPartySendGift => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_PARTY_SEND_GIFT",
            Opcode::OpVirtualWorldResClientPktTeambuildElementRequirement => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_TEAMBUILD_ELEMENT_REQUIREMENT",
            Opcode::OpVirtualWorldResClientPktCurrentRoomPlayerList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_CURRENT_ROOM_PLAYER_LIST",
            Opcode::OpVirtualWorldResClientPktQueryQuestGroup => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_QUEST_GROUP",
            Opcode::OpVirtualWorldReqClientPktShowCreateRoleUi => "_OP_VIRTUAL_WORLD_REQ_CLIENT_PKT_SHOW_CREATE_ROLE_UI",
            Opcode::OpVirtualWorldReqClientPktCloseCreateRoleUi => "_OP_VIRTUAL_WORLD_REQ_CLIENT_PKT_CLOSE_CREATE_ROLE_UI",
            Opcode::OpVirtualWorldReqClientPktGuideData => "_OP_VIRTUAL_WORLD_REQ_CLIENT_PKT_GUIDE_DATA",
            Opcode::OpVirtualWorldResClientStartEditModel => "_OP_VIRTUAL_WORLD_RES_CLIENT_START_EDIT_MODEL",
            Opcode::OpVirtualWorldResClientEditModelResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_EDIT_MODEL_RESULT",
            Opcode::OpVirtualWorldReqClientNoticeReloadScene => "_OP_VIRTUAL_WORLD_REQ_CLIENT_NOTICE_RELOAD_SCENE",
            Opcode::OpVirtualWorldReqClientUnlockDone => "_OP_VIRTUAL_WORLD_REQ_CLIENT_UNLOCK_DONE",
            Opcode::OpVirtualWorldReqClientInvestigateSuccess => "_OP_VIRTUAL_WORLD_REQ_CLIENT_INVESTIGATE_SUCCESS",
            Opcode::OpVirtualWorldReqClientGameMode => "_OP_VIRTUAL_WORLD_REQ_CLIENT_GAME_MODE",
            Opcode::OpVirtualWorldResClientRoomList => "_OP_VIRTUAL_WORLD_RES_CLIENT_ROOM_LIST",
            Opcode::OpVirtualWorldResClientSelfRoomList => "_OP_VIRTUAL_WORLD_RES_CLIENT_SELF_ROOM_LIST",
            Opcode::OpVirtualWorldReqClientSyncActor => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SYNC_ACTOR",
            Opcode::OpVirtualWorldResClientPktDrawResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_DRAW_RESULT",
            Opcode::OpVirtualWorldResClientPktDrawStatusResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_DRAW_STATUS_RESULT",
            Opcode::OpVirtualWorldResClientPktCombineResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_COMBINE_RESULT",
            Opcode::OpVirtualWorldResClientPktForgeResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_FORGE_RESULT",
            Opcode::OpVirtualWorldResClientPktForgeListResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_FORGE_LIST_RESULT",
            Opcode::OpVirtualWorldResClientPktQueryChapterResult => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_QUERY_CHAPTER_RESULT",
            Opcode::OpVirtualWorldResClientPktExploreSummary => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_EXPLORE_SUMMARY",
            Opcode::OpVirtualWorldResClientPktExploreRequireList => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_EXPLORE_REQUIRE_LIST",
            Opcode::OpVirtualWorldResClientPktExploreChapterProgress => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_EXPLORE_CHAPTER_PROGRESS",
            Opcode::OpVirtualWorldResClientPktExploreShowCountdown => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_EXPLORE_SHOW_COUNTDOWN",
            Opcode::OpVirtualWorldResClientPktRoomShowGuideText => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_ROOM_SHOW_GUIDE_TEXT",
            Opcode::OpVirtualWorldResClientPktShopData => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_SHOP_DATA",
            Opcode::OpClientReqVirtualWorldPktUpdateGallery => "_OP_CLIENT_REQ_VIRTUAL_WORLD_PKT_UPDATE_GALLERY",
            Opcode::OpClientReqVirtualWorldPktUpdateGalleryCollectionData => "_OP_CLIENT_REQ_VIRTUAL_WORLD_PKT_UPDATE_GALLERY_COLLECTION_DATA",
            Opcode::OpClientReqVirtualWorldPktUpdateMailsData => "_OP_CLIENT_REQ_VIRTUAL_WORLD_PKT_UPDATE_MAILS_DATA",
            Opcode::OpClientReqVirtualRedDotStatus => "_OP_CLIENT_REQ_VIRTUAL_RED_DOT_STATUS",
            Opcode::OpVirtualWorldReqClientShowHighQualityRewardTips => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SHOW_HIGH_QUALITY_REWARD_TIPS",
            Opcode::OpVirtualWorldReqClientShowBlingPanel => "_OP_VIRTUAL_WORLD_REQ_CLIENT_SHOW_BLING_PANEL",
            Opcode::OpVirtualWorldResClientPktDressUpAvatar => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_DRESS_UP_AVATAR",
            Opcode::OpVirtualWorldResClientPktLevelUp => "_OP_VIRTUAL_WORLD_RES_CLIENT_PKT_LEVEL_UP",
            Opcode::OpVirtualWorldResClientTest => "_OP_VIRTUAL_WORLD_RES_CLIENT_TEST",
        }
    }
}
