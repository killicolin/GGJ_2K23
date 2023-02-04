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
pub struct RetryMenu;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerColor(pub Color);

#[derive(Component, Clone)]
pub struct Debuff {
    debuff_choice: [DebufChoices; 3],
}

#[derive(Clone)]
pub enum DebufChoices {
    Speed,
    Bullets,
    BulletsTtl,
    Damage,
    BulletsSpeed,
    FireRate,
}

impl Debuff {
    fn get_default(i: usize) -> DebufChoices {
        match i {
            0 => DebufChoices::Speed,
            1 => DebufChoices::Bullets,
            2 => DebufChoices::BulletsTtl,
            3 => DebufChoices::Damage,
            4 => DebufChoices::BulletsSpeed,
            5 => DebufChoices::FireRate,
            _ => DebufChoices::Speed,
        }
    }

    pub fn get_parent_random() -> (Self, Self) {
        let mut vec_mom: Vec<usize> = vec![1, 2, 3, 4, 5, 6];
        let mut vec_dad: Vec<usize> = vec![];
        let i1 = thread_rng().gen_range(0..6);
        vec_dad.push(vec_mom.remove(i1));
        let i1 = thread_rng().gen_range(0..5);
        vec_dad.push(vec_mom.remove(i1));
        let i1 = thread_rng().gen_range(0..4);
        vec_dad.push(vec_mom.remove(i1));

        let dad_default: [DebufChoices; 3] = [
            Self::get_default(vec_dad[0]),
            Self::get_default(vec_dad[1]),
            Self::get_default(vec_dad[2]),
        ];
        let mom_default: [DebufChoices; 3] = [
            Self::get_default(vec_mom[0]),
            Self::get_default(vec_mom[1]),
            Self::get_default(vec_mom[2]),
        ];
        (
            Self {
                debuff_choice: dad_default,
            },
            Self {
                debuff_choice: mom_default,
            },
        )
    }
}

impl Display for Debuff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n",
            self.debuff_choice[0], self.debuff_choice[1], self.debuff_choice[2]
        )
    }
}

impl Display for DebufChoices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebufChoices::Speed => write!(f, "- speed"),
            DebufChoices::Bullets => write!(f, "- nb of bullets"),
            DebufChoices::BulletsTtl => write!(f, "- bullets hitcount"),
            DebufChoices::Damage => write!(f, "- damage"),
            DebufChoices::BulletsSpeed => write!(f, "- bullets speed"),
            DebufChoices::FireRate => write!(f, "- fire rate"),
        }
    }
}
