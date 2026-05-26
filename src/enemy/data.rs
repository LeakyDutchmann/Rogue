

use super::*;

#[derive(Deserialize, Hash)]
pub enum EnemyKind {
    Melee,
    Charger,
    Exploder,
    Shooter,
    Ambusher, 
}


#[derive(Deserialize)]
pub struct EnemyDefinitionRaw {
    pub name: String,
    pub hp: i32,
    pub sprite_sheet: String,
    pub swarm_buff: Option<SwarmBuffRaw>,
    pub dead_sprite: String,
    pub kind: EnemyKind,
    pub speed: i32,
    pub awareness_range: i32,
    pub pursuit_time: i32,
    pub colider: ColiderRaw,
    pub hurt_radius: i32,
    pub fraction: FractionType,
    pub held_item: Option<String>,
}

#[derive(Deserialize, Hash)]
pub struct SwarmBuffRaw {
    pub hp: Option<i32>,
    pub speed: Option<i32>,
}