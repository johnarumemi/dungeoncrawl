#![warn(clippy::all, clippy::pedantic)]

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

/// all publicly accessible components of the application
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use super::state::*;
    pub use super::components::*;
    pub use super::spawner::*;
    pub use super::systems::*;
    pub use super::turn_state::*;

    pub use super::camera::*;
    pub use super::map::*;
    pub use super::map_builder::*;

    // world units size
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
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // console for displaying map
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // console for displaying player
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")  // console for displaying HUD
        .build()?;

    main_loop(context, State::new())
}
