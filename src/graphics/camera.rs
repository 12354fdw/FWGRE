pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}