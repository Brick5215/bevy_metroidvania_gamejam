
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::animation_components::{
        SpriteSheetAnimation, AnimationType, Animation, AutoAnimation
    }, 
    physics::physics_components::{
        MovementBundle, ColliderBundle, MaxVelocity, 
        MoveDir, Accel, CanJump, IsOnWall
    }, 
    weapons::weapon_components::WeaponInventoryBundle, general::tools::load_texture_atlas,
};

//===============================================================

pub const PLAYER_WIDTH:             f32 = 14.;
pub const PLAYER_HEIGHT:            f32 = 32.;

//===============================================================

pub const PLAYER_LEFT:              KeyCode = KeyCode::Left;
pub const PLAYER_RIGHT:             KeyCode = KeyCode::Right;

pub const PLAYER_JUMP:              KeyCode = KeyCode::Space;
pub const PLAYER_CLING:             KeyCode = KeyCode::X;

pub const PLAYER_PRIMARY_ATTACK:    KeyCode = KeyCode::C;
pub const PLAYER_SECONDARY_ATTACK:  KeyCode = KeyCode::Z;

//===============================================================

pub const PLAYER_MAX_SPEED:         f32 = 120.;
//pub const PLAYER_MAX_SPRINT_SPEED:  f32 = 180.;

pub const PLAYER_ACCELERATION:      f32 = 400.;
pub const PLAYER_DEACCELERATION:    f32 = 500.;
pub const PLAYER_AIR_DEACCELERATION:f32 = 430.;

pub const PLAYER_JUMP_FORCE:        f32 = 300.;
pub const PLAYER_FLING_SPEED:       f32 = 300.;
pub const PLAYER_FLING_COOLDOWN:    f32 = 0.5;

//===============================================================

#[derive(Component, Default, Clone)]
pub struct PlayerSprint {
    pub sprint_speed: f32,
    pub normal_speed: f32,
}


#[derive(Component, Default, Clone)]
pub struct PlayerWallCling {
    pub can_cling: bool,
    pub clinging: bool,
    pub cling_cooldown: Timer,
    pub fling_speed: f32,
    pub fling_dir: Vec2,
    pub flinging: bool,
}
impl PlayerWallCling {
    pub fn new(fling_speed: f32, cooldown: f32) -> Self {
        PlayerWallCling {
            can_cling: true,
            clinging: false,
            fling_speed,
            cling_cooldown: Timer::from_seconds(cooldown, false),
            fling_dir: Vec2::ZERO,
            flinging: false,
        }
    }
}

//===============================================================

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Bundle, Clone, Default)]
pub struct PlayerWallBundle {
    player_cling: PlayerWallCling,
    on_wall: IsOnWall,
}


#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    player:         Player,
    pub worldly:    Worldly,
    #[bundle]
    sprite:         SpriteSheetBundle,
    animation:      SpriteSheetAnimation,
    auto_anim:      AutoAnimation,
    #[bundle]
    physics:        ColliderBundle,
    #[bundle]
    movement:       MovementBundle,
    #[bundle]
    weapons:        WeaponInventoryBundle,
    //sprint:         PlayerSprint,

    #[bundle]
    wall_bundle:    PlayerWallBundle,
}

//Spawn the player
impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        assets: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {

        //Idle Animation
        let idle_atlas_handle = load_texture_atlas(assets, texture_atlases,
            "Textures/Alchemist/Mini_Alchemist_Idle.png",
            Vec2::new(32., 32.), 4, 1,
        );
        //Walk Animation
        let walk_atlas_handle = load_texture_atlas(assets, texture_atlases,
            "Textures/Alchemist/Mini_Alchemist_Walk.png",
            Vec2::new(32., 32.), 12, 1,
        );

        //Jump Animation
        let jump_atlas_handle = load_texture_atlas(assets, texture_atlases,
            "Textures/Alchemist/Mini_Alchemist_Jump.png",
            Vec2::new(32., 32.), 1, 1,
        );
        //Begin Fall Animation
        let begin_fall_atlas_handle = load_texture_atlas(assets, texture_atlases,
            "Textures/Alchemist/Mini_Alchemist_Begin_Fall.png",
            Vec2::new(32., 32.), 1, 1,
        );
        //Fall Animation
        let fall_atlas_handle = load_texture_atlas(assets, texture_atlases,
            "Textures/Alchemist/Mini_Alchemist_Fall.png",
            Vec2::new(32., 32.), 1, 1,
        );

        //Wall Hang Animation
        let wall_grab_atlas_handle = load_texture_atlas(assets, texture_atlases,
            "Textures/Alchemist/Mini_Alchemist_Wallgrab.png",
            Vec2::new(32., 40.), 9, 1,
        );
        //Wall Fling Animation
        let wall_fling_atlas_handle = load_texture_atlas(assets, texture_atlases, 
            "Textures/Alchemist/Mini_Alchemist_Wallfling.png",
            Vec2::new(32., 40.), 5, 1,
        );

        let mut sprite_sheet_animation = SpriteSheetAnimation::new(
            AnimationType::Idle,
            Animation::with_custom_framesteps(
                idle_atlas_handle.clone(),
                vec!(0.6, 0.5, 0.8, 0.2),
                4,
                true,
            ),
            false,
        );
        sprite_sheet_animation.add_animation(
            AnimationType::Walk,
            Animation::with_fixed_framesteps(
                walk_atlas_handle, 
                0.1, 
                12, 
                true, 
            )
        );


        sprite_sheet_animation.add_animation(
            AnimationType::Jump,
            Animation::with_fixed_framesteps(
                jump_atlas_handle, 
                1., 
                1, 
                false, 
            )
        );
        sprite_sheet_animation.add_animation(
            AnimationType::BeginFall,
            Animation::with_fixed_framesteps(
                begin_fall_atlas_handle,
                1.,
                1,
                false,
            )
        );
        sprite_sheet_animation.add_animation(
            AnimationType::Fall,
            Animation::with_fixed_framesteps(
                fall_atlas_handle,
                1.,
                1,
                false,
            )
        );



        sprite_sheet_animation.add_animation(
            AnimationType::Custom("WallGrab".to_string()),
            Animation::with_fixed_framesteps(
                wall_grab_atlas_handle,
                0.25,
                9,
                true,
            )
        );
        sprite_sheet_animation.add_animation(
            AnimationType::Custom("WallFling".to_string()),
            Animation::with_custom_framesteps(
                wall_fling_atlas_handle,
                vec!(0.09, 0.09, 0.20, 0.12, 0.09),
                5,
                true,
            )
        );

        Self {
            player: Player,
            worldly: Worldly::from_entity_info(entity_instance, layer_instance),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    //custom_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
                texture_atlas: idle_atlas_handle,
                transform: Transform::from_xyz(0., entity_instance.height as f32 * 2., 2.),
                ..Default::default()
            },
            animation: sprite_sheet_animation,
            physics: ColliderBundle::player(PLAYER_WIDTH, PLAYER_HEIGHT),
            movement: MovementBundle {
                move_dir: MoveDir(0.),
                max_velocity: MaxVelocity {
                    x: PLAYER_MAX_SPEED,
                    y: 600.,
                },
                acceleration: Accel {
                    accel:          PLAYER_ACCELERATION,
                    deaccel:        PLAYER_DEACCELERATION,
                    air_deaccel:    Some(PLAYER_AIR_DEACCELERATION),
                },
                jump: CanJump {
                    jump_force:     PLAYER_JUMP_FORCE,
                    jumps_left:     1,
                    total_jumps:    1,
                },
                ..Default::default()
            },
            //sprint: PlayerSprint{
                //sprint_speed:       PLAYER_MAX_SPRINT_SPEED,
                //normal_speed:       PLAYER_MAX_SPEED,
            //},
            wall_bundle: PlayerWallBundle {
                player_cling: PlayerWallCling::new(PLAYER_FLING_SPEED, PLAYER_FLING_COOLDOWN),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

//===============================================================