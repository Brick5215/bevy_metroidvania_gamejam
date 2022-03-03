//===============================================================

use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::{SpriteSheetAnimation, AnimationType, Animation}, 
    physics::{MovementBundle, ColliderBundle, MaxVelocity, MoveDir, Accel, CanJump, IsGrounded, JumpEvent}
};

//===============================================================

const PLAYER_MAX_SPEED: f32 = 120.;
const PLAYER_ACCELERATION: f32 = 400.;
const PLAYER_DEACCELERATION: f32 = 400.;
const PLAYER_JUMP_FORCE: f32 = 300.;

//===============================================================

#[derive(Component, Default, Clone, Debug)]
pub struct Player;
#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    pub worldly: Worldly,
    #[bundle]
    sprite: SpriteSheetBundle,
    animation: SpriteSheetAnimation,
    #[bundle]
    physics: ColliderBundle,
    #[bundle]
    movement: MovementBundle,
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

        let idle_texture_handle = assets.load("Textures/Alchemist/Mini_Alchemist_Idle.png");
        let idle_texture_atlas = TextureAtlas::from_grid(idle_texture_handle, Vec2::new(32., 32.), 4, 1);
        let idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);

        let walk_texture_handle = assets.load("Textures/Alchemist/Mini_Alchemist_Walk.png");
        let walk_texture_atlas = TextureAtlas::from_grid(walk_texture_handle, Vec2::new(32., 32.), 12, 1);
        let walk_texture_atlas_handle = texture_atlases.add(walk_texture_atlas);


        let mut sprite_sheet_animation = SpriteSheetAnimation::new(
            AnimationType::Idle,
            Animation::with_framesteps(
                idle_texture_atlas_handle.clone(),
                vec!(0.6, 0.5, 0.8, 0.2),
                4,
                true,
            ),
            false,
        );
        sprite_sheet_animation.add_animation(
            AnimationType::Walk,
            Animation::with_fixed_framestep(
                walk_texture_atlas_handle, 
                0.1, 
                12, 
                true, 
            )
        );


        let width = entity_instance.width as f32;
        let height = entity_instance.height as f32;

        Self {
            player: Player,
            worldly: Worldly::from_entity_info(entity_instance, layer_instance),
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
                texture_atlas: idle_texture_atlas_handle,
                transform: Transform::from_xyz(0., entity_instance.height as f32 * 2., 2.),
                ..Default::default()
            },
            animation: sprite_sheet_animation,
            physics: ColliderBundle::player(width, height),
            movement: MovementBundle {
                move_dir: MoveDir(0.),
                max_velocity: MaxVelocity {
                    x: PLAYER_MAX_SPEED,
                    y: 600.,
                },
                acceleration: Accel {
                    accel: PLAYER_ACCELERATION,
                    deaccel: PLAYER_DEACCELERATION,
                },
                velocity: Velocity::default(),
                jump: CanJump {
                    //can_jump: true,
                    jump_force: PLAYER_JUMP_FORCE,
                    //jump_start: false,
                    jumps_left: 1,
                    total_jumps: 1,
                },
                grounded: IsGrounded {
                    grounded: false,
                    time_since_grounded: 0.,
                    entities_below: Vec::new(),
                }
            }
        }
    }
}

//===============================================================

pub fn player_move(
    mut query: Query<&mut MoveDir, With<Player>>,
    key_input: Res<Input<KeyCode>>,
) {

    let mut x_dir = 0.;
    if key_input.pressed(KeyCode::A) {
        x_dir -= 1.;
    }
    if key_input.pressed(KeyCode::D) {
        x_dir += 1.;
    }

    for mut move_dir in query.iter_mut() {
        move_dir.0 = x_dir;
    }
}

pub fn player_jump(
    mut query: Query<(Entity, &IsGrounded), (With<Player>, With<CanJump>)>,
    key_input: Res<Input<KeyCode>>,
    mut jump_event: EventWriter<JumpEvent>,
) {

    let jump_pressed = key_input.just_pressed(KeyCode::Space);

    for (entity, grounded) in query.iter_mut() {
        if jump_pressed && grounded.time_since_grounded < 0.2 {
            jump_event.send(JumpEvent(entity));
        }
    }
}

//===============================================================

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")

            .add_system(player_move)
            .add_system(player_jump)
        ;
    }
}

//===============================================================