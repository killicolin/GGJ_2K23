use bevy::{
    prelude::{default, App, PluginGroup},
    window::{PresentMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_editor_pls::EditorPlugin;

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
    }));
    if cfg!(debug_assertions) {
        app.add_plugin(EditorPlugin);
    }
    app.run();
}
