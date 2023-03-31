use crate::{systems::in_game::*, AppState};
use bevy::prelude::{
    App, IntoSystemAppConfig, IntoSystemConfigs, OnEnter, OnExit, OnUpdate, Plugin,
};
pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_in_game.in_schedule(OnEnter(AppState::InGame)))
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
    }
}
