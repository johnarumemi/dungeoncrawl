mod map; // imports map module and sets up the 'map::' prefix
mod player;
mod map_builder;
mod camera;

// 'super::' access the module immediately above your module in the tree
// 'crate::' access the root of the tree, main.rs

// Create our own prelude that others can use
mod prelude { // use 'mod' to create a new module within the source code
    pub use bracket_lib::prelude::*; // re-export it
    pub use crate::map::*;  // use 'crate::' to reference the current module scope and re-export map with 'pub'
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    // viewport/camera size
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

// use our created prelude within this module's main scope
use prelude::*;
use crate::camera::Camera;

struct State {
    map: Map,
    player: Player,
    camera: Camera
}

impl State {

    fn new() -> Self {
        // random number generator for the game
        let mut rng = RandomNumberGenerator::new();

        // map builder
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map, // use the generated map from the map builder
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear screen and layers
        ctx.set_active_console(0);
        ctx.cls();

        ctx.set_active_console(1);
        ctx.cls();


        // do updates first
        self.player.update(ctx, &self.map, &mut self.camera);

        // now render objects
        // call map's render function
        self.map.render(ctx, &self.camera);

        self.player.render(ctx, &self.camera);

    }
}

fn main() -> BError {

    let context = BTermBuilder::new() // create generic terminal
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // size of subsequent consoles we add
        .with_tile_dimensions(32, 32)// size of each character in font file
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) // fontfile to load and character dimensions
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")// second console with no background so transparency shows through it
        .build()?;

    main_loop(context, State::new())
}
