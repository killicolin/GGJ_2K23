use bevy::{
    prelude::{Camera2dBundle, Color, Commands, Transform, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use crate::components::{
    Alive, CharacterBundle, Collider, Harm, Move, Orientated, Player, PlayerBundle, Weapon,
};

// Player starting stats
const PLAYER_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_SPEED: f32 = 1.0;
const PLAYER_DAMAGE: f32 = 1.0;
const PLAYER_HEALTH: f32 = 1.0;
const PLAYER_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SCALE: Vec3 = Vec3::new(10.0, 10.0, 10.0);
const PLAYER_ORIENTATION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PLAYER_FIRE_RATE: f32 = 1.0;
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
            orientated: Orientated {
                direction: PLAYER_ORIENTATION,
            },
        },
        player: Player,
        weapon: Weapon {
            fire_rate: PLAYER_FIRE_RATE,
            bullet_ttl: PLAYER_BULLETS_TTL,
            bullets: PLAYER_BULLETS,
            is_firing: false,
        },
    });
}
