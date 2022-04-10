//! System to handling changing between states
use crate::prelude::*;


#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState){
    let new_state = match turn_state {
        TurnState::AwaitingInput => return, // game is awaiting input, no change in state
        TurnState::PlayerTurn => TurnState::MonsterTurn, // next state is monster turn
        TurnState::MonsterTurn => TurnState::AwaitingInput // after monster turn is awaiting input state
    };

    // update global resource state
    *turn_state = new_state;
}