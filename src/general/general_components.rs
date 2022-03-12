
use bevy::prelude::*;

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
}
impl Health {
    pub fn new(health: i32) -> Self {
        Health {
            max_health: health,
            current_health: health,
        }
    }
    pub fn new_full(max_health: i32, current_health: i32) -> Self {
        Health {
            max_health,
            current_health,
        }
    }

    pub fn get_health(&self) -> i32 {
        return self.current_health;
    }
    pub fn get_max_health(&self) -> i32 {
        return self.max_health;
    }
    pub fn add_health(&mut self, to_add: i32) {
        self.current_health = (self.current_health + to_add).min(self.max_health);
    }
    pub fn set_health(&mut self, to_set: i32) {
        self.current_health = to_set.min(self.max_health);
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

//================================================================