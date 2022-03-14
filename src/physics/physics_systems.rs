//===============================================================

use bevy::prelude::*;
use heron::{
    prelude::*, 
    rapier_plugin::{
        RigidBodyHandle, 
        rapier2d::prelude::RigidBodySet, 
        convert::IntoRapier
        },
    };

use crate::general::tools::{clamp_shift, lerp};

use super::physics_components::*;

//===============================================================

pub fn cap_velocity(
    mut query: Query<(&mut Velocity, &MaxVelocity, Option<&Accel>, Option<&IsGrounded>)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();

    for (mut velocity, cap, acceleration, is_grounded) in query.iter_mut() {


        if let Some(accel) = acceleration {

            let mut deacceleration = accel.deaccel;

            if let Some(grounded) = is_grounded {
                if !grounded.grounded {
                    if let Some(air_deaccel) = accel.air_deaccel {
                        deacceleration = air_deaccel;
                    }
                }
            }

            deacceleration *= delta;

            velocity.linear.x = clamp_shift(
                velocity.linear.x,
                -cap.x,
                cap.x,
                deacceleration
            );
            velocity.linear.y = clamp_shift(
                velocity.linear.y,
                -cap.y,
                cap.y,
                deacceleration
            );
            continue;
        }

        velocity.linear.x = velocity.linear.x.clamp(-cap.x, cap.x);
        velocity.linear.y = velocity.linear.y.clamp(-cap.y, cap.y);
    }
}

pub fn apply_movespeed (
    mut query: Query<(&MoveDir, &Accel, &mut Velocity)>,
    time: Res<Time>,
) {
    for (move_dir, accel, mut velocity) in query.iter_mut() {
        if move_dir.0 != 0. {
            velocity.linear.x += move_dir.0 * accel.accel * time.delta().as_secs_f32();
        }
        else {
            if velocity.linear.x > 0. {
                velocity.linear.x = (velocity.linear.x - accel.deaccel * time.delta().as_secs_f32()).max(0.);
            }
            else if velocity.linear.x < 0. {
                velocity.linear.x = (velocity.linear.x + accel.deaccel * time.delta().as_secs_f32()).min(0.);
            }       
        }
    }
}

pub fn apply_full_movespeed (
    mut query: Query<(&FullMoveDir, &Accel, &mut Velocity)>,
    time: Res<Time>,
) {

    for (move_dir, accel, mut velocity) in query.iter_mut() {
        if move_dir.0 != Vec2::ZERO {
            velocity.linear += (move_dir.0 * accel.accel * time.delta().as_secs_f32()).extend(0.);
        }
        else {
            if velocity.linear.x > 0. {
                velocity.linear.x = (velocity.linear.x - accel.deaccel * time.delta().as_secs_f32()).max(0.);
            }
            else if velocity.linear.x < 0. {
                velocity.linear.x = (velocity.linear.x + accel.deaccel * time.delta().as_secs_f32()).min(0.);
            }

            if velocity.linear.y > 0. {
                velocity.linear.y = (velocity.linear.y - accel.deaccel * time.delta().as_secs_f32()).max(0.);
            }
            else if velocity.linear.y < 0. {
                velocity.linear.y = (velocity.linear.y + accel.deaccel * time.delta().as_secs_f32()).min(0.);
            }  
        }

    }

}

//=================================================================================

pub fn apply_jump (
    mut query: Query<(&mut CanJump, &mut Velocity, &IsGrounded)>,
    time: Res<Time>
) {

    for (mut can_jump, mut velocity, grounded) in query.iter_mut() {
        
        //Already jumping and jump is still pressed and still time left on jump
        if can_jump.jumping && can_jump.jump_pressed && !can_jump.jump_timer.finished() {
            velocity.linear.y += can_jump.jump_force * time.delta().as_secs_f32();
            can_jump.jump_force = lerp(
                can_jump.max_jump_force, 
                can_jump.max_jump_force * 0.2, 
                can_jump.jump_timer.percent());
        }
        //Entity stops jumping or jump expires
        else if can_jump.jumping && (!can_jump.jump_pressed|| can_jump.jump_timer.finished()) {
            can_jump.jumping = false;
            can_jump.jumps_left -= 1;
            can_jump.jump_force = can_jump.max_jump_force;
        }

        if can_jump.jumping {
            can_jump.jump_timer.tick(time.delta());
        }

        //Entity is not jumping, has jumps left, is grounded and pressed the jump button
        if     !can_jump.jumping 
            && can_jump.jumps_left > 0 
            && can_jump.jump_repressed 
            && grounded.time_since_grounded < 0.2
        {
            can_jump.jumping = true;
            can_jump.jump_timer.reset();

            //Stop falling and add initial jump force
            velocity.linear.y = can_jump.initial_jump_force;
        }
    }
}

pub fn reset_jump(
    mut query: Query<&mut CanJump>,
    mut grounded_event: EventReader<GroundedEvent>,
) {
    for event in grounded_event.iter() {
        if let Ok( mut jumps) = query.get_mut(event.0) {

            jumps.jumps_left = jumps.total_jumps;
        }
    }
}

//=================================================================================

pub fn check_on_ground (
    mut collision_event: EventReader<CollisionEvent>,
    mut grounded_query: Query<&mut IsGrounded>,
    mut grounded_event: EventWriter<GroundedEvent>,
    time: Res<Time>,
) {
    let down_dir = Vec2::new(0.,-1.);

    for event in collision_event.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {

                let d1_tile = if d1.collision_layers().contains_group(CollisionLayer::Tile) { 
                            true
                } else {    false };

                let d2_tile = if d2.collision_layers().contains_group(CollisionLayer::Tile) {
                            true
                } else {    false };

                //Either both or neither collisions were player
                if (d1_tile && d2_tile) || (!d1_tile && !d2_tile) { 
                    continue;
                }

                let (tile, to_check) = if d1_tile {
                            (d1, d2)
                } else {    (d2, d1) };

                //if d1_tile {println!("d1 is the tile. CheckNormals = {:?}, TileNormals = {:?}", to_check.normals(), tile.normals()); }
                //else {      println!("d2 is the tile. CheckNormals = {:?}, TileNormals = {:?}", to_check.normals(), tile.normals()); }


                if tile.normals().len() == 0 {
                    //println!("There are no normals");
                    continue;
                }

                //println!("d1 normals: {:#?}, d2 normals: {:#?}", d1.normals(), d2.normals());
                if to_check.normals()[0] == down_dir {
                    if let Ok( mut grounded) = grounded_query.get_mut(to_check.rigid_body_entity()) {
                        grounded.grounded = true;
                        grounded.time_since_grounded = 0.;
                        grounded.walls_below.push(tile.rigid_body_entity());
                        if grounded.walls_below.len() == 1 {
                            grounded_event.send(GroundedEvent(to_check.rigid_body_entity()));
                        }
                    }
                }
            }
            CollisionEvent::Stopped(d1, d2) => {

                let d1_tile = if d1.collision_layers().contains_group(CollisionLayer::Tile) {
                    true
                } else {
                    false
                };
                let d2_tile = if d2.collision_layers().contains_group(CollisionLayer::Tile) {
                    true
                } else {
                    false
                };

                //Either both or neither collisions were player
                if (d1_tile && d2_tile) || (!d1_tile && !d2_tile) { 
                    continue;
                }

                let (tile, to_check) = if d1_tile {
                    (d1, d2)
                } else {
                    (d2, d1)
                };

                if let Ok( mut grounded ) = grounded_query.get_mut(to_check.rigid_body_entity()) {

                    let to_remove = tile.rigid_body_entity();
                    grounded.walls_below.retain(|&x| x != to_remove);

                    if grounded.walls_below.len() == 0 {
                        grounded.grounded = false;
                    }
                }
            }
        }
    }

    let delta_time = time.delta().as_secs_f32();

    for mut grounded in grounded_query.iter_mut() {

        if !grounded.grounded {
            grounded.time_since_grounded += delta_time;
        }
    }
}

pub fn check_on_wall (
    mut collision_event: EventReader<CollisionEvent>,
    mut on_wall_query: Query<&mut IsOnWall>,
    //mut on_wall_event: EventWriter<GroundedEvent>,
) {
    let left_dir = Vec2::new(-1.,0.);
    let right_dir = Vec2::new(1., 0.);

    for event in collision_event.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {

                let d1_tile = if d1.collision_layers().contains_group(CollisionLayer::Tile) { 
                            true
                } else {    false };

                let d2_tile = if d2.collision_layers().contains_group(CollisionLayer::Tile) {
                            true
                } else {    false };

                //Either both or neither collisions were player
                if (d1_tile && d2_tile) || (!d1_tile && !d2_tile) { 
                    continue;
                }

                let (tile, to_check) = if d1_tile {
                            (d1, d2)
                } else {    (d2, d1) };



                if to_check.normals().len() == 0 {
                    continue;
                }

                if to_check.normals()[0] == left_dir || to_check.normals()[0] == right_dir {
                    if let Ok( mut on_wall) = on_wall_query.get_mut(to_check.rigid_body_entity()) {
                        on_wall.on_wall = true;
                        on_wall.walls_touching.push(tile.rigid_body_entity());
                        //println!("Something is now on a wall");
                    }
                }
            }
            CollisionEvent::Stopped(d1, d2) => {

                let d1_tile = if d1.collision_layers().contains_group(CollisionLayer::Tile) { 
                            true
                } else {    false };

                let d2_tile = if d2.collision_layers().contains_group(CollisionLayer::Tile) {
                            true
                } else {    false };

                //Either both or neither collisions were player
                if (d1_tile && d2_tile) || (!d1_tile && !d2_tile) { 
                    continue;
                }

                let (tile, to_check) = if d1_tile {
                            (d1, d2)
                } else {    (d2, d1) };



                if let Ok( mut on_wall ) = on_wall_query.get_mut(to_check.rigid_body_entity()) {
                    
                    let to_remove = tile.rigid_body_entity();
                    on_wall.walls_touching.retain(|&x| x != to_remove);

                    if on_wall.walls_touching.len() == 0 {
                        on_wall.on_wall = false;
                        //println!("Something has left the wall");
                    }
                }
            }
        }
    }
}

//===============================================================

pub fn set_gravity_scale(
    mut query: Query<(Entity, &SetGravityScale, &RigidBodyHandle, Option<&mut Velocity>)>,
    mut bodies: ResMut<RigidBodySet>,
    mut commands: Commands,
) {
    for (entity, new_scale, handle, velocity) in query.iter_mut() {
        if let Some(body) = bodies.get_mut(handle.into_rapier()) {

            body.set_gravity_scale(new_scale.scale, false);

            commands.entity(entity).remove::<SetGravityScale>();

            if new_scale.reset_velocity {
                if let Some(mut velocity) = velocity {
                    velocity.linear = Vec3::ZERO;
                }
            }

        }
    }
}

//===============================================================