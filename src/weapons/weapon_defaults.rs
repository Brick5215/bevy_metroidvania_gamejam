//================================================================================

use bevy::prelude::*;

use super::weapon_components::*;

//================================================================================

impl WeaponBundle {
    pub fn create_knife() -> WeaponBundle {
        WeaponBundle {
            charge: WeaponCharge::new(1.5),
            attack: WeaponAttack {
                to_spawn: Projectile::create_melee(
                    10.,
                    1.,
                    Vec2::new(20., 0.,),
                    20.,
                    40.,
                ),
                child_of_parent: true,
            },
            ..Default::default()
        }
    }

    pub fn create_throwing_knife() -> WeaponBundle {
        WeaponBundle {
            charge: WeaponCharge::new(3.),
            attack: WeaponAttack {
                to_spawn: Projectile::create_range(
                    10.,
                    2.,
                    Vec2::new(20., 0.,),
                    10.,
                    Vec2::new(300., 0.)
                ),
                child_of_parent: false,
            },
            ..Default::default()
        }
    }
}

//================================================================================