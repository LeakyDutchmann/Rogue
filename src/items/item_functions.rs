use super::*;

pub fn assemble_item(definition: &ItemDefinition, commands: &mut Commands, item_id: &String) -> Entity {
    let mut entity = commands.spawn(());
    if definition.usable {
        entity.insert(Usable);
    }
    entity.insert(Sprite::from_image(definition.sprite.clone()));
    entity.insert(Speed(50.0));
    entity.insert(ItemId{
        id: item_id.clone(),
    });
    entity.insert(Colider{
        shape: ColiderShape::Circle { radius: (5.0) },
        _offsety: 0.0,
        _sensor: false,
    });
    println!("asssembled: {:?}", entity.id());
    entity.id()
}