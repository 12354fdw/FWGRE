use macroquad::prelude::*;

use crate::entities::Tile;
use crate::entities::Types;
use crate::graphics::Graphics;
use crate::world::World;
use crate::world::CHUNK_SIZE;

use tracing::info;


pub struct ChunkTexture {
    pub texture: Texture2D,
    pub image: Image,
}

impl Graphics {
    pub fn rasterize_dirty_chunks(&mut self, world: &mut World) {
        for &chunk_id in &world.dirty_chunks {
            let (cx,cy) = self.chunk_coords(chunk_id);
            info!("Rasterizing dirty chunk at {} {}",cx,cy);

            let chunk = &mut self.chunks[chunk_id];

            for ly in 0..CHUNK_SIZE {
                for lx in 0..CHUNK_SIZE {
                    let wx = cx * CHUNK_SIZE + lx;
                    let wy = cy * CHUNK_SIZE + ly;

                    if wx >= world.width || wy >= world.height { continue; }

                    let tile_index = (wy * world.width + wx) as usize;
                    let tile = &world.terrain[tile_index];

                    if (tile.id == Types::Water) { continue; }

                    let count = self.textures.variant_count(tile.id);
                    let variant = (tile.uuid.as_u128() as usize) % count;
                    let color = self.textures.get_lod_variant(tile.id, variant);

                    chunk.image.set_pixel(lx,ly,color);
                }
            }
            chunk.texture.update(&chunk.image);
        }
        world.dirty_chunks.clear();
    }

    pub fn chunk_coords(&self, chunk_id: usize) -> (u32,u32) {
        let cx = chunk_id as u32 % self.world_chunk_width;
        let cy = chunk_id as u32 / self.world_chunk_width;
        (cx,cy)
    }
}