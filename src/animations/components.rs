use super::*;


#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum AnimationId {
    IdleRight,
    IdleLeft,
    WalkRight,
    WalkLeft,
    WalkUp,
    WalkDown,
}

impl AnimationId {
    pub fn idle_from(facing: Facing) -> Self {
        match facing {
            Facing::Right =>  AnimationId::IdleRight,
            Facing::Left =>  AnimationId::IdleLeft,
            Facing::Up =>  AnimationId::IdleRight,
            Facing::Down =>  AnimationId::IdleRight,
        }
    }
    
    pub fn walk_from(facing: Facing) -> Self {
        match facing {
            Facing::Right =>  AnimationId::WalkRight,
            Facing::Left =>  AnimationId::WalkLeft,
            Facing::Up =>  AnimationId::WalkUp,
            Facing::Down =>  AnimationId::WalkDown,
        }
    }
}


#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Component, PartialEq)]
pub struct ActiveAnimation{
    pub current: AnimationId,
    pub previous: AnimationId,
}

impl ActiveAnimation {
    pub fn set_animation(&mut self, new_animation: AnimationId) {
            self.previous = self.current;
            self.current = new_animation;
    }
}