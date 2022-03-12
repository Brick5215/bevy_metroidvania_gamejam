//===============================================================

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use heron::prelude::*;

//===============================================================

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Tile,
    Player,
    Enemy,
}

//===============================================================

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
    pub collision_layer: CollisionLayers,
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
            collision_layer: CollisionLayers::new(CollisionLayer::Player, CollisionLayer::Tile),
            ..Default::default()
        }
    }
    pub fn projectile(collider: CollisionShape) -> Self {
        Self {
            collider,
            rigid_body: RigidBody::Sensor,
            rotation_constraints: RotationConstraints::lock(),
            physic_material: PhysicMaterial {
                restitution: 0.,
                density: 0.,
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
    pub air_deaccel: Option<f32>,
}

#[derive(Component, Clone, Default)]
pub struct MoveDir(pub f32);

#[derive(Component, Clone, Default)]
pub struct CanJump {
    pub jump_force: f32,
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
    pub walls_below: Vec<Entity>,
}

pub struct JumpEvent(pub Entity);
pub struct GroundedEvent(pub Entity);

//===============================================================

#[derive(Component, Clone, Default)]
pub struct IsOnWall {
    pub on_wall: bool,
    pub walls_touching: Vec<Entity>,
}

//===============================================================
