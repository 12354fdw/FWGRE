use macroquad::prelude::*;
use uuid::Uuid;

use crate::entities::Tile;
use crate::entities::Types;
use crate::world::World;
use crate::graphics::chunks::ChunkTexture;
use crate::world::CHUNK_SIZE;

mod textures;
mod chunks;

const LOD_THRESHOLD: f32 = 0.05;
pub struct Graphics {
    pub cam: Camera2D,
    pub textures: textures::Textures,

    world_chunk_width: u32,
    world_chunk_height: u32,

    chunks: Vec<ChunkTexture>,

}

impl Graphics {
    fn draw_tile(&self, tile: &Tile) {
        let count = self.textures.variant_count(tile.id);
        let variant = (tile.uuid.as_u128() as usize) % count;
        let text = self.textures.get_variant(tile.id, variant);

        
        draw_texture_ex(text,
            tile.pos.x ,
            tile.pos.y ,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    tile.size.x ,
                    tile.size.y ,
                )),
                ..Default::default()
            }
        )
    }

    fn draw_lod(&self, drawn: &mut u32) {
        let (left, right, top, bottom) = self.visible_world_bounds();

        for (i, chunk) in self.chunks.iter().enumerate() {
            let (cx, cy) = self.chunk_coords(i);

            let world_x = cx as f32 * CHUNK_SIZE as f32;
            let world_y = cy as f32 * CHUNK_SIZE as f32;

            let chunk_left   = world_x;
            let chunk_right  = world_x + CHUNK_SIZE as f32;
            let chunk_top    = world_y;
            let chunk_bottom = world_y + CHUNK_SIZE as f32;

            if chunk_right < left ||
                chunk_left > right ||
                chunk_bottom < top ||
                chunk_top > bottom {
                    continue;
            }

            draw_texture_ex(
                &chunk.texture,
                world_x,
                world_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        CHUNK_SIZE as f32,
                        CHUNK_SIZE as f32,
                    )),
                    ..Default::default()
                },
            );
            *drawn += 1;
        }
    }


    pub fn draw(&self, world: &World) {
        set_camera(&self.cam);

        let mut drawn: u32 = 0;

        let (left, right, top, bottom) = self.visible_world_bounds();

        let use_lod = self.cam.zoom.x < LOD_THRESHOLD;

        if use_lod {
            self.draw_lod(&mut drawn);
        } else {
            for tile in &world.terrain {
                if tile.id == Types::Water {
                    continue;
                }

                // Cull here
    
                let tile_left   = tile.pos.x;
                let tile_right  = tile_left + tile.size.x;
                let tile_top    = tile.pos.y;
                let tile_bottom = tile_top + tile.size.y;

                if tile_right < left ||
                    tile_left > right ||
                    tile_bottom < top ||
                    tile_top > bottom {
                        continue;
                }

                self.draw_tile(tile);

                drawn += 1;
            }
        }
        set_default_camera();

        /*
        draw_text(
            *
            &format!("FPS: {}", get_fps()),
            20.0,
            20.0,
            30.0,
            WHITE,
        );*/

        draw_text(&format!("fps: {}",get_fps()),20.0,20.0,30.0,WHITE);
        draw_text(&format!("drawn: {}",drawn),20.0,45.0,30.0,WHITE);
        draw_text(&format!("scale: {}",self.cam.zoom.x),20.0,70.0,30.0,WHITE);

    }
    fn visible_world_bounds(&self) -> (f32, f32, f32, f32) {
        let top_left = self.cam.screen_to_world(vec2(0.0, 0.0));
        let bottom_right = self.cam.screen_to_world(vec2(screen_width(), screen_height()));

        let left = top_left.x;
        let top = top_left.y;
        let right = bottom_right.x;
        let bottom = bottom_right.y;

        (left, right, top, bottom)
    }




    pub fn calculate_camera_movement(&mut self) {
        let speed = 1.5 * (1.0 / self.cam.zoom.x);
        let dt = get_frame_time();

        if is_key_down(KeyCode::W) {
            self.cam.target.y -= speed * dt;
        }
        if is_key_down(KeyCode::S) {
            self.cam.target.y += speed * dt;
        }
        if is_key_down(KeyCode::A) {
            self.cam.target.x -= speed * dt;
        }
        if is_key_down(KeyCode::D) {
            self.cam.target.x += speed * dt;
        }

        let (_wheel_x, wheel_y) = mouse_wheel();

        if wheel_y != 0.0 {
            let zoom_speed = 0.1;

            let mouse_screen = mouse_position();
            let mouse_world_before = self.cam.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));

            let zoom_factor = 1.0 + wheel_y * zoom_speed;
            self.cam.zoom *= zoom_factor;

            let mouse_world_after = self.cam.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));
            self.cam.target += mouse_world_before - mouse_world_after;
        }

    }

    pub fn new(world: &mut World) -> Self {
        let world_chunk_width = (world.width as f32 / CHUNK_SIZE as f32).ceil() as u32;
        let world_chunk_height = (world.height as f32 / CHUNK_SIZE as f32).ceil() as u32;

        let mut chunks = Vec::new();
        for index in 0..(world_chunk_width * world_chunk_height) {
            let image = Image::gen_image_color(
                CHUNK_SIZE as u16,
                CHUNK_SIZE as u16,
                Color::from_rgba(0, 0, 0, 0),
            );
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);

            chunks.push(ChunkTexture { texture, image });
            world.dirty_chunks.push(index as usize);
        }

        Self {
            cam: Camera2D {
                zoom: vec2(
                    10.0,
                    10.0,
                ),
                target: vec2(0.0, 0.0),
                ..Default::default()
            },
            textures: textures::Textures::new(),
            world_chunk_width,
            world_chunk_height,
            chunks
        }
    }
}
