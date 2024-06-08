use merging_board_logic::*;
use pieces::movement::get_all_legal_moves;
pub trait Engine: Clone + Send + Sync {
    fn new() -> Self;

    fn think(&mut self, board_repr: &board_repr::BoardRepr) -> pieces::movement::Move;
}

#[derive(Clone)]
pub struct FirstMove {}

impl Engine for FirstMove {
    fn new() -> Self {
        Self {}
    }
    fn think(&mut self, board_repr: &board_repr::BoardRepr) -> pieces::movement::Move {
        let moves = get_all_legal_moves(board_repr, board_repr.side_to_move);

        moves[0]
    }
}
