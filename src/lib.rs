mod components;
mod constants;
mod plugins;
mod resource;
mod systems;

use bevy::{
    prelude::{default, App, Color, PluginGroup, Resource, States},
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
use systems::in_game::{
    CreateMapEvent, GameOverEvent, MobSpawnEvent, SpawnBulletEvent, WaveDoneEvent,
};

use plugins::{
    game_loop::GameLoopPlugin, game_ui_plugin::GameUIPlugin, launcher_ui_plugin::LauncherUiPlugin,
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
    .add_plugin(LauncherUiPlugin)
    .add_plugin(GameUIPlugin)
    .add_plugin(GameLoopPlugin);

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
