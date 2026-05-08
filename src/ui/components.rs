use super::*;

#[derive(Component)]
pub struct WorkBenchSlot {
    pub item: Option<String>,
    pub index: usize,
}


#[derive(Component)]
pub struct ChestSlot {
    pub item: Option<String>,
    pub quantity: usize,
    pub index: usize,
}