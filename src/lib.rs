mod components;
mod constants;
mod resource;
mod systems;

use bevy::{
    prelude::{default, App, Color, PluginGroup, Resource, SystemSet},
    window::{PresentMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;
use bevy_kira_audio::AudioPlugin;
use components::{Aim, Alive, Decay, HitCount, Move, Weapon};
use constants::{
    PLAYER_BULLETS, PLAYER_BULLETS_SPEED, PLAYER_BULLETS_TTL, PLAYER_COLOR, PLAYER_DAMAGE,
    PLAYER_FIRE_RATE, PLAYER_HEALTH, PLAYER_SPEED,
};
use resource::{ChunksMap, LastShot, Score, TotalKilled, TotalSpawned, TotalToSpawn};
use std::collections::HashMap;
use systems::{
    in_game::{
        animate_sprite, bullet_hitting_update, bullet_spawner, camera_position_update,
        change_level, clean_in_game, decay, despawn_health, despawn_ttl, enemy_direction_update,
        enemy_hitting_update, firing_bullet_emit, game_over, key_input_update, load_chunks,
        make_map, manage_mob_spawner_timer, mob_spawner, mouse_button_input_update,
        player_aim_update, setup_in_game, transform_update, wave_is_done_emit, GameOverEvent,
        MobSpawnEvent, SpawnBulletEvent, WaveDoneEvent,
    },
    level_menu::{
        clean_level_menu, decrement_date, down_pannel, heredity_button, setup_level_menu,
    },
    main_menu::{clean_main_menu, setup_main_menu, start_button},
    prestart_menu::{clean_pre_start_menu, ingame_button, setup_pre_start_menu},
    retry_menu::{clean_retry_menu, retry_button, setup_retry_menu},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
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
            player_color: PLAYER_COLOR,
        }
    }
}
pub fn run(width: f32, height: f32) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "GAME NAME".to_string(),
            width,
            height,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
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
    // To change to AppState::MainMenu when loop is finished
    .add_state(AppState::MainMenu)
    .init_resource::<StatsRes>()
    .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
    .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(start_button))
    .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(clean_main_menu))
    .add_system_set(SystemSet::on_enter(AppState::PreStartMenu).with_system(setup_pre_start_menu))
    .add_system_set(SystemSet::on_update(AppState::PreStartMenu).with_system(ingame_button))
    .add_system_set(SystemSet::on_exit(AppState::PreStartMenu).with_system(clean_pre_start_menu))
    .add_system_set(SystemSet::on_enter(AppState::RetryMenu).with_system(setup_retry_menu))
    .add_system_set(SystemSet::on_update(AppState::RetryMenu).with_system(retry_button))
    .add_system_set(SystemSet::on_exit(AppState::RetryMenu).with_system(clean_retry_menu))
    .add_system_set(SystemSet::on_enter(AppState::LevelMenu).with_system(setup_level_menu))
    .add_system_set(
        SystemSet::on_update(AppState::LevelMenu)
            .with_system(heredity_button)
            .with_system(down_pannel)
            .with_system(decrement_date),
    )
    .add_system_set(SystemSet::on_exit(AppState::LevelMenu).with_system(clean_level_menu))
    .add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(setup_in_game)
            .with_system(make_map),
    )
    .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(clean_in_game))
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(player_aim_update)
            .with_system(camera_position_update)
            .with_system(load_chunks)
            .with_system(mouse_button_input_update)
            .with_system(key_input_update)
            .with_system(transform_update)
            .with_system(firing_bullet_emit)
            .with_system(bullet_spawner)
            .with_system(manage_mob_spawner_timer)
            .with_system(enemy_direction_update)
            .with_system(mob_spawner)
            .with_system(despawn_health)
            .with_system(despawn_ttl)
            .with_system(decay)
            .with_system(bullet_hitting_update)
            .with_system(enemy_hitting_update)
            .with_system(wave_is_done_emit)
            .with_system(change_level)
            .with_system(animate_sprite)
            .with_system(game_over),
    );

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
