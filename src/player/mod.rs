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

            .add_system(player_systems::player_move.label("PlayerMoveInput"))
            .add_system(player_systems::player_sprint)

            .add_system(player_systems::player_jump)

            .add_system(player_systems::player_wall_cling.before("PlayerMoveInput"))
            .add_system(player_systems::player_cling_cooldown)
            .add_system(player_systems::player_wall_fling)
            
            .add_system(player_systems::player_attack)
            .add_system(player_systems::player_weapon_aim)

            //Debug systems
            //.add_system(player_systems::_equip_player_weapon)
            //.add_system(player_systems::_player_damage)
            //.add_system(player_systems::_player_on_which_ground)


        ;
    }
}

//===============================================================