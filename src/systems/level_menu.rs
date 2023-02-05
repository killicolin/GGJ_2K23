use std::cmp::max;

use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, ChildBuilder,
        Children, Color, Commands, Entity, NodeBundle, Query, Res, ResMut, State, TextBundle, With,
    },
    text::{Text, TextStyle},
    time::Time,
    ui::{
        AlignItems, AlignSelf, BackgroundColor, FlexDirection, Interaction, JustifyContent,
        PositionType, Size, Style, UiImage, UiRect, Val,
    },
    utils::default,
};
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{DateText, DebufChoices, LevelMenu, LevelMenuPannel},
    constants::{BEGIN_DATE, LORE_PARENT_CHOICE},
    resource::Score,
    StatsRes,
};
use crate::{
    components::{Debuff, PlayerColor},
    AppState,
};

// UI
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVER_BUTTON: Color = Color::rgb(0.30, 0.30, 0.30);
const DATE_COLOR_TEXT: Color = Color::rgb(1.0, 0.0, 0.0);
const WHITE_TEXT: Color = Color::rgb(0.8, 0.8, 0.8);
const BACKGROUND_COLOR_UI: Color = Color::rgb(0.65, 0.65, 0.65);
const PANNEL_SPEED: f32 = 100.0;
const DATE_SPEED: f32 = 30.0;
fn heredity_button_layout(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    debuf: Debuff,
    color: Color,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Px(100.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            LevelMenu,
            PlayerColor(color),
            debuf.clone(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Flaws :\n"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.8, 0.3),
                    },
                ),
                LevelMenu,
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{debuf}"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.3, 0.3),
                    },
                ),
                LevelMenu,
            ));
        });
}

fn heredity_sprite_layout(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) -> Color {
    let color = Color::rgb(
        thread_rng().gen_range(0.0..1.0),
        thread_rng().gen_range(0.0..1.0),
        thread_rng().gen_range(0.0..1.0),
    );

    parent.spawn((
        ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            image: UiImage(asset_server.load("images/sprite.png")),
            background_color: BackgroundColor(color.clone()),
            ..default()
        },
        LevelMenu,
    ));
    return color;
}

fn heredity_layout(
    asset_server: &Res<AssetServer>,
    parent: &mut ChildBuilder,
    debuf: Debuff,
    parent_name: &str,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(35.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                background_color: BACKGROUND_COLOR_UI.into(),
                ..default()
            },
            LevelMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(95.0), Val::Percent(10.0)),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceAround,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    LevelMenu,
                ))
                .with_children(|background_title| {
                    background_title.spawn((
                        TextBundle::from_section(
                            format!("{parent_name}"),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: WHITE_TEXT,
                            },
                        ),
                        LevelMenu,
                    ));
                });
        })
        .with_children(|parent| {
            let color = heredity_sprite_layout(&asset_server, parent);
            heredity_button_layout(&asset_server, parent, debuf, color);
        });
}

pub fn setup_level_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), LevelMenu));
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            LevelMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(70.0), Val::Percent(70.0)),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceAround,
                            position: UiRect {
                                top: Val::Percent(-83.0),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    LevelMenu,
                    LevelMenuPannel,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceAround,
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                ..default()
                            },
                            LevelMenu,
                        ))
                        .with_children(|date_place| {
                            date_place.spawn((
                                TextBundle::from_section(
                                    format!("{}", BEGIN_DATE),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: DATE_COLOR_TEXT,
                                    },
                                ),
                                LevelMenu,
                                DateText,
                            ));
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceAround,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            },
                            LevelMenu,
                        ))
                        .with_children(|parent| {
                            content_layout(&asset_server, parent);
                        });
                });
        });
}

pub fn content_layout(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    let (debuf_mom, debuf_dad) = Debuff::get_parent_random();
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Percent(20.0)),
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            LevelMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{LORE_PARENT_CHOICE}"),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: WHITE_TEXT,
                    },
                ),
                LevelMenu,
            ));
        });
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(90.0), Val::Percent(75.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            LevelMenu,
        ))
        .with_children(|parent| {
            heredity_layout(&asset_server, parent, debuf_dad, "Dad");
            heredity_layout(&asset_server, parent, debuf_mom, "Mom");
        });
    parent.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            //background_color: Color::YELLOW.into(),
            ..default()
        },
        LevelMenu,
    ));
}

pub fn clean_level_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<LevelMenu>>,
    score: Res<Score>,
    audio: Res<Audio>,
) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
    if score.should_start_music() {
        audio.stop();
    }
}

pub fn down_pannel(time: Res<Time>, mut query_panel: Query<&mut Style, With<LevelMenuPannel>>) {
    let mut panel = query_panel.single_mut();
    if let Val::Percent(y) = panel.position.top {
        let new_value = y + PANNEL_SPEED * time.delta_seconds();
        if new_value > 0.0 {
            panel.position.top = Val::Undefined;
        } else {
            panel.position.top = Val::Percent(new_value);
        }
    }
}

pub fn decrement_date(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut query_panel: Query<&mut Text, With<DateText>>,
) {
    let mut text = query_panel.single_mut();
    score.decrease(DATE_SPEED * time.delta_seconds());
    text.sections[0].value = score.to_text();
}

pub fn heredity_button(
    mut app_state: ResMut<State<AppState>>,
    mut stats: ResMut<StatsRes>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
            &Debuff,
            &PlayerColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut button_color, _, debuf, color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                debuf
                    .get_defaults()
                    .iter()
                    .for_each(|debuff_choice| match debuff_choice {
                        DebufChoices::Speed => stats.player_speed *= 0.8,
                        DebufChoices::Bullets => {
                            stats.player_bullets = max(stats.player_bullets / 2, 1)
                        }
                        DebufChoices::BulletsTtl => {
                            stats.player_bullets_ttl = max(stats.player_bullets_ttl / 2, 1)
                        }
                        DebufChoices::Damage => stats.player_damage *= 0.7,
                        DebufChoices::BulletsSpeed => {
                            stats.player_bullets_speed *= 0.6;
                            stats.player_decay *= 0.6;
                        }
                        DebufChoices::FireRate => stats.player_fire_rate *= 1.3,
                    });
                stats.player_color = color.0;
                app_state.set(AppState::InGame).unwrap();
                *button_color = NORMAL_BUTTON.into()
            }
            Interaction::Hovered => *button_color = HOVER_BUTTON.into(),
            Interaction::None => *button_color = NORMAL_BUTTON.into(),
        }
    }
}
