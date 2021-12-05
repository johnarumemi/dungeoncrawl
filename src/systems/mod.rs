mod player_input; // accessible via player_input::*
mod map_render;
mod entity_render;
mod collisions;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    // uses builder pattern to build a Schedule - an execution play for our systems
    Schedule::builder()
        .add_system(player_input::player_input_system()) // the system macro appends _system to functions name
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(collisions::collisions_system())
        .build()
}