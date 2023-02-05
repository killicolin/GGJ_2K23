use core::f32;

use bevy::prelude::Resource;
use std::collections::HashMap;

use crate::constants::{BEGIN_DATE, DECREMENT_DATE_PER_LEVEL};

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
    level: u32,
    date_pannel_level_effect: f32,
    date: i32,
}

impl Score {
    pub fn level_up(&mut self) {
        self.level += 1;
        self.date -= DECREMENT_DATE_PER_LEVEL;
    }

    pub fn decrease(&mut self, amount: f32) {
        self.date_pannel_level_effect -= amount;
        self.date_pannel_level_effect = self.date_pannel_level_effect.max(self.date as f32);
    }

    pub fn to_text(&self) -> String {
        format!("{}", self.date_pannel_level_effect.round() as i32)
    }

    pub fn historic_period_theme(&self) -> i32 {
        match self.level {
            s if s < 2 => 0,
            s if s >= 2 && s < 5 => 1,
            _ => 2,
        }
    }
}

impl Default for Score {
    fn default() -> Self {
        Self {
            level: 0,
            date_pannel_level_effect: BEGIN_DATE as f32,
            date: BEGIN_DATE,
        }
    }
}

pub enum ChunkType {
    Basic,
}

#[derive(Resource)]
pub struct ChunksMap {
    pub chunks: HashMap<(i32, i32), ChunkType>,
}
