#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Access {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(enumeration="super::op_def::AccessEnum", required, tag="2")]
    pub access: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, optional, tag="2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub is_moss: ::core::option::Option<bool>,
    /// 场景图层标志，标示实体应该在哪个图层渲染
    #[prost(int32, optional, tag="4")]
    pub layer: ::core::option::Option<i32>,
}
/// 继承关系的基类
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(enumeration="super::op_def::NodeType", required, tag="2")]
    pub r#type: i32,
    /// 父类id
    #[prost(int32, required, tag="3")]
    pub parent: i32,
    /// 可继承的父类类型
    #[prost(enumeration="super::op_def::NodeType", repeated, packed="false", tag="4")]
    pub lock_parent: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, required, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// 序列编号
    #[prost(string, optional, tag="6")]
    pub sn: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SceneOverView {
    #[prost(int32, required, tag="1")]
    pub id: i32,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// 游戏关联
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Game {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, repeated, tag="2")]
    pub entity: ::prost::alloc::vec::Vec<Entity>,
    #[prost(message, repeated, tag="3")]
    pub loc: ::prost::alloc::vec::Vec<Location>,
    #[prost(message, repeated, tag="4")]
    pub movable: ::prost::alloc::vec::Vec<Movable>,
    #[prost(message, repeated, tag="5")]
    pub display: ::prost::alloc::vec::Vec<Display>,
    #[prost(message, repeated, tag="6")]
    pub attr: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(message, repeated, tag="7")]
    pub func: ::prost::alloc::vec::Vec<Function>,
    #[prost(message, repeated, tag="8")]
    pub event: ::prost::alloc::vec::Vec<Event>,
    #[prost(message, repeated, tag="9")]
    pub map_size: ::prost::alloc::vec::Vec<MapSize>,
    #[prost(message, repeated, tag="10")]
    pub ui: ::prost::alloc::vec::Vec<Ui>,
    #[prost(message, repeated, tag="11")]
    pub timer: ::prost::alloc::vec::Vec<Timer>,
    #[prost(message, repeated, tag="12")]
    pub mutex: ::prost::alloc::vec::Vec<Mutex>,
    #[prost(message, repeated, tag="13")]
    pub avatar: ::prost::alloc::vec::Vec<Avatar>,
    #[prost(message, repeated, tag="14")]
    pub camp: ::prost::alloc::vec::Vec<Camp>,
    #[prost(message, repeated, tag="15")]
    pub animations: ::prost::alloc::vec::Vec<Animations>,
    #[prost(message, repeated, tag="16")]
    pub animationdatas: ::prost::alloc::vec::Vec<AnimationData>,
    #[prost(message, repeated, tag="17")]
    pub button: ::prost::alloc::vec::Vec<Button>,
    #[prost(message, optional, tag="18")]
    pub settings: ::core::option::Option<Settings>,
    #[prost(message, repeated, tag="19")]
    pub text: ::prost::alloc::vec::Vec<Text>,
    #[prost(message, repeated, tag="20")]
    pub spawnpoint: ::prost::alloc::vec::Vec<SpawnPoint>,
    #[prost(message, repeated, tag="21")]
    pub access: ::prost::alloc::vec::Vec<Access>,
    #[prost(message, repeated, tag="22")]
    pub commodities: ::prost::alloc::vec::Vec<Commodity>,
    #[prost(message, optional, tag="23")]
    pub palette: ::core::option::Option<Palette>,
    #[prost(message, optional, tag="24")]
    pub terrain: ::core::option::Option<TerrainCollection>,
    #[prost(message, repeated, tag="25")]
    pub scenes: ::prost::alloc::vec::Vec<SceneOverView>,
    #[prost(message, optional, tag="26")]
    pub assets: ::core::option::Option<Assets>,
    #[prost(message, optional, tag="27")]
    pub moss: ::core::option::Option<Moss>,
    #[prost(message, optional, tag="28")]
    pub moss_list: ::core::option::Option<MossCollection>,
    #[prost(message, repeated, tag="29")]
    pub seneries: ::prost::alloc::vec::Vec<Scenery>,
    #[prost(message, optional, tag="30")]
    pub mods: ::core::option::Option<Mods>,
    /// pi
    #[prost(int32, optional, tag="31")]
    pub program_code: ::core::option::Option<i32>,
    #[prost(message, optional, tag="32")]
    pub walls: ::core::option::Option<MossCollection>,
    #[prost(message, repeated, tag="33")]
    pub mount_sprites: ::prost::alloc::vec::Vec<MountSprites>,
    #[prost(message, optional, tag="34")]
    pub entity_palette: ::core::option::Option<EntityPalette>,
}
// BaseNode

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(int32, optional, tag="2")]
    pub max_number: ::core::option::Option<i32>,
    #[prost(int32, required, tag="3")]
    pub spawnpointid: i32,
    #[prost(enumeration="super::op_def::KeyCode", optional, tag="4")]
    pub voice_keycode: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub sceneid: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Camp {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(int32, optional, tag="2")]
    pub max_number: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Avatar {
    #[prost(message, required, tag="1")]
    pub node: Node,
    /// avatar 美术资源可用方向， 人物一般固定为\[1,3,5,7\] 四个方向
    #[prost(int32, repeated, packed="false", tag="80")]
    pub dirable: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, required, tag="81")]
    pub default_dir: i32,
    #[prost(string, optional, tag="82")]
    pub backbone_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="83")]
    pub default_animation: ::core::option::Option<::prost::alloc::string::String>,
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
// end BaseNode

// ExtraNode

/// 这个真的要么？
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutex {
    #[prost(message, required, tag="1")]
    pub node: Node,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(float, required, tag="2")]
    pub x: f32,
    #[prost(float, required, tag="3")]
    pub y: f32,
    #[prost(float, required, tag="4")]
    pub z: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpawnPoint {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, required, tag="2")]
    pub loc: Location,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Movable {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(int32, optional, tag="2")]
    pub dir: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="3")]
    pub angle: ::core::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub speed: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Animations {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub default_animation: ::prost::alloc::string::String,
    #[prost(int32, optional, tag="3")]
    pub dir: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="11", default="false")]
    pub scale: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimationData {
    #[prost(message, required, tag="1")]
    pub node: Node,
    /// 弃用，兼容老数据
    #[prost(int32, repeated, packed="false", tag="2")]
    pub frame: ::prost::alloc::vec::Vec<i32>,
    /// 动画的帧频
    #[prost(int32, required, tag="3")]
    pub frame_rate: i32,
    /// 行走区域
    #[prost(string, optional, tag="4")]
    pub walkable_area: ::core::option::Option<::prost::alloc::string::String>,
    /// 碰撞区域
    #[prost(string, optional, tag="5")]
    pub collision_area: ::core::option::Option<::prost::alloc::string::String>,
    /// 是否循环播放
    #[prost(bool, optional, tag="6")]
    pub r#loop: ::core::option::Option<bool>,
    /// 原点，编辑器里的小红点
    #[prost(int32, repeated, packed="false", tag="7")]
    pub origin_point: ::prost::alloc::vec::Vec<i32>,
    /// 弃用，和origin_point重合 
    #[prost(int32, repeated, packed="false", tag="8")]
    pub walk_origin_point: ::prost::alloc::vec::Vec<i32>,
    /// 物件动画左上角和原点的偏移量
    #[prost(string, optional, tag="9")]
    pub base_loc: ::core::option::Option<::prost::alloc::string::String>,
    /// 弃用，兼容老数据。单层的动画序列 
    #[prost(string, repeated, tag="10")]
    pub frame_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 交互区域，有交互区域代表此动画可以触发事件。
    /// 交互区域存在多个，由main_interactive_points决定主要的交互区域
    #[prost(message, repeated, tag="11")]
    pub interactive_area: ::prost::alloc::vec::Vec<super::op_def::PbPoint2i>,
    #[prost(message, repeated, tag="12")]
    pub main_interactive_points: ::prost::alloc::vec::Vec<super::op_def::PbPoint2i>,
    /// 每一帧动画的修正时间（ms），播放期间每帧应出现的额外时间
    #[prost(float, repeated, packed="false", tag="13")]
    pub frame_duration: ::prost::alloc::vec::Vec<f32>,
    /// 代替#10的单层动画序列，每层都有一个动画序列
    #[prost(message, repeated, tag="14")]
    pub layer: ::prost::alloc::vec::Vec<AnimationLayer>,
    /// 挂载层,和外部动画组合时，如本动画是容器或者底座，外部动画将插入到此层并将外部动画的原点和此层内的挂载点锁定
    #[prost(message, optional, tag="15")]
    pub mount_layer: ::core::option::Option<AnimationMountLayer>,
}
/// 动画图层
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimationLayer {
    #[prost(string, repeated, tag="1")]
    pub frame_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 图层左上角和原点的偏移量
    #[prost(message, optional, tag="2")]
    pub offset_loc: ::core::option::Option<super::op_def::PbPoint2f>,
    /// 一个Bool型的数组，每个值代表对应index的帧是否可视。此阈值没有中间值并不能表示透明度，只能代表显示与否。
    #[prost(bool, repeated, packed="false", tag="3")]
    pub frame_visible: ::prost::alloc::vec::Vec<bool>,
    /// 图层名字
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="5")]
    pub id: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimationMountLayer {
    /// 插入到AnimationLayer的第几层
    #[prost(int32, required, tag="1")]
    pub index: i32,
    /// 多个挂载点
    /// 挂载点是相对于origin_point的坐标位置（可以有小数）
    #[prost(message, repeated, tag="2")]
    pub mount_point: ::prost::alloc::vec::Vec<super::op_def::PbPoint3f>,
    /// 一个整数数组，每个值描述了对应每一帧，每个挂载点是否可视。 从地位到高位，每一位0代表对应index的挂载点不可见。
    /// 最多定义8个挂载点。使用位运算 0x00000000 代表挂载点都不可见, 0x11111111 代表所有挂载点都可见。0x10000001代表index(0)和index(7)的挂载点可见
    ///                               ^      ^
    ///                              最后     首个点
    #[prost(int32, repeated, packed="false", tag="3")]
    pub frame_visible: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Display {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub data_path: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub texture_path: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub tips: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="super::op_def::VerticalAlignment", optional, tag="5")]
    pub vertical: ::core::option::Option<i32>,
    #[prost(enumeration="super::op_def::HorizontalAlignment", optional, tag="6")]
    pub horizontal: ::core::option::Option<i32>,
    #[prost(float, optional, tag="7", default="1")]
    pub scale_x: ::core::option::Option<f32>,
    #[prost(float, optional, tag="8", default="1")]
    pub scale_y: ::core::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub key: ::prost::alloc::string::String,
    /// 99999 size
    #[prost(int32, optional, tag="3")]
    pub int_val: ::core::option::Option<i32>,
    /// 999 size
    #[prost(string, optional, tag="4")]
    pub str_val: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="5")]
    pub bool_val: ::core::option::Option<bool>,
    #[prost(string, optional, tag="6")]
    pub media: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Function {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub func_name: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(enumeration="super::op_def::GameEvent", required, tag="2")]
    pub event_name: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapSize {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(float, required, tag="2")]
    pub cols: f32,
    #[prost(float, required, tag="3")]
    pub rows: f32,
    #[prost(int32, required, tag="4")]
    pub tile_width: i32,
    #[prost(int32, required, tag="5")]
    pub tile_height: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3", default="#FFFFFF")]
    pub color: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputText {
    #[prost(message, required, tag="1")]
    pub node: Node,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Button {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub tips: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="4")]
    pub param: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MenuItem {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, required, tag="2")]
    pub text: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub child: ::prost::alloc::vec::Vec<MenuItem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ui {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(string, repeated, tag="2")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timer {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(int32, required, tag="2")]
    pub timeout: i32,
    #[prost(int32, required, tag="3")]
    pub interval: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commodity {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(float, optional, tag="2")]
    pub price: ::core::option::Option<f32>,
    #[prost(enumeration="super::op_def::CoinType", optional, tag="3")]
    pub coin_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
// end ExtraNode

/// 0119-2020 edit by 7.
/// 地块索引数据,全局节点
/// 每个游戏只有一个Palette Node，内部定义了一个entity数组，加入游戏的地块类型都存于此节点中
/// 地形结构用Palette.entity 的index来描述
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Palette {
    #[prost(message, required, tag="1")]
    pub node: Node,
    /// 这里存放 Terrain 节点
    #[prost(message, repeated, tag="2")]
    pub peers: ::prost::alloc::vec::Vec<EntityKeyPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityKeyPair {
    #[prost(int32, required, tag="1")]
    pub key: i32,
    #[prost(message, optional, tag="2")]
    pub entity: ::core::option::Option<Entity>,
}
/// 地形数据结构
/// 每个场景节点下有一个此节点。此节点包括一个二维数组。数组里存放的是场景网格每个格子对应的Palette Index
/// 地块具体数据从Palette结构中获取
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerrainCollection {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, required, tag="2")]
    pub data: super::op_def::IntArrays,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assets {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Attribute>,
}
/// 0402-2020 edit by 7.
/// 非交互物件索引数据，全局节点。
/// 所谓非交互物件的定义是只有View没有业务逻辑不能交互的物件或装饰物件，正如名字Moss（苔藓、青苔）一样，就是用来装饰摆设的物件。
/// 每个游戏只有一个Moss Node，内部定义了一个entity数组，没有交互区域的物件会被优化掉，模板Entity存于此节点中
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Moss {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, repeated, tag="2")]
    pub peers: ::prost::alloc::vec::Vec<EntityKeyPair>,
}
/// 非交互物件集
/// 每个场景节点下有一个此节点。此节点包括一个Map。Map.id 是 message Moss 里Entity的id，Map.x，Map.y是坐标位置
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MossCollection {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, required, tag="2")]
    pub data: super::op_def::LocationMap,
}
/// editer by 7. at0410-2020
/// 游戏舞台布景，包括前景和后景，通过depth来表示
/// Scenery 节点只能挂在场景节点下，每个场景可以有多个布景，如果depth数值相同则按照节点挂载顺序排列。
/// Update at0427
///   - photo_uri 改为 uris 从单一uri 变为uri的二维数组来描述图片分块
///   - 新增布景宽高数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scenery {
    #[prost(message, required, tag="1")]
    pub node: Node,
    /// 布景的图片资源地址,相对路径。二维数组
    #[prost(message, required, tag="2")]
    pub uris: super::op_def::StrArrays,
    /// 景深：舞台（游戏画面）的景深为0，背景的景深为负值，前景的景深为正值
    #[prost(int32, required, tag="3")]
    pub depth: i32,
    /// 相对于坐标系0，0点的偏移量
    #[prost(message, optional, tag="4")]
    pub offset: ::core::option::Option<super::op_def::PbPoint2f>,
    /// 跟随舞台的加速度
    #[prost(float, optional, tag="5")]
    pub speed: ::core::option::Option<f32>,
    /// 图片填充模式：居中、填充、拉伸、平铺
    #[prost(int32, optional, tag="6")]
    pub fit: ::core::option::Option<i32>,
    /// 布景高
    #[prost(int32, optional, tag="7")]
    pub height: ::core::option::Option<i32>,
    /// 布景宽
    #[prost(int32, optional, tag="8")]
    pub width: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModKeyPair {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mods {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<ModKeyPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MountSprites {
    #[prost(message, required, tag="1")]
    pub node: Node,
    #[prost(int32, repeated, packed="false", tag="2")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityPalette {
    #[prost(message, required, tag="1")]
    pub node: Node,
    /// 这里存放 Terrain 节点
    #[prost(message, repeated, tag="2")]
    pub peers: ::prost::alloc::vec::Vec<EntitySource>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntitySource {
    #[prost(string, required, tag="1")]
    pub sn: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(enumeration="super::op_def::NodeType", required, tag="3")]
    pub r#type: i32,
    #[prost(string, optional, tag="4")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
}
