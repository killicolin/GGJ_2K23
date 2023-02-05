use crate::components::PreStartMenu;
use crate::{constants::LORE_INTRO, AppState};
use bevy::prelude::ImageBundle;
use bevy::ui::{PositionType, UiImage, UiRect};
use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Children, Color,
        Commands, Entity, NodeBundle, Query, Res, ResMut, State, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Size, Style, Val,
    },
    utils::default,
};
use bevy_kira_audio::prelude::*;

// UI
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const BACKGROUND_COLOR_UI: Color = Color::rgb(0.65, 0.65, 0.65);

pub fn setup_pre_start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), PreStartMenu));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            PreStartMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    image: UiImage(asset_server.load("images/title_screen.png")),
                    ..default()
                },
                PreStartMenu,
            ));
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(50.0), Val::Percent(40.0)),
                            align_items: AlignItems::Center,
                            margin: UiRect {
                                top: Val::Percent(10.0),
                                ..default()
                            },
                            justify_content: JustifyContent::SpaceAround,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    PreStartMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            LORE_INTRO,
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 18.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        PreStartMenu,
                    ));
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(170.0), Val::Px(65.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BACKGROUND_COLOR_UI.into(),
                                ..default()
                            },
                            PreStartMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "KILL THEM",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 32.0,
                                        color: NORMAL_BUTTON,
                                    },
                                ),
                                PreStartMenu,
                            ));
                        });
                });
        });
}

pub fn clean_pre_start_menu(
    mut commands: Commands,
    pre_start_menu_query: Query<Entity, With<PreStartMenu>>,
    audio: Res<Audio>,
) {
    for entity in pre_start_menu_query.iter() {
        commands.entity(entity).despawn();
    }
    audio.stop();
}

pub fn ingame_button(
    mut app_state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, _, _) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                app_state.set(AppState::InGame).unwrap();
            }
            _ => {}
        }
    }
}
