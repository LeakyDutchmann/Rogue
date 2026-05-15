use super::*;

pub fn spawn_structure(
    mut commands: Commands,
    mut reader: MessageReader<SpawnStructureRequest>,
    structure_reg: Res<StructureRegistry>,
    mut chunkgrid: ResMut<ChunkGrid>,
) {
    for msg in reader.read() {
        if let Some(def) = structure_reg.structures.get(&msg.item_id) {
            let structure = assemble_structure(&def, &mut commands, &msg.item_id);
            if let Some(chunk) = chunkgrid.chunks.get_mut(&msg.chunk_position) {
                chunk.changed = true;
                commands.entity(structure).insert(ParrentChunk { position: msg.chunk_position });
                commands.entity(structure).insert(Transform::from_translation(msg.position.extend((- msg.position.y + 1.0) * 0.001)));
            }
           
        }
    }
}
