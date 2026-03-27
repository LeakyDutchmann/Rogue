use super::*;

pub fn assemble_structure(definition: &StructureDefinition, commands: &mut Commands, item_id: &String) -> Entity {
    let mut entity = commands.spawn(());
    entity.insert(Sprite::from_image(definition.sprite.clone()));
    entity.insert(StructureId{
        id: item_id.clone(),
    });
    entity.insert(Colider{
        shape: ColiderShape::Rectangle { width: 20.0, height: 20.0 },
        _offsety: 0.0,
        _sensor: true,
    });
    println!("asssembled structure: {:?}", entity.id());
    entity.id()
}