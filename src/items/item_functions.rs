use super::*;

pub fn assemble_item(definition: &ItemDefinition, commands: &mut Commands, item_id: &ItemId) -> Entity {
    let mut entity = commands.spawn(());
    if definition.usable {
        entity.insert(Usable);
    }
    entity.insert(Sprite::from_image(definition.sprite.clone()));
    entity.insert(item_id.clone());
    println!("asssembled: {:?}", entity.id());
    entity.id()
}