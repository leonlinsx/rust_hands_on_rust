#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    GameOver,
    Victory,
}
