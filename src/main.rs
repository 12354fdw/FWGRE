use macroquad::prelude::*;
use tracing::info;

mod graphics;
mod logging;
mod entities;

#[macroquad::main("FWGRE")]
async fn main() {
    logging::init();

    let mut graphicsCtx: graphics::Graphics = graphics::Graphics::new();

    info!("Starting FWGRE...");

    info!("Loading textures...");
    graphicsCtx.textures.load("grass","assets/grass/1.png").await;
    graphicsCtx.textures.load("grass","assets/grass/2.png").await;
    graphicsCtx.textures.load("grass","assets/grass/3.png").await;
    graphicsCtx.textures.load("grass","assets/grass/4.png").await;
    graphicsCtx.textures.load("grass","assets/grass/5.png").await;
    graphicsCtx.textures.load("grass","assets/grass/6.png").await;
    graphicsCtx.textures.load("grass","assets/grass/7.png").await;
    graphicsCtx.textures.load("grass","assets/grass/8.png").await;
    graphicsCtx.textures.load("grass","assets/grass/9.png").await;
    graphicsCtx.textures.load("grass","assets/grass/10.png").await;

    info!("Textures loaded!");

    let test_tile1 = entities::Tile::new("grass",ivec2(1,1),ivec2(1,1));
    let test_tile2 = entities::Tile::new("grass",ivec2(1,2),ivec2(1,1));
    let test_tile3 = entities::Tile::new("grass",ivec2(2,1),ivec2(1,1));
    let test_tile4 = entities::Tile::new("grass",ivec2(2,2),ivec2(1,1));

    loop {
        clear_background(BLACK);

        graphicsCtx.draw_tile(&test_tile1);
        graphicsCtx.draw_tile(&test_tile2);
        graphicsCtx.draw_tile(&test_tile3);
        graphicsCtx.draw_tile(&test_tile4);


        next_frame().await;
    }
}
