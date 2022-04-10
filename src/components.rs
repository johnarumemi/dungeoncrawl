use crate::prelude::*;

// component describes appearance of the entity and hence how to render it
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // ColorPair stores a foreground and background color
    pub glyph: FontCharType, // store single character or glyph
}

// below component 'tag' (empty struct) marks entity as Player, ensure there is only 1 entity like this
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

// below component tag marks entity as an Enemy
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

// entities with below component tag will move randomly
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;


/// Used as a 'message of intent'
/// A messaging system can be built within the ECS by
/// treating each message as its own entity
/// It will need to store references to the entity within Legion to act on
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove { // this component contains data via its fields, we are storing an entity
    pub entity : Entity, // store reference to an entity inside Legion
    pub destination: Point
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);
// instance.0 contains the string
