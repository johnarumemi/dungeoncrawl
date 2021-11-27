use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// tiles are limited to a pre-defined set of tile types
// Clone enable deep copy
// Copy changes default behaviour when assigning a TileType from one variable to another
// PartialEq allows us to compare TileType values with == operator
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

