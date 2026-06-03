use super::*;

pub fn tick_spawner_system(
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawnerTimer>,
    mut writer: MessageWriter<EnemySpawnRequest>,
) {
    spawner.timer.tick(time.delta());
    if spawner.timer.just_finished() {
        writer.write(EnemySpawnRequest);
    }
}

pub fn spawn_enemy_system(
    mut commands: Commands,
    mut reader: MessageReader<EnemySpawnRequest>,
    asset_server: Res<AssetServer>,
    player_tf: Res<PlayerTransform>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    empty_cells: Res<EmptyCellsWorldPos>,
    enemy_reg: Res<EnemyRegistry>,
) {
    for _ in reader.read() {
        let pos = generate_position_near(&empty_cells.cells, player_tf.0.translation.xy());       
        let name = "Grunt".to_string();
        assemble_enemy(
            &mut commands,
            &asset_server,
            name,
            &enemy_reg,
            &mut texture_atlas_layouts,
            pos,  
        )
    }
}