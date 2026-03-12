use bevy::prelude::*;


pub fn setup(mut commands: Commands) {
    // spawn in camera
    commands.spawn(Camera2d);

    commands.spawn(Sprite::from_color(
        Color::srgb(1.0, 0.0, 0.0),
        Vec2::new(100.0, 100.0),
    ));
}