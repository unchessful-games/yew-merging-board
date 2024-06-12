use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GameId(String);

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for GameId {
    fn from(game_id: String) -> Self {
        Self(game_id)
    }
}

impl FromStr for GameId {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl AsRef<str> for GameId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatchmakingFoundGame {
    pub game_id: GameId,
    pub token: String,
}
