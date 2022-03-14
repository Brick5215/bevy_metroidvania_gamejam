//===============================================================

use bevy::prelude::*;
use bevy_ecs_ldtk::{LayerInstance, EntityInstance, prelude::{TilesetDefinition, LdtkEntity}};
use heron::Velocity;

use crate::{
    general::{tools::load_texture_atlas, general_components::Health},
    animation::animation_components::{
        SpriteSheetAnimation, AnimationType, Animation, AutoAnimation
    }, physics::physics_components::{MovementBundle, MaxVelocity, Accel, ColliderBundle, FullMoveDir, SetGravityScale}
};

use super::non_player_components::*;

//===============================================================

//===============================================================



#[derive(Bundle, Default, Clone)]
pub struct FoxBundle {
    #[bundle]
    pub non_player: NonPlayerBundle,
    pub health:     Health,
    pub state:      NonPlayerPassiveState,
}

impl LdtkEntity for FoxBundle {
    fn bundle_entity(
        _: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        assets: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {

        const FOX_MAX_HEALTH:   i32 = 50;

        const FOX_MAX_SPEED:    f32 = 200.;
        const FOX_ACCELERATION: f32 = 500.;



        let idle_atlas_handle =  load_texture_atlas(assets, texture_atlases,
            "Textures/NonPlayer/Fox_Idle.png",
            Vec2::new(32., 16.,), 1, 1,
        );

        let sprite_sheet_animation = SpriteSheetAnimation::new(
            AnimationType::Idle,
            Animation::with_fixed_framesteps(
                idle_atlas_handle.clone(),
                0.5,
                1,
                true,
            ),
            false,
        );

        FoxBundle {
            non_player: NonPlayerBundle {
                non_player: NonPlayer(layer_instance.level_id),
                sprite: SpriteSheetBundle {
                    texture_atlas: idle_atlas_handle,
                    ..Default::default()
                },
                animation: sprite_sheet_animation,
                auto_anim: AutoAnimation::default(),
                physics: ColliderBundle::non_player(32., 16.),
                movement: MovementBundle {
                    max_velocity: MaxVelocity {
                        x: FOX_MAX_SPEED,
                        y: 600.,
                    },
                    acceleration: Accel {
                        accel:          FOX_ACCELERATION,
                        deaccel:        FOX_ACCELERATION,
                        air_deaccel:    None,
                    },
                    ..Default::default()
                },
            },
            health: Health::new(FOX_MAX_HEALTH, 0.1),
            state: NonPlayerPassiveState::default(),
        }
    }
}

//===============================================================

#[derive(Bundle, Default, Clone)]
pub struct BatBundle {
    #[bundle]
    pub non_player:     NonPlayerFlyingBundle,
    pub health:         Health,
    pub state:          NonPlayerAggressiveState,
    pub player_attack:  NonPlayerAttackPlayer,
    pub damage:         NonPlayerDamage,
}
impl LdtkEntity for BatBundle {
    fn bundle_entity(
        _: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        assets: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {

        const BAT_MAX_HEALTH:   i32 = 25;

        const BAT_MAX_SPEED:    f32 = 70.;
        const BAT_ACCELERATION: f32 = 500.;

        const BAT_ATTACK_RANGE: f32 = 165.;//260.;
        const BAT_LOST_RANGE:   f32 = 290.;//330.;

        let idle_atlas_handle =  load_texture_atlas(assets, texture_atlases,
            "Textures/NonPlayer/Bat_Idle.png",
            Vec2::new(32., 32.,), 1, 1,
        );

        let walk_atlas_handle =  load_texture_atlas(assets, texture_atlases,
            "Textures/NonPlayer/Bat_Idle.png",
            Vec2::new(32., 32.,), 1, 1,
        );

        let mut sprite_sheet_animation = SpriteSheetAnimation::new(
            AnimationType::Idle,
            Animation::with_fixed_framesteps(
                idle_atlas_handle.clone(),
                0.5,
                1,
                true,
            ),
            false,
        );
        sprite_sheet_animation.add_animation(
            AnimationType::Walk, 
            Animation::with_fixed_framesteps(
                walk_atlas_handle.clone(),
                0.5,
                1,
                true,
            )
        );
        
        BatBundle {

            non_player: NonPlayerFlyingBundle {
                non_player: NonPlayer(layer_instance.level_id),
                sprite: SpriteSheetBundle {
                    texture_atlas: idle_atlas_handle,
                    ..Default::default()
                },
                animation: sprite_sheet_animation,
                auto_anim: AutoAnimation::default(),
                physics: ColliderBundle::non_player(32., 32.),

                move_dir: FullMoveDir::default(),
                max_velocity: MaxVelocity {
                    x: BAT_MAX_SPEED,
                    y: BAT_MAX_SPEED,
                },
                acceleration: Accel {
                    accel:          BAT_ACCELERATION,
                    deaccel:        BAT_ACCELERATION,
                    air_deaccel:    None,
                },
                velocity: Velocity::default(),
                gravity: SetGravityScale {
                    scale: 0.,
                    reset_velocity: false,
                },
            },
            health:         Health::new(BAT_MAX_HEALTH, 0.1),
            state:          NonPlayerAggressiveState::default(),
            player_attack:  NonPlayerAttackPlayer {
                attack_range: BAT_ATTACK_RANGE,
                lost_range: BAT_LOST_RANGE,
            },
            damage:         NonPlayerDamage(16),

        }
    }
}

//===============================================================