use bevy::prelude::Resource;

#[derive(Resource)]
struct TotalToSpawn {
    amount: u32,
}

#[derive(Resource)]
struct TotalSpawned {
    amount: u32,
}

#[derive(Resource)]
struct TotalKilled {
    amount: u32,
}
