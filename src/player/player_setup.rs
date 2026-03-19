use crate::{components::Health, player::*};

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

) {
    let texture = asset_server.load("player_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(32), 
        4,
        7,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    
    let initial_transform = Transform {
        translation: Vec3::new(0.0, 0.0, 1.0),
        ..Default::default()
    };
    commands.insert_resource(PlayerTransform(initial_transform));
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0
            }
        ),
        Player,
        FacingDirection {
            facing: Facing::Right,
        },
        ActorState {
            state: ActorStateType::Idle
        },
        ActiveAnimation {
            current: AnimationId::IdleRight,
            previous: AnimationId::IdleRight,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        initial_transform,
        Speed(125.0),
        Colider {
            shape: ColiderShape::Circle { radius: 3.0},
            _offsety: -5.0,
            _sensor: true,
        },
        Inventory {
            capacity: 36,
            items: vec![ItemStack { item_stored: None, quantity: 0 }; 36],
        },
        ActiveSlot {
            index: 1,
        },
        FieldOfView {
            triangles: None,
        },
        HurtBox {
            radius: 3.0,
            fraction: FractionType::Player,
        },
        Health(100),
    )).with_children(|parent| {
        parent.spawn((
            HeldItem {
                held: None,
                last_held: None,
            },
            Transform::default(),
        ));

    });  
}
