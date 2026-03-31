mod systems;

use super::*;
use systems::*;
use std::collections::HashMap;
use serde::Deserialize;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
      app.insert_resource(RecipeRegistry {
          recipes: HashMap::new()
      });  
    }
}


#[derive(Deserialize)]
pub struct RecipeDefinition {
    pub ingredients: HashMap<String, u32>,
}


#[derive(Resource)]
pub struct RecipeRegistry {
    pub recipes: HashMap<String, RecipeDefinition>
}