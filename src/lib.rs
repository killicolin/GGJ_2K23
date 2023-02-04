mod components;
mod constants;
mod systems;

use bevy::{
    prelude::{default, App, PluginGroup, SystemSet},
    window::{PresentMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;
use components::{Aim, Alive, Decay, HitCount, Move, Weapon};
use systems::in_game::{
    bullet_hitting_update, bullet_spawner, camera_position_update, clean_in_game, decay,
    despawn_health, despawn_ttl, enemy_direction_update, firing_bullet_emit, key_input_update,
    manage_mob_spawner_timer, mob_spawner, mouse_button_input_update, player_aim_update,
    setup_in_game, transform_update, MobSpawnEvent, SpawnBulletEvent,
};
use systems::main_menu::{clean_main_menu, setup_main_menu, start_button, AppState};

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
    .add_event::<SpawnBulletEvent>()
    .add_event::<MobSpawnEvent>()
    .add_state(AppState::MainMenu)
    .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_main_menu))
    .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(start_button))
    .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(clean_main_menu))
    .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_in_game))
    .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(clean_in_game))
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(player_aim_update)
            .with_system(camera_position_update)
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
            .with_system(bullet_hitting_update),
    );

    app.register_type::<Alive>();
    app.register_type::<Move>();
    app.register_type::<Decay>();
    app.register_type::<HitCount>();
    app.register_type::<Aim>();
    app.register_type::<Weapon>();
    app.register_type::<Move>();
    app.register_type::<Move>();
    if cfg!(debug_assertions) {
        app.add_plugin(EditorPlugin);
    }
    app.run();
}
