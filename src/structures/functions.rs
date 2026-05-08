use super::*;

pub fn assemble_structure(definition: &StructureDefinition, commands: &mut Commands, item_id: &String) -> Entity {
    let mut entity = commands.spawn(());
    entity.insert(Sprite::from_image(definition.sprite.clone()));
    entity.insert(StructureId{
        id: item_id.clone(),
        
    });
    let width = match definition.width {
        Some(w) => w,
        None => 0.0,
    };
    let height = match definition.height {
        Some(h) => h,
        None => 0.0,
    };
    let radius = match definition.radius {
        Some(r) => r,
        None => 0.0,
    };
    if width > 0.0 && height > 0.0 {
        entity.insert(Colider{
            shape: ColiderShape::Rectangle { width, height },
            _offsety: 0.0,
            _sensor: true,
        });
    } else if radius > 0.0 {
        entity.insert(Colider{
            shape: ColiderShape::Circle { radius },
            _offsety: 0.0,
            _sensor: true,
        });
    }
    match definition.interaction {
        InteractionType::BasicOven => {
            entity.insert(Interactable);
            entity.insert(Processing {
                input: vec![ItemStack { item_stored: None, quantity: 0 }],
                output: vec![ItemStack { item_stored: None, quantity: 0 }],
                timer: Timer::from_seconds(10.0, TimerMode::Repeating),
            });
        }
        InteractionType::WorkBench => {
            entity.insert(WorkBench);
            entity.insert(Interactable);
        }
        _ => {}
    }
    entity.insert(Health(100));
    entity.insert(Wall);
    println!("asssembled structure: {:?}", entity.id());
    entity.id()
}