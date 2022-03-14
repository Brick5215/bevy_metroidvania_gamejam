//================================================================================

use bevy::prelude::*;
use heron::{Velocity, RigidBody, CollisionEvent, CollisionLayers};
use crate::{physics::physics_components::{SetGravityScale, CollisionLayer}, general::general_components::{HealthChangeEvent, HealthChangeType}};

use super::weapon_components::*;

//================================================================================

pub fn charge_weapon(
    mut query: Query<(&WeaponState, &mut WeaponCharge)>,
    time: Res<Time>,
) {
    for (state, mut charge) in query.iter_mut() {
        if state.charging {
            charge.tick(time.delta());
        }
    }
}

pub fn weapon_state_change (
    mut query: Query<(&WeaponState, &WeaponCharge, Entity), Changed<WeaponState>>,
    mut fire_weapon_event: EventWriter<FireWeaponEvent>,
) {

    for (state, charge, entity) in query.iter_mut() {

        if !state.charging && charge.get_charge_percent() != 0.{

            fire_weapon_event.send(FireWeaponEvent(entity));
        }
    }
}

//================================================================================

pub fn fire_weapon(
    mut fire_weapon_event: EventReader<FireWeaponEvent>,
    mut weapon_query: Query<(Entity, &GlobalTransform, &mut WeaponCharge, &WeaponAttack, &WeaponDirection)>,
    mut commands: Commands,
) {

    for event in fire_weapon_event.iter() {

        if let Ok((entity, transform, mut charge, attack, direction)) = weapon_query.get_mut(event.0) {

            let mut new_projectile = ProjectileAttackBundle::new(entity, &attack.to_spawn, &direction, attack.is_friendly);

            if !attack.child_of_parent {
                new_projectile.add_transform(transform.translation);
            }

            let mut new_attack = commands.spawn_bundle(new_projectile);

            if let Some(val) = &attack.gravity_scale {
                new_attack.insert(val.clone());
            }

            let new_attack = new_attack.id();

            if attack.child_of_parent {
                commands.entity(entity).add_child(new_attack);
            }

            charge.reset();
        }
    }
}

//================================================================================

//================================================================================

pub fn projectile_expire(
    mut projectile_query: Query<(&mut ProjectileExpire, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {

    for (mut expire, entity) in projectile_query.iter_mut() {

        if expire.finished() {
            commands.entity(entity).despawn();
        }
        expire.tick(time.delta());
    }
}

//================================================================================

pub fn projectile_collision(
    mut projectile_query: Query<(Entity, &mut CollisionLayers, Option<&ProjectileDamage>, Option<&mut Velocity>), With<Projectile>>,
    mut collision_events: EventReader<CollisionEvent>,
    mut health_events: EventWriter<HealthChangeEvent>,
    mut commands: Commands,
) {

    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {

                let d1_weapon = if d1.collision_layers().contains_group(CollisionLayer::Weapon) {
                    true
                } else {
                    false
                };
                let d2_weapon = if d2.collision_layers().contains_group(CollisionLayer::Weapon) {
                    true
                } else {
                    false
                };

                //This should never happen but since we're
                //comparing both, might as well check
                if d1_weapon && d2_weapon { 
                    continue;
                }
                //Neither of the collisions were weapons
                if !d1_weapon && !d2_weapon {
                    continue;
                }

                let (weapon, to_test) = if d1_weapon {
                    (d1, d2)
                } else {
                    (d2, d1)
                };


                //Collided with a wall. If ranged projectile, should be
                //disabled
                if to_test.collision_layers().contains_group(CollisionLayer::Tile) {
                    if let Ok((entity, mut layer, _, _)) = projectile_query.get_mut(weapon.rigid_body_entity()) {
                        //If collided, remove projectile damage and disable physics
                        commands.entity(entity)
                            .remove::<ProjectileDamage>()
                            //.insert(SetGravityScale(0.));
                            //.insert(RigidBody::Sensor);
                            .remove::<RigidBody>();
                        
                        *layer = CollisionLayers::none();
                        
                        //layer
                            //.without_mask(CollisionLayer::Tile)
                            //.without_mask(CollisionLayer::Entity)
                            //.without_mask(CollisionLayer::Player);
                    }
                }
                //Collided with an entity. Because weapon should not collide
                //with entity layer directly, it will have collided with
                //friendly or enemy layer and hence this so dont have to check
                //for friendly fire (thats what i'm hoping for at least)
                else if to_test.collision_layers().contains_group(CollisionLayer::Entity) {
                    if let Ok((_, mut layer, Some(damage), Some(mut velocity))) = projectile_query.get_mut(weapon.rigid_body_entity()) {

                        velocity.linear *= 0.2;

                        let to_damage = to_test.rigid_body_entity();
                        health_events.send(
                            HealthChangeEvent {
                                entity: to_damage,
                                change_type: HealthChangeType::Add{value: -damage.0},
                            }
                        );

                        *layer = layer
                            .without_mask(CollisionLayer::Entity)
                            .without_mask(CollisionLayer::Player)
                            .without_mask(CollisionLayer::Enemy);
                            //.without_group(CollisionLayer::Weapon);
                    }
                }
            },
            _ => {}
        }
    }
}

//================================================================================
