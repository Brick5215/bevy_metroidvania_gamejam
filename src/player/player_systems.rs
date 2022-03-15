//===============================================================

use bevy::prelude::*;
use heron::prelude::*;

use super::player_components::*;

use crate::{
    animation::animation_components::{
         AnimationType, AutoAnimation, ChangeAnimationEvent, FlipAnimation, AnimationFinishedEvent
    }, 
    physics::physics_components::{
         MaxVelocity, 
        MoveDir, CanJump, IsGrounded, IsOnWall, SetGravityScale
    }, 
    weapons::weapon_components::{
        WeaponState, WeaponBundle, WeaponInventory,
        WeaponDirection, WeaponDirections
    }, general::general_components::{HealthChangeEvent, HealthChangeType},
};

//===============================================================

pub fn player_move(
    mut query: Query<(&mut MoveDir, &PlayerWallCling), With<Player>>,
    key_input: Res<Input<KeyCode>>,
) {

    let mut x_dir = 0.;
    if key_input.pressed(PLAYER_LEFT) {
        x_dir -= 1.;
    }
    if key_input.pressed(PLAYER_RIGHT) {
        x_dir += 1.;
    }

    for (mut move_dir, cling) in query.iter_mut() {

        if !cling.clinging {
            move_dir.0 = x_dir;
        }
    }
}

pub fn player_sprint(
    mut player_query: Query<(&mut MaxVelocity, &PlayerSprint, &IsGrounded)>,
    key_input: Res<Input<KeyCode>>,
) {
    let sprinting = key_input.pressed(PLAYER_SPRINT);

    for (mut max_vel, sprint, grounded) in player_query.iter_mut() {

        if sprinting && grounded.grounded && sprint.can_sprint {
            max_vel.x = sprint.sprint_speed;
        }
        else {
            max_vel.x = sprint.normal_speed;
        }
    }
}

pub fn player_jump(
    mut query: Query<&mut CanJump, With<Player>>,
    key_input: Res<Input<KeyCode>>,
) {
    let jump_pressed = key_input.pressed(PLAYER_JUMP);
    let jump_just_pressed = key_input.just_pressed(PLAYER_JUMP);

    for mut can_jump in query.iter_mut() {


        can_jump.jump_pressed = jump_pressed;
        can_jump.jump_repressed = jump_just_pressed;

    }
}

//===============================================================

pub fn player_cling_cooldown(
    mut cling_query: Query<&mut PlayerWallCling>,
    time: Res<Time>,
) {
    for mut cling in cling_query.iter_mut() {
        cling.cling_cooldown.tick(time.delta());
    }
}

pub fn player_wall_cling(
    mut player_query: Query<(Entity, &IsOnWall, &IsGrounded, &mut PlayerWallCling, Option<&mut Velocity>, Option<&mut AutoAnimation>), With<Player> >,
    mut animation_event: EventWriter<ChangeAnimationEvent>,
    key_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    let climb_pressed = key_input.pressed(PLAYER_CLING);

    for (entity, on_wall, grounded, mut player_cling, velocity, auto_animation) in player_query.iter_mut() {

        //This entity is not allowed to cling to walls
        if !player_cling.can_cling {
            continue;
        }

        if on_wall.on_wall && climb_pressed && !grounded.grounded && !player_cling.flinging {
            if !player_cling.clinging {
                if player_cling.cling_cooldown.finished() {
                    //Start clinging here

                    player_cling.clinging = true;
                    //*rigid_body = RigidBody::Sensor;

                    commands.entity(entity).insert(SetGravityScale {
                        scale: 0.,
                        reset_velocity: true,
                    });
                    if let Some(mut velocity) = velocity {
                        velocity.linear = Vec3::ZERO;
                    }

                    if let Some(mut anim) = auto_animation {
                        anim.disabled = true;
                    }
                    animation_event.send(
                        ChangeAnimationEvent{
                            entity,
                            new_animation: AnimationType::Custom("WallGrab".to_string()),
                            restart_animation: false,
                            flipped: FlipAnimation::None,
                        }
                    )
                }
            }
            else {
                if let Some(mut velocity) = velocity {
                    velocity.linear = Vec3::ZERO;
                }
            }
        }
        else {
            if player_cling.clinging {
                //Stop clinging here
                
                //Climb button is released
                if on_wall.on_wall && !grounded.grounded {
                    //If charging a fling, skip over rest of code
                    if player_cling.flinging {
                        continue
                    }
                    //Start the fling here.
                    //Get player input and set animation.
                    else {

                        let mut dir = Vec2::ZERO;
                        if key_input.pressed(KeyCode::Right)    { dir.x += 1.; }
                        if key_input.pressed(KeyCode::Left)     { dir.x -= 1.; }
                        if key_input.pressed(KeyCode::Up)       { dir.y += 1.; }

                        if dir != Vec2::ZERO {
                            dir = dir.normalize();

                            player_cling.fling_dir = dir;


                            animation_event.send(
                                ChangeAnimationEvent {
                                    entity,
                                    new_animation: AnimationType::Custom("WallFling".to_string()),
                                    restart_animation: true,
                                    flipped: FlipAnimation::None,
                                }
                            );

                            player_cling.flinging = true;

                            continue;
                        }
                    }
                }

                //Clinger touched the ground or lost contact with wall
                player_cling.clinging = false;
                player_cling.flinging = false;
                //*rigid_body = RigidBody::Dynamic;
                commands.entity(entity).insert(SetGravityScale {
                    scale: 1.,
                    reset_velocity: false,
                });
                player_cling.cling_cooldown.reset();

                if let Some(mut velocity) = velocity {
                    velocity.linear = Vec3::ZERO;
                }
                if let Some(mut anim) = auto_animation {
                    anim.disabled = false;
                }

                println!("Slipped off");
            }
        }
    }
}

pub fn player_wall_fling(
    mut player_query: Query<(Entity, &IsOnWall, &mut PlayerWallCling, &mut Velocity, Option<&mut AutoAnimation>), With<Player> >,
    mut wall_fling_end_event: EventReader<AnimationFinishedEvent>,
    mut commands: Commands,
) {
    let target_animation = AnimationType::Custom("WallFling".to_string());

    for anim_end in wall_fling_end_event.iter() {

        if anim_end.animation_type != target_animation { continue }

        if let Ok((
            entity,
            on_wall, 
            mut player_cling, 
            mut velocity, 
            auto_anim))
        = player_query.get_mut(anim_end.entity) {

            if on_wall.on_wall {

                player_cling.clinging = false;
                player_cling.flinging = false;
                //*rigid_body = RigidBody::Dynamic;
                commands.entity(entity).insert(SetGravityScale {
                    scale: 1.,
                    reset_velocity: false,
                });
                player_cling.cling_cooldown.reset();

                velocity.linear = player_cling.fling_dir.extend(0.) * player_cling.fling_speed;

                if let Some(mut auto) = auto_anim {
                    auto.disabled = false;
                }
            }
            else {
                println!("Something WENT VERY WRONG IN THE FLING THING!?!?!?!??!!!");
            }
        }
    }
}

//===============================================================

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


pub fn _equip_player_weapon(
    mut player_query: Query<(&mut WeaponInventory, Entity), With<Player>>,
    key_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    
    if key_input.just_pressed(KeyCode::P) {

        let (mut inv, player) = player_query.single_mut();
        let new_weapon = commands.spawn_bundle(WeaponBundle::create_throwing_knife(&assets, &mut texture_atlases, true)).id();

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
        let new_weapon = commands.spawn_bundle(WeaponBundle::create_sword(&assets, &mut texture_atlases, true)).id();

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

pub fn _player_damage(
    mut event: EventWriter<HealthChangeEvent>,
    player: Query<Entity, With<Player>>,
    key_input: Res<Input<KeyCode>>,
) {

    for player in player.iter() {

        if key_input.just_pressed(KeyCode::M) {
            event.send(HealthChangeEvent {
                entity: player,
                change_type: HealthChangeType::Add{value: -10},
            })
        }
    }
}

//===============================================================


pub fn _player_on_which_ground(
    query: Query<&IsGrounded, With<Player>>,
    key_input: Res<Input<KeyCode>>,
) {

    if !key_input.just_pressed(KeyCode::M) {
        return;
    }
    for player in query.iter() {
        println!("Currently on: {:?}. You are grounded = {}", player.walls_below, player.grounded);
    }
}