
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