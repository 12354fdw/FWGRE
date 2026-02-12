use super::World;
use super::CHUNK_SIZE;
use tracing::info;
use crate::entities::Tile;
use crate::entities::Types;
use macroquad::prelude::vec2;

impl World {
    pub fn init() -> Self {
        info!("Initializing world...");

        let img = image::open("assets/map.png")
            .expect("Failed to load map.png")
            .to_rgba8();

        let (width, height) = img.dimensions();
        let mut terrain = Vec::with_capacity((width * height) as usize);

        for (x, y, pixel) in img.enumerate_pixels() {
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            let pos = vec2(x as f32, y as f32);

            let tile = if r == 0 && g == 0 && b == 0 && a == 255 {
                Tile::new(Types::Grass, pos, vec2(1.0, 1.0))
            } else {
                Tile::new(Types::Water, pos, vec2(1.0, 1.0))
            };

            terrain.push(tile);
        }

        info!("Initialized world with size {}x{}", width, height);

        World {
            width,
            height,
            terrain,
            buildings: Vec::with_capacity((width * height) as usize),

            dirty_chunks: Vec::with_capacity(((width / CHUNK_SIZE) * (height / CHUNK_SIZE)) as usize),
        }
    }
}
