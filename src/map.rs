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

pub fn map_idx(x: i32, y: i32) -> usize {
    // get vector index via 2d map index values
    ((y * SCREEN_WIDTH) + x ) as usize // cast to usize since vectors as indexed by variable of type usize
}


pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {

    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],  // a map entirely of floors
        }
    }

    pub fn render(&self, ctx: &mut BTerm){
        for y in 0..SCREEN_HEIGHT { // outer loop on y index, faster for row-first striding due to memory cache usage
            for x in 0..SCREEN_WIDTH {

                let idx = map_idx(x, y);

                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    },
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    },
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        // check that given point is within the boundary of the map
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >=0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_title(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x,point.y)]==TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point){
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
