//===============================================================

use bevy::prelude::*;

use std::{time::Duration, collections::HashMap};

//===============================================================

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum AnimationType {
    Idle,
    Walk,
    _Sprint,
    Jump,
    BeginFall,
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

pub struct AnimationFinishedEvent {
    pub entity: Entity,
    pub animation_type: AnimationType,
}

//==================================================================

#[derive(Component, Default, Clone)]
pub struct AutoAnimation {
    pub disabled: bool,
}

//==================================================================

#[derive(Clone, Default)]
pub struct Animation {
    texture_atlas: Handle<TextureAtlas>,
    timer: Timer,
    frame_steps: Vec<f32>,
    _total_frames: usize,
    current_frame: usize,
}
impl Animation {
    pub fn with_custom_framesteps(texture_atlas: Handle<TextureAtlas>, frame_steps: Vec<f32>, total_frames: usize, repeating: bool) -> Self {

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

    pub fn with_fixed_framesteps(texture_atlas: Handle<TextureAtlas>, frame_step: f32, total_frames: usize, repeating: bool) -> Self {

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
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
    pub fn next_frame(&mut self) {
        self.current_frame += 1;
    }
    pub fn restart_animation(&mut self) {
        self.current_frame = 0;
    }
    pub fn get_frame_step(&self, index: usize) -> f32 {
        self.frame_steps[index]
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

#[derive(Bundle, Clone)]
pub struct SimpleAnimationBundle {
    pub animation: SimpleAnimation,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}
impl SimpleAnimationBundle {
    pub fn new(
        animation_type: AnimationType,
        frame_steps: Vec<f32>,
        repeating: bool,
        animation_handle: Handle<TextureAtlas>,

    ) -> Self {

        SimpleAnimationBundle {

            animation: SimpleAnimation::with_custom_framesteps(
                animation_type,
                frame_steps,
                repeating,
            ),
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: animation_handle,
                ..Default::default()
            }
        }
    }

    pub fn flip_x(&mut self, flip: bool) {
        self.sprite_sheet.sprite.flip_x = flip;
    }
    pub fn flip_y(&mut self, flip: bool) {
        self.sprite_sheet.sprite.flip_y = flip;
    }
    
}


#[derive(Component, Clone, Default)]
pub struct SimpleAnimation {
    animation_type: AnimationType,
    timer: Timer,
    frame_steps: Vec<f32>,
    current_frame: usize,
}
impl SimpleAnimation {

    pub fn with_custom_framesteps(animation_type: AnimationType, frame_steps: Vec<f32>, repeating: bool) -> Self {

        let mut total_time = 0.;
        for time in frame_steps.iter() {
            total_time += time;
        }

        SimpleAnimation {
            animation_type,
            timer: Timer::from_seconds(total_time, repeating),
            frame_steps: SimpleAnimation::create_timesteps(frame_steps),
            current_frame: 0,
        }
    }

    pub fn with_fixed_framesteps(animation_type: AnimationType, frame_step: f32, total_frames: usize, repeating: bool) -> Self {

        let frame_steps = vec![frame_step; total_frames];

        SimpleAnimation {
            animation_type,
            timer: Timer::from_seconds(frame_step * total_frames as f32, repeating),
            frame_steps: SimpleAnimation::create_timesteps(frame_steps),
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


    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }
    pub fn get_time(&self) -> Duration {
        return self.timer.elapsed();
    }
    pub fn done(&self) -> bool {
        return self.timer.finished();
    }
    pub fn restart_animation(&mut self) {
        self.current_frame = 0;
    }
    pub fn current_frame(&self) -> usize {
        return self.current_frame;
    }
    pub fn frame_step(&self, index: usize) -> f32 {
        return self.frame_steps[index];
    }
    pub fn next_frame(&mut self) {
        self.current_frame += 1;
    }
    pub fn animation_type(&self) -> AnimationType {
        return self.animation_type.clone();
    }
}

//==================================================================