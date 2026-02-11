use macroquad::prelude::*;
use uuid::Uuid;

use crate::entities::Tile;

mod camera;
mod textures;

const TILE_SIZE_X: i32 = 128;
const TILE_SIZE_Y: i32 = 128;

pub struct Graphics {
    pub cam: camera::Camera,
    pub textures: textures::Textures,
}

impl Graphics {
    pub fn draw_tile(&self, tile: &Tile) {
        let count = self.textures.variant_count(&tile.id);
        let variant = (tile.uuid.as_u128() as usize) % count;
        let text = self.textures.get_variant(&tile.id, variant);

        
        draw_texture_ex(text,
            (tile.pos.x as f32) * TILE_SIZE_X as f32,
            (tile.pos.y as f32) * TILE_SIZE_Y as f32,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    (tile.size.x as f32) * TILE_SIZE_X as f32,
                    (tile.size.y as f32) * TILE_SIZE_Y as f32,
                )),
                ..Default::default()
            }
        )
    }

    pub fn new() -> Self {
        Self {
            cam: camera::Camera::default(),
            textures: textures::Textures::new(),
        }
    }
}
