use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Children, Color,
        Commands, Entity, NodeBundle, Query, Res, ResMut, State, TextBundle, With,
    },
    text::TextStyle,
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Size, Style, Val},
    utils::default,
};

use crate::components::MainMenu;
use crate::AppState;

// UI
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn((Camera2dBundle::default(), MainMenu));
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
            MainMenu,
        ))
        .with_children(|parent| {
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
                    MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        MainMenu,
                    ));
                });
        });
}

pub fn clean_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn start_button(
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
