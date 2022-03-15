//===============================================================

use bevy::prelude::*;

pub mod ui_components;
mod ui_systems;

//===============================================================

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_startup_system(ui_systems::ui_setup)

            .add_system(ui_systems::show_player_health)
            //.add_system(ui_systems::update_player_health)
            .add_system_to_stage(
                CoreStage::PostUpdate, 
                ui_systems::update_player_health
            )
        ;
    }
}

//===============================================================
