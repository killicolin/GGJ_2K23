use bevy::prelude::{Color, Vec2, Vec3};

//
pub const LORE_PARENT_CHOICE: &str = "The robots of the future failed to kill you !!\nThey decided to kill one of your parents before your procreation to erase you from reality.\n";
pub const LORE_INTRO: &str = "It is a beautiful day when suddenly an army of robots from the future appears !\nIt must probably be one of your clumsy future chidlren who mess up the future !!\nProtect human race from robots domination !";
// Player starting statscargo
pub const PLAYER_DIRECTION: Vec2 = Vec2 { x: 1.0, y: 1.0 };
pub const PLAYER_SPEED: f32 = 400.0;
pub const PLAYER_DAMAGE: f32 = 0.5;
pub const PLAYER_HEALTH: f32 = 1.0;
pub const PLAYER_POSITION: Vec3 = Vec3::new(0.0, 0.0, 5.0);
pub const PLAYER_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const PLAYER_AIM: Vec2 = Vec2 { x: 1.0, y: 1.0 };
pub const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const PLAYER_FIRE_RATE: f32 = 0.25;
pub const PLAYER_BULLETS_SPEED: f32 = 1250.0;
pub const PLAYER_BULLETS_TTL: i32 = 1;
pub const PLAYER_BULLETS: u32 = 31;

//Bullet const variables
pub const BULLET_TTL: i32 = 4;
pub const BULLET_HEALTH: f32 = 1.0;
pub const BULLETS_SCALE: Vec3 = Vec3::new(2.0, 2.0, 2.0);
pub const BULLETS_COLOR: Color = Color::rgb(0.8, 0.8, 0.4);
pub const BULLETS_SPREAD: f32 = 5.0 * std::f32::consts::PI / 180.0;
pub const BULLETS_DECAYS: f32 = 0.01;

// Mob starting stats
pub const MOB_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const MOB_COLOR_HURT: Color = Color::rgb(0.8, 0.0, 0.0);
pub const MOB_SPEED: f32 = 180.0;
pub const MOB_SPAWN_RADIUS: f32 = 700.0;
pub const MOB_DAMAGE: f32 = 1.0;
pub const MOB_HEALTH: f32 = 1.0;
pub const MOB_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

//Spawning

pub const MAX_WAVE_CALIBRATION: u32 = 9;
pub const ENEMY_BY_LVL: [u32; 10] = [500, 50, 75, 100, 150, 200, 400, 800, 1200, 1600];
pub const SPEED_SPAWN_BY_LVL: [f32; 10] =
    [0.33, 0.46, 0.46, 0.46, 0.46, 0.46, 0.46, 0.46, 0.46, 0.46];
pub const SPAWN_TICK_BY_LVL: [u32; 10] = [10, 1, 2, 2, 2, 3, 3, 3, 3, 4];

// Score parameter
pub const BEGIN_DATE: i32 = 2100;
pub const DECREMENT_DATE_PER_LEVEL: i32 = 35;

// Map scale
pub const MAP_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
