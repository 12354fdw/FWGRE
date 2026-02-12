use crate::entities::Tile;
use tracing::info;

pub mod load;

pub const CHUNK_SIZE: u32 = 64;

pub struct World {
    pub width: u32,
    pub height: u32,
    pub terrain: Vec<Tile>,
    pub buildings: Vec<Tile>,

    pub dirty_chunks: Vec<usize>,
}