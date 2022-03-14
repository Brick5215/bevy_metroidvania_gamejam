//===============================================================

use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkIntCell;
use heron::prelude::*;

//===============================================================

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Tile,
    Entity,
    Player,
    Enemy,
    Weapon,
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
            collision_layer: CollisionLayers::new(
                CollisionLayer::Player, 
                CollisionLayer::Tile)
                .with_group(CollisionLayer::Entity)
                .with_mask(CollisionLayer::Enemy),
            ..Default::default()
        }
    }

    pub fn non_player(width: f32, height: f32) -> Self {
        Self {
            collider: CollisionShape::Cuboid {
                half_extends: Vec3::new(width, height, 0.) / 2.,
                border_radius: None,
            },
            rigid_body: RigidBody::Dynamic,
            rotation_constraints: RotationConstraints::lock(),
            physic_material: PhysicMaterial {
                restitution: 0.5,
                density: 0.2,
                friction: 0.,
            },
            collision_layer: CollisionLayers::none()
                .with_group(CollisionLayer::Enemy)
                .with_group(CollisionLayer::Entity)
                
                .with_mask(CollisionLayer::Tile)
                .with_mask(CollisionLayer::Weapon)
                .with_mask(CollisionLayer::Player),
            ..Default::default()
        }
    }

    pub fn projectile(collider: CollisionShape, rigid_body: RigidBody, is_friendly: bool) -> Self {

        let mut collision_layer = CollisionLayers::new(
            CollisionLayer::Weapon,
            CollisionLayer::Tile,
        );//.with_mask(CollisionLayer::Entity);
        if is_friendly {
            //collision_layer = collision_layer.with_group(CollisionLayer::Player);
            collision_layer = collision_layer.with_mask(CollisionLayer::Enemy);
        } else {
            //collision_layer = collision_layer.with_group(CollisionLayer::Enemy);
            collision_layer = collision_layer.with_mask(CollisionLayer::Player);
        }


        Self {
            collider,
            rigid_body,
            rotation_constraints: RotationConstraints::restrict_to_z_only(),
            physic_material: PhysicMaterial {
                restitution: 0.,
                density: 0.1,
                friction: 0.,
            },
            collision_layer,
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
pub struct FullMoveDir(pub Vec2);


#[derive(Component, Clone)]
pub struct CanJump {
    pub jump_pressed: bool,
    pub jump_repressed: bool,
    pub jumping: bool,
    pub jump_timer: Timer,

    pub max_jump_force: f32,
    pub jump_force: f32,
    pub initial_jump_force: f32,

    pub jumps_left: u32,
    pub total_jumps: u32,
}
impl Default for CanJump {
    fn default() -> Self {
        CanJump {
            jump_pressed:       false,
            jump_repressed:     false,
            jumping:            false,
            jump_timer:         Timer::from_seconds(0.5, false),

            max_jump_force:     500.,
            jump_force:         600.,
            initial_jump_force: 130.,

            jumps_left:         1,
            total_jumps:        1,
        }
    }
}
impl CanJump {
    pub fn new(
        jump_time: f32, jump_force: f32, initial_jump_force: f32
    ) -> Self {
        CanJump {
            jump_timer: Timer::from_seconds(jump_time, false),
            max_jump_force: jump_force,
            jump_force,
            initial_jump_force,
            ..Default::default()
        }
    }
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

pub struct GroundedEvent(pub Entity);

//===============================================================

#[derive(Component, Clone, Default)]
pub struct IsOnWall {
    pub on_wall: bool,
    pub walls_touching: Vec<Entity>,
}

//===============================================================

#[derive(Component, Clone)]
pub struct SetGravityScale(pub f32);
impl Default for SetGravityScale {
    fn default() -> Self {
        SetGravityScale(1.)
    }
}

//===============================================================