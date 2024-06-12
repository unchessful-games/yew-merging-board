use api::{GameId, GameTermination, MatchmakingCounts, MatchmakingFoundGame};
use merging_board_logic::pieces::Color;
use tokio::sync::oneshot;

use crate::game::SingleGameHandle;

#[derive(Clone)]
pub struct AppState {
    pub matchmaker_handle: crate::matchmaking::Handle,
    pub game_coordinator_handle: crate::game::Handle,
}

pub fn launch() -> AppState {
    let matchmaker_handle = launch_matchmaking();

    let game_coordinator_handle = launch_game_coordinator();

    AppState {
        matchmaker_handle,
        game_coordinator_handle,
    }
}

fn launch_matchmaking() -> crate::matchmaking::Handle {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    tokio::spawn(matchmaking_manager(rx));

    crate::matchmaking::Handle { sender: tx }
}

async fn matchmaking_manager(
    mut rx: tokio::sync::mpsc::Receiver<crate::matchmaking::MatchmakerRequest>,
) {
    let mut pool = std::collections::HashMap::new();
    let mut matchmaking_interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        let msg = tokio::select! {
            _ = matchmaking_interval.tick() => {
                run_matchmaking(&mut pool);
                None
            }
            msg = rx.recv() => msg
        };

        let msg = match msg {
            Some(msg) => msg,
            None => {
                continue;
            }
        };
        match msg {
            crate::matchmaking::MatchmakerRequest::AskForCounts(tx) => {
                // TODO: real values
                tx.send(MatchmakingCounts {
                    waiting_for_game: pool.len() as u32,
                    games_running: 420,
                })
                .unwrap();
            }
            crate::matchmaking::MatchmakerRequest::JoinMatchmakingPool {
                connection_id,
                on_found_game,
            } => {
                pool.insert(connection_id, on_found_game);
            }
            crate::matchmaking::MatchmakerRequest::LeaveMatchmakingPool { connection_id } => {
                pool.remove(&connection_id);
            }
        }
    }
}

fn run_matchmaking(
    pool: &mut std::collections::HashMap<String, oneshot::Sender<MatchmakingFoundGame>>,
) {
    // Randomly iterate over the pool.
    // For every two connections,
    // generate a game ID and two player tokens,
    // then send them to the two senders.

    let mut pairs: Vec<(String, String)> = vec![];
    let mut item: Option<String> = None;

    for x in pool.keys() {
        if let Some(y) = item.take() {
            pairs.push((x.clone(), y.clone()));
        } else {
            item = Some(x.clone());
        }
    }

    for (a, b) in pairs {
        let game_id: GameId = nanoid::nanoid!().into();
        let white_token = nanoid::nanoid!(5);
        let black_token = nanoid::nanoid!(5);

        let tx1 = pool.remove(&a).unwrap();
        let tx2 = pool.remove(&b).unwrap();
        tx1.send(MatchmakingFoundGame {
            game_id: game_id.clone(),
            token: white_token,
        })
        .unwrap();
        tx2.send(MatchmakingFoundGame {
            game_id,
            token: black_token,
        })
        .unwrap();
    }
}

fn launch_game_coordinator() -> crate::game::Handle {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    tokio::spawn(game_manager(rx, tx.clone()));

    crate::game::Handle { sender: tx }
}

async fn game_manager(
    mut rx: tokio::sync::mpsc::Receiver<crate::game::GameCommand>,
    tx: tokio::sync::mpsc::Sender<crate::game::GameCommand>,
) {
    let mut game_pool = std::collections::HashMap::new();

    struct Game {
        white_token: String,
        black_token: String,
        tx: tokio::sync::mpsc::Sender<crate::game::SingleGameCommand>,
    }

    loop {
        let msg = rx.recv().await;
        let msg = match msg {
            Some(msg) => msg,
            None => {
                continue;
            }
        };
        match msg {
            crate::game::GameCommand::Create {
                id,
                white_token,
                black_token,
            } => {
                let (single_tx, single_rx) = tokio::sync::mpsc::channel(100);
                tokio::spawn(single_game_manager(single_rx, id.clone(), tx.clone()));
                game_pool.insert(
                    id,
                    Game {
                        white_token,
                        black_token,
                        tx: single_tx.clone(),
                    },
                );
            }
            crate::game::GameCommand::Connect { id, token, tx } => {
                // If the game is in the pool,
                // send the connection to the game.
                // Otherwise send nothing.
                let _ = if let Some(game) = game_pool.get(&id) {
                    let send = if token == game.white_token || token == game.black_token {
                        Some(SingleGameHandle {
                            tx: game.tx.clone(),
                            side: if token == game.white_token {
                                Color::White
                            } else {
                                Color::Black
                            },
                        })
                    } else {
                        None
                    };
                    tx.send(send)
                } else {
                    tx.send(None)
                };
            }
            crate::game::GameCommand::Terminate {
                id,
                move_history,
                termination,
            } => {
                // TODO: save the game to disk
                game_pool.remove(&id);
            }
        }
    }
}

async fn single_game_manager(
    mut rx: tokio::sync::mpsc::Receiver<crate::game::SingleGameCommand>,
    id: GameId,
    tx: tokio::sync::mpsc::Sender<crate::game::GameCommand>,
) {
    // Immediately close the game

    tx.send(crate::game::GameCommand::Terminate {
        id,
        move_history: vec![],
        termination: GameTermination::Aborted,
    })
    .await
    .unwrap();
}
