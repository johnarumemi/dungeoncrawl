// This module handles spawning entities
use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    // calling push creates an entity composed of the listed components
    ecs.push((
        Player,
        pos, // Legion can accept most types as components, so we use a structure from bracket-lib to represent a Point / Position component
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
