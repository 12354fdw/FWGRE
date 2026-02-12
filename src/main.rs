use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, hash};

use tracing::info;

mod graphics;
mod logging;
mod entities;
mod world;

#[macroquad::main("FWGRE")]
async fn main() {
    logging::init();

    info!("Starting FWGRE...");
    
    let mut world = world::World::init();

    info!("Starting Graphics...");
    let mut graphicsCtx: graphics::Graphics = graphics::Graphics::new(&mut world);
    
    info!("Loading textures...");
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/1.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/2.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/3.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/4.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/5.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/6.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/7.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/8.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/9.png").await;
    graphicsCtx.textures.load(entities::Types::Grass,"assets/grass/10.png").await;

    info!("Textures loaded!");


    loop {
        clear_background(BLUE);

        graphicsCtx.rasterize_dirty_chunks(&mut world);
        graphicsCtx.calculate_camera_movement();
        graphicsCtx.draw(&world);

        // ui stuff
        /*
        widgets::Window::new(hash!(), vec2(20., 20.), vec2(200., 120.))
            .titlebar(true)
            .movable(true)
            .ui(&mut root_ui(), |ui| {
                if ui.button(None, "Click Me") {
                    info!("Clicked!");
                }
        });
        */




        next_frame().await;
    }
}
