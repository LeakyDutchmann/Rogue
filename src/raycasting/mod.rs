mod math;
mod enemy_vision;

#[cfg(feature = "prototypes")]
mod prototype;

use super::*;
pub use math::*;
use enemy_vision::*;



pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_vision_system);
    }
}

pub enum AwarenessType {
    Unaware,
    Direct,
    Indirect,
}

#[derive(Component)]
pub struct EnemyAwareness {
    pub state: AwarenessType,
    pub player_seen: bool,
    pub radius: f32,
    pub awareness_timer: Timer,
}

