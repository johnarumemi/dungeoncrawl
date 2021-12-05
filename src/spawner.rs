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

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Render{
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                // various types of monsters
                0 => to_cp437('E'), // Ettin (two headed giant
                1 => to_cp437('O'), // Ogre
                2 => to_cp437('o'), // orc
                _ => to_cp437('g'), // goblin
            }
        },
        MovingRandomly
        ));
}