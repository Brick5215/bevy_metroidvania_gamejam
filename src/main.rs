//===============================================================

use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;

//===============================================================

mod arena;
mod physics;
mod animation;
mod player;

mod general;


mod weapons;

//===============================================================

fn main() {
    
    App::new()
        //-------------------------------------------------

        .add_plugins(DefaultPlugins)

        //-------------------------------------------------
        //Add External Plugins
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(LdtkPlugin)
        .add_plugin(ShapePlugin)

        //Add Own plugins
        .add_plugin(animation::AnimationPlugin)
        .add_plugin(physics::CustomPhysicsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(weapons::WeaponPlugin)
        .add_plugin(arena::ArenaPlugin)

        //-------------------------------------------------

        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Gravity::from(Vec2::new(0., -500.,)))
        
        //===============================================================

        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            load_level_neighbors: true,
            use_level_world_translations: true,
            //set_clear_color: true,
            ..Default::default()
        })

        //-------------------------------------------------

        .add_startup_system(general::systems::setup)
        .add_system(set_texture_filters_to_nearest)

        .add_system(general::systems::pause_physics_while_load)

        .add_system(general::systems::fade_in_out)

        //-------------------------------------------------

        .add_event::<general::general_components::HealthChangeEvent>()
        .add_system(general::systems::change_health)

        //-------------------------------------------------

        .run();
}

//===============================================================

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

//===============================================================
