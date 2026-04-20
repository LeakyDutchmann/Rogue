use super::*;

pub fn poll_pending_chunks(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PendingTaskChunk)>,
) {
    for (entity, mut pending_chunk) in query.iter_mut() {
        if let Some(chunk_data) = future::block_on(future::poll_once(&mut pending_chunk.task)) {
            commands.spawn(
                PendingChunk { chunk: chunk_data }
            );
            commands.entity(entity).despawn();
        }
    }
}

pub fn poll_saving_chunks(
    mut commands: Commands,
    mut chunks: Query<(Entity, &SavingPendingChunk)>,
    mut saved: ResMut<SavedChunks>,
    mut disable_writer: MessageWriter<DisableChunk>,
) {
   
    for (entity, pending) in chunks.iter_mut() {
        if pending.task.is_finished() {
            saved.chunks.insert(pending.pos);
            saved.saving_chunks.remove(&pending.pos);
            println!("chunk saveed {:?}", pending.pos);
            disable_writer.write(DisableChunk { position: pending.pos });
            commands.entity(entity).despawn();
        }
    }
}

pub fn poll_chunk_loading(
    mut commands: Commands,
    mut chunks: Query<(Entity, &mut LoadingPendingChunk)>,
) {
    for (entity, mut pending) in chunks.iter_mut() {
        if let Some(chunk) = future::block_on(future::poll_once(&mut pending.chunk)) {
            if let Some(chunk) = chunk {
                commands.spawn(
                    PendingChunk {
                        chunk,
                    }
                );
                commands.entity(entity).despawn();
            }
        }
    }
}