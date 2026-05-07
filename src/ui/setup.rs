use super::*;

pub fn spawn_tool_tip(
    mut commands: Commands,
) {
    commands.spawn((
        ToolTip,
        Node {
            border: UiRect::all(Val::Px(4.0)),
            width: Val::Px(300.0),
            height: Val::Px(150.0),
            ..Default::default()
        },
        // BackgroundColor(Color::BLACK),
        BorderColor::all(Color::WHITE),
        ZIndex(5),
        Text::new("_")
    ));
}

pub fn load_ui_winows(
    mut ui_reg: ResMut<UiWindowRegistry>,
    mut console: ResMut<Console>,
) {
    let path = String::from("./data/ui_windows");
    let result = load_definitions_for::<RawNode>(&path);
    let mut count = 0;
    if let Ok(defs) = result {
        for def in defs {
            ui_reg.windows.insert(def.name.clone(), def);
            count += 1;
        }
        console.log(format!("loaded {} ui windows", count))
    }
}