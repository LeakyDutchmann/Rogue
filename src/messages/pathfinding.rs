use super::*;


#[derive(Message)]
pub struct FindPath {
    pub seeker: Entity,
    pub seeker_pos: Vec2,
    pub target_pos: Vec2,
}
