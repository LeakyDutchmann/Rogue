use super::*;

pub fn load_structures(
    mut struct_registry: ResMut<StructureRegistry>,
    mut recipe_registry: ResMut<RecipeRegistry>,
    assest_server: Res<AssetServer>,
) { 
    let path = "./data/structures";
    if let Ok(structures) = load_definitions_for::<StructureDefinitionRaw>(path) {
        let mut count = 0;
        for structure in structures {
            let sprite = assest_server.load(&structure.sprite_path);
            let icon = assest_server.load(&structure.icon_path);
            if let Some(recipe) = structure.recipe {
                recipe_registry.recipes.insert(structure.name.clone(), recipe);
            }
            let definition = StructureDefinition {
                sprite,
                icon,
                width: structure.width,
                height: structure.height,
                radius: structure.radius,
                interaction: structure.interaction,
            };
            struct_registry.structures.insert(structure.name.clone(), definition);
            count += 1;
        }
        println!("Loaded {} structures", count);
    }
}
