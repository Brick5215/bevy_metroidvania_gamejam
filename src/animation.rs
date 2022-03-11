//==================================================================

use std::{time::Duration, collections::HashMap};

use bevy::prelude::*;

use heron::prelude::*;
//use crate::physics::{Velocity, HasGravity};

//==================================================================

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AnimationType {
    Idle,
    Walk,
    Sprint,
    Jump,
    Fall,
    Custom (String),
}
impl Default for AnimationType {
    fn default() -> Self {
        AnimationType::Idle
    }
}

#[derive(PartialEq)]
pub enum FlipAnimation {
    Flipped,
    UnFlipped,
    None,
}

pub struct ChangeAnimationEvent {
    pub entity: Entity,
    pub new_animation: AnimationType,
    pub restart_animation: bool,
    pub flipped: FlipAnimation,
}

//==================================================================

#[derive(Component, Default, Clone)]
pub struct AutoAnimation(bool);

//==================================================================

#[derive(Clone)]
pub struct Animation{
    texture_atlas: Handle<TextureAtlas>,
    timer: Timer,
    frame_steps: Vec<f32>,
    _total_frames: usize,
    current_frame: usize,
}
impl Animation {
    pub fn with_framesteps(texture_atlas: Handle<TextureAtlas>, frame_steps: Vec<f32>, total_frames: usize, repeating: bool) -> Self {

        if frame_steps.len() != total_frames {
            panic!("Error in animation. frame_steps were not equal to frame_count");
        }

        let mut total_time = 0.;
        for time in frame_steps.iter() {
            total_time += time;
        }

        Animation {
            texture_atlas,
            timer: Timer::from_seconds(total_time, repeating),
            frame_steps: Animation::create_timesteps(frame_steps),
            _total_frames: total_frames,
            current_frame: 0,
        }
    }

    pub fn with_fixed_framestep(texture_atlas: Handle<TextureAtlas>, frame_step: f32, total_frames: usize, repeating: bool) -> Self {

        let frame_steps = vec![frame_step; total_frames];

        Animation {
            texture_atlas,
            timer: Timer::from_seconds(frame_step * total_frames as f32, repeating),
            frame_steps: Animation::create_timesteps(frame_steps),
            _total_frames: total_frames,
            current_frame: 0,
        }
    }

    fn create_timesteps(time_steps: Vec<f32>) -> Vec<f32> {

        let mut to_return = vec!();
        let mut total = 0.;

        for n in 0..time_steps.len() {
            total += time_steps[n];
            to_return.push(total);
        }

        return to_return

    }

    pub fn tick(&mut self, time: Duration) {
        self.timer.tick(time);
    }
    pub fn get_time(&self) -> Duration {
        self.timer.elapsed()
    }
    pub fn done(&self) -> bool {
        self.timer.finished()
    }
}

//==================================================================

#[derive(Component, Clone, Default)] 
pub struct SpriteSheetAnimation {
    pub animations: HashMap<AnimationType, Animation>,
    pub current_animation: AnimationType,
    pub animation_flipped: bool,
}
impl SpriteSheetAnimation {
    pub fn new(animation_type: AnimationType, animation: Animation, flipped: bool) -> Self {
        SpriteSheetAnimation {
            animations: HashMap::from([(animation_type.clone(), animation)]),
            current_animation: animation_type,
            animation_flipped: flipped,
        }
    }

    pub fn add_animation(&mut self, animation_type: AnimationType, animation: Animation) {
        self.animations.insert(animation_type, animation);
    }

    pub fn current(&mut self) -> Option<&mut Animation> {
        let new_val = self.animations.get_mut(&self.current_animation);
        return new_val
    }

    pub fn set_current(&mut self, new_type: AnimationType, flipped: bool, restart: bool) -> Option<(Handle<TextureAtlas>, usize)> {

        match self.animations.get_mut(&new_type) {
            Some(value) => {

                self.current_animation = new_type;
                self.animation_flipped = flipped;

                if restart {
                    value.current_frame = 0;
                }

                return Some((value.texture_atlas.clone(), value.current_frame))

            },
            None => {
                return None
            },
        }
    }
}

//==================================================================

fn animate_spritesheet(
    time: Res<Time>,
    mut query: Query<(&mut SpriteSheetAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut sprite_sheet, mut sprite) in query.iter_mut() {

        if let Some(mut animation) = sprite_sheet.current() {

            animation.tick(time.delta());

            if animation.done() {
                animation.current_frame = 0;
                sprite.index = animation.current_frame;
            }
            else if animation.get_time().as_secs_f32() >= animation.frame_steps[animation.current_frame] {
                animation.current_frame += 1;
                sprite.index = animation.current_frame;
            }

        }
    }
}

//==================================================================

fn set_sprite(
    query: Query<(Entity, &Velocity), With<SpriteSheetAnimation>>,
    mut event: EventWriter<ChangeAnimationEvent>,
) {
    for (entity, velocity) in query.iter() {

        if velocity.linear.x == 0. {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Idle,
                restart_animation: false,
                flipped: FlipAnimation::None,
            });
        }

        else if velocity.linear.x > 0. {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Walk,
                restart_animation: false,
                flipped: FlipAnimation::UnFlipped,
            });
        }
        else if velocity.linear.x < 0. {
            event.send(ChangeAnimationEvent {
                entity,
                new_animation: AnimationType::Walk,
                restart_animation: false,
                flipped: FlipAnimation::Flipped,
            });
        }
    }
}



fn change_animation(
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

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_event::<ChangeAnimationEvent>()

            .add_system(animate_spritesheet)
            .add_system(change_animation)

            .add_system_to_stage(CoreStage::PreUpdate, set_sprite)
        ;
    }
}


//==================================================================