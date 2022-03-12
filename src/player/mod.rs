//===============================================================

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;

pub mod player_components;
mod player_systems;

//===============================================================

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<player_components::PlayerBundle>("Player")

            .add_system(player_systems::player_move)
            //.add_system(player_systems::player_sprint)

            .add_system(player_systems::player_jump)

            .add_system(player_systems::player_wall_cling)
            .add_system(player_systems::player_cling_cooldown)
            .add_system(player_systems::player_wall_fling)

            .add_system(player_systems::equip_player_weapon)
            
            .add_system(player_systems::player_attack)
            .add_system(player_systems::player_weapon_aim)


            //.add_system(player_systems::player_super_jump)
        ;
    }
}

//===============================================================