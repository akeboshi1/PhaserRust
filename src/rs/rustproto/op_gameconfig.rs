// Match OPCODE enum and move first "_" 
// message OP_WORLD_REQ_GALAXY_EXAMPLE
// {
//     required int32  port=1;
//     optional string name=2;
// }

// TODO 警告!!! 本文件所有声明都已弃用,需要被删除

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Display {
    #[prost(string, optional, tag="1")]
    pub data_path: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub texture_path: ::core::option::Option<::prost::alloc::string::String>,
}
// ///////////////////////////////////////////////
// /// Element
// //////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="2")]
    pub int_val: ::core::option::Option<i32>,
    #[prost(string, optional, tag="3")]
    pub str_val: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="4")]
    pub bool_val: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuncArg {
    #[prost(string, required, tag="1")]
    pub arg_name: ::prost::alloc::string::String,
    #[prost(oneof="func_arg::ArgValueOneOf", tags="2, 3, 4")]
    pub arg_value_one_of: ::core::option::Option<func_arg::ArgValueOneOf>,
}
/// Nested message and enum types in `FuncArg`.
pub mod func_arg {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ArgValueOneOf {
        #[prost(bool, tag="2")]
        BoolValue(bool),
        #[prost(double, tag="3")]
        NumberValue(f64),
        #[prost(string, tag="4")]
        StrValue(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Func {
    #[prost(string, required, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub func_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub func_args: ::prost::alloc::vec::Vec<FuncArg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemEvent {
    #[prost(message, optional, tag="1")]
    pub on_item_create: ::core::option::Option<Func>,
    #[prost(message, optional, tag="2")]
    pub on_item_destroy: ::core::option::Option<Func>,
    #[prost(message, optional, tag="3")]
    pub on_item_attribute_change: ::core::option::Option<Func>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementEvent {
    #[prost(message, optional, tag="1")]
    pub on_element_create: ::core::option::Option<Func>,
    #[prost(message, optional, tag="2")]
    pub on_element_destroy: ::core::option::Option<Func>,
    #[prost(message, optional, tag="3")]
    pub on_element_hit: ::core::option::Option<Func>,
    #[prost(message, optional, tag="4")]
    pub on_element_move: ::core::option::Option<Func>,
    #[prost(message, optional, tag="5")]
    pub on_element_attribute_change: ::core::option::Option<Func>,
    #[prost(message, optional, tag="6")]
    pub on_element_leave: ::core::option::Option<Func>,
    #[prost(message, optional, tag="7")]
    pub on_element_stop: ::core::option::Option<Func>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterEvent {
    #[prost(message, optional, tag="1")]
    pub on_character_hit: ::core::option::Option<Func>,
    #[prost(message, optional, tag="2")]
    pub on_character_attribute_change: ::core::option::Option<Func>,
    #[prost(message, optional, tag="3")]
    pub on_character_move: ::core::option::Option<Func>,
    #[prost(message, optional, tag="4")]
    pub on_character_stop: ::core::option::Option<Func>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerrainEvent {
    #[prost(message, optional, tag="1")]
    pub on_terrain_destroy: ::core::option::Option<Func>,
    #[prost(message, optional, tag="2")]
    pub on_terrain_over_func: ::core::option::Option<Func>,
    #[prost(message, optional, tag="3")]
    pub on_terrain_leave_func: ::core::option::Option<Func>,
    #[prost(message, optional, tag="4")]
    pub on_terrain_schedule: ::core::option::Option<Func>,
    #[prost(message, optional, tag="5")]
    pub on_terrain_create: ::core::option::Option<Func>,
}
//
// message ObjectEvent
// {
// optional Func onHitFunc                     = 1; // pi_onHitFunc(pi_eventType, pi_eventID)
// optional Func onDamageFunc                  = 2; // pi_eventType, pi_eventDirection
// optional Func onDestroy                     = 3;
// optional Func onMoveFunc                    = 4; // pi_eventDirection
// optional Func onOverFunc                    = 5;
// optional Func onLeaveFunc                   = 6;
// optional Func onSchedule                    = 7; // 计划任务触发
// optional Func onSceneAttributeChange        = 8; // 自定义属性变化 pi_AttributeName, pi_AttributeValueType, pi_AttributeValueBefore, pi_AttributeValueAfter
// optional Func onStyleChange                 = 9;
// optional Func onDisappear                   = 10;
// optional Func onItemAttributeChange         = 11; //item
// optional Func onCharacterAttributeChange    = 12; //Character
// optional Func onElementAttributeChange      = 13; //element
// optional Func onElementHit              = 14; // element hit
// optional Func onCharacterHitFunc            = 15; // Character hit
// optional Func onMouseClick                  = 20;
// optional Func onItemCreate                  = 21;
// optional Func onItemDestroy                 = 22;
// //    optional string onMouseHover             = 21 [default = ""]；
// //    optional string onKeyboard               = 30 [default = ""]; onScene trigger
// }

// ///////////////////////////////////////////////
// /// ElementType
// //////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementType {
    #[prost(string, required, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="10")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    /// required ObjectEvent object_event     = 11;
    #[prost(message, optional, tag="11")]
    pub element_event: ::core::option::Option<ElementEvent>,
    #[prost(message, repeated, tag="12")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(string, optional, tag="13")]
    pub animation_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="14")]
    pub display: ::core::option::Option<Display>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Element {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(string, required, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// 美术资源方向
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
    /// optional ObjectEvent object_event               = 16;
    #[prost(message, optional, tag="16")]
    pub element_event: ::core::option::Option<ElementEvent>,
    #[prost(message, repeated, tag="17")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(string, optional, tag="18")]
    pub animation_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="14")]
    pub display: ::core::option::Option<Display>,
}
// ///////////////////////////////////////////
// //  ItemType
// ////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// 金额
    #[prost(float, required, tag="1")]
    pub price: f32,
    /// 货币类型
    #[prost(enumeration="super::op_def::CoinType", required, tag="2")]
    pub coin_type: i32,
    /// 保留多少位小数
    #[prost(int32, optional, tag="3")]
    pub display_precision: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemType {
    #[prost(string, required, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="7")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    /// required ObjectEvent object_event              = 8;
    #[prost(message, optional, tag="8")]
    pub item_event: ::core::option::Option<ItemEvent>,
    #[prost(message, repeated, tag="9")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(int32, required, tag="10")]
    pub maxcount: i32,
    #[prost(message, optional, tag="14")]
    pub display: ::core::option::Option<Display>,
    #[prost(message, repeated, tag="15")]
    pub price: ::prost::alloc::vec::Vec<Price>,
    #[prost(string, optional, tag="16")]
    pub shop_des: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    /// ID
    #[prost(int32, required, tag="1")]
    pub id: i32,
    /// 道具类型
    #[prost(string, required, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// 道具数量
    #[prost(int32, required, tag="3", default="1")]
    pub count: i32,
    /// 道具名称
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 道具描述
    #[prost(string, optional, tag="5")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    /// 道具动画
    #[prost(message, repeated, tag="9")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    /// optional ObjectEvent object_event               = 10;  // 道具触发事件， 游戏端无需考虑，编辑器需要给出lua脚本的位置
    #[prost(message, optional, tag="10")]
    pub item_event: ::core::option::Option<ItemEvent>,
    /// 道具属性， 游戏端无需考虑，编辑器需要给出lua脚本的位置
    #[prost(message, repeated, tag="11")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    /// 道具最大可对叠数量
    #[prost(int32, optional, tag="12")]
    pub maxcount: ::core::option::Option<i32>,
    #[prost(string, optional, tag="13")]
    pub animation_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="14")]
    pub display: ::core::option::Option<Display>,
    #[prost(message, repeated, tag="15")]
    pub price: ::prost::alloc::vec::Vec<Price>,
    #[prost(string, optional, tag="16")]
    pub shop_des: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopItem {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub subcategory: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub currency: ::prost::alloc::string::String,
    #[prost(int32, required, tag="6")]
    pub price: i32,
}
// //////////////////////////////////////////
// ////   Layers
// /////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerrainType {
    #[prost(string, required, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(string, required, tag="6")]
    pub animation_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub terrain_event: ::core::option::Option<TerrainEvent>,
    #[prost(enumeration="super::op_def::TerrainAnimationType", required, tag="8")]
    pub mode: i32,
    #[prost(message, optional, tag="14")]
    pub display: ::core::option::Option<Display>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terrain {
    #[prost(int32, optional, tag="1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, required, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, required, tag="6")]
    pub animation_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub des: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(message, optional, tag="7")]
    pub terrain_event: ::core::option::Option<TerrainEvent>,
    #[prost(enumeration="super::op_def::TerrainAnimationType", optional, tag="8")]
    pub mode: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub z: ::core::option::Option<i32>,
    #[prost(message, optional, tag="14")]
    pub display: ::core::option::Option<Display>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shop {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Item>,
}
// /////////////////////////////////////////////////////
// ///   Scene
// ///////////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SceneEvent {
    #[prost(message, optional, tag="1")]
    pub on_scene_create: ::core::option::Option<Func>,
    #[prost(message, optional, tag="2")]
    pub on_scene_run: ::core::option::Option<Func>,
    #[prost(message, optional, tag="3")]
    pub on_scene_destroy: ::core::option::Option<Func>,
    #[prost(message, optional, tag="4")]
    pub on_scene_enter: ::core::option::Option<Func>,
    #[prost(message, optional, tag="5")]
    pub on_scene_leave: ::core::option::Option<Func>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scene {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(string, required, tag="2", default="")]
    pub name: ::prost::alloc::string::String,
    #[prost(float, required, tag="3")]
    pub cols: f32,
    #[prost(float, required, tag="4")]
    pub rows: f32,
    #[prost(int32, required, tag="5")]
    pub tile_width: i32,
    #[prost(int32, required, tag="6")]
    pub tile_height: i32,
    #[prost(float, optional, tag="7", default="0")]
    pub z_start: ::core::option::Option<f32>,
    #[prost(float, optional, tag="8", default="0")]
    pub z_end: ::core::option::Option<f32>,
    #[prost(message, repeated, tag="9")]
    pub elements: ::prost::alloc::vec::Vec<Element>,
    #[prost(message, repeated, tag="10")]
    pub terrains: ::prost::alloc::vec::Vec<Terrain>,
    #[prost(message, optional, tag="11")]
    pub event: ::core::option::Option<SceneEvent>,
}
// ////////////////////////
// /  Avatar
// //////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvatarBackbone {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="7")]
    pub animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(string, optional, tag="8")]
    pub animation_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Avatar {
    #[prost(string, required, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// avatar 美术资源可用方向， 人物一般固定为\[1,3,5,7\] 四个方向
    #[prost(int32, repeated, packed="false", tag="80")]
    pub dirable: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag="2")]
    pub head_base_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub head_hair_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub head_eyes_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub head_hair_back_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub head_mous_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub head_hats_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="8")]
    pub head_mask_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="9")]
    pub head_spec_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="10")]
    pub head_face_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="11")]
    pub body_base_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="12")]
    pub body_cost_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="13")]
    pub body_cost_dres_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="14")]
    pub body_tail_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="15")]
    pub body_wing_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="16")]
    pub body_spec_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="21")]
    pub farm_base_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="22")]
    pub farm_cost_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="23")]
    pub farm_shld_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="24")]
    pub farm_weap_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="25")]
    pub farm_spec_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="31")]
    pub barm_base_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="32")]
    pub barm_cost_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="33")]
    pub barm_shld_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="34")]
    pub barm_weap_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="35")]
    pub barm_spec_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="41")]
    pub fleg_base_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="42")]
    pub fleg_cost_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="43")]
    pub fleg_spec_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="51")]
    pub bleg_base_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="52")]
    pub bleg_cost_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="53")]
    pub bleg_spec_id: ::core::option::Option<::prost::alloc::string::String>,
}
// //////////////////////////////////////
// //  Slot
// //////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slot {
    #[prost(string, optional, tag="1")]
    pub bond_attr_curkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, required, tag="2")]
    pub bond_attr_maxkey: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub color: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub bond_name: ::prost::alloc::string::String,
    #[prost(bool, optional, tag="5", default="false")]
    pub public: ::core::option::Option<bool>,
}
// //////////////////////////////////////////////
// // Game
// /////////////////////////////////////////////

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    #[prost(int32, required, tag="1")]
    pub max_index: i32,
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Item>,
    #[prost(int32, required, tag="3")]
    pub id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Character {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(string, required, tag="2")]
    pub name: ::prost::alloc::string::String,
    ///   类型最大人数
    #[prost(int32, required, tag="3", default="20")]
    pub max_num: i32,
    #[prost(string, required, tag="4")]
    pub camp: ::prost::alloc::string::String,
    #[prost(message, required, tag="5")]
    pub avatar: Avatar,
    #[prost(message, repeated, tag="6")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(message, repeated, tag="7")]
    pub package: ::prost::alloc::vec::Vec<Package>,
    #[prost(int32, optional, tag="8")]
    pub scene_id: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="9")]
    pub x: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="10")]
    pub y: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="11")]
    pub z: ::core::option::Option<i32>,
    /// required ObjectEvent object_event   = 12;
    #[prost(message, optional, tag="12")]
    pub character_event: ::core::option::Option<CharacterEvent>,
    /// 美术资源方向 与人物初始方向不同， 人物一般固定为\[1,3,5,7\] 四个方向中的一个
    #[prost(int32, required, tag="13")]
    pub avatar_dir: i32,
    #[prost(message, repeated, tag="14")]
    pub slots: ::prost::alloc::vec::Vec<Slot>,
    #[prost(message, optional, tag="15")]
    pub editor_display: ::core::option::Option<Display>,
    #[prost(message, repeated, tag="16")]
    pub editor_animations: ::prost::alloc::vec::Vec<super::op_gameconfig_01::AnimationData>,
    #[prost(string, optional, tag="17")]
    pub editor_animation_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="18")]
    pub shops: ::prost::alloc::vec::Vec<Shop>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameEvent {
    #[prost(message, optional, tag="1")]
    pub on_game_create: ::core::option::Option<Func>,
    #[prost(message, optional, tag="2")]
    pub on_game_run: ::core::option::Option<Func>,
    #[prost(message, optional, tag="3")]
    pub on_game_destroy: ::core::option::Option<Func>,
    #[prost(message, optional, tag="4")]
    pub on_select_character: ::core::option::Option<Func>,
    #[prost(message, optional, tag="5")]
    pub on_game_enter: ::core::option::Option<Func>,
    #[prost(message, optional, tag="6")]
    pub on_game_leave: ::core::option::Option<Func>,
    #[prost(message, optional, tag="7")]
    pub on_right_mouse_down: ::core::option::Option<Func>,
    #[prost(message, optional, tag="8")]
    pub on_right_mouse_up: ::core::option::Option<Func>,
    #[prost(message, optional, tag="9")]
    pub on_left_mouse_down: ::core::option::Option<Func>,
    #[prost(message, optional, tag="10")]
    pub on_left_mouse_up: ::core::option::Option<Func>,
    #[prost(message, optional, tag="11")]
    pub on_mouse_out: ::core::option::Option<Func>,
    #[prost(message, optional, tag="12")]
    pub on_mouse_over: ::core::option::Option<Func>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Game {
    #[prost(string, required, tag="1")]
    pub game_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="2", default="")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub scenes: ::prost::alloc::vec::Vec<Scene>,
    /// 类型
    #[prost(message, repeated, tag="6")]
    pub characters: ::prost::alloc::vec::Vec<Character>,
    #[prost(string, repeated, tag="7")]
    pub resource_root: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="8")]
    pub terrain_types: ::prost::alloc::vec::Vec<TerrainType>,
    #[prost(message, repeated, tag="9")]
    pub element_types: ::prost::alloc::vec::Vec<ElementType>,
    #[prost(message, repeated, tag="10")]
    pub item_types: ::prost::alloc::vec::Vec<ItemType>,
    #[prost(message, optional, tag="11")]
    pub game_event: ::core::option::Option<GameEvent>,
    /// 总是给全量？
    #[prost(message, repeated, tag="12")]
    pub avatar_backbone: ::prost::alloc::vec::Vec<AvatarBackbone>,
    #[prost(message, repeated, tag="13")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
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
