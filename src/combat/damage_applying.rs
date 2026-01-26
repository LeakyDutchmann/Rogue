use super::*;



pub fn destruction_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDestruction>,
    mut writer: MessageWriter<MapChanged>,
    mut health: Query<&mut Health>,
) {
    for destruction in reader.read() {
        if let Ok(mut hp) = health.get_mut(destruction.entity) {
            hp.0 -= destruction.damage;
            if hp.0 <= 0 {
                commands.entity(destruction.entity).despawn();
                writer.write(MapChanged {
                    position: destruction.position
                });
            }
        }
    }
}