use super::*;


#[derive(Component)]
pub struct Processing {
    pub input: Vec<ItemStack>,
    pub output: Vec<ItemStack>,
    pub timer: Timer,
}