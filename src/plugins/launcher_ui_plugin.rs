use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

use crate::{
    systems::{
        main_menu::{clean_main_menu, setup_main_menu, start_button},
        prestart_menu::{clean_pre_start_menu, ingame_button, setup_pre_start_menu},
    },
    AppState,
};

pub struct LauncherUiPlugin;

impl Plugin for LauncherUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(start_button.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(clean_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(setup_pre_start_menu.in_schedule(OnEnter(AppState::PreStartMenu)))
            .add_system(ingame_button.in_set(OnUpdate(AppState::PreStartMenu)))
            .add_system(clean_pre_start_menu.in_schedule(OnExit(AppState::PreStartMenu)));
    }
}
