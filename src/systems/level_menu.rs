use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, ChildBuilder,
        Children, Color, Commands, Entity, NodeBundle, Query, Res, ResMut, State, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Size, Style,
        UiImage, Val,
    },
    utils::default,
};
use rand::{thread_rng, Rng};

use crate::{
    components::{DebufChoices, LevelMenu},
    StatsRes,
};
use crate::{
    components::{Debuff, PlayerColor},
    AppState,
};

// UI
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

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
                    size: Size::new(Val::Px(370.0), Val::Px(65.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
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

fn heredity_layout(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, debuf: Debuff) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(70.0), Val::Percent(70.0)),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            LevelMenu,
        ))
        .with_children(|parent| {
            let color = heredity_sprite_layout(&asset_server, parent);
            heredity_button_layout(&asset_server, parent, debuf, color);
        });
}

pub fn setup_level_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (debuf_mom, debuf_dad) = Debuff::get_parent_random();

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
                            ..default()
                        },
                        ..default()
                    },
                    LevelMenu,
                ))
                .with_children(|parent| {
                    heredity_layout(&asset_server, parent, debuf_dad);
                    heredity_layout(&asset_server, parent, debuf_mom);
                });
        });
}

pub fn clean_level_menu(mut commands: Commands, main_menu_query: Query<Entity, With<LevelMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn heredity_button(
    mut app_state: ResMut<State<AppState>>,
    mut stats: ResMut<StatsRes>,
    mut interaction_query: Query<
        (
            &Interaction,
            &BackgroundColor,
            &Children,
            &Debuff,
            &PlayerColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, _, _, debuf, color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                debuf
                    .get_defaults()
                    .iter()
                    .for_each(|debuff_choice| match debuff_choice {
                        DebufChoices::Speed => stats.player_speed /= 2.0,
                        DebufChoices::Bullets => stats.player_bullets /= 2,
                        DebufChoices::BulletsTtl => stats.player_bullets_ttl /= 2,
                        DebufChoices::Damage => stats.player_damage /= 2.0,
                        DebufChoices::BulletsSpeed => stats.player_bullets_speed /= 2.0,
                        DebufChoices::FireRate => stats.player_fire_rate /= 2.0,
                    });
                stats.player_color = color.0;
                app_state.set(AppState::InGame).unwrap();
            }
            _ => {}
        }
    }
}
