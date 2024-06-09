use minimax_alpha_beta::strategy::game_strategy::GameStrategy;

use crate::{
    board_repr::BoardRepr,
    pieces::{
        movement::{get_all_legal_moves, Move},
        Color,
    },
};

pub struct MergingChessStrategy {
    board: BoardRepr,

    prev_boards: Vec<(BoardRepr, Move)>,
}

impl From<BoardRepr> for MergingChessStrategy {
    fn from(board: BoardRepr) -> Self {
        Self {
            board,
            prev_boards: Vec::new(),
        }
    }
}

impl GameStrategy for MergingChessStrategy {
    type Player = Color;

    type Move = Move;

    type Board = BoardRepr;

    fn evaluate(&self) -> f64 {
        // Return the amount of material advantage that the white player has.
        let mut material = 0.0;

        for (_square, piece) in self.board.iter_pieces() {
            let mut value: f64 = 0.0;
            piece.piece().for_components(|component| {
                value += match component {
                    crate::pieces::UnitaryPiece::Queen => 9.0,
                    crate::pieces::UnitaryPiece::Bishop => 3.0,
                    crate::pieces::UnitaryPiece::Knight => 3.0,
                    crate::pieces::UnitaryPiece::Rook => 5.0,
                    crate::pieces::UnitaryPiece::Pawn => 1.0,
                    crate::pieces::UnitaryPiece::King => 1000.0,
                }
            });
            match piece.color() {
                Color::White => material += value,
                Color::Black => material -= value,
            }
        }

        log::debug!("Material advantage: {material}");
        -material
    }

    fn get_winner(&self) -> Option<Self::Player> {
        // If a player has no legal moves, and is in check,
        // the other player is the winner
        for player in [Color::White, Color::Black] {
            let legal_moves = get_all_legal_moves(&self.board, player);
            if legal_moves.is_empty() && self.board.king_in_check(player) {
                return Some(player.opposite());
            }
        }
        None
    }

    fn is_game_tied(&self) -> bool {
        // The game is a draw:
        // - when the player to move has no legal moves, and is not in check (stalemate)
        // - when both players have insufficient material
        if self.get_available_moves().is_empty() {
            return true;
        }

        if self
            .board
            .has_insufficient_material(self.board.side_to_move)
        {
            return true;
        }

        false
    }

    fn is_game_complete(&self) -> bool {
        self.is_game_tied() || self.get_winner().is_some()
    }

    fn get_available_moves(&self) -> Vec<Self::Move> {
        let legal_moves = get_all_legal_moves(&self.board, self.board.side_to_move);
        legal_moves.into_iter().collect()
    }

    fn play(&mut self, mv: &Self::Move, _maximizer: bool) {
        self.prev_boards.push((self.board, *mv));
        self.board
            .play(*mv)
            .expect("alpha-beta algorithm produced illegal move");
    }

    fn clear(&mut self, mv: &Self::Move) {
        let last = self.prev_boards.last().unwrap();
        if last.1 == *mv {
            self.board = last.0;
            self.prev_boards.pop();
        } else {
            panic!("Attempted to clear a move that was not played: {mv:?}");
        }
    }

    fn get_board(&self) -> &Self::Board {
        &self.board
    }

    fn is_a_valid_move(&self, mv: &Self::Move) -> bool {
        let legal_moves = self.get_available_moves();
        legal_moves.contains(mv)
    }

    fn get_a_sentinel_move(&self) -> Self::Move {
        // A move that moves from A1 to H7 is guaranteed to be invalid:
        // it is not a proper diagonal, so no piece (or piece combination) can
        // move this way.
        Move {
            from: crate::square::Square::A1,
            to: crate::square::Square::H7,
            which_half: None,
        }
    }
}
