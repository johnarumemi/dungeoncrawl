use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,  // works on its on map and then after building it, pass result to the game
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {

        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);

        mb.player_start = mb.rooms[0].center();  // starting location is in center of one of the rooms

        mb
    }

    fn fill(self: &mut Self, tile: TileType){
        // 1. obtain a mutable iterator
        // 2. use for_each to change every tile into a wall (or whatever arg is passed into 'tile')
        // 3. deference with '*' to write to the reference value, not to the reference
        self.map.tiles.iter_mut().for_each(|t|*t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator){
        // use same random number generator throughout map generation to ensure that
        // for a given seed you get the same map

        while self.rooms.len()  < NUM_ROOMS { // keep generating rooms until there are NUM_ROOMS on the map

            // generate randomly sized room with a random location
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH -10),  // random x position of room
                rng.range(1, SCREEN_HEIGHT -10), // random y position of room
                rng.range(2, 10),    // width of room
                rng.range(2,10), // height of room
            );

            let mut overlap = false;

            for r in self.rooms.iter(){ // test the room against each placed room to see if they overlap
                if r.intersect(&room){
                    overlap = true;
                }
            }

            // if room does not overlap check that all its points are within map boundaries
            // and set each tile to 'floor'
            if !overlap {
                room.for_each(|p| { // iterate over every Point within the room
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y); // use room's Point get get corresponding map index
                        self.map.tiles[idx] = TileType::Floor; // set tile at index to 'Floor'
                    }
                });

                self.rooms.push(room)
            }
        }
    }


    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32){
        use std::cmp::{min, max}; // bring min and max into scope

        // range iterators expect starting value to be minimum and end value to be maximum
        // iterate from start to end of corridor and change tile to 'Floor'
        for y in min(y1, y2)..=max(y1, y2){

            if let Some(idx) = self.map.try_idx(Point::new(x, y)){
                self.map.tiles[idx as usize] = TileType::Floor;
            }

        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32){
        use std::cmp::{min, max};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)){
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator){
        let mut rooms = self.rooms.clone();

        // sort rooms in-place based on their center points
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1){

            // center of previous room (hence why we skipped first entry)
            let prev = rooms[i-1].center();

            // center of new/current room
            let current = room.center();

            // randomly dig the horizontal then vertical corridors or vice versa
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, current.x, prev.y);
                self.apply_vertical_tunnel(prev.y, current.y, current.x);
            } else {
                self.apply_vertical_tunnel(prev.y, current.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, current.x, current.y);
            }

        }
    }
}