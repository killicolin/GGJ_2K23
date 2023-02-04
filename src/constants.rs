use bevy::prelude::{Color, Vec2, Vec3};

// Player starting stats
pub const PLAYER_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_DAMAGE: f32 = 1.0;
pub const PLAYER_HEALTH: f32 = 1.0;
pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const PLAYER_SCALE: Vec3 = Vec3::new(0.5, 0.5, 0.5);
pub const PLAYER_AIM: Vec2 = Vec2 { x: 1.0, y: 1.0 };
pub const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PLAYER_FIRE_RATE: f32 = 1.0;
pub const PLAYER_BULLETS_SPEED: f32 = 30.0;
pub const PLAYER_BULLETS_TTL: i32 = 1;
pub const PLAYER_BULLETS: u32 = 20;

//Bullet const variables
pub const BULLET_TTL: i32 = 4;
pub const BULLET_HEALTH: f32 = 1.0;
pub const BULLETS_SCALE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
pub const BULLETS_COLOR: Color = Color::rgb(0.8, 0.8, 0.4);
pub const BULLETS_SPREAD: f32 = 5.0 * std::f32::consts::PI / 180.0;
pub const BULLETS_DECAYS: f32 = 0.001;

pub const MOB_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
pub const MOB_SPEED: f32 = 90.0;
pub const MOB_SPAWN_RADIUS: f32 = 700.0;
pub const MOB_DAMAGE: f32 = 1.0;
pub const MOB_HEALTH: f32 = 1.0;
pub const MOB_SCALE: Vec3 = Vec3::new(0.5, 0.5, 0.5);
