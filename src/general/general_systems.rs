use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{tools, general_components::*};

//================================================================

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //let ldtk_handle = assets.load("Tilemaps/TileMapMain.ldtk");
    let ldtk_handle = assets.load("Tilemaps/TileMapTheNew.ldtk");
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

pub fn change_health(
    mut health_query: Query<(Entity, &mut Health)>,
    mut health_event: EventReader<HealthChangeEvent>,
    mut commands: Commands,
) {
    for event in health_event.iter() {
        if let Ok((entity, mut health)) = health_query.get_mut(event.entity) {

            match event.change_type {
                HealthChangeType::Set { value } => {
                    health.set_health(value);
                },
                HealthChangeType::Add { value } => {

                    if value > 0 {  //Healing
                        commands.entity(entity).insert(HealthFlash::new(Color::WHITE, Color::GREEN, 0.2));
                    }
                    else if value < 0 && !health.invincible() { //Damage
                        commands.entity(entity).insert(HealthFlash::new(Color::WHITE, Color::RED, 0.2));
                    }

                    health.add_health(value);
                },
            }
        }
    }
}

pub fn do_iframes(
    mut health_query: Query<&mut Health>,
    time: Res<Time>,
) {
    for mut health in health_query.iter_mut() {
        health.tick(time.delta());
    }
}


pub fn health_flash(
    mut health_flash_query: Query<(Entity, &mut HealthFlash, Option<&mut Sprite>, Option<&mut TextureAtlasSprite>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut flash, sprite, sprite_sheet) in health_flash_query.iter_mut() {

        flash.change_timer.tick(time.delta());

        let new_color = flash.start_color.lerp(flash.target_color, flash.change_timer.percent());


        let new_color = Color::rgb(
            new_color.x,
            new_color.y,
            new_color.z,
        );

        if let Some(mut sprite) = sprite {
            sprite.color = new_color;
        }
        if let Some(mut sprite_sheet) = sprite_sheet {
            sprite_sheet.color = new_color;
        }


        if flash.change_timer.finished() {

            if flash.returning_to_original {
                commands.entity(entity).remove::<HealthFlash>();
            }
            else {
                flash.returning_to_original = true;
                flash.start_color = flash.target_color;
                flash.target_color = Vec3::ONE;

                flash.change_timer.reset();
            }
        }
    }
}

//================================================================