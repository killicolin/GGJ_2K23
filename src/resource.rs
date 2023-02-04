use bevy::prelude::Resource;

#[derive(Resource)]
pub struct TotalToSpawn {
    pub amount: u32,
}

#[derive(Resource, Default)]
pub struct TotalSpawned {
    pub amount: u32,
}

#[derive(Resource, Default)]
pub struct TotalKilled {
    pub amount: u32,
}
