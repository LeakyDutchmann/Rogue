use super::*;

#[derive(Component)]
pub struct FieldOfView {
    pub triangles: Option<Vec<(Vec3, Vec3, Vec3)>>,
}

#[derive(Component)]
pub struct CustomShape;