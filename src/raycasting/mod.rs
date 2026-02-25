mod math;
mod fov_mesh;
mod enemy_vision;

use super::*;
use math::*;
use fov_mesh::*;
use enemy_vision::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::sprite_render::AlphaMode2d;


pub struct VisionPlugin;

impl Plugin for VisionPlugin {
    fn build(&self, app: &mut App) {
        
    }
}


#[derive(Component)]
pub struct FieldOfView {
    pub triangles: Option<Vec<(Vec3, Vec3, Vec3)>>,
}


#[derive(Component)]
pub struct CustomShape;