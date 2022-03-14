//===============================================================

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;

pub mod non_player_components;
mod non_player_systems;
pub mod non_player_prefabs;

//===============================================================

pub struct NonPlayerPlugin;
impl Plugin for NonPlayerPlugin {
    fn build(&self, app: &mut App) {

        app

            //.register_ldtk_entity::<non_player_prefabs::FoxBundle>("Fox")
            .register_ldtk_entity::<non_player_prefabs::BatBundle>("Bat")

            .add_system(non_player_systems::enemy_target_player)
            .add_system(non_player_systems::enemy_attack_target)
            .add_system(non_player_systems::enemy_idle)

            .add_system(non_player_systems::enemy_damage)
        ;

    }
}

//===============================================================