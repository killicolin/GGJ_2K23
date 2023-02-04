use bevy::{
    audio::AudioSink,
    prelude::{Handle, Resource},
};

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

#[derive(Resource, Default)]
pub struct LastShot {
    pub delta_time: f32,
}

#[derive(Resource)]
pub struct Score {
    pub level: u32,
}

#[derive(Resource)]
pub struct MusicController(pub Handle<AudioSink>);
