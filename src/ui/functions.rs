use bevy::asset;

use super::*;

pub fn match_kind(kind: &RawNodeKind, window: Entity, commands: &mut Commands, asset_server: &AssetServer) {
    match kind {
        RawNodeKind::Container => {
            
        },
        RawNodeKind::Slot{kind, index} => {
            match kind {
                SlotKind::Output => {
                    commands.entity(window).insert(
                        OvenOutputSlot { 
                            index: index.clone(),
                        }
                    );
                },
                SlotKind::Input => {
                    commands.entity(window).insert(
                        OvenInputSlot { 
                            index: index.clone(),
                        }
                    );
                },
            }
        },
        RawNodeKind::Image{path} => {
            let image = asset_server.load(path);
            commands.entity(window).insert(
                ImageNode::new(image)
            );
        },
        RawNodeKind::Text{content} => {
            commands.entity(window).insert(
                Text::new(content)
            );
        }
    }
}

pub fn apply_style(style: &RawStyle) -> Node {
    let mut node = Node::default();

    if let Some(w) = &style.width {
        node.width = match w {
            RawVal::Auto => Val::Auto,
            RawVal::Px(v) => Val::Px(*v),
            RawVal::Percent(v) => Val::Percent(*v),
        };
    }

    if let Some(h) = &style.height {
        node.height = match h {
            RawVal::Auto => Val::Auto,
            RawVal::Px(v) => Val::Px(*v),
            RawVal::Percent(v) => Val::Percent(*v),
        };
    }

    if let Some(fd) = &style.flex_direction {
        node.flex_direction = match fd {
            RawFlexDirection::Row => FlexDirection::Row,
            RawFlexDirection::Column => FlexDirection::Column,
            RawFlexDirection::RowReverse => FlexDirection::RowReverse,
            RawFlexDirection::ColumnReverse => FlexDirection::ColumnReverse,
        };
    }

    if let Some(j) = &style.justify_content {
        node.justify_content = match j {
            RawJustifyContent::FlexStart => JustifyContent::FlexStart,
            RawJustifyContent::FlexEnd => JustifyContent::FlexEnd,
            RawJustifyContent::Center => JustifyContent::Center,
            RawJustifyContent::SpaceBetween => JustifyContent::SpaceBetween,
            RawJustifyContent::SpaceAround => JustifyContent::SpaceAround,
            RawJustifyContent::SpaceEvenly => JustifyContent::SpaceEvenly,
        };
    }

    if let Some(a) = &style.align_items {
        node.align_items = match a {
            RawAlignItems::FlexStart => AlignItems::FlexStart,
            RawAlignItems::FlexEnd => AlignItems::FlexEnd,
            RawAlignItems::Center => AlignItems::Center,
            RawAlignItems::Stretch => AlignItems::Stretch,
        };
    }

    if let Some(g) = &style.gap {
        let val = match g {
            RawVal::Px(v) => Val::Px(*v),
            RawVal::Percent(v) => Val::Percent(*v),
            RawVal::Auto => Val::Auto,
        };
        node.row_gap = val;
        node.column_gap = val;
    }

    if let Some(m) = &style.margin {
        node.margin = UiRect {
            left: to_val(&m.left),
            right: to_val(&m.right),
            top: to_val(&m.top),
            bottom: to_val(&m.bottom),
        };
    }

    if let Some(pt) = &style.position_type {
        node.position_type = match pt {
            RawPositionType::Relative => PositionType::Relative,
            RawPositionType::Absolute => PositionType::Absolute,
        };
    }

    if let Some(v) = &style.left {
        node.left = to_val(v);
    }
    if let Some(v) = &style.top {
        node.top = to_val(v);
    }
    if let Some(v) = &style.right {
        node.right = to_val(v);
    }
    if let Some(v) = &style.bottom {
        node.bottom = to_val(v);
    }

    if let Some(b) = &style.border {
        node.border = UiRect::all(to_val(b));
    }

    node
}

pub fn to_val(v: &RawVal) -> Val {
    match v {
        RawVal::Auto => Val::Auto,
        RawVal::Px(x) => Val::Px(*x),
        RawVal::Percent(x) => Val::Percent(*x),
    }
}

pub fn assemble_node(
    commands: &mut Commands,
    ui_window: &RawNode,
    asset_server: &AssetServer
) -> Entity {
    let window = commands.spawn((
        apply_style(&ui_window.style),
    )).id();
    match_kind(&ui_window.kind, window, commands, asset_server);
    if let Some(zindex) = &ui_window.zindex {
        commands.entity(window).insert(
            ZIndex(*zindex),
        );
    }
    if let Some(bg_color) = &ui_window.background_color {
        let color = match bg_color {
            RawColor::srgb(r, g, b) => Color::srgb(*r, *g, *b),
            RawColor::srgba(r, g, b, a) => Color::srgba(*r, *g, *b, *a),
        };
        commands.entity(window).insert(        
            BackgroundColor(color),
        );
    }
    if let Some(border_color) = &ui_window.border_color {
        let color = match border_color {
            RawColor::srgb(r, g, b) => Color::srgb(*r, *g, *b),
            RawColor::srgba(r, g, b, a) => Color::srgba(*r, *g, *b, *a),
        };
        commands.entity(window).insert(
            BorderColor::all(color),
        );
    }
    if let Some(interactive) = &ui_window.interactive {
        if *interactive {
            commands.entity(window).insert(
                Interaction::None
            );
        }
    }
    if let Some(marker) = &ui_window.marker {
        match marker {
            RawMarker::UiStructureWindow => {
                commands.entity(window).insert(
                    UiStructureWindow
                );
            }
            RawMarker::SlotCounter => {
                commands.entity(window).insert(
                    SlotCounter
                );
            }
        }
    }
    if let Some(focus_policy) = &ui_window.focus_policy {
        match focus_policy {
            RawPolicy::Block => {
                commands.entity(window).insert(
                    FocusPolicy::Block
                );
            }
            RawPolicy::Pass => {
                commands.entity(window).insert(
                    FocusPolicy::Pass
                );
            }
        }
    }
    if let Some(child_vec) = &ui_window.children {
        for child_raw in child_vec {
            let child = assemble_node(commands, child_raw, asset_server);
            commands.entity(window).add_child(child);
        }
    }
    window
}


pub fn assemble_ui(
    window_name: String,
    ui_reg: &UiWindowRegistry,
    commands: &mut Commands,
    asset_server: &AssetServer) {
    if let Some(ui_window) = ui_reg.windows.get(&window_name) {
        assemble_node(commands, ui_window, asset_server);
    }
}