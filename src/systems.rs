use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let ldtk_handle = assets.load("Tilemaps/TileMapMain.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });

}

pub fn pause_physics_while_load(
    mut level_events: EventReader<LevelEvent>,
    mut physics_time: ResMut<PhysicsTime>,
) {
    for event in level_events.iter() {
        match event {
            LevelEvent::SpawnTriggered(_) => physics_time.set_scale(0.),
            LevelEvent::Transformed(_) => physics_time.set_scale(1.),
            _ => (),
        }
    }
}