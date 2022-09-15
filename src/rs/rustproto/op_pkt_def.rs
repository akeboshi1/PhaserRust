/// 等级
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktLevel {
    #[prost(int32, optional, tag="1")]
    pub level: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="2")]
    pub current_level_exp: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub next_level_exp: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// 装扮
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktAvatar {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    #[prost(message, optional, tag="4")]
    pub avatar: ::core::option::Option<super::op_gameconfig::Avatar>,
    #[prost(string, optional, tag="5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub visibility: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
}
/// 称号
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktTitle {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
}
/// 徽章
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktBadge {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
}
/// 技能
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktSkill {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    /// 等级
    #[prost(message, optional, tag="4")]
    pub level: ::core::option::Option<PktLevel>,
    /// 品质
    #[prost(string, optional, tag="5")]
    pub quality: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否已经激活/拥有
    #[prost(bool, optional, tag="6")]
    pub active: ::core::option::Option<bool>,
    /// 是否可以释放
    #[prost(bool, optional, tag="7")]
    pub qualified: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktProperty {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag="3")]
    pub value: ::core::option::Option<f32>,
    #[prost(float, optional, tag="4")]
    pub temp_value: ::core::option::Option<f32>,
    #[prost(message, optional, tag="5")]
    pub display: ::core::option::Option<super::op_gameconfig::Display>,
    /// -1 代表无限
    #[prost(float, optional, tag="6", default="-1")]
    pub max: ::core::option::Option<f32>,
    #[prost(string, optional, tag="7")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktUi {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="2")]
    pub visible: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub disabled: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktRoomModel {
    /// 店铺模板类型
    #[prost(string, optional, tag="1")]
    pub store_type: ::core::option::Option<::prost::alloc::string::String>,
    /// 店铺模板说明
    #[prost(string, optional, tag="3")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    /// 店铺模板id
    #[prost(string, optional, tag="4")]
    pub model_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<super::op_gameconfig::Price>,
    #[prost(string, optional, tag="6")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktIndustry {
    #[prost(string, optional, tag="5")]
    pub industry_type: ::core::option::Option<::prost::alloc::string::String>,
    /// 行业名称
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub room_models: ::prost::alloc::vec::Vec<PktRoomModel>,
    /// 状态,buff附加
    #[prost(string, optional, tag="3")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    /// 目前状态描述
    #[prost(string, optional, tag="4")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    /// buff本身描述
    #[prost(string, optional, tag="6")]
    pub buff_des: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktRankChampion {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub champion: ::core::option::Option<::prost::alloc::string::String>,
    /// 行业类型  按规则拼底图
    #[prost(string, optional, tag="3")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// 排行数值
    #[prost(string, optional, tag="4")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktStoreRankItem {
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub owner_name: ::core::option::Option<::prost::alloc::string::String>,
    /// 排名, -1表示不在榜内
    #[prost(int32, optional, tag="3")]
    pub index: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub store_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub industry_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="6")]
    pub value: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktCompareValue {
    #[prost(string, optional, tag="1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(double, optional, tag="2")]
    pub value: ::core::option::Option<f64>,
    #[prost(enumeration="super::op_def::CompareType", optional, tag="3")]
    pub compare_type: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PktPlayerInfo {
    #[prost(string, optional, tag="1")]
    pub nickname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub platform_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub level: ::core::option::Option<PktLevel>,
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
pub enum PktQuestStage {
    /// 未获得  目前没有展示需求
    NotAccepted = 0,
    /// 可接取  目前没有展示需求
    Acceptable = 1,
    /// 进行中
    Processing = 2,
    /// 完成
    Finished = 3,
    /// 领取奖励结束  目前没有展示需求
    End = 4,
}
impl PktQuestStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktQuestStage::NotAccepted => "PKT_QUEST_STAGE_NOT_ACCEPTED",
            PktQuestStage::Acceptable => "PKT_QUEST_STAGE_ACCEPTABLE",
            PktQuestStage::Processing => "PKT_QUEST_STAGE_PROCESSING",
            PktQuestStage::Finished => "PKT_QUEST_STAGE_FINISHED",
            PktQuestStage::End => "PKT_QUEST_STAGE_END",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktOrderOperator {
    /// 送货
    PktOrderDelivery = 0,
    /// 删除
    PktOrderDelete = 2,
    /// 加速
    PktOrderSpeedUp = 1,
    PktOrderGetReward = 3,
}
impl PktOrderOperator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktOrderOperator::PktOrderDelivery => "PKT_ORDER_DELIVERY",
            PktOrderOperator::PktOrderDelete => "PKT_ORDER_DELETE",
            PktOrderOperator::PktOrderSpeedUp => "PKT_ORDER_SPEED_UP",
            PktOrderOperator::PktOrderGetReward => "PKT_ORDER_GET_REWARD",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktQuestType {
    /// 主线任务
    QuestMainMission = 1,
    /// 支线任务
    QuestSideMission = 2,
    /// 每日任务
    QuestDailyGoal = 3,
    /// 订单任务
    OrderQuestMission = 4,
    /// 皇家订单任务
    OrderQuestRoyalMission = 5,
}
impl PktQuestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktQuestType::QuestMainMission => "QUEST_MAIN_MISSION",
            PktQuestType::QuestSideMission => "QUEST_SIDE_MISSION",
            PktQuestType::QuestDailyGoal => "QUEST_DAILY_GOAL",
            PktQuestType::OrderQuestMission => "ORDER_QUEST_MISSION",
            PktQuestType::OrderQuestRoyalMission => "ORDER_QUEST_ROYAL_MISSION",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktSubscript {
    Unset = 0,
    Checkmark = 1,
}
impl PktSubscript {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktSubscript::Unset => "PKT_SUBSCRIPT_UNSET",
            PktSubscript::Checkmark => "PKT_SUBSCRIPT_CHECKMARK",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktPackageType {
    FurniturePackage = 1,
    AvatarPackage = 2,
    PropPackage = 3,
    MinePackage = 4,
    EditFurniturePackage = 5,
    ManorPackage = 6,
}
impl PktPackageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktPackageType::FurniturePackage => "FurniturePackage",
            PktPackageType::AvatarPackage => "AvatarPackage",
            PktPackageType::PropPackage => "PropPackage",
            PktPackageType::MinePackage => "MinePackage",
            PktPackageType::EditFurniturePackage => "EditFurniturePackage",
            PktPackageType::ManorPackage => "ManorPackage",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktPlayerInteraction {
    PktInvitePlayer = 1,
    PktTracePlayer = 2,
}
impl PktPlayerInteraction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktPlayerInteraction::PktInvitePlayer => "PKT_invitePlayer",
            PktPlayerInteraction::PktTracePlayer => "PKT_tracePlayer",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktManorOp {
    Buy = 1,
    Edit = 2,
    ChangeName = 3,
}
impl PktManorOp {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktManorOp::Buy => "PKT_MANOR_OP_Buy",
            PktManorOp::Edit => "PKT_MANOR_OP_Edit",
            PktManorOp::ChangeName => "PKT_MANOR_OP_Change_Name",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PktManorCommodityState {
    PktManorOwned = 1,
    PktManorInUse = 2,
    PktManorNotOwned = 3,
}
impl PktManorCommodityState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PktManorCommodityState::PktManorOwned => "PKT_MANOR_Owned",
            PktManorCommodityState::PktManorInUse => "PKT_MANOR_InUse",
            PktManorCommodityState::PktManorNotOwned => "PKT_MANOR_NotOwned",
        }
    }
}
