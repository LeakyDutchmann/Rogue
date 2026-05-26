use super::*;

#[derive(Message)]
pub struct EnemySpawnRequest;


#[derive(Message)]
pub struct ApplySwarmBuff {
    pub entity: Entity
}
