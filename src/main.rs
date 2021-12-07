mod state;
mod camera;
mod components;
mod map; // imports map module and sets up the 'map::' prefix
mod map_builder;
mod spawner;
mod turn_state;

mod systems;

// 'super::' access the module immediately above your module in the tree
// 'crate::' access the root of the tree, main.rs

/*
Create an inline module rather than one defined in a .rs file
 */
mod prelude {
    // use 'mod' to create a new module within the source code
    pub use bracket_lib::prelude::*; // re-export it
    pub use legion::*;

    pub use crate::state::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub use crate::camera::*;
    pub use crate::map::*; // use 'crate::' to reference the current module scope and re-export map with 'pub'
    pub use crate::map_builder::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    // viewport/camera size
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

// use our created prelude within this module's main scope
use prelude::*;

fn main() -> BError {
    let context = BTermBuilder::new() // create generic terminal
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // size of subsequent consoles we add
        .with_tile_dimensions(32, 32) // size of each character in font file
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // second console with no background so transparency shows through it
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_WIDTH * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
