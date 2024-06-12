use api::GameId;
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_hooks::use_websocket;

#[autoprops]
#[function_component]
pub fn PlayGame(game_id: &GameId, token: &String) -> Html {
    let ws = use_websocket(format!("/api/game/{game_id}/{token}"));
    let status = match *ws.ready_state {
        yew_hooks::UseWebSocketReadyState::Connecting => {
            html!(<div>{"Connecting to game..."}<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span></div>)
        }
        yew_hooks::UseWebSocketReadyState::Open => {
            html!(<div>{"Connected to game"}<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span></div>)
        }
        yew_hooks::UseWebSocketReadyState::Closing => {
            html!(<div>{"Closing connection"}<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span></div>)
        }
        yew_hooks::UseWebSocketReadyState::Closed => {
            html!(<div>{"Connection closed"}<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span></div>)
        }
    };

    html! {
        <div>
            <h1>{"Game "}{game_id}</h1>
            {status}
        </div>
    }
}
