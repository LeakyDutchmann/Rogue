use rand::Rng;

use super::*;

pub fn pick_up_item(
    mut commands: Commands,
    mut intender: Query<(Entity,&mut Inventory, &Transform, &IntentPickingUp)>,
    mut item: Query<(&Transform, &Sprite), With<OnGround>>,
) {
    for (entity, mut inventory, transform, intent ) in intender.iter_mut() {
        commands.entity(entity).remove::<IntentPickingUp>();
        println!("Picking up item");
        let _intender_pos = transform.translation.truncate();
        let target = intent.target;
        if let Ok((_target_tf, _sprite)) = item.get_mut(target) {
            println!("Item found");
            for slot in inventory.items.iter_mut() {
                if slot.is_none() {
                    *slot = Some(target);
                    commands.entity(target).remove::<OnGround>();
                    commands.entity(target).remove::<Sprite>();
                    commands.entity(target).insert(InInventory);
                    println!("Picked up item");
                    break;
                }
            }
        } else {
            commands.entity(entity).remove::<IntentPickingUp>();
            println!("failed to get item")
        }
    }
}

pub fn update_dropped_items(
    mut commands: Commands,
    player_pos: Query<&Transform, With<Player>>,
    mut reader: MessageReader<ItemDropped>,
    img: Query<&Item>, 
) {
    for msg in reader.read() {
        println!("event received");
        if let Ok(pos) = player_pos.single() {
            println!("player position found");
            let player_pos = pos.translation.truncate();
            let new_pos = generate_random_coords(player_pos);
            if let Some(item) = msg.item {
                println!("item found in msg");
                if let Ok(img) = img.get(item) {
                    commands.entity(item).remove::<InInventory>();
                    commands.entity(item).insert(OnGround);
                    commands.entity(item).insert(Sprite::from_image(img.image.clone()),);
                    commands.entity(item).insert(Transform::from_xyz(new_pos.x, new_pos.y, 1.0));
                } 
            }
        } 
    }
}

fn generate_random_coords(pos: Vec2) -> Vec2 {
    let mut rng = rand::rng();
    let dx = rng.random_range(-30.0..30.0);
    let dy = rng.random_range(-30.0..30.0);
    Vec2::new(pos.x + dx, pos.y + dy)
}

pub fn sync_player_inventory(
    mut commands: Commands,
    inventory: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    mut slots: Query<(Entity, &mut SlotIcon)>, item_query: Query<&Item>,
) {
    if let Ok(inventory) = inventory.single() {
        println!("foundi nventory");
        for (slot_e, slot_icon) in slots.iter_mut() {
            let slot_idx = slot_icon.index;
            println!("found idx");
            if let Some(Some(item_entity)) = inventory.items.get(slot_idx) {
                let texture = item_query.get(*item_entity).unwrap();
                let image = texture.image.clone();
                commands.entity(slot_e).insert(ImageNode::new(image));
                println!("inserted img");
            } else {
                commands.entity(slot_e).remove::<ImageNode>();
            } 
        } 
    } 
}


