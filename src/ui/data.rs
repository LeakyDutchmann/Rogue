use super::*;

#[derive(Deserialize)]
pub struct RawNode {
    pub name: String,
    pub kind: RawNodeKind,
    pub style: RawStyle,
    pub children: Option<Vec<RawNode>>,
    pub border_color: Option<RawColor>,
    pub background_color: Option<RawColor>,
    pub zindex: Option<i32>,
    pub interactive: Option<bool>,
    pub focus_policy: Option<RawPolicy>,
    pub marker: Option<RawMarker>,
}

#[derive(Deserialize)]
pub enum RawPolicy {
    Block,
    Pass,
}


#[derive(Deserialize)]
pub enum RawMarker {
    UiStructureWindow,
    SlotCounter,
}


#[derive(Deserialize)]
pub enum RawColor {
    srgb(f32, f32, f32),
    srgba(f32, f32, f32, f32),
}


#[derive(Deserialize)]
pub enum RawVal {
    Auto,
    Px(f32),
    Percent(f32),
}


#[derive(Deserialize)]
pub enum SlotKind {
    Output,
    Input,
}

#[derive(Deserialize)]
pub enum RawNodeKind {
    Container,
    Slot { kind: SlotKind, index: usize},
    Image { path: String },
    Text { content: String },
}


#[derive(Deserialize)]
pub struct RawRect {
    pub left: RawVal,
    pub right: RawVal,
    pub top: RawVal,
    pub bottom: RawVal,
}


#[derive(Deserialize)]
pub enum RawFlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}


#[derive(Deserialize)]
pub enum RawJustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}


#[derive(Deserialize)]
pub enum RawAlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
}


#[derive(Deserialize)]
pub enum RawPositionType {
    Relative,
    Absolute,
}

 
#[derive(Deserialize)]
pub struct RawStyle {
    pub width: Option<RawVal>,
    pub height: Option<RawVal>,
    pub flex_direction: Option<RawFlexDirection>,
    pub justify_content: Option<RawJustifyContent>,
    pub align_items: Option<RawAlignItems>,
    pub gap: Option<RawVal>,
    pub margin: Option<RawRect>,
    pub position_type: Option<RawPositionType>,
    pub left: Option<RawVal>,
    pub top: Option<RawVal>,
    pub right: Option<RawVal>,
    pub bottom: Option<RawVal>,
    pub border: Option<RawVal>,
    
}