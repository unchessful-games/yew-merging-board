use api::GameId;
use yew::prelude::*;
use yew_autoprops::autoprops;

#[autoprops]
#[function_component]
pub fn PlayGame(game_id: &GameId, token: &String) -> Html {
    html! {
        <div>
            <h1>{"Game "}{game_id}</h1>
            <p>{"Token: "}{token}</p>
        </div>
    }
}
