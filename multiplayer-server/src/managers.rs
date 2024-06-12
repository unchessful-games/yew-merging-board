use api::{GameId, MatchmakingCounts, MatchmakingFoundGame};
use tokio::sync::oneshot;

#[derive(Clone)]
pub struct AppState {
    pub matchmaker_handle: crate::matchmaking::Handle,
}

pub fn launch() -> AppState {
    let matchmaker_handle = launch_matchmaking();

    AppState { matchmaker_handle }
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
