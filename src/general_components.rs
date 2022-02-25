
use bevy::prelude::*;
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

//================================================================


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

//================================================================

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
    pub can_jump: bool,
    pub jump_force: f32,
    pub jump_start: bool,
}

#[derive(Bundle, Clone, Default)]
pub struct MovementBundle {
    pub move_dir: MoveDir,
    pub max_velocity: MaxVelocity,
    pub acceleration: Accel,
    pub velocity: Velocity,
    pub jump: CanJump,
}

//================================================================

#[derive(Component)]
pub struct FadeInOut {
    pub timer: Timer,
    pub from: f32,
    pub to: f32,
    pub remove_on_finish: bool,
    pub remove_component_on_finish: bool,
}

//================================================================