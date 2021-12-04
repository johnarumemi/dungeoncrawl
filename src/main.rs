mod map; // imports map module and sets up the 'map::' prefix
mod player;
mod map_builder;

// 'super::' access the module immediately above your module in the tree
// 'crate::' access the root of the tree, main.rs

// Create our own prelude that others can use
mod prelude { // use 'mod' to create a new module within the source code
    pub use bracket_lib::prelude::*; // re-export it
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;  // use 'crate::' to reference the current module scope and re-export map with 'pub'
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

// use our created prelude within this module's main scope
use prelude::*;

struct State {
    map: Map,
    player: Player
}

impl State {

    fn new() -> Self {
        // random number generator for the game
        let mut rng = RandomNumberGenerator::new();

        // map builder
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map, // use the generated map from the map builder
            player: Player::new(map_builder.player_start) //
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear screen

        // do updates first
        self.player.update(ctx, &self.map);

        // now render objects
        // call map's render function
        self.map.render(ctx);

        self.player.render(ctx);

    }
}

fn main() -> BError {

    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;

    main_loop(context, State::new())
}
