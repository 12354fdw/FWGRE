use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Types {
    Water,
    Grass,
    Dirt,
}

pub struct Tile {
    // simulation concern
    pub id: Types,         // what type of tile
    pub uuid: uuid::Uuid,   // UUID of tile

    // graphical concern
    pub pos: Vec2,          // pos in tiles
    pub size: Vec2,         // size in tiles
}

pub struct Entity {
    // simulation concern
    pub id: Types,         // what type of tile
    pub uuid: uuid::Uuid,   // UUID of tile

    // graphical concern
    pub pos: Vec2,          // pos in pixels
    pub size: Vec2,         // size in pixels
}

impl Tile {
    pub fn new(id: Types, pos: Vec2, size: Vec2) -> Self {
        Self {
            id: id,
            uuid: uuid::Uuid::new_v4(),
            pos,
            size,
        }
    }
}

impl Entity {
    pub fn new(id: Types, pos: Vec2, size: Vec2) -> Self {
        Self {
            id: id,
            uuid: uuid::Uuid::new_v4(),
            pos,
            size,
        }
    }
}
