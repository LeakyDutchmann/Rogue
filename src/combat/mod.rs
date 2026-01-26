mod damage_applying;
mod hit_detections;

use bevy::prelude::*;
use hit_detections::*;
use damage_applying::*;
use crate::mouse::*;
use crate::map_setup::*;
use crate::player::*;
use crate::world::*;


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ApplyDestruction>();
        app.add_systems(Update, detect_hit_system);
    }
}

//messages 


#[derive(Message)]
pub struct ApplyDestruction {
    pub entity: Entity,
    pub position: IVec2,
}





//enums




//resources


//components


