use super::*;

#[derive(Component)]
pub struct WorkBenchSlot {
    pub item: Option<String>,
    pub index: usize,
}
