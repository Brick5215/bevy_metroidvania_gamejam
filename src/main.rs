use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

mod arena;
mod physics;
mod player;

mod general_components;
mod systems;
mod tools;

fn main() {
    
    App::new()
        //-------------------------------------------------

        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(LdtkPlugin)

        .add_plugin(physics::CustomPhysicsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(arena::ArenaPlugin)

        //-------------------------------------------------

        .insert_resource(Gravity::from(Vec2::new(0., -400.,)))

        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
        })

        //-------------------------------------------------

        .add_startup_system(systems::setup)

        .add_system(systems::pause_physics_while_load)

        .add_system(systems::fade_in_out)

        //-------------------------------------------------

        //-------------------------------------------------

        .run();
}