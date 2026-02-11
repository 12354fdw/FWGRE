use crate::entities::Tile;
use tracing::info;

pub mod load;

pub struct World {
    pub width: u32,
    pub height: u32,
    pub terrain: Vec<Tile>
}