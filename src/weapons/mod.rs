//===============================================================

use bevy::prelude::*;

pub mod weapon_components;
mod weapon_systems;
pub mod weapon_prefabs;

//===============================================================

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_event::<weapon_components::FireWeaponEvent>()

            .add_system(weapon_systems::charge_weapon)
            .add_system(weapon_systems::weapon_state_change)
            .add_system(weapon_systems::fire_weapon)

            .add_system(weapon_systems::projectile_travel)
            .add_system(weapon_systems::projectile_expire)

            .add_system(weapon_systems::projectile_collision)
        ;
    }
}

//===============================================================