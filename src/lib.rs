mod components;
mod constants;
mod resource;
mod systems;

use bevy::{
    prelude::{
        default, App, Color, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter,
        OnExit, OnUpdate, PluginGroup, Resource, States,
    },
    window::{PresentMode, Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;
use bevy_kira_audio::AudioPlugin;
use components::{Aim, Alive, Decay, HitCount, Move, Weapon};
use constants::{
    BULLETS_DECAYS, PLAYER_BULLETS, PLAYER_BULLETS_SPEED, PLAYER_BULLETS_TTL, PLAYER_COLOR,
    PLAYER_DAMAGE, PLAYER_FIRE_RATE, PLAYER_HEALTH, PLAYER_SPEED,
};
use resource::{ChunksMap, LastShot, Score, TotalKilled, TotalSpawned, TotalToSpawn};
use std::collections::HashMap;
use systems::{
    in_game::{
        animate_sprite, bullet_hitting_update, bullet_spawner, camera_position_update,
        change_level, clean_in_game, decay, despawn_health, despawn_ttl, enemy_direction_update,
        enemy_hitting_update, firing_bullet_emit, game_over, key_input_update, load_chunks,
        make_map, manage_mob_spawner_timer, mob_spawner, mouse_button_input_update,
        player_aim_update, setup_in_game, transform_update, wave_is_done_emit, CreateMapEvent,
        GameOverEvent, MobSpawnEvent, SpawnBulletEvent, WaveDoneEvent,
    },
    level_menu::{
        clean_level_menu, decrement_date, down_pannel, heredity_button, setup_level_menu,
    },
    main_menu::{clean_main_menu, setup_main_menu, start_button},
    prestart_menu::{clean_pre_start_menu, ingame_button, setup_pre_start_menu},
    retry_menu::{clean_retry_menu, retry_button, setup_retry_menu},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    RetryMenu,
    LevelMenu,
    PreStartMenu,
}

#[derive(Resource)]
pub struct StatsRes {
    pub player_speed: f32,
    pub player_damage: f32,
    pub player_health: f32,
    pub player_fire_rate: f32,
    pub player_bullets: u32,
    pub player_bullets_ttl: i32,
    pub player_bullets_speed: f32,
    pub player_decay: f32,
    pub player_color: Color,
}
impl Default for StatsRes {
    fn default() -> Self {
        StatsRes {
            player_speed: PLAYER_SPEED,
            player_damage: PLAYER_DAMAGE,
            player_health: PLAYER_HEALTH,
            player_fire_rate: PLAYER_FIRE_RATE,
            player_bullets: PLAYER_BULLETS,
            player_bullets_ttl: PLAYER_BULLETS_TTL,
            player_bullets_speed: PLAYER_BULLETS_SPEED,
            player_decay: BULLETS_DECAYS,
            player_color: PLAYER_COLOR,
        }
    }
}
pub fn run(width: f32, height: f32) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "BACK TO THE ROOTS".to_string(),
            resolution: WindowResolution::new(width, height),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(AudioPlugin)
    .insert_resource(TotalToSpawn::default())
    .insert_resource(TotalSpawned::default())
    .insert_resource(TotalKilled::default())
    .insert_resource(LastShot::default())
    .insert_resource(Score::default())
    .insert_resource(ChunksMap {
        chunks: HashMap::new(),
    })
    .add_event::<SpawnBulletEvent>()
    .add_event::<MobSpawnEvent>()
    .add_event::<GameOverEvent>()
    .add_event::<WaveDoneEvent>()
    .add_event::<CreateMapEvent>()
    // To change to AppState::MainMenu when loop is finished
    .add_state::<AppState>()
    .init_resource::<StatsRes>()
    //
    .add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
    .add_system(start_button.in_set(OnUpdate(AppState::MainMenu)))
    .add_system(clean_main_menu.in_schedule(OnExit(AppState::MainMenu)))
    //
    .add_system(setup_pre_start_menu.in_schedule(OnEnter(AppState::PreStartMenu)))
    .add_system(ingame_button.in_set(OnUpdate(AppState::PreStartMenu)))
    .add_system(clean_pre_start_menu.in_schedule(OnExit(AppState::PreStartMenu)))
    //
    .add_system(setup_retry_menu.in_schedule(OnEnter(AppState::RetryMenu)))
    .add_system(retry_button.in_set(OnUpdate(AppState::RetryMenu)))
    .add_system(clean_retry_menu.in_schedule(OnExit(AppState::RetryMenu)))
    //
    .add_system(setup_level_menu.in_schedule(OnEnter(AppState::LevelMenu)))
    .add_systems(
        (heredity_button, down_pannel, decrement_date).in_set(OnUpdate(AppState::LevelMenu)),
    )
    .add_system(clean_level_menu.in_schedule(OnExit(AppState::LevelMenu)))
    //
    .add_system(setup_in_game.in_schedule(OnEnter(AppState::InGame)))
    .add_systems(
        (
            player_aim_update,
            camera_position_update,
            make_map,
            load_chunks,
            mouse_button_input_update,
            key_input_update,
            transform_update,
            firing_bullet_emit,
            bullet_spawner,
            manage_mob_spawner_timer,
            enemy_direction_update,
            mob_spawner,
            despawn_health,
            despawn_ttl,
            decay,
        )
            .in_set(OnUpdate(AppState::InGame)),
    )
    .add_systems(
        (
            bullet_hitting_update,
            enemy_hitting_update,
            wave_is_done_emit,
            change_level,
            animate_sprite,
            game_over,
        )
            .in_set(OnUpdate(AppState::InGame)),
    )
    .add_system(clean_in_game.in_schedule(OnExit(AppState::InGame)));
    //

    app.register_type::<Alive>();
    app.register_type::<Move>();
    app.register_type::<Decay>();
    app.register_type::<HitCount>();
    app.register_type::<Aim>();
    app.register_type::<Weapon>();
    if cfg!(debug_assertions) {
        app.add_plugin(EditorPlugin);
    }
    app.run();
}
