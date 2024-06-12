use api::{MatchmakingCounts, MatchmakingFoundGame, MatchmakingWsServerMessage};
use axum::{extract::State, Json};
use tokio::sync::oneshot;

use crate::managers::AppState;

#[derive(Debug, Clone)]
pub struct Handle {
    pub sender: tokio::sync::mpsc::Sender<MatchmakerRequest>,
}

impl Handle {
    pub async fn ask_for_counts(&self) -> MatchmakingCounts {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .send(MatchmakerRequest::AskForCounts(sender))
            .await
            .expect("Matchmaker seems to have died");
        receiver.await.expect("Matchmaker died")
    }

    pub async fn join_matchmaking_pool(
        &self,
        connection_id: &str,
    ) -> oneshot::Receiver<MatchmakingFoundGame> {
        let (sender, receiver) = oneshot::channel();
        self.sender
            .send(MatchmakerRequest::JoinMatchmakingPool {
                connection_id: connection_id.to_string(),
                on_found_game: sender,
            })
            .await
            .expect("Matchmaker seems to have died");
        receiver
    }

    pub async fn leave_matchmaking_pool(&self, connection_id: String) {
        self.sender
            .send(MatchmakerRequest::LeaveMatchmakingPool { connection_id })
            .await
            .expect("Matchmaker seems to have died");
    }
}

pub enum MatchmakerRequest {
    AskForCounts(oneshot::Sender<MatchmakingCounts>),
    JoinMatchmakingPool {
        connection_id: String,
        on_found_game: oneshot::Sender<MatchmakingFoundGame>,
    },
    LeaveMatchmakingPool {
        connection_id: String,
    },
}

/// Return the current player counts for matchmaking
pub async fn counts() -> Json<MatchmakingCounts> {
    Json(MatchmakingCounts {
        waiting_for_game: 0,
        games_running: 0,
    })
}

pub async fn handle_matchmaking_request(
    State(state): State<AppState>,
    ws: axum::extract::ws::WebSocketUpgrade,
) -> axum::response::Response {
    ws.on_upgrade(move |ws| async { handle_matchmaking_request_inner(ws, state).await })
}

async fn handle_matchmaking_request_inner(mut ws: axum::extract::ws::WebSocket, state: AppState) {
    let connection_id = uuid::Uuid::new_v4().to_string();
    let handle = state.matchmaker_handle;
    let mut found_game = handle.join_matchmaking_pool(&connection_id).await;
    loop {
        let msg;
        tokio::select! {
            rcv = ws.recv() => {
                msg = rcv.map(|v| v.expect("Failed to receive message"));
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(500)) => {
                // Timeout
                msg = None;
            }
        };

        if let Ok(game) = found_game.try_recv() {
            ws.send(axum::extract::ws::Message::Text(
                serde_json::to_string(&MatchmakingWsServerMessage::FoundGame(game)).unwrap(),
            ))
            .await
            .unwrap();
            ws.close().await.unwrap();
            return;
        }

        match msg {
            None => {
                ws.send(axum::extract::ws::Message::Text(
                    serde_json::to_string(&MatchmakingWsServerMessage::Counts(
                        handle.ask_for_counts().await,
                    ))
                    .unwrap(),
                ))
                .await
                .unwrap();
            }
            Some(axum::extract::ws::Message::Text(text)) => {
                tracing::info!("Received message: {}", text);
            }
            Some(axum::extract::ws::Message::Close(reason)) => {
                tracing::info!("Received close: {:?}", reason);
                handle.leave_matchmaking_pool(connection_id).await;
                return;
            }
            _ => {}
        }
    }
}
