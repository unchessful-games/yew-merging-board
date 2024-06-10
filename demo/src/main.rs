use board::Board;
use merging_board_logic::board_repr::BoardRepr;
use merging_board_logic::pieces::movement::find_any_legal_move;
use merging_board_logic::pieces::{movement::Move, Color};
use merging_engine::{AlphaBetaMinimax, Engine};
use yew::prelude::*;
use yew_merging_board::*;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/engine")]
    Engine,

    #[at("/for-screenshot")]
    ForScreenshot,
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

#[function_component]
fn EngineDemo() -> Html {
    let board_state = use_state(BoardRepr::default);
    let engine = use_state(AlphaBetaMinimax::new);
    let keep_playing = use_state(|| true);
    let onmove = {
        let board_state = board_state.clone();
        let engine = engine.clone();
        let keep_playing = keep_playing.clone();
        Callback::from(move |move_: Move| {
            log::info!("Move: {move_:?}");
            let mut state = *board_state;
            state.play(move_).expect("Move from board was illegal");
            board_state.set(state);

            // Check if the engine has any legal moves
            if find_any_legal_move(&state, Color::Black).is_none() {
                keep_playing.set(false);
                return;
            }

            // Find a move from the engine and play it

            let mut engine_val = (*engine).clone();
            let engine_move = engine_val.think(&state);
            state
                .play(engine_move)
                .expect("Move from engine was illegal");

            engine.set(engine_val);
            board_state.set(state);

            // Check if the player has any legal moves
            if find_any_legal_move(&state, Color::White).is_none() {
                keep_playing.set(false);
            }
        })
    };

    html! {
        <>
            <div class="row">
                <Board style="max-width: 33%;" class={"container"} onmove={onmove.clone()} as_black={false} board={*board_state} interactable={board_state.side_to_move == Color::White && *keep_playing}/>
            </div>
            <p>{"White king in check: "}{board_state.king_in_check(merging_board_logic::pieces::Color::White)}</p>
            <p>{"Black king in check: "}{board_state.king_in_check(merging_board_logic::pieces::Color::Black)}</p>
            <code><pre>
                {format!("{:#?}", *board_state)}
            </pre></code>
        </>
    }
}

#[function_component]
fn ForScreenshot() -> Html {
    use merging_board_logic::pieces;
    use merging_board_logic::square::Square;
    use pieces::{ColorPiece, CombinationPiece, Piece, UnitaryPiece};
    use UnitaryPiece::*;
    let mut board = BoardRepr::empty();
    board[Square::E4] = Some(ColorPiece::White(Piece::Combination(
        CombinationPiece::new(Queen, Knight).unwrap(),
    )));
    board[Square::A1] = Some(ColorPiece::White(Piece::Unitary(King)));
    board[Square::H8] = Some(ColorPiece::Black(Piece::Unitary(King)));
    html! {
        <>
            <Board style="width: 50%;" class={"container"} as_black={false} {board} interactable={true}/>
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Engine => html! { <EngineDemo /> },
        Route::ForScreenshot => html! { <ForScreenshot /> },
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
