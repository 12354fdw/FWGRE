use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

pub fn camera_controller(
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    if let Projection::Orthographic(ref mut ortho) = *projection {
        let speed = 1.5 * ortho.scale;

        if keys.pressed(KeyCode::KeyW) {
            transform.translation.y += speed * dt * 500.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * dt * 500.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * dt * 500.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * dt * 500.0;
        }

        for ev in scroll.read() {
            let zoom_speed = 0.1;
            let zoom_factor = 1.0 + ev.y * zoom_speed;
            ortho.scale /= zoom_factor;
        }
    }
}