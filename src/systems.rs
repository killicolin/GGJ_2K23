use bevy::{
    prelude::{
        Camera2dBundle, Color, Commands, EventReader, EventWriter, Input, KeyCode, MouseButton,
        OrthographicProjection, Query, Res, Transform, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    utils::default,
    window::Windows,
};
use rand::{thread_rng, Rng};

use crate::components::{
    Aim, Alive, BulletBundle, BulletSpawnerTimer, CharacterBundle, Collider, Enemy, EnemyBundle,
    Harm, MobSpawnerTimer, Move, Player, PlayerBundle, Weapon,
};

// Player starting stats
const PLAYER_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_SPEED: f32 = 100.0;
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

const MOB_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const MOB_SPEED: f32 = 90.0;
const MOB_SPAWN_RADIUS: f32 = 1000.0;
const MOB_DAMAGE: f32 = 1.0;
const MOB_HEALTH: f32 = 1.0;
const MOB_SCALE: Vec3 = Vec3::new(15.0, 15.0, 15.0);

#[derive(Default)]
pub struct MobSpawnEvent;

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
    commands.spawn(MobSpawnerTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
    commands.spawn(BulletSpawnerTimer(Timer::from_seconds(
        PLAYER_FIRE_RATE,
        TimerMode::Repeating,
    )));
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
    mut query_timer: Query<&mut BulletSpawnerTimer>,
) {
    let mut weapon = query.single_mut();
    let mut timer = query_timer.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        weapon.is_firing = true;
        timer.unpause();
    } else if buttons.just_released(MouseButton::Left) {
        weapon.is_firing = false;
        timer.pause();
    }
}

pub fn enemy_direction_update(
    mut query: Query<(&mut Move, &Transform), With<Enemy>>,
    query_player: Query<&Transform, With<Player>>,
) {
    let player_tranform = query_player.single();
    query.for_each_mut(|(mut movable, transform)| {
        movable.direction = (player_tranform.translation - transform.translation)
            .truncate()
            .normalize_or_zero();
    });
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
    time: Res<Time>,
    mut ev_spawn_bullet: EventWriter<SpawnBulletEvent>,
    query: Query<&Weapon, With<Player>>,
    mut query_timer: Query<&mut BulletSpawnerTimer>,
) {
    let weapon = query.single();
    let mut timer = query_timer.single_mut();
    if weapon.is_firing && timer.tick(time.delta()).just_finished() {
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

// Spawn a mob event every seconds
pub fn manage_mob_spawner_timer(
    time: Res<Time>,
    mut query: Query<&mut MobSpawnerTimer>,
    mut mob_spawn_event: EventWriter<MobSpawnEvent>,
) {
    let mut timer = query.single_mut();
    if timer.tick(time.delta()).just_finished() {
        mob_spawn_event.send(MobSpawnEvent);
    }
}

// Spawn the mob facing towards the player
pub fn mob_spawner(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    mob_spawn_event: EventReader<MobSpawnEvent>,
) {
    if mob_spawn_event.is_empty() {
        return;
    }
    mob_spawn_event.clear();
    let x: f32 = thread_rng().gen_range(-MOB_SPAWN_RADIUS..MOB_SPAWN_RADIUS) as f32;
    let y: f32 = thread_rng().gen_range(-MOB_SPAWN_RADIUS..MOB_SPAWN_RADIUS) as f32;

    let player = query.single();

    let mob_spawn_position = Vec3 {
        x: player.translation.x + x,
        y: player.translation.y + y,
        z: player.translation.z,
    };
    // Spawn
    commands.spawn(EnemyBundle {
        character: CharacterBundle {
            move_component: Move {
                speed: MOB_SPEED,
                direction: (player.translation - mob_spawn_position)
                    .truncate()
                    .normalize(),
            },
            harm: Harm { damage: MOB_DAMAGE },
            alive: Alive { health: MOB_HEALTH },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: mob_spawn_position,
                    scale: MOB_SCALE,
                    ..default()
                },
                sprite: Sprite {
                    color: MOB_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        },
        enemy: Enemy,
    });
}
