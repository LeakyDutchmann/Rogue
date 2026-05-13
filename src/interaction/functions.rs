use super::*;

pub fn determine_target_container(from: ContainerType, interaction_state: &InteractionState) -> Option<ContainerType> {
    match from {
        ContainerType::Inventory => {
            match interaction_state.interaction_type {
                InteractionType::BasicOven => {
                    Some(ContainerType::Input { entity: interaction_state.entity.unwrap() })
                }
                InteractionType::Chest => {
                    Some(ContainerType::Chest { entity: interaction_state.entity.unwrap() })
                }
                _ => {
                    None
                }
            }
        }
        _ => {
            Some(ContainerType::Inventory)
        }
    }
}
