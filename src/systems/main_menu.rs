use bevy::{
    audio::AudioSink,
    prelude::{
        AssetServer, Assets, Audio, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed,
        Children, Color, Commands, Entity, NodeBundle, Query, Res, ResMut, State, TextBundle, With,
    },
    text::TextStyle,
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Size, Style, Val},
    utils::default,
};

use crate::AppState;
use crate::{components::MainMenu, resource::MusicController};

// UI
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("sounds/theme.ogg");
    let handle = audio_sinks.get_handle(audio.play(music));
    commands.insert_resource(MusicController(handle));
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

pub fn clean_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Res<MusicController>,
) {
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn();
    }
    if let Some(sink) = audio_sinks.get(&music_controller.0) {
        sink.stop();
    }
    commands.remove_resource::<MusicController>();
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
