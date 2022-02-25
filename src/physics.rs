//===============================================================

use bevy::prelude::*;
use heron::prelude::*;

use crate::general_components::{
    MaxVelocity, Accel, MoveDir, CanJump
};

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

pub fn apply_jump (
    mut query: Query<(&mut CanJump, &mut Velocity)>
) {

    for (mut jump, mut velocity) in query.iter_mut() {

        if jump.can_jump && jump.jump_start {
            velocity.linear.y += jump.jump_force;
            jump.jump_start = false;
        }
    }
}

//===============================================================

pub struct CustomPhysicsPlugin;
impl Plugin for CustomPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(cap_velocity)
            .add_system(apply_movespeed)
            .add_system(apply_jump)
        ;
    }
}

//===============================================================