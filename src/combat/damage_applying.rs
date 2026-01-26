use super::*;



pub fn destruction_system(
    mut commands: Commands,
    mut reader: MessageReader<ApplyDestruction>,
    mut writer: MessageWriter<MapChanged>,
) {
    for destruction in reader.read() {
        commands.entity(destruction.entity).despawn();
        writer.write(MapChanged {
            position: destruction.position
        });
    }
}