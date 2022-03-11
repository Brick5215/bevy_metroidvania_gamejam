//===============================================================

use bevy::prelude::*;
use heron::prelude::*;

use super::physics_components::*;

//===============================================================

pub fn cap_velocity(
    mut query: Query<(&mut Velocity, &MaxVelocity)>,
) {
    for (mut velocity, cap) in query.iter_mut() {

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

pub fn check_on_wall (

) {
    
}

pub fn check_grounded (
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
                        grounded.entities_below.push(d2.rigid_body_entity());
                        if grounded.entities_below.len() == 1 {
                            grounded_event.send(GroundedEvent(d1.rigid_body_entity()));
                        }
                    }
                }
            }
            CollisionEvent::Stopped(d1, d2) => {

                if let Ok( mut grounded ) = grounded_query.get_mut(d1.rigid_body_entity()) {
                    
                    let to_remove = d2.rigid_body_entity();
                    grounded.entities_below.retain(|&x| x != to_remove);

                    if grounded.entities_below.len() == 0 {
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

//===============================================================
