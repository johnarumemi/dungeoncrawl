use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// Clone enable deep copy
// Copy changes default behaviour to taking a copy rather than moving when assigning
// PartialEq allows us to compare TileType values with == operator

/// Pre-defined set of tile types on the map
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}


/// convert 2d screen values into and index into the vector containing map tiles
/// uses row-first encoding for striding
///
/// Example
/// for a 5 x 5 grid (screen width = 5)
///
/// y = 1 and x = 2  index into vector of tiles = (1 * screen width) + 2 = 7
///
/// Reciprocal can be found via
///
/// index = 7, screen width = 5
///
/// x = index % width
/// y = index / width
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize // cast to usize since vectors are indexed by variable of type usize
}

/// Abstract that holds a grid of tile
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {

    /// Constructor that generates a Map composed entirely of floor tiles
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],  // [type: num_to_create]
        }
    }

    /// check that given point is within the boundary of the map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// check if point is within bounds of map and tile is also a Floor
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// given a 2d point, return None if not in bounds, else Some(tiles vector index)
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
