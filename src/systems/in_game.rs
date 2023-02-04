use bevy::{
    prelude::{
        Camera, Camera2dBundle, Commands, Entity, EventReader, EventWriter, Input, KeyCode,
        MouseButton, OrthographicProjection, Query, Res, Transform, Vec2, Vec3, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    utils::default,
    window::Windows,
};
use rand::{thread_rng, Rng};

use crate::{
    components::{
        Aim, Alive, BulletBundle, BulletSpawnerTimer, CharacterBundle, Collider, Decay, Enemy,
        EnemyBundle, Harm, HitCount, InGame, MobSpawnerTimer, Move, Player, PlayerBundle, Weapon,
    },
    constants::{
        BULLETS_COLOR, BULLETS_SCALE, BULLETS_SPREAD, BULLET_HEALTH, MOB_COLOR, MOB_DAMAGE,
        MOB_HEALTH, MOB_SCALE, MOB_SPAWN_RADIUS, MOB_SPEED, PLAYER_AIM, PLAYER_BULLETS,
        PLAYER_BULLETS_SPEED, PLAYER_BULLETS_TTL, PLAYER_COLOR, PLAYER_DAMAGE, PLAYER_DIRECTION,
        PLAYER_FIRE_RATE, PLAYER_HEALTH, PLAYER_POSITION, PLAYER_SCALE, PLAYER_SPEED,
    },
};

pub struct SpawnBulletEvent;

#[derive(Default)]
pub struct MobSpawnEvent;

pub fn setup_in_game(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), InGame));

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
            in_game: InGame,
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

pub fn clean_in_game(mut commands: Commands, in_game_query: Query<Entity, With<InGame>>) {
    for entity in in_game_query.iter() {
        commands.entity(entity).despawn();
    }
}

//todo, fix the player direction
pub fn player_aim_update(
    windows: Res<Windows>,
    mut query: Query<(&Transform, &mut Aim), With<Player>>,
    query_camera: Query<(&Transform, &OrthographicProjection)>,
) {
    let window = windows.get_primary().unwrap();
    let (player_transform, mut player_aim) = query.single_mut();
    let (transform, projection) = query_camera.iter().last().unwrap();
    if let Some(position) = window.cursor_position() {
        player_aim.direction = Vec2::new(
            (position.x + transform.translation.x + projection.left)
                - player_transform.translation.x,
            (position.y + transform.translation.y + projection.bottom)
                - player_transform.translation.y,
        )
        .normalize_or_zero()
    }
}

pub fn camera_position_update(
    mut query_camera: Query<&mut Transform, With<OrthographicProjection>>,
    query: Query<&Transform, (With<Player>, Without<OrthographicProjection>)>,
) {
    let player_transform = query.single();
    query_camera.for_each_mut(|mut camera_transform| {
        let pos = (player_transform.translation).truncate();
        camera_transform.translation = Vec3::new(pos.x, pos.y, 999.0);
    });
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
            let i_f32 = i as f32;
            let i_div2_f32 = (weapon.bullets / 2) as f32;
            let offset = i_f32 - i_div2_f32;
            let bullet_direction = angle + offset * BULLETS_SPREAD;
            let direction = Vec2::new(bullet_direction.cos(), -bullet_direction.sin());
            commands.spawn(BulletBundle {
                character: CharacterBundle {
                    in_game: InGame,
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
            in_game: InGame,
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

pub fn despawn_health(mut commands: Commands, mut query: Query<(Entity, &Alive)>) {
    for (entity, alive) in query.iter_mut() {
        if alive.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn decay(mut query: Query<(&Decay, &mut Alive)>) {
    for (decay, mut alive) in query.iter_mut() {
        alive.health -= decay.amount;
    }
}

pub fn despawn_ttl(mut commands: Commands, mut query: Query<(Entity, &HitCount)>) {
    for (entity, hit_count) in query.iter_mut() {
        if hit_count.ttl <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
