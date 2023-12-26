use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WorldHeightGenerator {
    PerlinNoise {
        scale: f64,
        max: i8,
        min: i8,
        weights: Vec<u16>,
    },
}

#[derive(Resource, Serialize, Deserialize)]
pub struct WorldSetting {
    pub id: String,
    pub name: String,
    pub description: String,
    pub width: usize,
    pub height: usize,
    pub generator: WorldHeightGenerator,
}
