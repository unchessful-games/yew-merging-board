use merging_board_logic::*;
use minimax_alpha_beta::strategy::alpha_beta_minimax::AlphaBetaMiniMaxStrategy;
use pieces::movement::get_all_legal_moves;
use strategy::MergingChessStrategy;
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

#[derive(Clone)]
pub struct AlphaBetaMinimax {
    depth: i64,
}

impl Engine for AlphaBetaMinimax {
    fn new() -> Self {
        Self { depth: 2 }
    }
    fn think(&mut self, board_repr: &board_repr::BoardRepr) -> pieces::movement::Move {
        let mut strat = MergingChessStrategy::from(*board_repr);
        strat.get_best_move(self.depth, board_repr.side_to_move == pieces::Color::White)
    }
}
