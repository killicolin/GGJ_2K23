use std::fmt::Display;

use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Color, Deref, DerefMut, ReflectComponent, Vec2},
    reflect::Reflect,
    sprite::SpriteBundle,
    time::Timer,
};
use rand::{thread_rng, Rng};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Alive {
    pub health: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Move {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Decay {
    pub amount: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HitCount {
    pub ttl: i32,
}
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Harm {
    pub damage: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Collider;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Bullet;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Aim {
    pub direction: Vec2,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Weapon {
    pub fire_rate: f32,
    pub bullet_speed: f32,
    pub bullet_ttl: i32,
    pub bullets: u32,
    pub is_firing: bool,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub move_component: Move,
    pub harm: Harm,
    pub alive: Alive,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
    pub in_game: InGame,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub character: CharacterBundle,
    pub weapon: Weapon,
    pub aim: Aim,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub character: CharacterBundle,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub character: CharacterBundle,
    pub hit_count: HitCount,
    pub decay: Decay,
}

#[derive(Component, Deref, DerefMut)]
pub struct MobSpawnerTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BulletSpawnerTimer(pub Timer);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MainMenu;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct InGame;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct LevelMenu;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerColor(pub Color);

#[derive(Component, Clone)]
pub enum DebufChoices {
    Health,
    Speed,
    Bullets,
    BulletsTtl,
    Damage,
    BulletsSpeed,
    FireRate,
}

impl DebufChoices {
    pub fn get_random() -> Self {
        match thread_rng().gen_range(0..6) {
            0 => DebufChoices::Health,
            1 => DebufChoices::Speed,
            2 => DebufChoices::Bullets,
            3 => DebufChoices::BulletsTtl,
            4 => DebufChoices::Damage,
            5 => DebufChoices::BulletsSpeed,
            6 => DebufChoices::FireRate,
            _ => DebufChoices::Health,
        }
    }
}
impl Display for DebufChoices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebufChoices::Health => write!(f, "Lose health"),
            DebufChoices::Speed => write!(f, "Lose speed"),
            DebufChoices::Bullets => write!(f, "Lose nb of bullets"),
            DebufChoices::BulletsTtl => write!(f, "Lose bullets hitcount"),
            DebufChoices::Damage => write!(f, "Lose damage"),
            DebufChoices::BulletsSpeed => write!(f, "Lose bullets speed"),
            DebufChoices::FireRate => write!(f, "Lose fire rate"),
        }
    }
}
