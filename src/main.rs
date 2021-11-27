mod map; // imports map module and sets up the 'map::' prefix

// 'super::' access the module immediately above your module in the tree
// 'crate::' access the root of the tree, main.rs

// Create our own prelude that others can use
mod prelude { // use 'mod' to create a new module within the source code
    pub use bracket_lib::prelude::*; // re-export it
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;  // use 'crate::' to reference the current module scope and re-export map with 'pub'
}

// use our created prelude within this module's main scope
use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(0, "Hello Dungeon Crawler!");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .build()?;

    main_loop(context, State{})
}
