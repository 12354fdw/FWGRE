use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;

mod camera;
mod setup;
mod atlas;

fn main() {
    App::new()
        // plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsCounterPlugin)

        // resources
        .init_resource::<world::ChunkMap>()

        // startup
        .add_systems(Startup, atlas::load_textures)
        .add_systems(Startup, setup::setup)

        // update
        .add_systems(Update, atlas::build_atlas)
        .add_systems(Update, camera::camera_controller)

        .run();
}
