pub mod matchmaking;
pub mod play;
pub use matchmaking::*;
use merging_board_logic::pieces::Color;
pub use play::*;

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct GameId(String);

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameTermination {
    Aborted,
    CheckmateBy(Color),
    Stalemate,
    ResignationBy(Color),
    TimeoutBy(Color),
}

impl GameTermination {
    pub fn winner(self) -> Option<Color> {
        match self {
            GameTermination::Aborted => None,
            GameTermination::CheckmateBy(color) => Some(color),
            GameTermination::ResignationBy(color) => Some(color.opposite()),
            GameTermination::TimeoutBy(color) => Some(color.opposite()),
            GameTermination::Stalemate => None,
        }
    }
}
