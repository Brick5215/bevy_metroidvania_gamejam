//================================================================================

use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::*,
    entity::ShapeBundle,
};
use heron::{Velocity, CollisionShape, RigidBody};

use std::time::Duration;

use crate::{
    general::tools::rotate_vector,
    physics::physics_components::ColliderBundle, animation::animation_components::SimpleAnimationBundle
};

//================================================================================


#[derive(Bundle, Clone, Default)]
pub struct WeaponInventoryBundle {
    pub wielder: WeaponWielder,
    pub inventory: WeaponInventory,
}

#[derive(Component, Clone, Default)]
pub struct WeaponWielder;

#[derive(Component, Clone, Default)]
pub struct WeaponInventory {
    slot1: Option<Entity>,
    slot2: Option<Entity>,
}
impl WeaponInventory {
    pub fn add_slot1_weapon(&mut self, entity: Entity) -> bool {

        if self.slot1.is_none() {
            self.slot1 = Some(entity);  
            return true
        }
        return false
    }
    pub fn add_slot2_weapon(&mut self, entity: Entity) -> bool {
        if self.slot2.is_none() {
            self.slot2 = Some(entity);
            return true
        }
        return false
    }

    pub fn get_slot1(&self) -> &Option<Entity> {
        return &self.slot1
    }
    pub fn get_slot2(&self) -> &Option<Entity> {
        return &self.slot2
    }
    pub fn _has_slot1(&self) -> bool {
        return self.slot1.is_some()
    }
    pub fn _has_slot2(&self) -> bool {
        return self.slot2.is_some()
    }
}


//================================================================================

#[derive(Bundle)]
pub struct WeaponBundle {
    pub state:      WeaponState,
    pub direction:  WeaponDirection,
    pub charge:     WeaponCharge,
    #[bundle]
    pub preview:    WeaponPreviewBundle,
    pub attack:     WeaponAttack,
}

//----------------------------------------------
//Weapon states, Charge and previews

#[derive(Component, Default)]
pub struct WeaponState {
    pub charging: bool,
}

#[derive(Component, Default)]
pub struct WeaponDirection {
    pub right_facing: bool,
    pub direction: WeaponDirections,
}

pub enum WeaponDirections {
    Up,
    ForwardUp,
    Forward,
    ForwardDown,
    Down,
}
impl Default for WeaponDirections {
    fn default() -> Self {
        WeaponDirections::Forward
    }
}
impl WeaponDirections {
    pub fn from_vec2(dir: Vec2) -> Self {
        
        let x = dir.x as i8;
        let y = dir.y as i8;
        match (x, y) {
            (0, 1,)     => {WeaponDirections::Up},
            (1, 1,)     => {WeaponDirections::ForwardUp},
            (1, 0,)     => {WeaponDirections::Forward},
            (1, -1,)    => {WeaponDirections::ForwardDown},
            (0, -1,)    => {WeaponDirections::Down},
            (-1, -1,)   => {WeaponDirections::ForwardDown},
            (-1, 0,)    => {WeaponDirections::Forward},
            (-1, 1,)    => {WeaponDirections::ForwardUp},
            _           => {WeaponDirections::default()}
        }
    }
    fn get_angle(&self) -> f32 {

        match self {
            WeaponDirections::Up => 90.,
            WeaponDirections::ForwardUp => 45.,
            WeaponDirections::Forward => 0.,
            WeaponDirections::ForwardDown => -45.,
            WeaponDirections::Down => -90.,
        }
    }
}

//----------------------------------------------

#[derive(Component)]
pub struct WeaponCharge(Timer);
impl WeaponCharge {
    pub fn new(max_charge: f32) -> Self {
        WeaponCharge(Timer::from_seconds(max_charge, false))
    }

    pub fn _set_max_charge(&mut self, max_charge: u64) {
        self.0.set_duration(Duration::from_secs(max_charge));
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }

    pub fn get_charge_percent(&self) -> f32 {
        self.0.percent()
    }
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
impl Default for WeaponCharge {
    fn default() -> Self {
        WeaponCharge::new(1.)
    }
}

//----------------------------------------------

#[derive(Component)]
struct WeaponPreview;

#[derive(Bundle)]
pub struct WeaponPreviewBundle {
    preview: WeaponPreview,
    #[bundle]
    display: ShapeBundle,
}
impl WeaponPreviewBundle {
    pub fn new(color: Color ) -> Self {

        let line = shapes::Line(Vec2::new(0., 0.), Vec2::new(0., 0.));

        WeaponPreviewBundle {
            preview: WeaponPreview,
            display: GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::color(color)),
                Transform::from_xyz(0., 0., 0.,)
            ),
        }
    }
}
impl Default for WeaponPreviewBundle {
    fn default() -> Self {
        WeaponPreviewBundle::new(Color::RED)
    }
}

//----------------------------------------------
//Weapon attack and projectile templates

#[derive(Component)]
pub struct WeaponAttack {
    pub to_spawn: ProjectileTemplate,
    pub child_of_parent: bool,
    pub is_friendly: bool,
    pub gravity_scale: Option<f32>,
}
/*impl Default for WeaponAttack {
    fn default() -> Self {
        WeaponAttack {
            to_spawn: ProjectileTemplate::create_melee(
                1,
                1.,
                Vec2::ZERO,
                1.,
                1.,
            ),
            child_of_parent: false,
            is_friendly: true,
            gravity_scale: None,
        }
    }
}*/

pub struct ProjectileTemplate {
    pub damage: i32,
    pub expire: f32,
    pub size: CollisionShape,

    pub initial_speed: Vec2,
    pub spawn_offset: Vec2,

    pub rigid_body: RigidBody,
    pub animation_bundle: SimpleAnimationBundle,
}
impl ProjectileTemplate {
    pub fn create_range(
        damage: i32, expire: f32, spawn_offset: Vec2, 
        size: f32, initial_speed: Vec2,
        rigid_body: RigidBody,
        animation_bundle: SimpleAnimationBundle,

    ) -> Self {

        ProjectileTemplate {
            damage,
            expire,
            size: CollisionShape::Sphere{radius: size},
            initial_speed,
            spawn_offset,
            rigid_body,
            animation_bundle,
        }
    }

    pub fn create_melee(
        damage: i32, expire: f32, spawn_offset: Vec2, 
        width: f32, height: f32,
        animation_bundle: SimpleAnimationBundle,
    ) -> Self {
        ProjectileTemplate {
            damage,
            expire,
            size: CollisionShape::Cuboid {
                half_extends: Vec3::new(width, height, 0.) / 2.,
                border_radius: None,
            },
            initial_speed: Vec2::ZERO,
            spawn_offset,
            rigid_body: RigidBody::Sensor,
            animation_bundle,
        }
    }
}

//================================================================================

pub struct FireWeaponEvent(pub Entity);

//================================================================================
//Projectile to be spawned on fire

#[derive(Bundle)]
pub struct ProjectileAttackBundle {
    projectile: Projectile,
    damage: ProjectileDamage,
    expire: ProjectileExpire,
    velocity: Velocity,
    #[bundle]
    collision: ColliderBundle,
    #[bundle]
    animation_bundle: SimpleAnimationBundle,
}
impl ProjectileAttackBundle {
    pub fn new(parent: Entity, base: &ProjectileTemplate, dir: &WeaponDirection, is_friendly: bool) -> ProjectileAttackBundle {

        let rotation_angle = dir.direction.get_angle().to_radians();
        
        let mut projectile_velocity = rotate_vector(base.initial_speed, rotation_angle);
        let mut projectile_offset = rotate_vector(base.spawn_offset, rotation_angle);

        if !dir.right_facing {
            projectile_velocity.x *= -1.; 
            projectile_offset.x *= -1.;
        }
        

        let mut animation_copy = base.animation_bundle.clone();
        animation_copy.sprite_sheet.transform = Transform::from_translation(projectile_offset.extend(40.));


        ProjectileAttackBundle {
            projectile: Projectile(parent),
            damage: ProjectileDamage(base.damage),
            expire: ProjectileExpire::new(base.expire),
            velocity: Velocity::from_linear(projectile_velocity.extend(0.)),
            collision: ColliderBundle::projectile(base.size.clone(), base.rigid_body, is_friendly),
            animation_bundle: animation_copy,
        }
    }

    // Used by projectiles not attached to parent 
    // to add global transform coords.
    pub fn add_transform(&mut self, to_add: Vec3) {
        self.animation_bundle.sprite_sheet.transform.translation += to_add;
    }
}

//================================================================================

#[derive(Component, Clone)] 
pub struct Projectile (pub Entity);

#[derive(Component)] pub struct ProjectileDamage(pub i32);
#[derive(Component)] pub struct ProjectileExpire(Timer);
impl ProjectileExpire {
    pub fn new(expire_time: f32) -> Self {
        ProjectileExpire(Timer::from_seconds(expire_time, false))
    }
    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
    pub fn finished(&self) -> bool {
        self.0.finished()
    }
}

//================================================================================
