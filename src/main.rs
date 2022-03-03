use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

mod arena;
mod physics;
mod player;

mod animation;
mod general_components;
mod systems;
mod tools;

fn main() {
    
    App::new()
        //-------------------------------------------------

        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(LdtkPlugin)

        .add_plugin(animation::AnimationPlugin)
        .add_plugin(physics::CustomPhysicsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(arena::ArenaPlugin)

        //-------------------------------------------------

        .insert_resource(Gravity::from(Vec2::new(0., -450.,)))

        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
        })

        //-------------------------------------------------

        .add_startup_system(systems::setup)

        .add_system(systems::pause_physics_while_load)

        .add_system(systems::fade_in_out)
        .add_system(set_texture_filters_to_nearest)

        //-------------------------------------------------

        //-------------------------------------------------

        .run();
}


pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}