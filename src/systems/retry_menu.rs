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

use crate::{
    components::RetryMenu,
    resource::{TotalKilled, TotalSpawned, TotalToSpawn},
    StatsRes,
};
use crate::{resource::Score, AppState};

// UI
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub fn setup_retry_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut total_to_spawn: ResMut<TotalToSpawn>,
    mut total_killed: ResMut<TotalKilled>,
    mut total_spawned: ResMut<TotalSpawned>,
) {
    // ui camera
    total_to_spawn.update_paramter_for_level_id(0);
    total_killed.amount = 0;
    total_spawned.amount = 0;
    commands.spawn((Camera2dBundle::default(), RetryMenu));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            RetryMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                RetryMenu,
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    RetryMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Retry",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        RetryMenu,
                    ));
                });
        });
}

pub fn clean_retry_menu(
    mut commands: Commands,
    retry_menu_query: Query<Entity, With<RetryMenu>>,
    mut stats: ResMut<StatsRes>,
    mut score: ResMut<Score>,
    audio: Res<Audio>,
) {
    for entity in retry_menu_query.iter() {
        commands.entity(entity).despawn();
    }
    *stats = StatsRes::default();
    *score = Score::default();
    audio.stop();
}

pub fn retry_button(
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
