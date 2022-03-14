//===============================================================

use bevy::prelude::*;
use heron::Velocity;

use crate::{
    animation::animation_components::{
        SpriteSheetAnimation, AutoAnimation
    }, 
    physics::physics_components::{
        ColliderBundle, MovementBundle, MaxVelocity, Accel, FullMoveDir, SetGravityScale,
    },
};

//===============================================================

#[derive(Clone, Default, Bundle)]
pub struct NonPlayerBundle {
    pub non_player: NonPlayer,
    #[bundle]
    pub sprite:     SpriteSheetBundle,
    pub animation:  SpriteSheetAnimation,
    pub auto_anim:  AutoAnimation,
    #[bundle]
    pub physics:    ColliderBundle,
    #[bundle]
    pub movement:   MovementBundle,
}

#[derive(Clone, Default, Bundle)]
pub struct NonPlayerFlyingBundle {
    pub non_player: NonPlayer,
    #[bundle]
    pub sprite:     SpriteSheetBundle,
    pub animation:  SpriteSheetAnimation,
    pub auto_anim:  AutoAnimation,
    #[bundle]
    pub physics:    ColliderBundle,

    pub move_dir: FullMoveDir,
    pub max_velocity: MaxVelocity,
    pub acceleration: Accel,
    pub velocity: Velocity,

    pub gravity: SetGravityScale,
}

//===============================================================

#[derive(Component, Default, Clone)]
pub struct NonPlayer(pub i32);


#[derive(Component, Clone)]
pub enum NonPlayerPassiveState {
    Wander,
    Flee {target: Entity},
}
impl Default for NonPlayerPassiveState {
    fn default() -> Self {
        NonPlayerPassiveState::Wander
    }
}


#[derive(Component, Clone, PartialEq, Eq)]
pub enum NonPlayerAggressiveState {
    Wander,
    Search,
    Attack {target: Entity},
}
impl Default for NonPlayerAggressiveState {
    fn default() -> Self {
        NonPlayerAggressiveState::Wander
    }
}


#[derive(Component, Default, Clone)]
pub struct NonPlayerAttackPlayer {
    pub attack_range: f32,
    pub lost_range: f32,
}

//===============================================================

#[derive(Component, Default, Clone)]
pub struct NonPlayerDamage (pub i32);

//===============================================================