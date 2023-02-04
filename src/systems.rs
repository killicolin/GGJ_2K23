use bevy::{
    prelude::{
        Camera2dBundle, Color, Commands, EventReader, EventWriter, Input, KeyCode, MouseButton,
        Query, Res, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::default,
    window::Windows,
};

use crate::components::{
    Aim, Alive, BulletBundle, CharacterBundle, Collider, Harm, Move, Player, PlayerBundle, Weapon,
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
const PLAYER_BULLETS: u32 = 20;

//Bullet const variables
const BULLET_HEALTH: f32 = 1.0;
const BULLETS_SCALE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
const BULLETS_COLOR: Color = Color::rgb(0.8, 0.8, 0.4);
const BULLETS_SPREAD: f32 = 5.0 * std::f32::consts::PI / 180.0;

pub struct SpawnBulletEvent;

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

pub fn firing_bullet_emit(
    mut ev_spawn_bullet: EventWriter<SpawnBulletEvent>,
    query: Query<&Weapon, With<Player>>,
) {
    let weapon = query.single();
    if weapon.is_firing {
        ev_spawn_bullet.send(SpawnBulletEvent);
    }
}

pub fn bullet_spawner(
    mut commands: Commands,
    mut ev_spawn_bullet: EventReader<SpawnBulletEvent>,
    query: Query<(&Transform, &Weapon, &Aim, &Harm), With<Player>>,
) {
    for _ in ev_spawn_bullet.iter() {
        let (player_transform, weapon, aim, harm) = query.single();
        let angle = aim.direction.angle_between(Vec2::new(1.0, 0.0));
        for i in 0..weapon.bullets {
            let bullet_direction = angle + ((i - i / 2) as f32) * BULLETS_SPREAD;
            let direction = Vec2::new(bullet_direction.cos(), bullet_direction.sin());
            commands.spawn(BulletBundle {
                character: CharacterBundle {
                    move_component: Move {
                        speed: weapon.bullet_speed,
                        direction: direction,
                    },
                    harm: Harm {
                        damage: harm.damage,
                    },
                    alive: Alive {
                        health: BULLET_HEALTH,
                    },
                    sprite_bundle: SpriteBundle {
                        transform: Transform {
                            translation: player_transform.translation,
                            scale: BULLETS_SCALE,
                            ..default()
                        },
                        sprite: Sprite {
                            color: BULLETS_COLOR,
                            ..default()
                        },
                        ..default()
                    },
                    collider: Collider {},
                },
            });
        }
    }
}
