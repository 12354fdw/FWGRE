use std::collections::HashMap;
use macroquad::prelude::*;
use tracing::info;
use crate::entities;

use super::Textures;

impl Textures {
    pub fn get(&self, id: entities::Types) -> &Texture2D {
        let list = self.textures.get(&id)
            .expect("Texture id not found");

        let index = macroquad::rand::gen_range(0, list.len());
        &list[index]
    }

    pub fn get_variant(&self, id: entities::Types, variant: usize) -> &Texture2D {
        &self.textures[&id][variant]
    }

    pub fn get_lod(&self, id: entities::Types) -> &Color {
        let list = self.avg_color.get(&id)
            .expect("LOD id not found");
        
        let index = macroquad::rand::gen_range(0, list.len());
        &list[index]
    }

    pub fn get_lod_variant(&self, id: entities::Types, variant: usize) -> Color {
        self.avg_color[&id][variant]
    }
}