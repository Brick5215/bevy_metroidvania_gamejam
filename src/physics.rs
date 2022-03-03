//===============================================================

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

//===============================================================

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
}
impl ColliderBundle {
    pub fn player(width: f32, height: f32) -> Self {
        Self {
            collider: CollisionShape::Cuboid {
                half_extends: Vec3::new(width, height, 0.) / 2.,
                border_radius: None,
            },
            rigid_body: RigidBody::Dynamic,
            rotation_constraints: RotationConstraints::lock(),
            physic_material: PhysicMaterial {
                restitution: 0.,
                density: 1.,
                friction: 0.,
            },
            ..Default::default()
        }
    }
}

//===============================================================

#[derive(Component, Clone, Default)]
pub struct MaxVelocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Clone, Default)]
pub struct Accel {
    pub accel: f32,
    pub deaccel: f32,
}

#[derive(Component, Clone, Default)]
pub struct MoveDir(pub f32);

#[derive(Component, Clone, Default)]
pub struct CanJump {
    //pub can_jump: bool,
    pub jump_force: f32,
    //pub jump_start: bool,
    pub jumps_left: u32,
    pub total_jumps: u32,
}

#[derive(Bundle, Clone, Default)]
pub struct MovementBundle {
    pub move_dir: MoveDir,
    pub max_velocity: MaxVelocity,
    pub acceleration: Accel,
    pub velocity: Velocity,
    pub jump: CanJump,
    pub grounded: IsGrounded,
}

//===============================================================

#[derive(Component, Clone, Default)]
pub struct IsGrounded {
    pub grounded: bool,
    pub time_since_grounded: f32,
    pub entities_below: Vec<Entity>,
}

pub struct JumpEvent(pub Entity);
pub struct GroundedEvent(pub Entity);

//===============================================================

fn cap_velocity(
    mut query: Query<(&mut Velocity, &MaxVelocity)>,
) {
    for (mut velocity, cap) in query.iter_mut() {
        velocity.linear.x = velocity.linear.x.clamp(-cap.x, cap.x);
        velocity.linear.y = velocity.linear.y.clamp(-cap.y, cap.y);
    }
}

fn apply_movespeed (
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

fn apply_jump (
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

    //for (mut jump, mut velocity) in query.iter_mut() {
//
        //if jump.can_jump && jump.jump_start {
            //velocity.linear.y = jump.jump_force;
            //jump.jump_start = false;
        //}
    //}
}

fn reset_jump(
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

fn check_grounded (
    mut collision_event: EventReader<CollisionEvent>,
    mut grounded_query: Query<&mut IsGrounded>,
    mut grounded_event: EventWriter<GroundedEvent>,
    time: Res<Time>,
) {
    let down_dir = Vec2::new(0.,-1.);

    for event in collision_event.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {

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

pub struct CustomPhysicsPlugin;
impl Plugin for CustomPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<JumpEvent>()
            .add_event::<GroundedEvent>()

            .add_system(cap_velocity)
            .add_system(apply_movespeed)
            .add_system(apply_jump)
            .add_system(check_grounded)
            .add_system(reset_jump)
        ;
    }
}

//===============================================================