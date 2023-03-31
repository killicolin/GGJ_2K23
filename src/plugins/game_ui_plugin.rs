use crate::{
    systems::{
        level_menu::{
            clean_level_menu, decrement_date, down_pannel, heredity_button, setup_level_menu,
        },
        retry_menu::{clean_retry_menu, retry_button, setup_retry_menu},
    },
    AppState,
};
use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate,
    Plugin,
};
pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_retry_menu.in_schedule(OnEnter(AppState::RetryMenu)))
            .add_system(retry_button.in_set(OnUpdate(AppState::RetryMenu)))
            .add_system(clean_retry_menu.in_schedule(OnExit(AppState::RetryMenu)))
            .add_system(setup_level_menu.in_schedule(OnEnter(AppState::LevelMenu)))
            .add_systems(
                (heredity_button, down_pannel, decrement_date)
                    .in_set(OnUpdate(AppState::LevelMenu)),
            )
            .add_system(clean_level_menu.in_schedule(OnExit(AppState::LevelMenu)));
    }
}
