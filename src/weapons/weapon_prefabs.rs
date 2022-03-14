//================================================================================

use bevy::prelude::*;
use heron::RigidBody;

use crate::{animation::animation_components::{SimpleAnimationBundle, AnimationType}, general::tools::load_texture_atlas, physics::physics_components::SetGravityScale};

use super::weapon_components::*;

//================================================================================

impl WeaponBundle {
    pub fn create_sword(
        assets: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
        is_friendly: bool
    ) -> WeaponBundle {


        let sword_atlas_handle = load_texture_atlas(assets, texture_atlases, 
            "Textures/Weapons/Sword.png",
            Vec2::new(32., 32.), 5, 1,
        );

        let animation_bundle = SimpleAnimationBundle::new(
            AnimationType::Custom("Attack".to_string()),
            vec!(0.15, 0.06, 0.09, 0.06, 0.1),
            false,
            sword_atlas_handle,
        );

        WeaponBundle {
            charge: WeaponCharge::new(1.5),
            attack: WeaponAttack {
                to_spawn: ProjectileTemplate::create_melee(
                    10,
                    0.46,
                    Vec2::new(25., 5.,),
                    20.,
                    40.,
                    animation_bundle,
                ),
                child_of_parent: true,
                is_friendly,
                gravity_scale: Some(SetGravityScale::reset_velocity(0.)),
            },
            state: WeaponState::default(),
            direction: WeaponDirection::default(),
            preview: WeaponPreviewBundle::default(),
        }
    }

    pub fn create_throwing_knife(
        assets: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
        is_friendly: bool
    ) -> WeaponBundle {

        let knife_atlas_handle = load_texture_atlas(assets, texture_atlases, 
            "Textures/Weapons/Knife.png",
            Vec2::new(16., 16.), 1, 1,
        );

        let animation_bundle = SimpleAnimationBundle::new(
            AnimationType::Custom("Attack".to_string()),
            vec!(0.5),
            false,
            knife_atlas_handle,
        );

        WeaponBundle {
            charge: WeaponCharge::new(3.),
            attack: WeaponAttack {
                to_spawn: ProjectileTemplate::create_range(
                    10,
                    2.,
                    Vec2::new(20., 0.,),
                    16.,
                    4.,
                    Vec2::new(370., 0.),
                    80.,
                    RigidBody::Dynamic,
                    animation_bundle,
                ),
                child_of_parent: false,
                is_friendly,
                gravity_scale: Some(SetGravityScale {
                    scale: 0.8,
                    ..Default::default()
                }),
            },
            state: WeaponState::default(),
            direction: WeaponDirection::default(),
            preview: WeaponPreviewBundle::default(),
        }
    }
}

//================================================================================