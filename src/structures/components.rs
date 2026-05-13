use super::*;


#[derive(Component)]
pub struct Processing {
    pub input: Vec<ItemStack>,
    pub output: Vec<ItemStack>,
    pub timer: Timer,
}


#[derive(Component)]
pub struct WorkBench;


#[derive(Component)]
pub struct Chest {
    pub items: Vec<ItemStack>,
}
