use super::*;

pub fn track_chunks(
    player_tf: Res<PlayerTransform>,
    mut player_chunk: ResMut<PlayerChunk>,
) {
    let player_pos = player_tf.0.translation.truncate();
    let player_chunk_pos = get_chunk_pos(player_pos);
    if player_chunk.position != player_chunk_pos {
        player_chunk.position = player_chunk_pos;
    }  
}

pub fn chunk_handler(
    mut chunkgrid: ResMut<ChunkGrid>,
    player_chunk: Res<PlayerChunk>,
    mut saved: ResMut<SavedChunks>,
    mut writer: MessageWriter<PrepareChunk>,
    mut disable_writer: MessageWriter<DisableChunk>,
    mut save_writer: MessageWriter<SaveChunk>,
    mut load_writer: MessageWriter<LoadChunk>,
) {
    let active_chunks = vec![
        player_chunk.position,
        player_chunk.position + IVec2::new(0, 1),
        player_chunk.position + IVec2::new(1, 1),
        player_chunk.position + IVec2::new(1, 0),
        player_chunk.position + IVec2::new(1, -1),
        player_chunk.position + IVec2::new(0, -1),
        player_chunk.position + IVec2::new(-1, -1),
        player_chunk.position + IVec2::new(-1, 0),
        player_chunk.position + IVec2::new(-1, 1),
    ];
    for chunk_pos in &active_chunks {
        if !chunkgrid.chunks.contains_key(&chunk_pos) && !chunkgrid.pending_chunks.contains(&chunk_pos) {
            if saved.chunks.contains(chunk_pos) {
                load_writer.write(LoadChunk { position: chunk_pos.clone() });
            } else {
                writer.write(PrepareChunk { position: chunk_pos.clone() });
            }
            
        }
    }
    for (pos, chunk) in chunkgrid.chunks.iter_mut() {
        if !active_chunks.contains(pos) {
            if chunk.changed {
                save_writer.write(SaveChunk { position: pos.clone() });
                saved.saving_chunks.insert(pos.clone());
                chunk.changed = false;
                continue;
            }
            if saved.saving_chunks.contains(pos) {
                continue;
            }
            disable_writer.write(DisableChunk { position: pos.clone() });
        }
    }
}

pub fn track_of_saved_chunks(
    saved: Res<SavedChunks>,
) {
    if !saved.is_changed() {
        return;
    }
    let mut count = 0;
    for _pos in saved.chunks.iter() {
        count += 1;
    }
    println!("saved chunks: {}", count);
}