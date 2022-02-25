//============================================================================

use std::collections::{HashMap, HashSet};

use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResized};
use heron::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;

//============================================================================

//Using 16:10 aspect ratio on 15 times scale ((16:10) * 15)
const GAME_WIDTH: u32 = 240;    //16 * 15
const GAME_HEIGHT: u32 = 150;   //10 * 15

//============================================================================

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;
#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

//============================================================================


//Algorithm taken from the bevy_ecs_ldtk github in their platformer example
fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &Handle<LdtkLevel>)>,
    levels: Res<Assets<LdtkLevel>>,
) {

    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    // consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.for_each(|(&grid_coords, &Parent(parent))| {
        // the intgrid tiles' direct parents will be bevy_ecs_tilemap chunks, not the level
        // To get the level, you need their grandparents, which is where parent_query comes in
        if let Ok(&Parent(level_entity)) = parent_query.get(parent) {
            level_to_wall_locations
                .entry(level_entity)
                .or_insert(HashSet::new())
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_handle)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let level = levels
                    .get(level_handle)
                    .expect("Level should be loaded by this point");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level
                    .level
                    .layer_instances
                    .clone()
                    .expect("Level asset should have layers")[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right
                    // edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut wall_rects: Vec<Rect<i32>> = Vec::new();
                let mut previous_rects: HashMap<Plate, Rect<i32>> = HashMap::new();

                // an extra empty row so the algorithm "terminates" the rects that touch the top
                // edge
                plate_stack.push(Vec::new());

                for (y, row) in plate_stack.iter().enumerate() {
                    let mut current_rects: HashMap<Plate, Rect<i32>> = HashMap::new();
                    for plate in row {
                        if let Some(previous_rect) = previous_rects.remove(&plate) {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    top: previous_rect.top + 1,
                                    ..previous_rect
                                },
                            );
                        } else {
                            current_rects.insert(
                                *plate,
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                            );
                        }
                    }

                    // Any plates that weren't removed above have terminated
                    wall_rects.append(&mut previous_rects.values().copied().collect());
                    previous_rects = current_rects;
                }

                // spawn colliders for every rectangle
                for wall_rect in wall_rects {
                    commands
                        .spawn()
                        .insert(CollisionShape::Cuboid {
                            half_extends: Vec3::new(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                0.,
                            ),
                            border_radius: None,
                        })
                        .insert(RigidBody::Static)
                        .insert(PhysicMaterial {
                            friction: 0.,
                            ..Default::default()
                        })
                        .insert(Transform::from_xyz(
                            (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32 / 2.,
                            (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32 / 2.,
                            10.,
                        ))
                        .insert(GlobalTransform::default())
                        // Making the collider a child of the level serves two purposes:
                        // 1. Adjusts the transforms to be relative to the level for free
                        // 2. the colliders will be despawned automatically when levels unload
                        .insert(Parent(level_entity));
                }
            }
        });
    }
}

//============================================================================

fn change_level(
    level_query: Query<(&Handle<LdtkLevel>, &Transform), Without<Player>>,
    mut current_level: ResMut<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,

    mut player_query: Query<&Transform, With<Player>>,
) {

    //Iterate over each of the levels in the world
    for (level_handle, level_transform) in level_query.iter() {
        //Check if the current level handle exists and get its data
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            
            let level_bounds = Rect {
                left:   level_transform.translation.x,
                right:  level_transform.translation.x + ldtk_level.level.px_wid as f32,
                bottom: level_transform.translation.y,
                top:    level_transform.translation.y + ldtk_level.level.px_hei as f32,
            };

            for player_transform in player_query.iter_mut() {

                if     player_transform.translation.x < level_bounds.right
                    && player_transform.translation.x > level_bounds.left
                    && player_transform.translation.y < level_bounds.top
                    && player_transform.translation.y > level_bounds.bottom
                {

                    if !current_level.is_match(&0, &ldtk_level.level) {
                        *current_level = LevelSelection::Uid(ldtk_level.level.uid);
                        //println!("parent = {}", parent.0.id());
                        //println!("Level changed to level {}", ldtk_level.level.uid);
                        //println!("Level has entity id {}", level_entity.id());
                        return
                    }
                }
            }
        }
    }
}


const ASPECT_RATIO_WIDTH: f32 = 16.;
const ASPECT_RATIO_HEIGHT: f32 = 10.;

const ASPECT_RATIO: f32 = ASPECT_RATIO_WIDTH / ASPECT_RATIO_HEIGHT;


const MIN_CAMERA_WIDTH: f32 =   ASPECT_RATIO_WIDTH  * 13.;
const MIN_CAMERA_HEIGHT: f32 =  ASPECT_RATIO_HEIGHT * 13.;

const MAX_CAMERA_WIDTH: f32 = ASPECT_RATIO_WIDTH    * 35.;
const MAX_CAMERA_HEIGHT: f32 = ASPECT_RATIO_HEIGHT  * 35.;


fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), (Without<OrthographicProjection>, Without<Player>)>,
    current_level: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {

    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation;
        let (mut camera_projection, mut camera_transform) = camera_query.single_mut();

        
        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if current_level.is_match(&0, &level) {


                    //Reset the camera positions. These probably wont ever change for the time being
                    camera_projection.scaling_mode = ScalingMode::None;
                    camera_projection.left = 0.;
                    camera_projection.bottom = 0.;
                    camera_projection.right = MAX_CAMERA_WIDTH;
                    camera_projection.top = MAX_CAMERA_HEIGHT;


                    let level_height = level.px_hei as f32;
                    if level_height < MAX_CAMERA_HEIGHT {
                        camera_transform.translation.y = level_transform.translation.y + (level_height / 2.);
                    }
                    else {
                        let level_bottom = level_transform.translation.y;
                        let level_top = level_bottom + level.px_hei as f32 - MAX_CAMERA_HEIGHT;
                        
                        camera_transform.translation.y = (player_pos.y - MAX_CAMERA_HEIGHT / 2.).clamp(level_bottom, level_top);
                    }

                    
                    let level_width = level.px_wid as f32;
                    if level_width < MAX_CAMERA_WIDTH {
                        camera_transform.translation.x = level_transform.translation.x + (level_width / 2.);
                    }
                    else {

                        let level_left = level_transform.translation.x;
                        let level_right = level_left + level.px_wid as f32 - MAX_CAMERA_WIDTH;

                        camera_transform.translation.x = (player_pos.x - MAX_CAMERA_WIDTH / 2.).clamp(level_left, level_right);
                    }
                }
            }
        }
    }
}


fn camera_follow_current_level(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), (Without<OrthographicProjection>, Without<Player>)>,
    current_level: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;
        let (mut camera_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if current_level.is_match(&0, &level) {


                    camera_projection.scaling_mode = ScalingMode::None;
                    camera_projection.left = 0.;
                    camera_projection.bottom = 0.;

                    let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                    if level_ratio > ASPECT_RATIO { //Level is wider than the screen

                        camera_projection.top = (level.px_hei as f32 / ASPECT_RATIO_HEIGHT).round() * ASPECT_RATIO_HEIGHT;
                        camera_projection.right = camera_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - camera_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - camera_projection.right);
                        camera_transform.translation.y = 0.;

                    }
                    else {
                        //Level is taller than the screen
                        
                        camera_projection.right = (level.px_wid as f32 / ASPECT_RATIO_WIDTH).round() * ASPECT_RATIO_WIDTH;
                        camera_projection.top = camera_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - camera_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - camera_projection.top);
                        camera_transform.translation.x = 0.;


                    }

                    //camera_transform.translation.x = player_location.translation.x - camera_projection.right / 2.;
                    //camera_transform.translation.y = player_location.translation.y - camera_projection.top / 2.;

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;



                }
            }
        }
    }
}






pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        Without<Player>,
    >,
    player_query: Query<&Transform, With<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    if let Ok(Transform {
        translation: player_translation,
        ..
    }) = player_query.get_single()
    {
        let player_translation = player_translation.clone();

        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                if level_selection.is_match(&0, &level) {
                    let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

                    orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                    orthographic_projection.bottom = 0.;
                    orthographic_projection.left = 0.;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                        orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                        camera_transform.translation.x = (player_translation.x
                            - level_transform.translation.x
                            - orthographic_projection.right / 2.)
                            .clamp(0., level.px_wid as f32 - orthographic_projection.right);
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                        orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                        camera_transform.translation.y = (player_translation.y
                            - level_transform.translation.y
                            - orthographic_projection.top / 2.)
                            .clamp(0., level.px_hei as f32 - orthographic_projection.top);
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;


                    println!("Camera Width: {}, Height: {}", orthographic_projection.right, orthographic_projection.top);


                }
            }
        }
    }
}




//============================================================================

pub struct ArenaPlugin;
impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_system(spawn_wall_collision)
            .add_system(change_level)
            .add_system(camera_follow_player)
            //.add_system(camera_follow_current_level)
            //.add_system(camera_fit_inside_current_level)
        ;
    }
}

//============================================================================