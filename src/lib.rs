mod components;
mod systems;

use bevy::{
    prelude::{default, App, PluginGroup},
    window::{PresentMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;
use components::{Aim, Alive, Decay, HitCount, Move, Weapon};
use systems::{
    bullet_spawner, enemy_direction_update, firing_bullet_emit, key_input_update,
    manage_mob_spawner_timer, mob_spawner, mouse_button_input_update, player_aim_update, setup,
    transform_update, MobSpawnEvent, SpawnBulletEvent,
};

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
    .add_startup_system(setup)
    .add_system(player_aim_update)
    .add_system(mouse_button_input_update)
    .add_system(key_input_update)
    .add_system(transform_update)
    .add_system(firing_bullet_emit)
    .add_system(bullet_spawner)
    .add_system(manage_mob_spawner_timer)
    .add_system(mob_spawner)
    .add_system(enemy_direction_update);

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
