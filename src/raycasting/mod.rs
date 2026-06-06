mod math;
mod enemy_vision;

#[cfg(feature = "prototypes")]
mod prototype;

use super::*;
pub use math::*;
use enemy_vision::*;
use crate::enemy::Surrounding;



pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
    }
}


