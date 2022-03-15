//===============================================================

use bevy::prelude::*;

pub mod ui_components;
mod ui_systems;

//===============================================================


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShowPopup {
    Show,
    Hide,
}

pub enum Popups {
    Intro,
    Gem,
    Axe,
    Knives,
    Boots,
    Coin,
    End,
}

pub struct PopupExpire (pub Timer);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShowCoinCounter {
    Show,
    Hide,
}
pub struct CoinsCollected(pub usize);

//===============================================================

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app

            .add_state(ShowPopup::Show)

            .add_startup_system(ui_systems::ui_setup)

            .add_system(ui_systems::show_player_health)
            //.add_system(ui_systems::update_player_health)
            .add_system_to_stage(
                CoreStage::PostUpdate, 
                ui_systems::update_player_health
            )

            .insert_resource(Popups::Intro)
            .insert_resource(PopupExpire(Timer::from_seconds(5., false)))

            .add_system_set(
                SystemSet::on_update(ShowPopup::Show)
                    .with_system(ui_systems::spawn_text_popup)
                    .with_system(ui_systems::popup_removal)
            )
            .add_system_set(
                SystemSet::on_enter(ShowPopup::Show)
                    .with_system(ui_systems::start_clock)
            )


            .add_state(ShowCoinCounter::Hide)
            .insert_resource(CoinsCollected(0))
            .add_system_set(
                SystemSet::on_update(ShowCoinCounter::Show)
                .with_system(ui_systems::spawn_coin_popup)
            )
        ;
    }
}

//===============================================================
