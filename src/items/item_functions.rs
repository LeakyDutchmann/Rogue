use super::*;

pub fn assemble_item(definition: &ItemDefinition, commands: &mut Commands, item_id: &ItemId) -> Entity {
    let mut entity = commands.spawn(());
    if let Some(combat_stats) = &definition.combat_stats {
        entity.insert(CombatStats{
            attack_speed: combat_stats.attack_speed as f32,
            swing_angle: combat_stats.swing_angle as f32,
            radius: combat_stats.radius as f32,
        });
    }
    if let Some(weapon_stats) = &definition.weapon_stats {
        entity.insert(WeaponStats{
            enemy_damage: weapon_stats.enemy_damage,
        });
    }
    if let Some(tool_stats) = &definition.tool_stats {
        entity.insert(ToolStats {
            structure_damage: tool_stats.structure_damage,
        });
    }
    if let Some(durability) = definition.durability {
        entity.insert(Durability {
            durability: durability,
        });
    }
    if definition.usable {
        entity.insert(Usable);
    }
    entity.insert(Sprite::from_image(definition.sprite.clone()));
    entity.insert(item_id.clone());
    if let Some(animation_style) = definition.animation_style {
        entity.insert(AnimationPattern {
            pattern: animation_style,
        });
    }
    println!("asssembled: {:?}", entity.id());
    entity.id()
}