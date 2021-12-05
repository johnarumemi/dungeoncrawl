mod camera;
mod components;
mod map; // imports map module and sets up the 'map::' prefix
mod map_builder;
mod spawner;

mod systems;

// 'super::' access the module immediately above your module in the tree
// 'crate::' access the root of the tree, main.rs

// Create our own prelude that others can use
mod prelude {
    // use 'mod' to create a new module within the source code
    pub use bracket_lib::prelude::*; // re-export it
    pub use legion::*;

    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

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
use crate::camera::Camera;
use prelude::*;

struct State {
    ecs: World,           // use by Legion for storing all entities and components
    resources: Resources, // shared data between systems
    systems: Schedule,    // holds systems
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default(); // list of resources

        // random number generator for the game
        let mut rng = RandomNumberGenerator::new();

        // map builder
        let map_builder = MapBuilder::new(&mut rng);

        // spawn player entity and add to ecs/World
        spawn_player(&mut ecs, map_builder.player_start);

        // spawn monsters
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Add camera and generated map to list of resources
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
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

        // when we add a resource of the same type, it replaces previous resource of same type
        // hence the below insert will not result in duplicate resources
        self.resources.insert(ctx.key);

        // execute the Schedule to run the various systems
        // Schedule requires access to the World (entities & components) and resources (shared data)
        self.systems.execute(&mut self.ecs, &mut self.resources);

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new() // create generic terminal
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // size of subsequent consoles we add
        .with_tile_dimensions(32, 32) // size of each character in font file
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32) // fontfile to load and character dimensions
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // second console with no background so transparency shows through it
        .build()?;

    main_loop(context, State::new())
}
