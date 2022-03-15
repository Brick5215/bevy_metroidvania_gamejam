
use std::time::Duration;

use bevy::prelude::*;

//================================================================

#[derive(Component)]
pub struct GameCamera;

//================================================================

#[derive(Component)]
pub struct FadeInOut {
    pub timer: Timer,
    pub from: f32,
    pub to: f32,
    pub remove_on_finish: bool,
    pub remove_component_on_finish: bool,
}

//================================================================

#[derive(Component, Clone, Default)]
pub struct Health {
    max_health: i32,
    current_health: i32,
    iframes: Timer,
    invincible: bool,
}
impl Health {
    pub fn new(health: i32, iframes: f32) -> Self {
        Health {
            max_health: health,
            current_health: health,
            iframes: Timer::from_seconds(iframes, false),
            invincible: false,
        }
    }
    pub fn _new_full(max_health: i32, current_health: i32, iframes: f32) -> Self {
        Health {
            max_health,
            current_health,
            iframes: Timer::from_seconds(iframes, false),
            invincible: false,
        }
    }

    pub fn get_health(&self) -> i32 {
        return self.current_health;
    }
    pub fn get_max_health(&self) -> i32 {
        return self.max_health;
    }
    pub fn add_health(&mut self, to_add: i32) {
        if self.invincible {
            return;
        }
        self.current_health = (self.current_health + to_add).min(self.max_health);

        //println!("Health lost. Now: {}", self.current_health);

        if to_add < 0 {
            self.invincible = true;
            self.iframes.reset();
        }
    }
    pub fn set_health(&mut self, to_set: i32) {
        self.current_health = to_set.min(self.max_health);
    }
    pub fn tick(&mut self, delta: Duration) {
        self.iframes.tick(delta);

        if self.iframes.finished() {
            self.invincible = false;
        }
    }
    pub fn invincible(&self) -> bool {
        return self.invincible;
    }
}

pub enum HealthChangeType{
    Set { value: i32,},
    Add { value: i32,},
}

pub struct HealthChangeEvent{
    pub entity: Entity,
    pub change_type: HealthChangeType,
}

#[derive(Component, Default, Clone)]
pub struct HealthFlash {
    pub returning_to_original:  bool,
    pub start_color:         Vec3,
    pub target_color:           Vec3,
    pub change_timer:           Timer,
}
impl HealthFlash {
    pub fn new(start_color: Color, target_color: Color, time: f32) -> Self {

        let new_start_color = Vec3::new(
            start_color.r(),
            start_color.g(),
            start_color.b(),
        );
        let new_target_color = Vec3::new(
            target_color.r(),
            target_color.g(),
            target_color.b(),
        );
        HealthFlash {
            returning_to_original: false,
            start_color: new_start_color,
            target_color: new_target_color,
            change_timer: Timer::from_seconds(time, false),
        }
    }
}

//================================================================

pub struct EntityDiedEvent(pub Entity);

//================================================================