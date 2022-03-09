//================================================================================

use bevy::prelude::*;
use heron::Velocity;
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

            //println!("Weapon fired at {}% charge with facing right = {}", charge.get_charge_percent() * 100., direction.right_facing);

            let mut new_projectile = ProjectileAttackBundle::new(&attack.to_spawn, &direction);

            if !attack.child_of_parent {
                new_projectile.add_transform(transform.translation);
            }

            let new_attack = commands.spawn_bundle(new_projectile).id();

            if attack.child_of_parent {
                commands.entity(entity).add_child(new_attack);
            }

            charge.reset();


        }
    }
}

//================================================================================

pub fn projectile_travel(
    mut query: Query<(&Velocity, &mut Transform), With<ProjectileDamage>>,
    time: Res<Time>,
) {

    let delta = time.delta().as_secs_f32();
    for (velocity, mut transform) in query.iter_mut() {

        transform.translation += velocity.linear * delta;
        
    }
}

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
