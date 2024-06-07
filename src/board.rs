use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_hooks::use_size;

use crate::board_bg::BoardBackground;
use crate::board_repr::BoardRepr;
use crate::pieces::movement::get_moves_from_square;
use crate::square::Square;

#[derive(Properties, PartialEq, Default)]
pub struct BoardProps {
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub class: AttrValue,

    #[prop_or_default]
    pub board: BoardRepr,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    let BoardProps {
        style,
        class,
        board,
    } = props;

    let wrap_node = use_node_ref();
    let size = use_size(wrap_node.clone());

    let selected_square = use_state(|| None);

    let square_to_transform = |s: Square| {
        let (file, rank) = s.coords();
        // squares start at bottom left,
        // but screen coordinates start at top left.
        // so, Y coordinate is inverted
        // Also, first rank should have value of 1 (not 0 as default),
        // otherwise the top row is empty and one extra row is used at the bottom
        let translate_x = u32::from(file) * size.0 / 8;
        let translate_y = size.0 - (u32::from(rank) + 1) * size.0 / 8;
        format!("transform: translate({translate_x}px, {translate_y}px)")
    };

    let mut pieces = vec![];
    for (square, piece) in board.iter_pieces() {
        pieces.push(html! {
            <piece class={format!("{piece}")} style={square_to_transform(square)}></piece>
        })
    }

    let onclick_square = {
        shadow_clone!(selected_square);
        move |square: Square| {
            log::info!("Clicked square: {square:?}");
            let old_selection = *selected_square;
            match old_selection {
                None => selected_square.set(Some(square)),
                Some(old) if old == square => selected_square.set(None),
                _ => selected_square.set(Some(square)),
            }
        }
    };

    // If a square is selected,
    // display the possible moves
    if let Some(square) = *selected_square {
        let moves = get_moves_from_square(board, square, None);
        for move_ in moves {
            pieces.push(html! {
                <square class="move-dest" style={square_to_transform(move_.to)}></square>
            })
        }
    }

    let style = format!("height: {}px !important; {style}", size.0);
    html! {
        <div class={format!("cg-wrap {class}")} {style} ref={wrap_node}>
            <cg-container>
                <BoardBackground {onclick_square}>
                    {pieces}

                    if let Some(square) = *selected_square {
                        <square class="selected" style={square_to_transform(square)}></square>
                    }
                </BoardBackground>
            </cg-container>
        </div>
    }
}
