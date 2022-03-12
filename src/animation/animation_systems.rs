//==================================================================

use bevy::prelude::*;
use heron::Velocity;

use crate::physics::physics_components::IsGrounded;

use super::animation_components::*;

//==================================================================

pub fn animate_spritesheet(
    time: Res<Time>,
    mut query: Query<(Entity, &mut SpriteSheetAnimation, &mut TextureAtlasSprite)>,
    mut animation_finished_event: EventWriter<AnimationFinishedEvent>,
) {
    for (entity, mut sprite_sheet, mut sprite) in query.iter_mut() {

        if let Some(animation) = sprite_sheet.current() {

            animation.tick(time.delta());

            if animation.done() {
                animation.restart_animation();
                sprite.index = animation.current_frame();

                animation_finished_event.send(AnimationFinishedEvent {
                    entity,
                    animation_type: sprite_sheet.current_animation.clone(),
                })

            }
            else if animation.get_time().as_secs_f32() >= animation.get_frame_step(animation.current_frame()) {
                animation.next_frame();
                sprite.index = animation.current_frame();
            }
        }
    }
}

pub fn animate_simple_spritesheet(
    time: Res<Time>,
    mut query: Query<(Entity, &mut SimpleAnimation, &mut TextureAtlasSprite)>,
    mut animation_finished_event: EventWriter<AnimationFinishedEvent>,
) {

    for (entity, mut animation, mut sprite) in query.iter_mut() {

        animation.tick(time.delta());

        if animation.done() {
            animation.restart_animation();
            sprite.index = animation.current_frame();

            animation_finished_event.send(AnimationFinishedEvent {
                entity,
                animation_type: animation.animation_type(),
            })

        }
        else if animation.get_time().as_secs_f32() >= animation.frame_step(animation.current_frame()) {
            animation.next_frame();
            sprite.index = animation.current_frame();
        }

    }
}

//==================================================================

pub fn set_sprite_auto(
    query: Query<(Entity, &AutoAnimation, &Velocity, Option<&IsGrounded>), With<SpriteSheetAnimation>>,
    mut event: EventWriter<ChangeAnimationEvent>,
) {
    for (entity, auto_anim, velocity, entity_grounded) in query.iter() {

        if auto_anim.disabled { continue; }

        let flipped = 
            if      velocity.linear.x > 0.  { FlipAnimation::UnFlipped } 
            else if velocity.linear.x < 0.  { FlipAnimation::Flipped }
            else                            { FlipAnimation::None };

        let mut grounded = true;
        if let Some(is_grounded) = entity_grounded {
            grounded = is_grounded.grounded;
        }

        //Entity is rising quickly
        if !grounded && velocity.linear.y > 60. {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Jump,
                restart_animation: false,
                flipped,
            });
        }
        //Entity is starting to fall or falling slowly
        else if !grounded && velocity.linear.y > -60. {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::BeginFall,
                restart_animation: false,
                flipped,
            });
        }
        else if !grounded {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Fall,
                restart_animation: false,
                flipped,
            });
        }

        else if velocity.linear.x == 0. {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Idle,
                restart_animation: false,
                flipped,
            });
        }

        else {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Walk,
                restart_animation: false,
                flipped,
            });
        }
    }
}

pub fn change_animation(
    mut query: Query<(&mut SpriteSheetAnimation, &mut Handle<TextureAtlas>, &mut TextureAtlasSprite)>,
    mut event_reader: EventReader<ChangeAnimationEvent>,
) {
    for event in event_reader.iter() {

        if let Ok(entity) = query.get_mut(event.entity) {

            let (mut sprite_sheet, mut texture_atlas, mut sprite) = entity;

            let mut flipped = false;
            if event.flipped == FlipAnimation::None {flipped = sprite_sheet.animation_flipped}
            else if event.flipped == FlipAnimation::Flipped {flipped = true}

            if  sprite_sheet.current_animation != event.new_animation || 
                sprite_sheet.animation_flipped != flipped
            {
                
                if let Some((handle, frame)) = sprite_sheet.set_current(event.new_animation.clone(), flipped, event.restart_animation) {
                    sprite.index = frame;
                    sprite.flip_x = flipped;
                    *texture_atlas = handle;
                }
            }
        }
    }
}

//==================================================================