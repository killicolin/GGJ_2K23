use bevy::prelude::{Color, Vec2, Vec3};

//
pub const LORE_PARENT_CHOICE: &str = "The robots of the future failed to kill you !!\nThey decided to kill one of your parents before your procreation to erase you from reality.\n";
// Player starting statscargo
pub const PLAYER_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_DAMAGE: f32 = 0.3;
pub const PLAYER_HEALTH: f32 = 1.0;
pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, 0.0, 5.0);
pub const PLAYER_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const PLAYER_AIM: Vec2 = Vec2 { x: 1.0, y: 1.0 };
pub const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PLAYER_FIRE_RATE: f32 = 1.0;
pub const PLAYER_BULLETS_SPEED: f32 = 500.0;
pub const PLAYER_BULLETS_TTL: i32 = 1;
pub const PLAYER_BULLETS: u32 = 20;

//Bullet const variables
pub const BULLET_TTL: i32 = 4;
pub const BULLET_HEALTH: f32 = 1.0;
pub const BULLETS_SCALE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
pub const BULLETS_COLOR: Color = Color::rgb(0.8, 0.8, 0.4);
pub const BULLETS_SPREAD: f32 = 5.0 * std::f32::consts::PI / 180.0;
pub const BULLETS_DECAYS: f32 = 0.001;

// Mob starting stats
pub const MOB_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const MOB_COLOR_HURT: Color = Color::rgb(0.8, 0.0, 0.0);
pub const MOB_SPEED: f32 = 64.0;
pub const MOB_SPAWN_RADIUS: f32 = 700.0;
pub const MOB_DAMAGE: f32 = 1.0;
pub const MOB_HEALTH: f32 = 1.0;
pub const MOB_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const MOB_MAX_SPAWN_PER_WAVE: u32 = 5;

// Score parameter
pub const BEGIN_DATE: i32 = 2100;
pub const DECREMENT_DATE_PER_LEVEL: i32 = 35;
