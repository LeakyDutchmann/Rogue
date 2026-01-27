use crate::player::*;


pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

) {
    //PLAYER SETUP
    let texture = asset_server.load("player_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), 
        4,
        6,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0
            }
        ),
        Player {
            state: PlayerState::Idle,
            facing: Facing::Right,
        },
        ActiveAnimation {
            current: AnimationId::IdleRight,
            previous: AnimationId::IdleRight,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Speed(125.0),
        Colider {
            shape: ColiderShape::Circle { radius: 3.0},
            offsety: -5.0,
            sensor: true,
        },
        Inventory {
            capacity: 9,
            items: vec!{None; 9}
        },
        ActiveSlot {
            index: 1,
        },
        
        
    )).with_children(|parent| {
        parent.spawn((
            HeldItem {
                last_held: None,
            },
            Transform::default(),
            AttackAnimation {
                progress: 0.0,
                duration: 0.0,
                max_angle: std::f32::consts::PI / 4.0,
                active: false,
                hit_triggered: false,
                target: None,
                item: None,
                
            }
        ));
    });
    
    //Spawning entity to test colisions, don't forget to remove!
    
}
