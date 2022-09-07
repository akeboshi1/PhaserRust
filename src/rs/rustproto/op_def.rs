#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPoint2i {
    #[prost(int32, required, tag="1")]
    pub x: i32,
    #[prost(int32, required, tag="2")]
    pub y: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPoint3f {
    #[prost(float, required, tag="1")]
    pub x: f32,
    #[prost(float, required, tag="2")]
    pub y: f32,
    #[prost(float, optional, tag="3", default="0")]
    pub z: ::core::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MovePoint {
    #[prost(message, optional, tag="1")]
    pub pos: ::core::option::Option<PbPoint3f>,
    #[prost(uint64, optional, tag="2")]
    pub timestamp: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MovePath {
    #[prost(int32, optional, tag="1")]
    pub id: ::core::option::Option<i32>,
    #[prost(message, repeated, tag="2")]
    pub move_pos: ::prost::alloc::vec::Vec<MovePoint>,
}
/// editer by 7.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbPoint2f {
    #[prost(float, required, tag="1")]
    pub x: f32,
    #[prost(float, required, tag="2")]
    pub y: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionCount {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(int32, required, tag="2")]
    pub connection_count: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeCard {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub thumbnail: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="3")]
    pub valid_date: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MovingKeys {
    #[prost(enumeration="KeyCode", repeated, packed="false", tag="1")]
    pub up_keys: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="KeyCode", repeated, packed="false", tag="2")]
    pub down_keys: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="KeyCode", repeated, packed="false", tag="3")]
    pub right_keys: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="KeyCode", repeated, packed="false", tag="4")]
    pub left_keys: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyCodeEvent {
    #[prost(enumeration="TqEvent", required, tag="1")]
    pub tq_event: i32,
    #[prost(enumeration="KeyCode", repeated, packed="false", tag="2")]
    pub key_codes: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(int32, optional, tag="3")]
    pub id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="1")]
    pub current_frame: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub total_frame: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiDisplay {
    #[prost(enumeration="VerticalAlignment", required, tag="1")]
    pub vertical: i32,
    #[prost(enumeration="HorizontalAlignment", required, tag="2")]
    pub horizontal: i32,
    #[prost(string, optional, tag="3")]
    pub data_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub texture_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub tips: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    #[prost(message, repeated, tag="1")]
    pub table_pair: ::prost::alloc::vec::Vec<TablePair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TablePair {
    #[prost(oneof="table_pair::KeyOneof", tags="1, 2")]
    pub key_oneof: ::core::option::Option<table_pair::KeyOneof>,
    #[prost(oneof="table_pair::ValueOneof", tags="3, 4, 5, 6")]
    pub value_oneof: ::core::option::Option<table_pair::ValueOneof>,
}
/// Nested message and enum types in `TablePair`.
pub mod table_pair {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum KeyOneof {
        #[prost(string, tag="1")]
        Key(::prost::alloc::string::String),
        #[prost(int32, tag="2")]
        Index(i32),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ValueOneof {
        #[prost(double, tag="3")]
        NumberVal(f64),
        #[prost(string, tag="4")]
        StrVal(::prost::alloc::string::String),
        #[prost(bool, tag="5")]
        BoolVal(bool),
        #[prost(message, tag="6")]
        SubTable(super::Table),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntArray {
    #[prost(int32, repeated, packed="false", tag="1")]
    pub value: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntArrays {
    #[prost(message, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<IntArray>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCommodities {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(int32, required, tag="2")]
    pub quantity: i32,
    #[prost(string, required, tag="3")]
    pub category: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCategory {
    #[prost(message, required, tag="1")]
    pub category: StrPair,
    #[prost(message, repeated, tag="2")]
    pub subcategory: ::prost::alloc::vec::Vec<StrPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MossMetaData {
    #[prost(int32, required, tag="1")]
    pub key: i32,
    #[prost(float, required, tag="2")]
    pub x: f32,
    #[prost(float, required, tag="3")]
    pub y: f32,
    #[prost(float, optional, tag="4")]
    pub z: ::core::option::Option<f32>,
    #[prost(int32, optional, tag="5")]
    pub dir: ::core::option::Option<i32>,
    #[prost(int32, required, tag="6")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMap {
    #[prost(message, repeated, tag="1")]
    pub locations: ::prost::alloc::vec::Vec<MossMetaData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrPair {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntPair {
    #[prost(string, optional, tag="1")]
    pub keys: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="2")]
    pub value: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrMap {
    #[prost(message, repeated, tag="1")]
    pub pairs: ::prost::alloc::vec::Vec<StrPair>,
}
/// edit by 7. at0427-2020
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrArray {
    #[prost(string, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrArrays {
    #[prost(message, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<StrArray>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueBar {
    #[prost(int32, required, tag="1")]
    pub max: i32,
    #[prost(int32, required, tag="2")]
    pub current_value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="2")]
    pub packet: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// 现在都是UnknownNodeType,即发送方和接收方自定义packet结构，并解析，未来考虑将游戏原有结构类型也按此结构同步
    #[prost(enumeration="NodeType", optional, tag="3", default="UnknownNodeType")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(enumeration="ExecCode", optional, tag="4", default="Update")]
    pub exec_code: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mail {
    #[prost(int32, optional, tag="13")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, optional, tag="1")]
    pub mailbox_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub sender_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub receiver_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub sender_nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub receiver_nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub attachment: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="9")]
    pub created_at: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="10", default="false")]
    pub read: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="11", default="false")]
    pub received_attachment: ::core::option::Option<bool>,
    #[prost(string, repeated, tag="12")]
    pub param: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="14")]
    pub snap_mail: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailStatus {
    #[prost(int32, optional, tag="13")]
    pub id: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="10", default="false")]
    pub read: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="11", default="false")]
    pub received_attachment: ::core::option::Option<bool>,
    #[prost(string, repeated, tag="12")]
    pub param: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneralParam {
    #[prost(enumeration="GeneralParamType", required, tag="1")]
    pub t: i32,
    /// 为var_bytes预留
    #[prost(int32, optional, tag="2")]
    pub opcode: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub val_str: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub val_bool: ::core::option::Option<bool>,
    #[prost(int32, optional, tag="5")]
    pub val_num: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub val_bytes: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(double, optional, tag="1")]
    pub start: ::core::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub stop: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbGroup {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="DbGroupValueType", optional, tag="2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(float, optional, tag="3")]
    pub val_number: ::core::option::Option<f32>,
    #[prost(string, optional, tag="4")]
    pub val_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="5")]
    pub val_bool: ::core::option::Option<bool>,
    #[prost(message, optional, boxed, tag="6")]
    pub val_group: ::core::option::Option<::prost::alloc::boxed::Box<DbGroup>>,
    #[prost(message, repeated, tag="7")]
    pub val_group_array: ::prost::alloc::vec::Vec<DbGroup>,
    #[prost(float, repeated, packed="false", tag="8")]
    pub val_number_array: ::prost::alloc::vec::Vec<f32>,
    #[prost(string, repeated, tag="9")]
    pub val_string_array: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, repeated, packed="false", tag="10")]
    pub val_bool_array: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageData {
    #[prost(string, optional, tag="1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3", default="false")]
    pub deleted: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStatusData {
    #[prost(int32, required, tag="1")]
    pub uuid: i32,
    #[prost(enumeration="ServerType", required, tag="2")]
    pub r#type: i32,
    #[prost(enumeration="ServerStatus", required, tag="3")]
    pub status: i32,
    #[prost(string, optional, tag="4")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="5")]
    pub port: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceStatus {
    #[prost(enumeration="ServerStatus", required, tag="1")]
    pub status: i32,
    #[prost(string, optional, tag="2")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub port: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub uuid: ::core::option::Option<i32>,
    #[prost(enumeration="ServerType", optional, tag="5")]
    pub server_type: ::core::option::Option<i32>,
    #[prost(bytes="vec", optional, tag="6")]
    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpServiceReqInit {
    #[prost(message, required, tag="1")]
    pub service_status: ServiceStatus,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpServiceReqUploadStatus {
    #[prost(message, required, tag="1")]
    pub service_status: ServiceStatus,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpServiceReqRegisterServiceStatusListener {
    #[prost(uint32, optional, tag="1", default="0")]
    pub listener_mask: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpServiceReqTerminate {
    #[prost(int32, required, tag="1")]
    pub uuid: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    #[prost(uint32, optional, tag="1", default="0")]
    pub skip: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="2", default="1")]
    pub page: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub per_page: ::core::option::Option<u32>,
}
/// TODO auto gen Message by Header file
/// can not use the same name of message, because proto define unique error
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Opcode {
    /// _OP_WORLD_REQ_GALAXY_EXAMPLE                   = 0x00120000;
    OpUnknown = 0,
}
impl Opcode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Opcode::OpUnknown => "_OP_UNKNOWN",
        }
    }
}
// Match OPCODE enum and move first "_" 
// message OP_WORLD_REQ_GALAXY_EXAMPLE
// {
//     required int32  port=1;
//     optional string name=2;
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecCode {
    Find = 0,
    Add = 1,
    Update = 2,
    Delete = 3,
}
impl ExecCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecCode::Find => "EXEC_CODE_FIND",
            ExecCode::Add => "EXEC_CODE_ADD",
            ExecCode::Update => "EXEC_CODE_UPDATE",
            ExecCode::Delete => "EXEC_CODE_DELETE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CoinType {
    TuDingCoin = 0,
    QingSongTang = 1,
    GoldCoin = 2,
    Coin = 3,
    Diamond = 4,
}
impl CoinType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CoinType::TuDingCoin => "TU_DING_COIN",
            CoinType::QingSongTang => "QING_SONG_TANG",
            CoinType::GoldCoin => "GOLD_COIN",
            CoinType::Coin => "COIN",
            CoinType::Diamond => "DIAMOND",
        }
    }
}
/// 高位保留定义, 由服务器自定义, 后4位留给用户自定义
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TitleMask {
    TqNickName = 65536,
    /// TQ_   = 0x0004;
    TqBadge = 131072,
}
impl TitleMask {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TitleMask::TqNickName => "TQ_NickName",
            TitleMask::TqBadge => "TQ_Badge",
        }
    }
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserInput {
    TqInputMouse = 1,
    TqInputKeyboard = 2,
}
impl UserInput {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserInput::TqInputMouse => "TQ_Input_Mouse",
            UserInput::TqInputKeyboard => "TQ_Input_Keyboard",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResponseStatus {
    Success = 0,
    /// 服务端 未知异常
    ServerErrorUnknown = 4096,
    /// 服务端 网络忙
    ServerErrorBusy = 4098,
    /// 服务超时
    ServerErrorTimeout = 4099,
    /// 找不到对应的服务
    ServerErrorServiceNotFound = 4100,
    /// 0x20 gateway error
    ///
    /// 网关异常
    ServerErrorBadGateway = 8193,
    /// 0x30 galaxy error
    ///
    /// 没找到galaxy
    ServerErrorGalaxyServiceNotFound = 12289,
    /// 0x40 virtual world error
    ///
    /// 没找到world
    ServerErrorWorldServiceNotFound = 16385,
    /// 0x50 virtual error
    ///
    /// 没找到virtual world
    ServerErrorVirtualWorldServiceNotFound = 20481,
    /// 0x60 cop error
    /// 0x70 property error
    /// 0x80 login error
    /// 0x90 request error
    ///
    /// 参数错误
    RequestErrorWrongParam = 37888,
    /// 鉴权失败
    RequestUnauthorized = 37889,
    /// 0xb0 platform
    ///
    /// 支付失败
    RansferToGameBillerError = 45056,
}
impl ResponseStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResponseStatus::Success => "SUCCESS",
            ResponseStatus::ServerErrorUnknown => "SERVER_ERROR_UNKNOWN",
            ResponseStatus::ServerErrorBusy => "SERVER_ERROR_BUSY",
            ResponseStatus::ServerErrorTimeout => "SERVER_ERROR_TIMEOUT",
            ResponseStatus::ServerErrorServiceNotFound => "SERVER_ERROR_SERVICE_NOT_FOUND",
            ResponseStatus::ServerErrorBadGateway => "SERVER_ERROR_BAD_GATEWAY",
            ResponseStatus::ServerErrorGalaxyServiceNotFound => "SERVER_ERROR_GALAXY_SERVICE_NOT_FOUND",
            ResponseStatus::ServerErrorWorldServiceNotFound => "SERVER_ERROR_WORLD_SERVICE_NOT_FOUND",
            ResponseStatus::ServerErrorVirtualWorldServiceNotFound => "SERVER_ERROR_VIRTUAL_WORLD_SERVICE_NOT_FOUND",
            ResponseStatus::RequestErrorWrongParam => "REQUEST_ERROR_WRONG_PARAM",
            ResponseStatus::RequestUnauthorized => "REQUEST_UNAUTHORIZED",
            ResponseStatus::RansferToGameBillerError => "RANSFER_TO_GAME_BILLER_ERROR",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerType {
    Unknown = 0,
    Gateway = 1,
    Cop = 2,
    Galaxy = 4,
    World = 8,
    VWorld = 16,
    Portal = 32,
    Mail = 64,
    TicketSeller = 128,
    Storage = 256,
    Client = 512,
    Lobby = 1024,
    PortalCtl = 2048,
    Property = 251658240,
    Login = 15728641,
}
impl ServerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServerType::Unknown => "UNKNOWN",
            ServerType::Gateway => "GATEWAY",
            ServerType::Cop => "COP",
            ServerType::Galaxy => "GALAXY",
            ServerType::World => "WORLD",
            ServerType::VWorld => "V_WORLD",
            ServerType::Portal => "PORTAL",
            ServerType::Mail => "MAIL",
            ServerType::TicketSeller => "TICKET_SELLER",
            ServerType::Storage => "STORAGE",
            ServerType::Client => "CLIENT",
            ServerType::Lobby => "LOBBY",
            ServerType::PortalCtl => "PORTAL_CTL",
            ServerType::Property => "PROPERTY",
            ServerType::Login => "LOGIN",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChatChannel {
    CurrentScene = 0,
    World = 1,
    Team = 2,
    Private = 3,
    System = 4,
}
impl ChatChannel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChatChannel::CurrentScene => "CurrentScene",
            ChatChannel::World => "World",
            ChatChannel::Team => "Team",
            ChatChannel::Private => "Private",
            ChatChannel::System => "System",
        }
    }
}
//
// 方向`
// ---------------------
// |
// |    1   0    7
// |     ↖  ↑  ↗
// |   2 ← 方向 → 6
// |     ↙  ↓  ↘
// |    3   4    5
// ---------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    Up = 0,
    UpperLeft = 1,
    Left = 2,
    LowerLeft = 3,
    Down = 4,
    LowerRight = 5,
    Right = 6,
    UpperRight = 7,
}
impl Direction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Direction::Up => "UP",
            Direction::UpperLeft => "UPPER_LEFT",
            Direction::Left => "LEFT",
            Direction::LowerLeft => "LOWER_LEFT",
            Direction::Down => "DOWN",
            Direction::LowerRight => "LOWER_RIGHT",
            Direction::Right => "RIGHT",
            Direction::UpperRight => "UPPER_RIGHT",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoveType {
    Character = 0,
    Element = 1,
}
impl MoveType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MoveType::Character => "CHARACTER",
            MoveType::Element => "ELEMENT",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoiceRoomStatus {
    InVoiceRoom = 1,
    OutsideVoiceRoom = 2,
}
impl VoiceRoomStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoiceRoomStatus::InVoiceRoom => "InVoiceRoom",
            VoiceRoomStatus::OutsideVoiceRoom => "OutsideVoiceRoom",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameStatus {
    /// 失去焦点
    Blur = 1,
    /// 获得焦点
    Focus = 2,
}
impl GameStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GameStatus::Blur => "Blur",
            GameStatus::Focus => "Focus",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MouseEvent {
    RightMouseDown = 1,
    RightMouseUp = 2,
    LeftMouseDown = 3,
    LeftMouseUp = 4,
    WheelDown = 5,
    WheelUp = 6,
    RightMouseHolding = 7,
    LeftMouseHolding = 8,
    Tap = 9,
}
impl MouseEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MouseEvent::RightMouseDown => "RightMouseDown",
            MouseEvent::RightMouseUp => "RightMouseUp",
            MouseEvent::LeftMouseDown => "LeftMouseDown",
            MouseEvent::LeftMouseUp => "LeftMouseUp",
            MouseEvent::WheelDown => "WheelDown",
            MouseEvent::WheelUp => "WheelUp",
            MouseEvent::RightMouseHolding => "RightMouseHolding",
            MouseEvent::LeftMouseHolding => "LeftMouseHolding",
            MouseEvent::Tap => "Tap",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyCode {
    Unknown = -1,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    NumpadMultiply = 106,
    NumpadAdd = 107,
    NumpadEnter = 108,
    NumpadSubtract = 109,
    NumpadDecimal = 110,
    NumpadDivide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    Colon = 186,
    Equals = 187,
    Comma = 188,
    Underscore = 189,
    Period = 190,
    QuestionMark = 191,
    Tilde = 192,
    OpenBracket = 219,
    BackwardSlash = 220,
    ClosedBracket = 221,
    Quotes = 222,
    Backspace = 8,
    Tab = 9,
    Clear = 12,
    Enter = 13,
    Shift = 16,
    Control = 17,
    Alt = 18,
    CapsLock = 20,
    Esc = 27,
    Spacebar = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    KbLeft = 37,
    KbUp = 38,
    KbRight = 39,
    KbDown = 40,
    Plus = 43,
    Minus = 44,
    Insert = 45,
    Delete = 46,
    Help = 47,
    NumLock = 144,
}
impl KeyCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyCode::Unknown => "Unknown",
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",
            KeyCode::Zero => "ZERO",
            KeyCode::One => "ONE",
            KeyCode::Two => "TWO",
            KeyCode::Three => "THREE",
            KeyCode::Four => "FOUR",
            KeyCode::Five => "FIVE",
            KeyCode::Six => "SIX",
            KeyCode::Seven => "SEVEN",
            KeyCode::Eight => "EIGHT",
            KeyCode::Nine => "NINE",
            KeyCode::Numpad0 => "NUMPAD_0",
            KeyCode::Numpad1 => "NUMPAD_1",
            KeyCode::Numpad2 => "NUMPAD_2",
            KeyCode::Numpad3 => "NUMPAD_3",
            KeyCode::Numpad4 => "NUMPAD_4",
            KeyCode::Numpad5 => "NUMPAD_5",
            KeyCode::Numpad6 => "NUMPAD_6",
            KeyCode::Numpad7 => "NUMPAD_7",
            KeyCode::Numpad8 => "NUMPAD_8",
            KeyCode::Numpad9 => "NUMPAD_9",
            KeyCode::NumpadMultiply => "NUMPAD_MULTIPLY",
            KeyCode::NumpadAdd => "NUMPAD_ADD",
            KeyCode::NumpadEnter => "NUMPAD_ENTER",
            KeyCode::NumpadSubtract => "NUMPAD_SUBTRACT",
            KeyCode::NumpadDecimal => "NUMPAD_DECIMAL",
            KeyCode::NumpadDivide => "NUMPAD_DIVIDE",
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            KeyCode::F13 => "F13",
            KeyCode::F14 => "F14",
            KeyCode::F15 => "F15",
            KeyCode::Colon => "COLON",
            KeyCode::Equals => "EQUALS",
            KeyCode::Comma => "COMMA",
            KeyCode::Underscore => "UNDERSCORE",
            KeyCode::Period => "PERIOD",
            KeyCode::QuestionMark => "QUESTION_MARK",
            KeyCode::Tilde => "TILDE",
            KeyCode::OpenBracket => "OPEN_BRACKET",
            KeyCode::BackwardSlash => "BACKWARD_SLASH",
            KeyCode::ClosedBracket => "CLOSED_BRACKET",
            KeyCode::Quotes => "QUOTES",
            KeyCode::Backspace => "BACKSPACE",
            KeyCode::Tab => "TAB",
            KeyCode::Clear => "CLEAR",
            KeyCode::Enter => "ENTER",
            KeyCode::Shift => "SHIFT",
            KeyCode::Control => "CONTROL",
            KeyCode::Alt => "ALT",
            KeyCode::CapsLock => "CAPS_LOCK",
            KeyCode::Esc => "ESC",
            KeyCode::Spacebar => "SPACEBAR",
            KeyCode::PageUp => "PAGE_UP",
            KeyCode::PageDown => "PAGE_DOWN",
            KeyCode::End => "END",
            KeyCode::Home => "HOME",
            KeyCode::KbLeft => "KB_LEFT",
            KeyCode::KbUp => "KB_UP",
            KeyCode::KbRight => "KB_RIGHT",
            KeyCode::KbDown => "KB_DOWN",
            KeyCode::Plus => "PLUS",
            KeyCode::Minus => "MINUS",
            KeyCode::Insert => "INSERT",
            KeyCode::Delete => "DELETE",
            KeyCode::Help => "HELP",
            KeyCode::NumLock => "NUM_LOCK",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TerrainAnimationType {
    DynamicType = 16,
    StaticType = 1,
}
impl TerrainAnimationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TerrainAnimationType::DynamicType => "DYNAMIC_TYPE",
            TerrainAnimationType::StaticType => "STATIC_TYPE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodeType {
    UnknownNodeType = 0,
    GameNodeType = 1,
    SceneNodeType = 2,
    ElementNodeType = 3,
    TerrainNodeType = 4,
    CharacterNodeType = 5,
    LocationType = 6,
    MovableType = 7,
    DisplayType = 8,
    AttributeType = 9,
    FunctionType = 10,
    AnimationsType = 11,
    EventType = 12,
    MapSizeType = 13,
    UiType = 14,
    TimerType = 15,
    PackageType = 16,
    PackageItemType = 17,
    AvatarType = 18,
    SettingsType = 19,
    CampType = 20,
    MutexType = 21,
    AnimationDataType = 22,
    ForkType = 23,
    ButtonType = 24,
    TextType = 25,
    AccessType = 26,
    SpawnPointType = 27,
    CommodityType = 28,
    ShopType = 29,
    PaletteType = 30,
    TerrainCollectionType = 31,
    AssetsType = 32,
    MossType = 33,
    MossCollectionType = 34,
    SceneryType = 35,
    ModsType = 36,
    InputTextType = 37,
    WallNodeType = 38,
    WallCollectionType = 39,
    MountSpritesType = 40,
}
impl NodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NodeType::UnknownNodeType => "UnknownNodeType",
            NodeType::GameNodeType => "GameNodeType",
            NodeType::SceneNodeType => "SceneNodeType",
            NodeType::ElementNodeType => "ElementNodeType",
            NodeType::TerrainNodeType => "TerrainNodeType",
            NodeType::CharacterNodeType => "CharacterNodeType",
            NodeType::LocationType => "LocationType",
            NodeType::MovableType => "MovableType",
            NodeType::DisplayType => "DisplayType",
            NodeType::AttributeType => "AttributeType",
            NodeType::FunctionType => "FunctionType",
            NodeType::AnimationsType => "AnimationsType",
            NodeType::EventType => "EventType",
            NodeType::MapSizeType => "MapSizeType",
            NodeType::UiType => "UIType",
            NodeType::TimerType => "TimerType",
            NodeType::PackageType => "PackageType",
            NodeType::PackageItemType => "PackageItemType",
            NodeType::AvatarType => "AvatarType",
            NodeType::SettingsType => "SettingsType",
            NodeType::CampType => "CampType",
            NodeType::MutexType => "MutexType",
            NodeType::AnimationDataType => "AnimationDataType",
            NodeType::ForkType => "ForkType",
            NodeType::ButtonType => "ButtonType",
            NodeType::TextType => "TextType",
            NodeType::AccessType => "AccessType",
            NodeType::SpawnPointType => "SpawnPointType",
            NodeType::CommodityType => "CommodityType",
            NodeType::ShopType => "ShopType",
            NodeType::PaletteType => "PaletteType",
            NodeType::TerrainCollectionType => "TerrainCollectionType",
            NodeType::AssetsType => "AssetsType",
            NodeType::MossType => "MossType",
            NodeType::MossCollectionType => "MossCollectionType",
            NodeType::SceneryType => "SceneryType",
            NodeType::ModsType => "ModsType",
            NodeType::InputTextType => "InputTextType",
            NodeType::WallNodeType => "WallNodeType",
            NodeType::WallCollectionType => "WallCollectionType",
            NodeType::MountSpritesType => "MountSpritesType",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameEventDetailStatus {
    OnTouching = 1,
    OnKeyboardPress = 2,
}
impl GameEventDetailStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GameEventDetailStatus::OnTouching => "onTouching",
            GameEventDetailStatus::OnKeyboardPress => "onKeyboardPress",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessEnum {
    Accessible = 1,
    TakeOut = 2,
    Unlimited = 65535,
}
impl AccessEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessEnum::Accessible => "Accessible",
            AccessEnum::TakeOut => "TakeOut",
            AccessEnum::Unlimited => "Unlimited",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameEvent {
    UnknownEvent = 0,
    OnGameCreateBefore = 1,
    OnGameRun = 2,
    OnGameDestroy = 3,
    OnGameSelectCharacter = 4,
    OnGameEnter = 5,
    OnGameLeave = 6,
    OnGameRightMouseDown = 7,
    OnGameRightMouseUp = 8,
    OnGameLeftMouseDown = 9,
    OnGameLeftMouseUp = 10,
    OnGameMouseOut = 11,
    OnGameMouseOver = 12,
    OnSceneCreate = 13,
    OnSceneRun = 14,
    OnSceneDestroy = 15,
    OnSceneEnter = 16,
    OnSceneLeave = 17,
    OnElementCreate = 18,
    OnElementDestroy = 19,
    OnElementHit = 20,
    OnElementMove = 21,
    OnElementAttributeChange = 22,
    OnElementLeave = 23,
    OnElementStop = 24,
    OnElementActive = 38,
    OnElementUmount = 47,
    OnElementMount = 48,
    OnCharacterHit = 25,
    OnCharacterAttributeChange = 26,
    OnCharacterMove = 27,
    OnCharacterStop = 28,
    OnTerrainDestroy = 29,
    OnTerrainOverFunc = 30,
    OnTerrainLeaveFunc = 31,
    OnTerrainCreate = 33,
    OnUiClick = 34,
    OnPackageItemUse = 35,
    OnPackageItemMove = 36,
    OnTimerUpdate = 37,
    OnGameKeyboardDown = 39,
    OnGameKeyboardUp = 40,
    OnCharacterLeave = 41,
    OnChat = 42,
    OnGameCreateAfter = 43,
    OnPackageItemAdd = 44,
    OnPackageItemRemove = 45,
    OnPackageItemAttributeChange = 46,
    OnCharacterActive = 49,
}
impl GameEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GameEvent::UnknownEvent => "UnknownEvent",
            GameEvent::OnGameCreateBefore => "onGameCreateBefore",
            GameEvent::OnGameRun => "onGameRun",
            GameEvent::OnGameDestroy => "onGameDestroy",
            GameEvent::OnGameSelectCharacter => "onGameSelectCharacter",
            GameEvent::OnGameEnter => "onGameEnter",
            GameEvent::OnGameLeave => "onGameLeave",
            GameEvent::OnGameRightMouseDown => "onGameRightMouseDown",
            GameEvent::OnGameRightMouseUp => "onGameRightMouseUp",
            GameEvent::OnGameLeftMouseDown => "onGameLeftMouseDown",
            GameEvent::OnGameLeftMouseUp => "onGameLeftMouseUp",
            GameEvent::OnGameMouseOut => "onGameMouseOut",
            GameEvent::OnGameMouseOver => "onGameMouseOver",
            GameEvent::OnSceneCreate => "onSceneCreate",
            GameEvent::OnSceneRun => "onSceneRun",
            GameEvent::OnSceneDestroy => "onSceneDestroy",
            GameEvent::OnSceneEnter => "onSceneEnter",
            GameEvent::OnSceneLeave => "onSceneLeave",
            GameEvent::OnElementCreate => "onElementCreate",
            GameEvent::OnElementDestroy => "onElementDestroy",
            GameEvent::OnElementHit => "onElementHit",
            GameEvent::OnElementMove => "onElementMove",
            GameEvent::OnElementAttributeChange => "onElementAttributeChange",
            GameEvent::OnElementLeave => "onElementLeave",
            GameEvent::OnElementStop => "onElementStop",
            GameEvent::OnElementActive => "onElementActive",
            GameEvent::OnElementUmount => "onElementUmount",
            GameEvent::OnElementMount => "onElementMount",
            GameEvent::OnCharacterHit => "onCharacterHit",
            GameEvent::OnCharacterAttributeChange => "onCharacterAttributeChange",
            GameEvent::OnCharacterMove => "onCharacterMove",
            GameEvent::OnCharacterStop => "onCharacterStop",
            GameEvent::OnTerrainDestroy => "onTerrainDestroy",
            GameEvent::OnTerrainOverFunc => "onTerrainOverFunc",
            GameEvent::OnTerrainLeaveFunc => "onTerrainLeaveFunc",
            GameEvent::OnTerrainCreate => "onTerrainCreate",
            GameEvent::OnUiClick => "onUiClick",
            GameEvent::OnPackageItemUse => "onPackageItemUse",
            GameEvent::OnPackageItemMove => "onPackageItemMove",
            GameEvent::OnTimerUpdate => "onTimerUpdate",
            GameEvent::OnGameKeyboardDown => "onGameKeyboardDown",
            GameEvent::OnGameKeyboardUp => "onGameKeyboardUp",
            GameEvent::OnCharacterLeave => "onCharacterLeave",
            GameEvent::OnChat => "onChat",
            GameEvent::OnGameCreateAfter => "onGameCreateAfter",
            GameEvent::OnPackageItemAdd => "onPackageItemAdd",
            GameEvent::OnPackageItemRemove => "onPackageItemRemove",
            GameEvent::OnPackageItemAttributeChange => "onPackageItemAttributeChange",
            GameEvent::OnCharacterActive => "onCharacterActive",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerStatus {
    ServerStarting = 0,
    ServerAlive = 1,
    ServerOffline = 2,
    /// 等待关闭
    ServerWaitforshutdown = 3,
    ServerRejectInit = 4,
    ServerForceShutdown = 5,
    ServerAliveUnlessStop = 6,
}
impl ServerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServerStatus::ServerStarting => "SERVER_STARTING",
            ServerStatus::ServerAlive => "SERVER_ALIVE",
            ServerStatus::ServerOffline => "SERVER_OFFLINE",
            ServerStatus::ServerWaitforshutdown => "SERVER_WAITFORSHUTDOWN",
            ServerStatus::ServerRejectInit => "SERVER_REJECT_INIT",
            ServerStatus::ServerForceShutdown => "SERVER_FORCE_SHUTDOWN",
            ServerStatus::ServerAliveUnlessStop => "SERVER_ALIVE_UNLESS_STOP",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TqEvent {
    TqActive = 1,
    TqConfirm = 2,
    TqCancel = 3,
    TqJump = 4,
    TqMoveRight = 5,
    TqMoveLeft = 6,
    TqMoveUp = 7,
    TqMoveDown = 8,
    TqRunRight = 9,
    TqRunLeft = 10,
    TqRunUp = 11,
    TqRunDown = 12,
    TqShortcut = 13,
    TqKeyboard = 14,
}
impl TqEvent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TqEvent::TqActive => "TQ_ACTIVE",
            TqEvent::TqConfirm => "TQ_CONFIRM",
            TqEvent::TqCancel => "TQ_CANCEL",
            TqEvent::TqJump => "TQ_JUMP",
            TqEvent::TqMoveRight => "TQ_MOVE_RIGHT",
            TqEvent::TqMoveLeft => "TQ_MOVE_LEFT",
            TqEvent::TqMoveUp => "TQ_MOVE_UP",
            TqEvent::TqMoveDown => "TQ_MOVE_DOWN",
            TqEvent::TqRunRight => "TQ_RUN_RIGHT",
            TqEvent::TqRunLeft => "TQ_RUN_LEFT",
            TqEvent::TqRunUp => "TQ_RUN_UP",
            TqEvent::TqRunDown => "TQ_RUN_DOWN",
            TqEvent::TqShortcut => "TQ_SHORTCUT",
            TqEvent::TqKeyboard => "TQ_KEYBOARD",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HorizontalAlignment {
    HorizontalLeft = 0,
    HorizontalCenter = 1,
    /// HORIZONTAL_Stretch?
    HorizontalRight = 2,
}
impl HorizontalAlignment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HorizontalAlignment::HorizontalLeft => "HORIZONTAL_LEFT",
            HorizontalAlignment::HorizontalCenter => "HORIZONTAL_CENTER",
            HorizontalAlignment::HorizontalRight => "HORIZONTAL_RIGHT",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerticalAlignment {
    VerticalTop = 0,
    VerticalCenter = 1,
    /// VERTICAL_Stretch?
    VerticalBottom = 2,
}
impl VerticalAlignment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VerticalAlignment::VerticalTop => "VERTICAL_TOP",
            VerticalAlignment::VerticalCenter => "VERTICAL_CENTER",
            VerticalAlignment::VerticalBottom => "VERTICAL_BOTTOM",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SceneTypeEnum {
    NormalSceneType = 1,
    EditSceneType = 2,
}
impl SceneTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SceneTypeEnum::NormalSceneType => "NORMAL_SCENE_TYPE",
            SceneTypeEnum::EditSceneType => "EDIT_SCENE_TYPE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomTypeEnum {
    /// 玩家房间
    NormalRoom = 1,
    /// 商店街
    Shop = 2,
    /// 关卡房间
    Level = 3,
    /// 公共场景
    Public = 4,
}
impl RoomTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoomTypeEnum::NormalRoom => "NORMAL_ROOM",
            RoomTypeEnum::Shop => "SHOP",
            RoomTypeEnum::Level => "LEVEL",
            RoomTypeEnum::Public => "PUBLIC",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MoveStyle {
    DirectionMoveStyle = 1,
    FollowMouseMoveStyle = 2,
    PathMoveStyle = 3,
    FrontMoveStyle = 4,
}
impl MoveStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MoveStyle::DirectionMoveStyle => "DIRECTION_MOVE_STYLE",
            MoveStyle::FollowMouseMoveStyle => "FOLLOW_MOUSE_MOVE_STYLE",
            MoveStyle::PathMoveStyle => "PATH_MOVE_STYLE",
            MoveStyle::FrontMoveStyle => "FRONT_MOVE_STYLE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PathReachableStatus {
    PathReachableArea = 1,
    PathUnreachableArea = 2,
    PathReachableWithInteractionSprite = 3,
    PathUnreachableWithInteractionSprite = 4,
}
impl PathReachableStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PathReachableStatus::PathReachableArea => "PATH_REACHABLE_AREA",
            PathReachableStatus::PathUnreachableArea => "PATH_UNREACHABLE_AREA",
            PathReachableStatus::PathReachableWithInteractionSprite => "PATH_REACHABLE_WITH_INTERACTION_SPRITE",
            PathReachableStatus::PathUnreachableWithInteractionSprite => "PATH_UNREACHABLE_WITH_INTERACTION_SPRITE",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EditModeEnterRoomResult {
    EditModeEnterRoomSuccess = 1,
    EditModeEnterRoomIsFull = 2,
    EditModeEnterRoomNeedPassword = 3,
    EditModeEnterRoomPasswordCheckFailure = 4,
    EditModeEnterRoomDoseNotExists = 5,
}
impl EditModeEnterRoomResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EditModeEnterRoomResult::EditModeEnterRoomSuccess => "EDIT_MODE_ENTER_ROOM_SUCCESS",
            EditModeEnterRoomResult::EditModeEnterRoomIsFull => "EDIT_MODE_ENTER_ROOM_IS_FULL",
            EditModeEnterRoomResult::EditModeEnterRoomNeedPassword => "EDIT_MODE_ENTER_ROOM_NEED_PASSWORD",
            EditModeEnterRoomResult::EditModeEnterRoomPasswordCheckFailure => "EDIT_MODE_ENTER_ROOM_PASSWORD_CHECK_FAILURE",
            EditModeEnterRoomResult::EditModeEnterRoomDoseNotExists => "EDIT_MODE_ENTER_ROOM_DOSE_NOT_EXISTS",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EditModeRoomPrivacy {
    EditModeRoomPublic = 1,
    EditModeRoomPrivate = 2,
    EditModeRoomLocked = 3,
}
impl EditModeRoomPrivacy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EditModeRoomPrivacy::EditModeRoomPublic => "EDIT_MODE_ROOM_PUBLIC",
            EditModeRoomPrivacy::EditModeRoomPrivate => "EDIT_MODE_ROOM_PRIVATE",
            EditModeRoomPrivacy::EditModeRoomLocked => "EDIT_MODE_ROOM_LOCKED",
        }
    }
}
/// nevermore 2020.05.11
/// 客户端对服务端一个资源状态的查看以及请求,使用类restful风格规定该类型协议,对此类协议添加协议操作符概念
/// 添加协议更新操作符
/// message STH
/// {
///     required int32 id = 1;
///     repeated int32 arrayField = 2;
///     optional int32 intField = 3;
/// }
///
/// message OP_STH
/// {
///     required STH  sth = 1;    // required/optional/repeated
///     required OpCommand command = 2;
/// }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpCommand {
    /// 查找对于资源查找请求
    Find = 0,
    /// 声明创建某一个资源,全量更新
    Create = 1,
    /// 更新某一资源,全量更新
    Update = 2,
    /// 删除某一个资源
    Del = 3,
    /// 部分更新某一资源
    Patch = 4,
}
impl OpCommand {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpCommand::Find => "OP_COMMAND_FIND",
            OpCommand::Create => "OP_COMMAND_CREATE",
            OpCommand::Update => "OP_COMMAND_UPDATE",
            OpCommand::Del => "OP_COMMAND_DEL",
            OpCommand::Patch => "OP_COMMAND_PATCH",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GeneralParamType {
    Str = 1,
    Boolean = 2,
    Num = 3,
    ArrayBuffer = 4,
}
impl GeneralParamType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GeneralParamType::Str => "str",
            GeneralParamType::Boolean => "boolean",
            GeneralParamType::Num => "num",
            GeneralParamType::ArrayBuffer => "arrayBuffer",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompareType {
    Eq = 1,
    Ne = 2,
    Le = 3,
    Lt = 4,
    Ge = 5,
    Gt = 6,
}
impl CompareType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CompareType::Eq => "eq",
            CompareType::Ne => "ne",
            CompareType::Le => "le",
            CompareType::Lt => "lt",
            CompareType::Ge => "ge",
            CompareType::Gt => "gt",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DbGroupValueType {
    Number = 1,
    String = 2,
    Bool = 3,
    Group = 4,
    GroupArray = 5,
    NumberArray = 6,
    StringArray = 7,
    BoolArray = 8,
}
impl DbGroupValueType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DbGroupValueType::Number => "DBGroupValueTypeNumber",
            DbGroupValueType::String => "DBGroupValueTypeString",
            DbGroupValueType::Bool => "DBGroupValueTypeBool",
            DbGroupValueType::Group => "DBGroupValueTypeGroup",
            DbGroupValueType::GroupArray => "DBGroupValueTypeGroupArray",
            DbGroupValueType::NumberArray => "DBGroupValueTypeNumberArray",
            DbGroupValueType::StringArray => "DBGroupValueTypeStringArray",
            DbGroupValueType::BoolArray => "DBGroupValueTypeBoolArray",
        }
    }
}
/// 定义avatar slot,0x0000每一位表示的插槽
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AvatarSlot {
    HeadBase = 0,
    HeadHair = 1,
    HeadEyes = 2,
    HeadHairBack = 3,
    HeadMous = 4,
    HeadHats = 5,
    HeadMask = 6,
    HeadSpec = 7,
    BodyBase = 8,
    BodyCost = 9,
    BodyCostDres = 10,
    BodyTail = 11,
    BodyWing = 12,
    BodySpec = 13,
    FarmBase = 14,
    FarmCost = 15,
    FarmShld = 16,
    FarmWeap = 17,
    FarmSpec = 18,
    BarmBase = 19,
    BarmCost = 20,
    BarmShld = 21,
    BarmWeap = 22,
    BarmSpec = 23,
    FlegBase = 24,
    FlegCost = 25,
    FlegSpec = 26,
    BlegBase = 27,
    BlegCost = 28,
    BlegSpec = 29,
    HeadFace = 30,
}
impl AvatarSlot {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AvatarSlot::HeadBase => "head_base",
            AvatarSlot::HeadHair => "head_hair",
            AvatarSlot::HeadEyes => "head_eyes",
            AvatarSlot::HeadHairBack => "head_hair_back",
            AvatarSlot::HeadMous => "head_mous",
            AvatarSlot::HeadHats => "head_hats",
            AvatarSlot::HeadMask => "head_mask",
            AvatarSlot::HeadSpec => "head_spec",
            AvatarSlot::BodyBase => "body_base",
            AvatarSlot::BodyCost => "body_cost",
            AvatarSlot::BodyCostDres => "body_cost_dres",
            AvatarSlot::BodyTail => "body_tail",
            AvatarSlot::BodyWing => "body_wing",
            AvatarSlot::BodySpec => "body_spec",
            AvatarSlot::FarmBase => "farm_base",
            AvatarSlot::FarmCost => "farm_cost",
            AvatarSlot::FarmShld => "farm_shld",
            AvatarSlot::FarmWeap => "farm_weap",
            AvatarSlot::FarmSpec => "farm_spec",
            AvatarSlot::BarmBase => "barm_base",
            AvatarSlot::BarmCost => "barm_cost",
            AvatarSlot::BarmShld => "barm_shld",
            AvatarSlot::BarmWeap => "barm_weap",
            AvatarSlot::BarmSpec => "barm_spec",
            AvatarSlot::FlegBase => "fleg_base",
            AvatarSlot::FlegCost => "fleg_cost",
            AvatarSlot::FlegSpec => "fleg_spec",
            AvatarSlot::BlegBase => "bleg_base",
            AvatarSlot::BlegCost => "bleg_cost",
            AvatarSlot::BlegSpec => "bleg_spec",
            AvatarSlot::HeadFace => "head_face",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AvatarStyle {
    OriginAvatar = 1,
    SuitType = 2,
}
impl AvatarStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AvatarStyle::OriginAvatar => "OriginAvatar",
            AvatarStyle::SuitType => "SuitType",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gender {
    Male = 1,
    Female = 2,
}
impl Gender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LevelUpType {
    Player = 1,
}
impl LevelUpType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LevelUpType::Player => "Player",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FrontEndUniqueUiEnum {
    PicaNewRole = 4097,
}
impl FrontEndUniqueUiEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FrontEndUniqueUiEnum::PicaNewRole => "PicaNewRole",
        }
    }
}
/// 红点 enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RedDotTypeEnum {
    MailReddotstatus = 1,
    GalleryReddotstatus = 2,
    PackageReddotstatus = 3,
    OrderReddotstatus = 4,
    DailyQuestReddotstatus = 5,
    MainQuestReddotstatus = 6,
    FriendReddotstatus = 7,
    DressReddotstatus = 8,
    GalleryProgressrewardReddotstatus = 9,
    GalleryCollectionrewardReddotstatus = 10,
}
impl RedDotTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RedDotTypeEnum::MailReddotstatus => "MAIL_REDDOTSTATUS",
            RedDotTypeEnum::GalleryReddotstatus => "GALLERY_REDDOTSTATUS",
            RedDotTypeEnum::PackageReddotstatus => "PACKAGE_REDDOTSTATUS",
            RedDotTypeEnum::OrderReddotstatus => "ORDER_REDDOTSTATUS",
            RedDotTypeEnum::DailyQuestReddotstatus => "DAILY_QUEST_REDDOTSTATUS",
            RedDotTypeEnum::MainQuestReddotstatus => "MAIN_QUEST_REDDOTSTATUS",
            RedDotTypeEnum::FriendReddotstatus => "FRIEND_REDDOTSTATUS",
            RedDotTypeEnum::DressReddotstatus => "DRESS_REDDOTSTATUS",
            RedDotTypeEnum::GalleryProgressrewardReddotstatus => "GALLERY_PROGRESSREWARD_REDDOTSTATUS",
            RedDotTypeEnum::GalleryCollectionrewardReddotstatus => "GALLERY_COLLECTIONREWARD_REDDOTSTATUS",
        }
    }
}
