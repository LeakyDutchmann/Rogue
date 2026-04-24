use super::*;


#[derive(PartialEq, Clone, Copy)]
pub enum DamageType {
    ToTileDamage,
    ToEnemyDamage,
    ToStructureDamage,
}


#[derive(Message)]
pub struct ApplyDamage {
    pub entity: Entity,
    pub from_pos: Vec2,
    pub position: Vec2,
    pub damage: i32,
    pub damage_type: DamageType,
}


#[derive(Message)]
pub struct CalculateDamage {
    pub attack_item: String,
    pub target: Entity,
    pub from_pos: Vec2,
    pub position: Vec2,
    pub damage_type: DamageType,
}



