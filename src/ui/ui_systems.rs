//===============================================================

use bevy::prelude::*;
use bevy_egui::{EguiContext, egui::{self, Align2}};

use super::{ui_components::*, Popups, PopupExpire, ShowPopup, CoinsCollected};

use crate::{
    general::general_components::Health,
    player::player_components::Player, world::ItemPickedUpEvent
};

//===============================================================



//===============================================================

pub fn ui_setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn show_player_health (
    player_query: Query<&Health, Added<Player>>,
    assets: Res<AssetServer>,
    mut commands: Commands,
){
    for health in player_query.iter() {

        commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Auto),
                justify_content: JustifyContent::SpaceBetween,
                align_self: AlignSelf::FlexEnd,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            color: Color::rgba(0.741, 0.741, 0.741, 0.4).into(),
            ..Default::default()
        })
        .insert(PlayerUIContainer)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(50.)),
                    //align_self: AlignSelf::FlexEnd,
                    //position_type: PositionType::Absolute,
                    //position: Rect {
                        //bottom: Val::Px(5.0),
                        //ri/ght: Val::Px(15.0),
                        //..Default::default()
                    //},
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Health: ",
                    TextStyle {
                        font: assets.load("fonts/LEMONMILK-Regular.otf"),
                        font_size: 30.0,
                        color: Color::RED,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    margin: Rect {
                        left: Val::Undefined,
                        right: Val::Percent(0.),
                        top: Val::Undefined,
                        bottom: Val::Undefined,
                    },
                    ..Default::default()
                },
                color: Color::rgba(1., 0., 0., 0.5).into(),
                ..Default::default()
            }).insert(PlayerUIHealth);

        });
    }
}

pub fn update_player_health (
    player_query: Query<&Health,(With<Player>, Without<Style>)>,
    mut health_ui_query: Query<&mut Style, (With<PlayerUIHealth>, Without<Player>)>,
) {
    for health in player_query.iter() {
        for mut style in health_ui_query.iter_mut() {

            let new_health_percent = (health.get_health() as f32 / health.get_max_health() as f32) * 100.;

            style.size.width = Val::Percent(new_health_percent);  
            //style.margin.right = Val::Percent(100. - new_health_percent);
            
        }
    }
}

//===============================================================

pub fn spawn_text_popup(
    mut egui_context: ResMut<EguiContext>,
    current_popup: Res<Popups>,
) {
    let label_text = match *current_popup {
        Popups::Intro => {
            "You see a small red gem fall from the sky into the village behind you.

            -----------------------------------------------------------------------
            
            Use the arrow keys to move.
            Press space to jump.
            
            -----------------------------------------------------------------------
            
            Developer Note:
            Thanks for trying out this game. Please note there are a few bugs you
            may encounter such as certain parts of the floor disabling your jump,
            platforms not working or needing to jump from level to level.
            I'm still working to address these.
            Thanks again, Hope you enjoy"
        },
        Popups::Gem => {
            "A small red gem on the floor. It's warm to the touch
            
            Press Z to pick it up."
        },
        Popups::Axe => {
            "A sturdy looking climbing axe.
            You could probably climb anything you wanted with this.
            
            Press and hold X to hold onto walls. Release while moving
            in a direction to jump
            
            Press Z to pick it up."
        },
        Popups::Knives => {
            "You found a stash of knives on the floor here.

            There's... a lot of them.

            You shouln't have to worry about running out.

            Press C to throw a knife
            Press Z to pick them up."
        },
        Popups::Boots => {
            "A pair of nice looking boots.
            
            What are they doing out here?
            
            Press and hold Z to sprint
            Press Z to pick them up."
        },
        Popups::Coin => {
            "A shiny coin.
            Who might have carelessly dropped something like this
            
            Well, it's yours now.
            
            Press Z to pick it up"
        },
        Popups::End => {
            "You've made it to the top mountain. 
            You can see the village looking so small beneath you.
            
            Thanks for player. That's it for now. Hope you enjoyed.
            
            Did you find all the hidden items around the map?
            
            See if you can find all the coins."
        },
    };

    egui::Window::new("")
        .min_width(500.)
        .anchor(Align2::CENTER_TOP, egui::Vec2::new(0., 200.))
        .show(egui_context.ctx_mut(), |ui| {
        ui.label(label_text);
    });
    
}

pub fn popup_removal(
    time: Res<Time>,
    mut expire: ResMut<PopupExpire>,
    key_input: Res<Input<KeyCode>>,
    mut popup_state: ResMut<State<ShowPopup>>,
) {

    expire.0.tick(time.delta());

    if key_input.get_just_pressed().len() > 0 && expire.0.finished() {
        popup_state.set(ShowPopup::Hide).unwrap();
    }

}

pub fn start_clock(
    mut expire: ResMut<PopupExpire>,
) {

    expire.0.reset();
}

pub fn spawn_coin_popup (
    mut egui_context: ResMut<EguiContext>,
    coins: Res<CoinsCollected>,
) {
    egui::Window::new("Coin Counter")
        .min_width(500.)
        .anchor(Align2::RIGHT_TOP, egui::Vec2::new(0., 0.))
        .show(egui_context.ctx_mut(), |ui| {
        ui.label(
            format!("Coins Collected: {}/7", coins.0)
        );
    });
}

//===============================================================