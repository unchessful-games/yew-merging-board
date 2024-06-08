use board::Board;
use board_repr::BoardRepr;
use pieces::{movement::Move, Color};
use yew::prelude::*;
use yew_merging_board::*;

#[function_component]
fn App() -> Html {
    let board_state = use_state(|| BoardRepr::default());
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
            <p>{"White king in check: "}{board_state.king_in_check(yew_merging_board::pieces::Color::White)}</p>
            <p>{"Black king in check: "}{board_state.king_in_check(yew_merging_board::pieces::Color::Black)}</p>
            <code><pre>
                {format!("{:#?}", *board_state)}
            </pre></code>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
