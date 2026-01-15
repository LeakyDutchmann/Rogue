use super::*;

pub fn pick_up_item(
    mut commands: Commands,
    mut intender: Query<(Entity,&mut Inventory, &Transform, &IntentPickingUp)>,
    mut item: Query<(&Transform, &Sprite), With<OnGround>>,
) {
    for (entity, mut inventory, transform, intent ) in intender.iter_mut() {
        commands.entity(entity).remove::<IntentPickingUp>();
        println!("Picking up item");
        let intender_pos = transform.translation.truncate();
        let target = intent.target;
        if let Ok((target_tf, sprite)) = item.get_mut(target) {
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








