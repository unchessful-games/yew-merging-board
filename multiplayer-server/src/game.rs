use api::{GameId, GameTermination};
use axum::{
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use merging_board_logic::pieces::{movement::Move, Color};

use crate::managers::AppState;

#[derive(Clone)]
pub struct Handle {
    pub sender: tokio::sync::mpsc::Sender<GameCommand>,
}

impl Handle {
    pub async fn connect(&self, id: GameId, token: String) -> Option<SingleGameHandle> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender
            .send(GameCommand::Connect { id, token, tx })
            .await
            .unwrap();

        rx.await.unwrap()
    }
}

pub enum GameCommand {
    Create {
        id: GameId,
        white_token: String,
        black_token: String,
    },
    Connect {
        id: GameId,
        token: String,
        tx: tokio::sync::oneshot::Sender<Option<SingleGameHandle>>,
    },
    Terminate {
        id: GameId,
        move_history: Vec<Move>,
        termination: GameTermination,
    },
}

pub enum SingleGameCommand {
    RegisterEventReceiver(),
}

pub struct SingleGameHandle {
    pub tx: tokio::sync::mpsc::Sender<SingleGameCommand>,
    pub side: Color,
}

pub async fn handle_game_request(
    Path((game_id, token)): Path<(GameId, String)>,
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> Response {
    if let Some(handle) = state.game_coordinator_handle.connect(game_id, token).await {
        ws.on_upgrade(|ws| async move { manage_game(ws, handle).await })
    } else {
        (
            StatusCode::FORBIDDEN,
            "Either the game doesn't exist or the token is invalid",
        )
            .into_response()
    }
}

async fn manage_game(ws: WebSocket, game_handle: SingleGameHandle) {
    todo!();
    tokio::select! {
        msg = ws.recv() => {
            handle_msg(&mut ws, &mut game_handle);
        }
    }
}

fn handle_msg(ws: &mut WebSocket, game_handle: &mut SingleGameHandle) {}
