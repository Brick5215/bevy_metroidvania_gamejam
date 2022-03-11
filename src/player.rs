//===============================================================

use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::{SpriteSheetAnimation, AnimationType, Animation}, 
    physics::physics_components::{
        MovementBundle, ColliderBundle, MaxVelocity, 
        MoveDir, Accel, CanJump, IsGrounded, JumpEvent
    }, 
    weapons::weapon_components::{
        WeaponState, WeaponBundle, WeaponInventory,
        WeaponInventoryBundle, WeaponDirection, WeaponDirections
    }
};

//===============================================================

const PLAYER_WIDTH:             f32 = 18.;
const PLAYER_HEIGHT:            f32 = 32.;

//===============================================================

const PLAYER_JUMP:              KeyCode = KeyCode::Space;
const PLAYER_LEFT:              KeyCode = KeyCode::Left;
const PLAYER_RIGHT:             KeyCode = KeyCode::Right;
const PLAYER_PRIMARY_ATTACK:    KeyCode = KeyCode::C;
const PLAYER_SECONDARY_ATTACK:  KeyCode = KeyCode::X;

//===============================================================

const PLAYER_MAX_SPEED:         f32 = 120.;
const PLAYER_MAX_SPRINT_SPEED:  f32 = 180.;
const PLAYER_ACCELERATION:      f32 = 400.;
const PLAYER_DEACCELERATION:    f32 = 400.;
const PLAYER_JUMP_FORCE:        f32 = 300.;

//===============================================================

#[derive(Component, Default, Clone)]
pub struct PlayerSprint {
    sprint_speed: f32,
    normal_speed: f32,
}

//===============================================================

#[derive(Component, Default, Clone, Debug)]
pub struct Player;
#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    player:         Player,
    pub worldly:    Worldly,
    #[bundle]
    sprite:         SpriteSheetBundle,
    animation:      SpriteSheetAnimation,
    #[bundle]
    physics:        ColliderBundle,
    #[bundle]
    movement:       MovementBundle,
    #[bundle]
    weapons:        WeaponInventoryBundle,
    sprint:         PlayerSprint,
    
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
            physics: ColliderBundle::player(PLAYER_WIDTH, PLAYER_HEIGHT),
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
            },
            weapons: WeaponInventoryBundle::default(),
            sprint: PlayerSprint{
                sprint_speed: PLAYER_MAX_SPRINT_SPEED,
                normal_speed: PLAYER_MAX_SPEED,
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
    if key_input.pressed(PLAYER_LEFT) {
        x_dir -= 1.;
    }
    if key_input.pressed(PLAYER_RIGHT) {
        x_dir += 1.;
    }

    for mut move_dir in query.iter_mut() {
        move_dir.0 = x_dir;
    }
}

pub fn player_sprint(
    mut player_query: Query<(&mut MaxVelocity, &PlayerSprint)>,
    key_input: Res<Input<KeyCode>>,
) {
    let sprinting = key_input.pressed(KeyCode::LShift);

    for (mut max_vel, sprint) in player_query.iter_mut() {

        if sprinting {
            max_vel.x = sprint.sprint_speed;
        }
        else {
            max_vel.x = sprint.normal_speed;
        }

    }
}

pub fn player_jump(
    mut query: Query<(Entity, &IsGrounded), (With<Player>, With<CanJump>)>,
    key_input: Res<Input<KeyCode>>,
    mut jump_event: EventWriter<JumpEvent>,
) {

    let jump_pressed = key_input.just_pressed(PLAYER_JUMP);

    for (entity, grounded) in query.iter_mut() {
        if jump_pressed && grounded.time_since_grounded < 0.2 {
            jump_event.send(JumpEvent(entity));
        }
    }
}

pub fn player_weapon_aim(
    player_query: Query<&WeaponInventory, With<Player>>,
    mut weapon_query: Query<&mut WeaponDirection>,
    key_input: Res<Input<KeyCode>>,
) { 

    let mut dir = Vec2::ZERO;
    if key_input.pressed(KeyCode::Right)    { dir.x += 1.; }
    if key_input.pressed(KeyCode::Left)     { dir.x -= 1.; }

    if key_input.pressed(KeyCode::Up)       { dir.y += 1.; }
    if key_input.pressed(KeyCode::Down)     { dir.y -= 1.; }

    if dir == Vec2::ZERO { return }


    if let Ok(weapons) = player_query.get_single() {

        if let Some(slot1) = weapons.get_slot1() {
            if let Ok(mut direction) = weapon_query.get_mut(*slot1) {

                if      dir.x > 0. { direction.right_facing = true; }
                else if dir.x < 0. { direction.right_facing = false; }
                direction.direction = WeaponDirections::from_vec2(dir);
            }
        }

        if let Some(slot2) = weapons.get_slot2() {
            if let Ok(mut direction) = weapon_query.get_mut(*slot2) {

                if      dir.x > 0. { direction.right_facing = true; }
                else if dir.x < 0. { direction.right_facing = false; }
                direction.direction = WeaponDirections::from_vec2(dir);
            }
        }
    }
}

pub fn player_attack(
    player_query: Query<&WeaponInventory, With<Player>>,
    mut weapon_query: Query<&mut WeaponState>,
    key_input: Res<Input<KeyCode>>,
) {

    if let Ok(weapons) = player_query.get_single() {

        let mut set_weapon_charge = |pressed, released, slot: &Option<Entity>| {
            if pressed || released {
                if let Some(slot1) = slot {
                    if let Ok(mut weapon) = weapon_query.get_mut(*slot1) {
                        if pressed {
                            weapon.charging = true;
                        }
                        if released {
                            weapon.charging = false;
                        }
                    }
                }
            }
        };

        let primary_pressed = key_input.just_pressed(PLAYER_PRIMARY_ATTACK);
        let primary_released = key_input.just_released(PLAYER_PRIMARY_ATTACK);
        let primary_weapon = weapons.get_slot1();
        set_weapon_charge(primary_pressed, primary_released, primary_weapon);


        let secondary_pressed   = key_input.just_pressed(PLAYER_SECONDARY_ATTACK);
        let secondary_released  = key_input.just_released(PLAYER_SECONDARY_ATTACK);
        let secondary_weapon = weapons.get_slot2();
        set_weapon_charge(secondary_pressed, secondary_released, secondary_weapon);

    }
}


fn equip_player_weapon(
    mut player_query: Query<(&mut WeaponInventory, Entity), With<Player>>,
    key_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    if key_input.just_pressed(KeyCode::P) {

        let (mut inv, player) = player_query.single_mut();
        let new_weapon = commands.spawn_bundle(WeaponBundle::create_knife()).id();

        if inv.add_slot1_weapon(new_weapon) {
            //Weapon added successfully
            commands.entity(player).add_child(new_weapon);
            println!("Weapon 1 spawned and equipped successfully");
        }
        else {
            //Weapon was not added
            commands.entity(new_weapon).despawn();  
            println!("Weapon 1 was not added and is being despawned");
        }
    }
    else if key_input.just_pressed(KeyCode::O) {

        let (mut inv, player) = player_query.single_mut();
        let new_weapon = commands.spawn_bundle(WeaponBundle::create_throwing_knife()).id();

        if inv.add_slot2_weapon(new_weapon) {
            //Weapon added successfully
            commands.entity(player).add_child(new_weapon);
            println!("Weapon 2 spawned and equipped successfully");
        }
        else {
            //Weapon was not added
            commands.entity(new_weapon).despawn();  
            println!("Weapon 2 was not added and is being despawned");
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
            .add_system(player_sprint)
            .add_system(player_jump)

            .add_system(equip_player_weapon)
            
            .add_system(player_attack)
            .add_system(player_weapon_aim)
        ;
    }
}

//===============================================================