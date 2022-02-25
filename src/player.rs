//===============================================================

use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::general_components::{
    ColliderBundle, MovementBundle, MaxVelocity, Accel, 
    MoveDir, CanJump
};

//===============================================================


#[derive(Component, Default, Clone, Debug)]
pub struct Player;
#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    player: Player,
    #[bundle]
    sprite: SpriteSheetBundle,
    #[bundle]
    physics: ColliderBundle,
    #[bundle]
    movement: MovementBundle,
    pub worldly: Worldly,
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

        let width = entity_instance.width as f32;
        let height = entity_instance.height as f32;

        Self {
            player: Player,
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(width, height)),
                    ..Default::default()
                },
                texture_atlas: idle_texture_atlas_handle,
                transform: Transform::from_xyz(0., entity_instance.height as f32 * 2., 2.),
                ..Default::default()
            },
            physics: ColliderBundle::player(width, height),
            movement: MovementBundle {
                move_dir: MoveDir(0.),
                max_velocity: MaxVelocity {
                    x: 200.,
                    y: 800.,
                },
                acceleration: Accel {
                    accel: 400.,
                    deaccel: 400.,
                },
                velocity: Velocity::default(),
                jump: CanJump {
                    can_jump: true,
                    jump_force: 200.,
                    jump_start: false,
                }
            },
            worldly: Worldly::from_entity_info(entity_instance, layer_instance),
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
    mut query: Query<&mut CanJump, With<Player>>,
    key_input: Res<Input<KeyCode>>,
) {

    let jump_pressed = key_input.just_pressed(KeyCode::Space);

    for mut jump in query.iter_mut() {
        if jump_pressed && jump.can_jump {
            jump.jump_start = true;
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