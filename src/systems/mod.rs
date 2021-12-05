mod player_input; // accessible via player_input::*
mod map_render;
mod entity_render;
mod collisions;
mod random_move;
mod end_turn;
mod movement;

use crate::prelude::*;

// uses builder pattern to build a Schedule - an execution play for our systems
// when a system executes commands they don't take effect immediately, a hidden flush
// at end tells Legion to apply changes, we can manually call this ourselves with 'flush()'

// separate scheduler for each input state

pub fn build_input_scheduler() -> Schedule {
    // while awaiting input the screen still needs to display the map and entities
    // also calls the player_input system
    Schedule::builder()
        .add_system(player_input::player_input_system()) // the system macro appends _system to functions name
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    // when it's the players turn the game doesn't accept input, but checks for collisions
    // as well as rendering everything.

    Schedule::builder()
        .add_system(movement::movement_system())
        .flush() // apply changes in command buffer from movement system
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system()) // change state from awaiting to player turn
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    // during monster turn, collisions are modelled and everything is rendered + monsters move randomly
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush() // apply changes in it command buffer from movement system
        .add_system(random_move::random_move_system()) // move is placed before collision system
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}