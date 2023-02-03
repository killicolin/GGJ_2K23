use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Transform, Vec2},
    sprite::SpriteBundle,
};

#[derive(Component)]
pub struct Alive {
    pub pv: f32,
}

#[derive(Component)]
pub struct Move {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Decay {
    pub amount: f32,
}

#[derive(Component)]
pub struct HitCount {
    pub ttl: u32,
}
#[derive(Component)]
pub struct Harm {
    pub damage: f32,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Orientated {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Weapon {
    pub fire_rate: f32,
    pub bullet_ttl: u32,
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
    pub transform: Transform,
    pub orientated: Orientated,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub character: CharacterBundle,
    pub weapon: Weapon,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub character: CharacterBundle,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub character: CharacterBundle,
}
