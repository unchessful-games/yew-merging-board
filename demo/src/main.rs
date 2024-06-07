use board::Board;
use board_repr::BoardRepr;
use pieces::movement::Move;
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
            <Board style="max-width: 50%;" class={"container"} {onmove} as_black={true} board={*board_state}/>
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
