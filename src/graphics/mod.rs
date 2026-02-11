use macroquad::prelude::*;
use uuid::Uuid;

use crate::entities::Tile;

mod textures;

const TILE_SIZE_X: i32 = 128;
const TILE_SIZE_Y: i32 = 128;

pub struct Graphics {
    pub cam: Camera2D,
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

    pub fn new() -> Self {
        Self {
            cam: Camera2D {
                zoom: vec2(
                    2.0 / screen_width(),
                    2.0 / screen_height(),
                ),
                target: vec2(0.0, 0.0),
                ..Default::default()
            },
            textures: textures::Textures::new(),
        }
    }
}
