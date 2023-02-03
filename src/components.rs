use bevy::{
    ecs::component::Component,
    prelude::{Bundle, Transform, Vec2},
    sprite::SpriteBundle,
};

#[derive(Component)]
struct Alive {
    pv: f32,
}

#[derive(Component)]
struct Move {
    speed: f32,
    direction: Vec2,
}

#[derive(Component)]
struct Decay {
    amount: f32,
}

#[derive(Component)]
struct HitCount {
    ttl: u32,
}
#[derive(Component)]
struct Harm {
    damage: f32,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Weapon {
    fire_rate: f32,
    bullet_ttl: u32,
    bullets: u32,
}

#[derive(Bundle)]
struct CharacterBundle {
    move_compoment: Move,
    harm: Harm,
    alive: Alive,
    sprite_bundle: SpriteBundle,
    collider: Collider,
    transform: Transform,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    character: CharacterBundle,
    weapon: Weapon,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    character: CharacterBundle,
}

#[derive(Bundle)]
struct BulletBundle {
    character: CharacterBundle,
}
