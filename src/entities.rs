use macroquad::prelude::*;

pub struct Tile {
    // simulation concern
    pub id: String,         // what type of tile
    pub uuid: uuid::Uuid,   // UUID of tile

    // graphical concern
    pub pos: IVec2,          // pos in tiles
    pub size: IVec2,         // size in tiles
}

pub struct Entity {
    // simulation concern
    pub id: String,         // what type of tile
    pub uuid: uuid::Uuid,   // UUID of tile

    // graphical concern
    pub pos: Vec2,          // pos in pixels
    pub size: Vec2,         // size in pixels
}

impl Tile {
    pub fn new(id: impl Into<String>, pos: IVec2, size: IVec2) -> Self {
        Self {
            id: id.into(),
            uuid: uuid::Uuid::new_v4(),
            pos,
            size,
        }
    }
}

impl Entity {
    pub fn new(id: impl Into<String>, pos: Vec2, size: Vec2) -> Self {
        Self {
            id: id.into(),
            uuid: uuid::Uuid::new_v4(),
            pos,
            size,
        }
    }
}
