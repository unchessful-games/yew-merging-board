use serde::{Deserialize, Serialize};

use crate::GameId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchmakingFoundGame {
    pub game_id: GameId,
    pub token: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MatchmakingCounts {
    pub waiting_for_game: u32,
    pub games_running: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MatchmakingWsServerMessage {
    Counts(MatchmakingCounts),
    FoundGame(MatchmakingFoundGame),
}
