use board::Board;
use merging_board_logic::board_repr::BoardRepr;
use merging_board_logic::pieces::{movement::Move, Color};
use yew::prelude::*;
use yew_merging_board::*;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

#[function_component]
fn Home() -> Html {
    let board_state = use_state(BoardRepr::default);
    let onmove = {
        let board_state = board_state.clone();
        Callback::from(move |move_: Move| {
            log::info!("Move: {move_:?}");
            let mut state = *board_state;
            state.play(move_).expect("Move from board was illegal");
            board_state.set(state);
        })
    };

    html! {
        <>
            <div class="row">
                <Board style="max-width: 33%;" class={"container"} onmove={onmove.clone()} as_black={false} board={*board_state} interactable={board_state.side_to_move == Color::White}/>
                <Board style="max-width: 33%;" class={"container"} onmove={onmove} as_black={true} board={*board_state} interactable={board_state.side_to_move == Color::Black}/>
            </div>
            <p>{"White king in check: "}{board_state.king_in_check(merging_board_logic::pieces::Color::White)}</p>
            <p>{"Black king in check: "}{board_state.king_in_check(merging_board_logic::pieces::Color::Black)}</p>
            <code><pre>
                {format!("{:#?}", *board_state)}
            </pre></code>
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
    }
}

#[function_component]
fn Root() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Root>::new().render();
}
