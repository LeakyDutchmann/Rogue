use super::*;



pub fn damage_execution_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDamage>,
    mut writer: MessageWriter<MapChanged>,
    mut health: Query<&mut Health>,
) {
    for destruction in reader.read() {
        if let Ok(mut hp) = health.get_mut(destruction.entity) {
            hp.0 -= destruction.damage;
            println!("Damage applied");
            if hp.0 <= 0 {
                commands.entity(destruction.entity).despawn();
                if destruction.damage_type == DamageType::ToTileDamage {
                    writer.write(MapChanged {
                        position: world_pos_to_tile_pos(destruction.position),
                    });
                    println!("msg sent");
                }
            }
        } else {
            println!("Health not found");
        }
    }
}