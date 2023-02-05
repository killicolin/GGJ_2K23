use bevy::{
    prelude::{
        AssetServer, Assets, Camera2dBundle, Color, Commands, Entity, EventReader, EventWriter,
        Handle, Input, KeyCode, MouseButton, OrthographicProjection, Query, Res, ResMut, State,
        Transform, Vec2, Vec3, Vec4, With, Without,
    },
    sprite::{
        collide_aabb::collide, Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas,
        TextureAtlasSprite,
    },
    time::{Time, Timer, TimerMode},
    utils::default,
    window::Windows,
};
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{
        Aim, Alive, AnimationTimer, Bullet, BulletBundle, CharacterBundle, Chunk, Collider, Decay,
        Enemy, EnemyBundle, Harm, HitCount, InGame, MobSpawnerTimer, Move, Player, PlayerBundle,
        Weapon,
    },
    constants::{
        BULLETS_COLOR, BULLETS_DECAYS, BULLETS_SCALE, BULLETS_SPREAD, BULLET_HEALTH, BULLET_TTL,
        MAP_SCALE, MOB_COLOR, MOB_COLOR_HURT, MOB_DAMAGE, MOB_HEALTH, MOB_SCALE, MOB_SPAWN_RADIUS,
        MOB_SPEED, PLAYER_AIM, PLAYER_DIRECTION, PLAYER_POSITION, PLAYER_SCALE, SPEED_SPAWN_BY_LVL,
    },
    resource::{ChunkType, ChunksMap, LastShot, Score, TotalKilled, TotalSpawned, TotalToSpawn},
    AppState, StatsRes,
};

pub struct SpawnBulletEvent;

pub struct WaveDoneEvent;

pub struct GameOverEvent;

pub struct CreateMapEvent;

#[derive(Default)]
pub struct MobSpawnEvent;

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Move,
        &Handle<TextureAtlas>,
        Option<&Player>,
    )>,
) {
    for (mut timer, mut sprite, move_component, texture_atlas_handle, maybe_player) in &mut query {
        if (move_component.direction != Vec2 { x: 0.0, y: 0.0 }) {
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                if maybe_player.is_some() {
                    sprite.index = (sprite.index + 1) % (texture_atlas.len() / 2);
                    if move_component.direction.y > 0.0 {
                        sprite.index += 6;
                    }
                } else {
                    sprite.index = (sprite.index + 1) % texture_atlas.len();
                }
            }
        } else {
            sprite.index = 0;
        }
    }
}

pub fn setup_in_game(
    mut commands: Commands,
    stats: Res<StatsRes>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    score: Res<Score>,
    audio: Res<Audio>,
    mut create_map_event: EventWriter<CreateMapEvent>,
    query_chunks: Query<Entity, With<Chunk>>,
    mut chunk_map_resource: ResMut<ChunksMap>,
) {
    if score.should_start_music() {
        let nb_music = score.historic_period_theme();
        let music = asset_server.load(format!("sounds/in_game_{nb_music}.ogg"));
        audio.play(music);
        for chunk in query_chunks.iter() {
            commands.entity(chunk).despawn();
        }
        chunk_map_resource.chunks.clear();
    }
    // Camera
    commands.spawn((Camera2dBundle::default(), InGame));
    let texture_handle = asset_server.load("images/atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    // Spawn player
    commands.spawn((
        PlayerBundle {
            character: CharacterBundle {
                move_component: Move {
                    speed: stats.player_speed,
                    direction: PLAYER_DIRECTION,
                },
                harm: Harm {
                    damage: stats.player_damage,
                },
                alive: Alive {
                    health: stats.player_health,
                },
                collider: Collider,
                in_game: InGame,
            },
            sprite_bundle: SpriteSheetBundle {
                transform: Transform {
                    translation: PLAYER_POSITION,
                    scale: PLAYER_SCALE,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    color: stats.player_color,
                    index: 0,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            player: Player,
            weapon: Weapon {
                fire_rate: stats.player_fire_rate,
                bullet_ttl: stats.player_bullets_ttl,
                bullet_speed: stats.player_bullets_speed,
                bullets: stats.player_bullets,
                is_firing: false,
            },
            aim: Aim {
                direction: PLAYER_AIM,
            },
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    commands.spawn((
        MobSpawnerTimer(Timer::from_seconds(
            SPEED_SPAWN_BY_LVL[0],
            TimerMode::Repeating,
        )),
        InGame,
    ));
    create_map_event.send(CreateMapEvent);
}

pub fn clean_in_game(mut commands: Commands, in_game_query: Query<Entity, With<InGame>>) {
    for entity in in_game_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn load_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chunk_map_resource: ResMut<ChunksMap>,
    query_camera: Query<(&Transform, &OrthographicProjection)>,
    score: Res<Score>,
) {
    // Get camera chunk position
    let (camera_transform, ortho) = query_camera.iter().last().unwrap();
    let x = camera_transform.translation.x;
    let y = camera_transform.translation.y;

    let chunk_min_x = ((x + ortho.left) / 256.0).floor() as i32;
    let chunk_max_x = ((x + ortho.right) / 256.0).floor() as i32;
    let chunk_min_y = ((y + ortho.bottom) / 256.0).floor() as i32;
    let chunk_max_y = ((y + ortho.top) / 256.0).floor() as i32;
    // Check every chunk around the camera
    (chunk_min_x - 2..chunk_max_x + 2).for_each(|i| {
        (chunk_min_y - 2..chunk_max_y + 2).for_each(|j| {
            // If chunk is not loaded, load it
            let chunk_type = chunk_map_resource.chunks.get(&(i, j));
            match chunk_type {
                Some(_chunk_type) => {}
                None => {
                    // Load chunk
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform {
                                translation: Vec3 {
                                    x: i as f32 * 256.0,
                                    y: j as f32 * 256.0,
                                    z: 0.0,
                                },
                                scale: MAP_SCALE,
                                ..default()
                            },
                            sprite: Sprite { ..default() },
                            texture: asset_server.load(format!(
                                "images/map_chunk_{}.png",
                                score.historic_period_theme()
                            )),
                            ..default()
                        },
                        Chunk,
                    ));
                    chunk_map_resource.chunks.insert((i, j), ChunkType::Basic);
                }
            }
        })
    })

    // If adjacent chunks are not loaded, load them
}

pub fn make_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_camera: Query<(&Transform, &OrthographicProjection)>,
    create_map_event: EventReader<CreateMapEvent>,
    score: Res<Score>,
) {
    if create_map_event.is_empty() {
        return;
    }
    create_map_event.clear();
    // Determine camera chunk position (chunk is 256x256 pixels image)
    let (camera_transform, orth) = query_camera.iter().last().unwrap();
    let x = camera_transform.translation.x;
    let y = camera_transform.translation.y;

    let chunk_x = (x / 256.0).floor();
    let chunk_y = (y / 256.0).floor();

    let n_chunks_to_make_horizontal = (orth.right / 256.0) as i32 + 2;
    let n_chunks_to_make_vertical = (orth.top / 256.0) as i32 + 2;

    let texture = asset_server.load(format!(
        "images/map_chunk_{}.png",
        score.historic_period_theme()
    ));
    let chunks = (-2..n_chunks_to_make_horizontal)
        .map(move |i| {
            let texture = texture.clone();
            (-2..n_chunks_to_make_vertical).map(move |j| {
                (
                    SpriteBundle {
                        transform: Transform {
                            translation: Vec3 {
                                x: i as f32 * 256.0,
                                y: j as f32 * 256.0,
                                z: 0.0,
                            },
                            scale: PLAYER_SCALE,
                            ..default()
                        },
                        sprite: Sprite { ..default() },
                        texture: texture.clone(),
                        ..default()
                    },
                    Chunk,
                )
            })
        })
        .flatten();
    commands.spawn_batch(chunks);

    print!("Camera position: ({chunk_x},{chunk_y})");
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
    time: Res<Time>,
    mut query_camera: Query<&mut Transform, With<OrthographicProjection>>,
    query: Query<&Transform, (With<Player>, Without<OrthographicProjection>)>,
) {
    let player_transform = query.single();
    query_camera.for_each_mut(|mut camera_transform| {
        let offset =
            (player_transform.translation).truncate() - camera_transform.translation.truncate();
        let direction = offset.normalize_or_zero();
        let magnitude = offset.distance(Vec2::ZERO);
        let result = direction * magnitude * 5.5 * time.delta_seconds();
        camera_transform.translation += Vec3::new(result.x, result.y, 0.0);
    });
}

pub fn mouse_button_input_update(
    time: Res<Time>,
    mut last_shot: ResMut<LastShot>,
    mut ev_spawn_bullet: EventWriter<SpawnBulletEvent>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<&mut Weapon, With<Player>>,
) {
    let mut weapon = query.single_mut();
    if buttons.just_pressed(MouseButton::Left) {
        let now = time.elapsed_seconds();
        if now - last_shot.delta_time > weapon.fire_rate {
            ev_spawn_bullet.send(SpawnBulletEvent);
            last_shot.delta_time = time.elapsed_seconds();
        }
        weapon.is_firing = true;
    } else if buttons.just_released(MouseButton::Left) {
        weapon.is_firing = false;
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
        transform.translation.z = (450. - (transform.translation.y) * 0.001)
            .max(0.0)
            .min(990.0);
    });
}

pub fn firing_bullet_emit(
    time: Res<Time>,
    mut ev_spawn_bullet: EventWriter<SpawnBulletEvent>,
    mut last_shot: ResMut<LastShot>,
    query: Query<&Weapon, With<Player>>,
) {
    let weapon = query.single();
    let now = time.elapsed_seconds();
    if weapon.is_firing && (now - last_shot.delta_time > weapon.fire_rate) {
        ev_spawn_bullet.send(SpawnBulletEvent);
        last_shot.delta_time = now;
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
                bullet: Bullet,
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
                    collider: Collider {},
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
                hit_count: HitCount { ttl: BULLET_TTL },
                decay: Decay {
                    amount: BULLETS_DECAYS,
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
    asset_server: Res<AssetServer>,
    to_spawn: Res<TotalToSpawn>,
    mut spawned: ResMut<TotalSpawned>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mob_is_robot = thread_rng().gen_range(0..4) == 3;
    let texture_handle = if mob_is_robot {
        asset_server.load("images/mob_atlas_1.png")
    } else {
        asset_server.load("images/mob_atlas_0.png")
    };
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    if mob_spawn_event.is_empty() || to_spawn.amount <= spawned.amount {
        return;
    }
    mob_spawn_event.clear();
    let angle = (thread_rng().gen_range(0..3600) as f32) / 10.0 * std::f32::consts::PI / 180.0;
    let (x, y) = (
        angle.cos() * MOB_SPAWN_RADIUS,
        -angle.sin() * MOB_SPAWN_RADIUS,
    );

    let player = query.single();

    let mob_spawn_position = Vec3 {
        x: player.translation.x + x,
        y: player.translation.y + y,
        z: player.translation.z,
    };
    // Spawn
    for _ in 0..to_spawn.quantity_per_spawn {
        if to_spawn.amount > spawned.amount {
            commands.spawn((
                EnemyBundle {
                    character: CharacterBundle {
                        in_game: InGame,
                        move_component: Move {
                            speed: MOB_SPEED,
                            direction: (player.translation - mob_spawn_position)
                                .truncate()
                                .normalize(),
                        },
                        harm: Harm { damage: MOB_DAMAGE },
                        alive: Alive {
                            health: if mob_is_robot {
                                MOB_HEALTH * 2.0
                            } else {
                                MOB_HEALTH
                            },
                        },
                        collider: Collider,
                    },
                    sprite_bundle: SpriteSheetBundle {
                        transform: Transform {
                            translation: mob_spawn_position,
                            scale: MOB_SCALE,
                            ..default()
                        },
                        sprite: TextureAtlasSprite {
                            color: MOB_COLOR,
                            index: 0,
                            ..default()
                        },
                        texture_atlas: texture_atlas_handle.clone(),
                        ..default()
                    },
                    enemy: Enemy,
                },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ));
            spawned.amount += 1;
        }
    }
}

pub fn despawn_health(
    mut commands: Commands,
    mut query: Query<(Entity, &Alive, Option<&Enemy>, Option<&Player>)>,
    mut total_killed: ResMut<TotalKilled>,
    mut game_over_event_emitter: EventWriter<GameOverEvent>,
) {
    for (entity, alive, maybe_enemy, maybe_player) in query.iter_mut() {
        if alive.health <= 0.0 {
            if maybe_enemy.is_some() {
                total_killed.amount += 1;
            }
            if maybe_player.is_some() {
                game_over_event_emitter.send(GameOverEvent);
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn wave_is_done_emit(
    mut wave_done_event: EventWriter<WaveDoneEvent>,
    total_killed: Res<TotalKilled>,
    total_to_spawn: Res<TotalToSpawn>,
) {
    if total_killed.amount >= total_to_spawn.amount {
        wave_done_event.send(WaveDoneEvent);
    }
}

pub fn change_level(
    mut app_state: ResMut<State<AppState>>,
    mut score: ResMut<Score>,
    wave_done_event: EventReader<WaveDoneEvent>,
    mut total_spawned: ResMut<TotalSpawned>,
    mut total_to_spawn: ResMut<TotalToSpawn>,
    mut total_killed: ResMut<TotalKilled>,
    mut quey_timer_spawner: Query<&mut MobSpawnerTimer>,
) {
    if !wave_done_event.is_empty() {
        wave_done_event.clear();
        score.level_up();
        let mut timer_spawner = quey_timer_spawner.single_mut();
        timer_spawner.0 = Timer::from_seconds(total_to_spawn.rate, TimerMode::Repeating);
        total_killed.amount = 0;
        total_spawned.amount = 0;
        total_to_spawn.update_paramter_for_level_id(score.get_level_index());
        app_state.set(AppState::LevelMenu).unwrap();
    }
}

pub fn game_over(
    mut app_state: ResMut<State<AppState>>,
    game_over_event_emitter: EventReader<GameOverEvent>,
) {
    if !game_over_event_emitter.is_empty() {
        game_over_event_emitter.clear();
        app_state.set(AppState::RetryMenu).unwrap();
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

pub fn lerp_color(color: Color, other_color: Color, value: f32) -> Color {
    let new_color_vec = Vec4::new(color.r(), color.g(), color.b(), color.a()).lerp(
        Vec4::new(
            other_color.r(),
            other_color.g(),
            other_color.b(),
            other_color.a(),
        ),
        value,
    );
    let new_color = Color::rgba_linear(
        new_color_vec.x,
        new_color_vec.y,
        new_color_vec.z,
        new_color_vec.w,
    );
    new_color
}

pub fn bullet_hitting_update(
    mut query_bullets: Query<(&Transform, &Harm, &mut HitCount), (With<Bullet>, Without<Enemy>)>,
    mut query_enemy: Query<
        (&Transform, &mut Alive, &mut TextureAtlasSprite),
        (With<Enemy>, Without<Bullet>),
    >,
) {
    query_bullets.for_each_mut(|(bullet_transform, bullet_harm, mut hit_count)| {
        query_enemy.for_each_mut(|(enemy_transform, mut enemy_alive, mut sprite)| {
            //collide
            if hit_count.ttl <= 0 {
                return;
            }
            if let Some(_) = collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                enemy_transform.translation,
                enemy_transform.scale.truncate() * 32.0,
            ) {
                enemy_alive.health -= bullet_harm.damage;
                sprite.color =
                    lerp_color(MOB_COLOR_HURT, MOB_COLOR, enemy_alive.health / MOB_HEALTH);
                hit_count.ttl -= 1;
            }
        });
    });
}

pub fn enemy_hitting_update(
    mut query_player: Query<(&Transform, &mut Alive), (With<Player>, Without<Enemy>)>,
    query_enemy: Query<(&Transform, &Alive, &Harm), (With<Enemy>, Without<Player>)>,
) {
    let (player_transform, mut player_life) = query_player.single_mut();
    query_enemy.for_each(|(enemy_transform, enemy_life, enemy_harm)| {
        //collide
        if player_life.health <= 0.0 {
            return;
        }
        if enemy_life.health > 0.0 {
            if let Some(_) = collide(
                player_transform.translation,
                player_transform.scale.truncate() * 32.0,
                enemy_transform.translation,
                enemy_transform.scale.truncate() * 32.0,
            ) {
                player_life.health -= enemy_harm.damage;
            }
        }
    });
}
