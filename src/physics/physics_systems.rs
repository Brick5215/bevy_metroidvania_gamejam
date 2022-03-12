//===============================================================

use bevy::prelude::*;
use heron::prelude::*;

use crate::general::tools::clamp_shift;

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

//=================================================================================

pub fn apply_jump (
    mut jump_event: EventReader<JumpEvent>,
    mut query: Query<(&mut CanJump, &mut Velocity)>
) {

    for event in jump_event.iter() {
        if let Ok((mut jumps, mut velocity)) = query.get_mut(event.0) {

            if jumps.jumps_left > 0 {

                velocity.linear.y = jumps.jump_force;
                jumps.jumps_left -= 1;
            }
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

                if d1.normals().len() == 0 {
                    continue;
                }

                //println!("d1 normals: {:#?}, d2 normals: {:#?}", d1.normals(), d2.normals());
                if d1.normals()[0] == down_dir {
                    if let Ok( mut grounded) = grounded_query.get_mut(d1.rigid_body_entity()) {
                        grounded.grounded = true;
                        grounded.time_since_grounded = 0.;
                        grounded.walls_below.push(d2.rigid_body_entity());
                        if grounded.walls_below.len() == 1 {
                            grounded_event.send(GroundedEvent(d1.rigid_body_entity()));
                        }
                    }
                }
            }
            CollisionEvent::Stopped(d1, d2) => {

                if let Ok( mut grounded ) = grounded_query.get_mut(d1.rigid_body_entity()) {
                    
                    let to_remove = d2.rigid_body_entity();
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

                if d1.normals().len() == 0 {
                    continue;
                }

                //println!("d1 normals: {:#?}, d2 normals: {:#?}", d1.normals(), d2.normals());
                if d1.normals()[0] == left_dir || d1.normals()[0] == right_dir {
                    if let Ok( mut on_wall) = on_wall_query.get_mut(d1.rigid_body_entity()) {
                        on_wall.on_wall = true;
                        on_wall.walls_touching.push(d2.rigid_body_entity());
                        //println!("Something is now on a wall");
                    }
                }
            }
            CollisionEvent::Stopped(d1, d2) => {

                if let Ok( mut on_wall ) = on_wall_query.get_mut(d1.rigid_body_entity()) {
                    
                    let to_remove = d2.rigid_body_entity();
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
