//===============================================================

use bevy::prelude::*;

pub mod physics_components;
mod physics_systems;

//===============================================================

pub struct CustomPhysicsPlugin;
impl Plugin for CustomPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<physics_components::GroundedEvent>()

            .add_system(physics_systems::cap_velocity)
            .add_system(physics_systems::apply_movespeed)

            .add_system(physics_systems::apply_jump)
            .add_system(physics_systems::reset_jump.after("GroundCheck"))

            .add_system(physics_systems::check_on_ground.label("GroundCheck"))
            .add_system(physics_systems::check_on_wall)

            .add_system(physics_systems::set_gravity_scale)
        ;
    }
}

//===============================================================