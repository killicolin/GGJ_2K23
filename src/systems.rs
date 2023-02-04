use bevy::{
    prelude::{
        Camera2dBundle, Color, Commands, Input, KeyCode, MouseButton, Query, Res, Transform, Vec2,
        Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::default,
    window::Windows,
};

use crate::components::{
    Aim, Alive, CharacterBundle, Collider, Harm, Move, Player, PlayerBundle, Weapon,
};

// Player starting stats
const PLAYER_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_SPEED: f32 = 20.0;
const PLAYER_DAMAGE: f32 = 1.0;
const PLAYER_HEALTH: f32 = 1.0;
const PLAYER_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SCALE: Vec3 = Vec3::new(10.0, 10.0, 10.0);
const PLAYER_AIM: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PLAYER_FIRE_RATE: f32 = 1.0;
const PLAYER_BULLETS_SPEED: f32 = 30.0;
const PLAYER_BULLETS_TTL: u32 = 1;
const PLAYER_BULLETS: u32 = 1;

pub fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Spawn player
    commands.spawn(PlayerBundle {
        character: CharacterBundle {
            move_component: Move {
                speed: PLAYER_SPEED,
                direction: PLAYER_DIRECTION,
            },
            harm: Harm {
                damage: PLAYER_DAMAGE,
            },
            alive: Alive {
                health: PLAYER_HEALTH,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: PLAYER_POSITION,
                    scale: PLAYER_SCALE,
                    ..default()
                },
                sprite: Sprite {
                    color: PLAYER_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        },
        player: Player,
        weapon: Weapon {
            fire_rate: PLAYER_FIRE_RATE,
            bullet_ttl: PLAYER_BULLETS_TTL,
            bullet_speed: PLAYER_BULLETS_SPEED,
            bullets: PLAYER_BULLETS,
            is_firing: false,
        },
        aim: Aim {
            direction: PLAYER_AIM,
        },
    });
}

//todo, fix the player direction
pub fn player_aim_update(
    windows: Res<Windows>,
    mut query: Query<(&Transform, &mut Aim), With<Player>>,
) {
    let window = windows.get_primary().unwrap();
    let (player_transform, mut player_aim) = query.single_mut();
    if let Some(position) = window.cursor_position() {
        player_aim.direction = Vec2::new(
            position.x - player_transform.translation.x,
            position.y - player_transform.translation.y,
        )
    }
}

pub fn mouse_button_input_update(
    buttons: Res<Input<MouseButton>>,
    mut query: Query<&mut Weapon, With<Player>>,
) {
    let mut weapon = query.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        weapon.is_firing = true;
    } else if buttons.just_released(MouseButton::Left) {
        weapon.is_firing = false;
    }
}

pub fn key_input_update(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Move, With<Player>>,
) {
    let mut move_player = query.single_mut();
    move_player.direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        move_player.direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        move_player.direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        move_player.direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        move_player.direction.y += 1.0;
    }

    if let Some(new_value) = move_player.direction.try_normalize() {
        move_player.direction = new_value;
    }
}

pub fn transform_update(time: Res<Time>, mut query: Query<(&mut Transform, &Move)>) {
    query.for_each_mut(|(mut transform, movable)| {
        transform.translation.x += movable.direction.x * movable.speed * time.delta_seconds();
        transform.translation.y += movable.direction.y * movable.speed * time.delta_seconds();
    });
}

// fn firing_update(time: Res<Time>, query: Query<(&mut Transform, &Move)>)
