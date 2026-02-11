use std::collections::HashMap;
use macroquad::prelude::*;
use tracing::info;
use crate::entities;

use super::Textures;

impl Textures {
    pub async fn load(&mut self, id: entities::Types, path: &str) {
        info!("Loading '{:?}' from '{}'", id, path);

        let tex = load_texture(path).await
            .expect("Failed to load texture");

        tex.set_filter(FilterMode::Nearest);

        self.compute_avg_color(id, path);

        self.textures
            .entry(id)
            .or_insert_with(Vec::new)
            .push(tex);
    }

    fn compute_avg_color(&mut self, id: entities::Types, path: &str) {
        let img = image::open(path)
            .expect("Failed to open texture")
            .to_rgba8();

        let mut total_r: u64 = 0;
        let mut total_g: u64 = 0;
        let mut total_b: u64 = 0;
        let mut count: u64 = 0;

        for pixel in img.pixels() {
            total_r += pixel[0] as u64;
            total_g += pixel[1] as u64;
            total_b += pixel[2] as u64;
            count += 1;
        }

        let color = Color::from_rgba(
            (total_r / count) as u8,
            (total_g / count) as u8,
            (total_b / count) as u8,
            255,
        );

        self.avg_color
            .entry(id)
            .or_insert_with(Vec::new)
            .push(color);
    }
}