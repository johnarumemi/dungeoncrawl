
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,  // ~ unit struct
    PlayerTurn,
    MonsterTurn,
}
