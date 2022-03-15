//===============================================================

use bevy::prelude::*;

use super::ui_components::*;

use crate::{
    general::general_components::Health,
    player::player_components::Player
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
                color: Color::rgb(1., 0., 0.).into(),
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