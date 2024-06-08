use shadow_clone::shadow_clone;
use yew::prelude::*;
use yew_hooks::use_size;

use crate::board_bg::BoardBackground;
use crate::board_repr::BoardRepr;
use crate::pieces::movement::{get_legal_moves_from_square, Move};
use crate::pieces::{Color, PieceHalf};
use crate::square::Square;

#[derive(Properties, PartialEq, Default)]
pub struct BoardProps {
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub class: AttrValue,

    #[prop_or_default]
    pub board: BoardRepr,

    #[prop_or(true)]
    pub interactable: bool,

    #[prop_or_default]
    pub onmove: Callback<Move>,

    #[prop_or_default]
    pub as_black: bool,
}

#[function_component]
pub fn Board(props: &BoardProps) -> Html {
    let BoardProps {
        style,
        class,
        board,
        onmove,
        as_black,
        interactable,
    } = props;

    let wrap_node = use_node_ref();
    let size = use_size(wrap_node.clone());

    let selected_square = use_state(|| None);
    let combo_selection = use_state(|| None);

    let square_to_transform = |s: Square| {
        // If displaying as black, rotate 180 degrees
        let s = if *as_black { s.rotate_180() } else { s };
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
        let class = if Some(square) == *selected_square {
            let selection_phase = match *combo_selection {
                Some(PieceHalf::Left) => "left",
                Some(PieceHalf::Right) => "right",
                None => "full",
            };
            format!("{piece} selected {selection_phase}")
        } else {
            format!("{piece}")
        };
        pieces.push(html! {
            <piece class={class} style={square_to_transform(square)}></piece>
        })
    }

    let onclick_square = {
        shadow_clone!(
            selected_square,
            board,
            as_black,
            combo_selection,
            interactable
        );
        move |clicked_square: Square| {
            // If the board is not interactable, do nothing
            if !interactable {
                return;
            }

            // rotate 180 degrees if displaying black pieces
            let clicked_square = if as_black {
                clicked_square.rotate_180()
            } else {
                clicked_square
            };
            log::info!("Clicked square: {clicked_square:?}");

            // If we clicked on an empty square, clear the selection
            if board[clicked_square].is_none() {
                selected_square.set(None);
                combo_selection.set(None);
                return;
            }

            // If nothing was selected, and we clicked on our piece,
            // select it
            let old_selection = *selected_square;
            if old_selection.is_none() {
                if board[clicked_square].map(|p| p.color()) == Some(board.side_to_move) {
                    selected_square.set(Some(clicked_square));
                    combo_selection.set(None);
                    return;
                }
            }

            // If a piece was already selected:
            if let Some(old_selection) = old_selection {
                // If we're clicking on the same piece again:
                if old_selection == clicked_square {
                    // If the piece is unitary, clear the selection
                    if let Some(piece) = board[old_selection] {
                        if let crate::pieces::Piece::Unitary(_) = piece.piece() {
                            selected_square.set(None);
                            return;
                        } else {
                            // If the piece is a combo, advance the state of combo selection
                            match *combo_selection {
                                None => combo_selection.set(Some(PieceHalf::Left)),
                                Some(PieceHalf::Left) => {
                                    combo_selection.set(Some(PieceHalf::Right))
                                }
                                Some(PieceHalf::Right) => {
                                    selected_square.set(None);
                                    combo_selection.set(None);
                                }
                            }
                        }
                    }
                } else {
                    // If we're clicking on a different place:
                    // if the other place contains an enemy piece, clear the selection
                    if let Some(piece) = board[clicked_square] {
                        if piece.color() != board.side_to_move {
                            selected_square.set(None);
                            combo_selection.set(None);
                            return;
                        }
                    }
                    // Otherwise, select it
                    selected_square.set(Some(clicked_square));
                    combo_selection.set(None);

                    return;
                }
            }

            // match old_selection {
            //     None => selected_square.set(Some(clicked_square)),
            //     Some(old) if old == clicked_square => selected_square.set(None),
            //     _ => selected_square.set(Some(clicked_square)),
            // }
        }
    };

    // If a square is selected,
    // display the possible moves
    if let Some(square) = *selected_square {
        let onmove_wrapper = {
            shadow_clone!(onmove, selected_square);
            Callback::from(move |move_: Move| {
                selected_square.set(None);
                onmove.emit(move_);
            })
        };

        let moves = get_legal_moves_from_square(board, square, *combo_selection);
        for move_ in moves {
            let onclick = {
                shadow_clone!(onmove_wrapper);
                Callback::from(move |ev: MouseEvent| {
                    ev.prevent_default();
                    onmove_wrapper.emit(move_)
                })
            };
            let cls = if board[move_.to].is_some() {
                "move-dest oc"
            } else {
                "move-dest"
            };
            pieces.push(html! {
                <square class={cls} style={square_to_transform(move_.to)} {onclick}></square>
            })
        }
    }

    // If either side is in check,
    // display the check indicator
    // on the king's square
    if board.king_in_check(Color::White) {
        pieces.push(html! {
            <square class="check" style={square_to_transform(board.king_square(Color::White))}></square>
        });
    }

    if board.king_in_check(Color::Black) {
        pieces.push(html! {
            <square class="check" style={square_to_transform(board.king_square(Color::Black))}></square>
        });
    }

    // If there is a previous move,
    // display it
    if let Some(prev_move) = board.previous_move {
        let src = prev_move.from;
        let dst = prev_move.to;
        pieces.push(html! {
            <>
                <square class="last-move" style={square_to_transform(src)}></square>
                <square class="last-move" style={square_to_transform(dst)}></square>
            </>
        });
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
