use std::collections::HashMap;
use macroquad::prelude::*;
use tracing::info;
use crate::entities;

mod loading;
mod fetch;

pub struct Textures {
    textures: HashMap<entities::Types, Vec<Texture2D>>,
    avg_color: HashMap<entities::Types, Vec<Color>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            avg_color: HashMap::new(),
        }
    }

    pub fn variant_count(&self, id: entities::Types) -> usize {
        self.textures[&id].len()
    }
}
