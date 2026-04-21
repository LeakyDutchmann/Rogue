use super::*;

pub fn check_if_inventory_has_item(inventory: &Inventory, item: &String, quantity_request: i32) -> bool {
    let mut quantity_inside = 0;
    for slot in &inventory.items {
        if quantity_inside >= quantity_request {
            break;
        }
        if slot.item_stored.as_ref() == Some(item) {
            quantity_inside += slot.quantity;
        }
    }
    if quantity_inside >= quantity_request {
        true
    } else {
        false
    }
}