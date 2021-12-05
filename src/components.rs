use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render { // describes appearance of the entity
    pub color: ColorPair,    // ColorPair stores a foreground and background color
    pub glyph: FontCharType, // store single character or glyph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // this component struct is essentially a 'tag'

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;
