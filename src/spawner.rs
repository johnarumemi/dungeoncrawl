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
        Health {
            current: 20,
            max: 20,
        },
        Name(String::from("Main Player"))  // I opted to add this component to player myself manually
    ));
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) { // note that Point's ownership of value is moved into function

    // match can only match on inclusive ranges, hence requires '=' sign
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=4 => goblin(),
        5..=7 => orc(),
        8..=9 => ogre(),
        _ => ettin()
    };

    // add tuple of components the describe the monster entity
    ecs.push((
        Enemy,
        pos, // note that post ownership is moved into the ECS
        Render{
            color: ColorPair::new(WHITE, BLACK),
            glyph
        },
        MovingRandomly,
        Health {current: hp, max: hp},
        Name(name)
        ));
}

fn goblin() -> (i32, String, FontCharType) {
    // hit points, name and reference to an ASCII character
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    // hit points, name and reference to an ASCII character
    (2, "Orc".to_string(), to_cp437('o'))
}

fn ogre() -> (i32, String, FontCharType) {
    (3, "Ogre".to_string(), to_cp437('O'))
}

fn ettin() -> (i32, String, FontCharType) {
    // note that the 2 below is a literal, hence has Copy trait defined
    (3, "Ettin".to_string(), to_cp437('E'))
}
