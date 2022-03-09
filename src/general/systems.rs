use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{tools, general_components::FadeInOut};

//================================================================

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

//================================================================

pub fn fade_in_out(
    mut query: Query<(Entity, &mut FadeInOut, Option<&mut TextureAtlasSprite>, Option<&mut Sprite>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut fade, atlas_sprite, sprite) in query.iter_mut() {

        fade.timer.tick(time.delta());

        let new_alpha = tools::lerp(fade.from, fade.to, fade.timer.percent());

        if let Some(mut atlas_sprite) = atlas_sprite {
            atlas_sprite.color.set_a(new_alpha);
        }
        if let Some(mut sprite) = sprite {
            sprite.color.set_a(new_alpha);
        }

        if fade.timer.finished() {
            if fade.remove_on_finish {
                commands.entity(entity).despawn();
            }
            else {
                commands.entity(entity).remove::<FadeInOut>();
            }
            continue
        }
    }
}

//================================================================