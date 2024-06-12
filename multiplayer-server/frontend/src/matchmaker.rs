use api::{MatchmakingCounts, MatchmakingWsServerMessage};
use yew::prelude::*;
use yew_hooks::use_websocket;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Matchmaker() -> Html {
    html! {
        <div>
            <h1>{"Matchmaker"}</h1>
            // Spinner
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">{"Waiting for match..."}</span>
            </div>

            <MatchmakerInner />

        </div>
    }
}

#[function_component]
fn MatchmakerInner() -> Html {
    let ws = use_websocket("/api/matchmaking/ws".to_string());
    let nav = use_navigator().unwrap();
    let is_closing = use_state(|| false);

    let latest_counts = use_state(MatchmakingCounts::default);

    let status = match *ws.ready_state {
        yew_hooks::UseWebSocketReadyState::Connecting => {
            html!(<div>{"Connecting to matchmaker"}</div>)
        }
        yew_hooks::UseWebSocketReadyState::Open => {
            let latest_counts = latest_counts.clone();
            html! {
                <>
                    <p>{"Players waiting for a match: "}{latest_counts.waiting_for_game}</p>
                    <p>{"Games in progress: "}{latest_counts.games_running}</p>
                </>
            }
        }
        yew_hooks::UseWebSocketReadyState::Closing => html!(<div>{"Closing connection"}</div>),
        yew_hooks::UseWebSocketReadyState::Closed => {
            if *is_closing {
                nav.replace(&Route::Home);
            }
            html!(<div>{"Connection closed"}</div>)
        }
    };

    let begin_exit = {
        let ws = ws.clone();
        let is_closing = is_closing.clone();
        Callback::from(move |_| {
            log::info!("Cancelling matchmaking");
            ws.close();
            is_closing.set(true);
        })
    };

    // On every message change, update the state
    let latest_counts = latest_counts.clone();
    use_effect_with((*ws.message).clone(), {
        let latest_counts = latest_counts.clone();
        let begin_exit = begin_exit.clone();
        move |msg| {
            if let Some(msg) = msg {
                let msg = serde_json::from_str::<MatchmakingWsServerMessage>(&msg)
                    .expect("Received invalid data from matchmaker websocket");

                match msg {
                    MatchmakingWsServerMessage::Counts(counts) => {
                        latest_counts.set(counts);
                    }
                    MatchmakingWsServerMessage::FoundGame(game) => {
                        log::info!("Matchmaking found game: {game:?}");
                        nav.push(&Route::PlayGame {
                            game_id: game.game_id,
                            token: game.token,
                        });
                        begin_exit.emit(());
                    }
                }
            }
        }
    });

    html! {
        <>
            {status}
            <button onclick={move |_| begin_exit.emit(())} class="btn btn-danger" disabled={*is_closing}>
                if *is_closing {
                    <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
                }
                {"Stop searching"}
            </button>
        </>
    }
}
