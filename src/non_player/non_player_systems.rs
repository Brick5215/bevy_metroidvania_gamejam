//===============================================================

use bevy::prelude::*;
use heron::CollisionEvent;

use crate::{player::player_components::Player, physics::physics_components::{MoveDir, FullMoveDir, CollisionLayer}, general::general_components::{HealthChangeEvent, HealthChangeType}};

use super::non_player_components::*;

//===============================================================

pub fn enemy_target_player(
    mut enemy_query: Query<(&NonPlayer, &mut NonPlayerAggressiveState, &NonPlayerAttackPlayer, &GlobalTransform), Without<Player>>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    //mut current_level: ResMut<LevelSelection>,
) {
    if let Ok((player, player_transform)) = player_query.get_single() {
        
        for (non_player, mut state, attack, transform) in enemy_query.iter_mut() {
            //Check if enemy is in the same room as player //Todo
            //if current_level.is_match(&0, non_player.0) {   

            //}
            let dist_to_player = player_transform.translation.distance(transform.translation);
            if dist_to_player == 0. {
                continue;
            }
            //println!("Distance to player = {}", dist_to_player);

            if dist_to_player < attack.attack_range {
                *state = NonPlayerAggressiveState::Attack{target: player};
            }
            else if dist_to_player > attack.lost_range && *state != NonPlayerAggressiveState::Wander {
                *state = NonPlayerAggressiveState::Wander;
            }
        }
    }
}

pub fn enemy_attack_target(
    mut enemy_query: Query<(&NonPlayerAggressiveState, &GlobalTransform, Option<&mut MoveDir>, Option<&mut FullMoveDir>), Without<Player>>,
    target_query: Query<&GlobalTransform, Without<NonPlayerAggressiveState>>,
) { 

    for (state, transform, move_dir, full_move_dir) in enemy_query.iter_mut() {

        match *state {
            NonPlayerAggressiveState::Attack { target } => {
                
                if let Ok(target_transform) = target_query.get(target) {

                    let new_move_direction = (target_transform.translation - transform.translation).truncate();

                    //new_move_direction = new_move_direction.normalize();

                    if let Some(mut move_dir) = move_dir {
                        if new_move_direction.x > 0. {
                            move_dir.0 = 1.
                        } else if new_move_direction.x < 0. {
                            move_dir.0 = -1.
                        } else {
                            move_dir.0 = 0.;
                        }
                    }
                    if let Some(mut full_move_dir) = full_move_dir {

                        if new_move_direction != Vec2::ZERO {
                            full_move_dir.0 = new_move_direction.normalize();
                        }
                        else {
                            full_move_dir.0 = Vec2::ZERO;
                        }

                    }
                }
            },
            _ => {}
        }
    }
}

pub fn enemy_idle (
    mut enemy_query: Query<(&NonPlayerAggressiveState, Option<&mut MoveDir>, Option<&mut FullMoveDir>)>,
) {
    for (state, move_dir, full_move_dir) in enemy_query.iter_mut() {

        match *state {
            NonPlayerAggressiveState::Wander => {

                if let Some(mut move_dir) = move_dir {

                    move_dir.0 = 0.;

                }
                if let Some(mut full_move_dir) = full_move_dir {
                    full_move_dir.0 = Vec2::ZERO;
                }

            }
            _ => {}
        }
    }
}


pub fn enemy_damage (
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<&NonPlayerDamage>,
    mut damage_event: EventWriter<HealthChangeEvent>,
) {

    for event in collision_events.iter() {
        match event {
            CollisionEvent::Started(d1, d2) => {
            
                let d1_player = if d1.collision_layers().contains_group(CollisionLayer::Player) {
                    true
                } else {
                    false
                };
                let d2_player = if d2.collision_layers().contains_group(CollisionLayer::Player) {
                    true
                } else {
                    false
                };

                //Either both or neither collisions were player
                if (d1_player && d2_player) || (!d1_player && !d2_player) { 
                    continue;
                }

                let (player, to_check) = if d1_player {
                    (d1, d2)
                } else {
                    (d2, d1)
                };


                if to_check.collision_layers().contains_group(CollisionLayer::Enemy) {
                    if let Ok(damage) = enemy_query.get(to_check.rigid_body_entity()) {
                        damage_event.send(HealthChangeEvent {
                            entity: player.collision_shape_entity(),
                            change_type: HealthChangeType::Add{value: -damage.0},
                        })
                    }
                }
            }
            _ => {}
        }
    }
}

//===============================================================