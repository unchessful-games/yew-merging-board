use merging_board_logic::{
    board_repr::BoardRepr,
    pieces::{movement::Move, Color},
};
use serde::{Deserialize, Serialize};

use crate::GameId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RunningGameState {
    pub id: GameId,
    pub your_color: Color,
    pub current_game_state: BoardRepr,
    pub move_history: Vec<Move>,
    pub your_clock: std::time::Duration,
    pub their_clock: std::time::Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameStatus {
    Running(RunningGameState),
    Finished(CompletedGameState),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompletedGameState {
    pub id: GameId,
    pub move_history: Vec<Move>,
    pub final_game_state: BoardRepr,
    pub winner: Color,
}
