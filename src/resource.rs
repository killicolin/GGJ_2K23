use core::f32;

use bevy::prelude::Resource;
use std::collections::HashMap;

use crate::constants::{
    BEGIN_DATE, DECREMENT_DATE_PER_LEVEL, ENEMY_BY_LVL, MAX_WAVE_CALIBRATION, SPAWN_TICK_BY_LVL,
    SPEED_SPAWN_BY_LVL,
};

#[derive(Resource)]
pub struct TotalToSpawn {
    pub amount: u32,
    pub rate: f32,
    pub quantity_per_spawn: u32,
}

impl TotalToSpawn {
    pub fn update_paramter_for_level_id(&mut self, index: u32) {
        let (i, multiple) = if index < MAX_WAVE_CALIBRATION {
            (index as usize, 1)
        } else {
            (
                MAX_WAVE_CALIBRATION as usize,
                ((index + 2) as i32 - MAX_WAVE_CALIBRATION as i32) as u32,
            )
        };
        self.amount = ENEMY_BY_LVL[i] * multiple;
        self.rate = SPEED_SPAWN_BY_LVL[i] * multiple as f32;
        self.quantity_per_spawn = SPAWN_TICK_BY_LVL[i] * multiple;
    }
}

impl Default for TotalToSpawn {
    fn default() -> Self {
        Self {
            amount: ENEMY_BY_LVL[0],
            rate: SPEED_SPAWN_BY_LVL[0],
            quantity_per_spawn: SPAWN_TICK_BY_LVL[0],
        }
    }
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
    pub fn get_level_index(&self) -> u32 {
        self.level
    }

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

    pub fn should_start_music(&self) -> bool {
        self.level == 0 || self.level == 2 || self.level == 5
    }

    pub fn should_stop_music(&self) -> bool {
        self.level == 1 || self.level == 3
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
