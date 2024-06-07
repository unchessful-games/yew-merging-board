use arrayvec::ArrayVec;

use crate::{
    board_repr::BoardRepr,
    square::{Rank, Square},
};

use super::{Color, ColorPiece, Piece, PieceHalf, UnitaryPiece};

pub type MovesList = ArrayVec<Move, 1024>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub which_half: Option<PieceHalf>,
}

pub fn get_moves_from_square(
    board_repr: &BoardRepr,
    from: Square,
    which_half: Option<PieceHalf>,
) -> MovesList {
    // If there is no piece of the current color on the square, there are no moves
    let piece = if let Some(piece) = board_repr[from] {
        if piece.color() == board_repr.side_to_move {
            piece
        } else {
            return MovesList::new();
        }
    } else {
        return MovesList::new();
    };
    let color = piece.color();
    let piece = piece.piece();

    // If the color of the piece is not the same as the side to move, there are no moves
    if color != board_repr.side_to_move {
        return MovesList::new();
    }

    // If the piece is unitary, then it has no halves
    // If a half is specified in such a case, then there are no moves
    if which_half.is_some() {
        if piece.is_unitary() {
            return MovesList::new();
        }
    }

    let mut moves = MovesList::new();
    let mut add_unitary = |piece: UnitaryPiece| match piece {
        super::UnitaryPiece::Pawn => {
            get_pawn_moves_from_square(board_repr, from, which_half, &mut moves)
        }
        super::UnitaryPiece::King => {
            get_king_moves_from_square(board_repr, from, which_half, &mut moves)
        }
        super::UnitaryPiece::Queen => {
            get_rook_moves_from_square(board_repr, from, which_half, &mut moves);
            get_bishop_moves_from_square(board_repr, from, which_half, &mut moves);
        }
        super::UnitaryPiece::Rook => {
            get_rook_moves_from_square(board_repr, from, which_half, &mut moves)
        }
        super::UnitaryPiece::Bishop => {
            get_bishop_moves_from_square(board_repr, from, which_half, &mut moves)
        }
        super::UnitaryPiece::Knight => {
            get_knight_moves_from_square(board_repr, from, which_half, &mut moves)
        }
    };

    // If the piece is unitary, then we just return the list matching its state
    if let Piece::Unitary(piece) = piece {
        add_unitary(piece);
    }

    // If the piece is not unitary, then we add the moves for the specified half
    // (or for the piece together)
    if let Piece::Combination(combo) = piece {
        if let Some(half) = which_half {
            add_unitary(combo[half]);
        } else {
            for half in [PieceHalf::Left, PieceHalf::Right] {
                add_unitary(combo[half]);
            }
        }
    }

    moves
}

fn get_pawn_moves_from_square(
    board_repr: &BoardRepr,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    match board_repr.side_to_move {
        Color::White => {
            // If the square up is empty, then the move is valid
            if let Some(up) = from.up() {
                if board_repr[up].is_none() {
                    moves.push(Move {
                        from,
                        to: up,
                        which_half,
                    });
                }

                // If the square up up is empty, and it is the second rank,
                // then the move is valid
                if let Some(up_up) = up.up() {
                    if board_repr[up_up].is_none() && from.rank() == Rank::Second {
                        moves.push(Move {
                            from,
                            to: up_up,
                            which_half,
                        });
                    }
                }

                // If the square up left is occupied by an enemy piece
                if let Some(left) = up.left() {
                    if let Some(piece) = board_repr[left] {
                        if piece.color() != board_repr.side_to_move {
                            moves.push(Move {
                                from,
                                to: left,
                                which_half,
                            });
                        }
                    }
                }

                // If the square up right is occupied by an enemy piece
                if let Some(right) = up.right() {
                    if let Some(piece) = board_repr[right] {
                        if piece.color() != board_repr.side_to_move {
                            moves.push(Move {
                                from,
                                to: right,
                                which_half,
                            });
                        }
                    }
                }

                // If this piece is unitary, and the piece up right or up left is also unitary,
                // then the move is valid (it will merge)
                if let Some(piece) = board_repr[from] {
                    if let Piece::Unitary(UnitaryPiece::Pawn) = piece.piece() {
                        if let Some(right) = up.right() {
                            if let Some(ColorPiece::White(Piece::Unitary(_))) = board_repr[right] {
                                moves.push(Move {
                                    from,
                                    to: right,
                                    which_half,
                                });
                            }
                        }
                        if let Some(left) = up.left() {
                            if let Some(ColorPiece::White(Piece::Unitary(_))) = board_repr[left] {
                                moves.push(Move {
                                    from,
                                    to: left,
                                    which_half,
                                });
                            }
                        }
                    }
                }

                // If standing next to the en passant square,
                // and the square up in the direction of the en passant square
                // is empty
                if let Some(ep_square) = board_repr.en_passant_square {
                    if let Some(up) = from.up() {
                        if Some(ep_square) == up.left() {
                            if board_repr[up.left().unwrap()].is_none() {
                                moves.push(Move {
                                    from,
                                    to: up.left().unwrap(),
                                    which_half,
                                });
                            }
                        } else if Some(ep_square) == up.right() {
                            if board_repr[up.right().unwrap()].is_none() {
                                moves.push(Move {
                                    from,
                                    to: up.right().unwrap(),
                                    which_half,
                                });
                            }
                        }
                    }
                }
            }
        }

        Color::Black => {
            // If the square down is empty, then the move is valid
            if let Some(down) = from.down() {
                if board_repr[down].is_none() {
                    moves.push(Move {
                        from,
                        to: down,
                        which_half,
                    });
                }

                // If the square down down is empty, and it is the 7th rank,
                // then the move is valid
                if let Some(down_down) = down.down() {
                    if board_repr[down_down].is_none() && from.rank() == Rank::Seventh {
                        moves.push(Move {
                            from,
                            to: down_down,
                            which_half,
                        });
                    }
                }

                // If the square down left is occupied by a piece (friendly or enemy)
                if let Some(left) = down.left() {
                    if let Some(piece) = board_repr[left] {
                        if piece.color() != board_repr.side_to_move {
                            moves.push(Move {
                                from,
                                to: left,
                                which_half,
                            });
                        }
                    }
                }

                // If the square down right is occupied by an enemy piece
                if let Some(right) = down.right() {
                    if let Some(piece) = board_repr[right] {
                        if piece.color() != board_repr.side_to_move {
                            moves.push(Move {
                                from,
                                to: right,
                                which_half,
                            });
                        }
                    }
                }

                // If this piece is unitary, and the piece down right or down left is also unitary,
                // then the move is valid (it will merge)
                if let Some(piece) = board_repr[from] {
                    if let Piece::Unitary(UnitaryPiece::Pawn) = piece.piece() {
                        if let Some(right) = down.right() {
                            if let Some(ColorPiece::Black(Piece::Unitary(_))) = board_repr[right] {
                                moves.push(Move {
                                    from,
                                    to: right,
                                    which_half,
                                });
                            }
                        }
                        if let Some(left) = down.left() {
                            if let Some(ColorPiece::Black(Piece::Unitary(_))) = board_repr[left] {
                                moves.push(Move {
                                    from,
                                    to: left,
                                    which_half,
                                });
                            }
                        }
                    }
                }

                // If standing next to the en passant square,
                // and the square down in the direction of the en passant square
                // is empty
                if let Some(down) = from.down() {
                    if board_repr.en_passant_square == down.left() {
                        if board_repr[down.left().unwrap()].is_none() {
                            moves.push(Move {
                                from,
                                to: down.left().unwrap(),
                                which_half,
                            });
                        }
                    } else if board_repr.en_passant_square == down.right() {
                        if board_repr[down.right().unwrap()].is_none() {
                            moves.push(Move {
                                from,
                                to: down.right().unwrap(),
                                which_half,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn try_add(
    board_repr: &BoardRepr,
    from: Square,
    square: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) -> bool {
    // If the given square is empty, then it's a valid move
    if board_repr[square].is_none() {
        moves.push(Move {
            from,
            to: square,
            which_half,
        });

        true
    } else {
        // If the square up is occupied by an enemy piece, the move is valid
        if let Some(dst_piece) = board_repr[square] {
            if dst_piece.color() != board_repr.side_to_move {
                moves.push(Move {
                    from,
                    to: square,
                    which_half,
                });
            } else {
                // If the square up is occupied by a friendly unitary piece,
                // and the source piece is also unitary,
                // the move is also valid
                let src_piece = board_repr[from].unwrap();
                if dst_piece.is_unitary() && src_piece.is_unitary() {
                    moves.push(Move {
                        from,
                        to: square,
                        which_half,
                    });
                }
            }
        }

        // stop going in this direction
        false
    }
}
pub fn get_rook_moves_from_square(
    board_repr: &BoardRepr,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    // Try all 4 directions from the square

    let mut current_square = from;

    while let Some(left) = current_square.left() {
        current_square = left;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }

    let mut current_square = from;

    while let Some(right) = current_square.right() {
        current_square = right;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }

    let mut current_square = from;

    while let Some(up) = current_square.up() {
        current_square = up;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }

    let mut current_square = from;

    while let Some(down) = current_square.down() {
        current_square = down;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }
}

pub fn get_bishop_moves_from_square(
    board_repr: &BoardRepr,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    let mut current_square = from;
    while let Some(upright) = current_square.up().and_then(|x| x.right()) {
        current_square = upright;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }

    let mut current_square = from;
    while let Some(upleft) = current_square.up().and_then(|x| x.left()) {
        current_square = upleft;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }

    let mut current_square = from;
    while let Some(downright) = current_square.down().and_then(|x| x.right()) {
        current_square = downright;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }

    let mut current_square = from;
    while let Some(downleft) = current_square.down().and_then(|x| x.left()) {
        current_square = downleft;
        if try_add(board_repr, from, current_square, which_half, moves) == false {
            break;
        }
    }
}

pub fn get_knight_moves_from_square(
    board_repr: &BoardRepr,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    let squares = [
        from.up().and_then(|x| x.left()).and_then(|x| x.left()),
        from.up().and_then(|x| x.right()).and_then(|x| x.right()),
        from.down().and_then(|x| x.left()).and_then(|x| x.left()),
        from.down().and_then(|x| x.right()).and_then(|x| x.right()),
        from.left().and_then(|x| x.up()).and_then(|x| x.up()),
        from.left().and_then(|x| x.down()).and_then(|x| x.down()),
        from.right().and_then(|x| x.up()).and_then(|x| x.up()),
        from.right().and_then(|x| x.down()).and_then(|x| x.down()),
    ];

    for square in squares {
        if let Some(square) = square {
            try_add(board_repr, from, square, which_half, moves);
        }
    }
}

pub fn get_king_moves_from_square(
    board_repr: &BoardRepr,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    let squares = [
        from.up().and_then(|x| x.left()),
        from.up(),
        from.up().and_then(|x| x.right()),
        from.left(),
        from.right(),
        from.down().and_then(|x| x.left()),
        from.down(),
        from.down().and_then(|x| x.right()),
    ];

    for square in squares {
        if let Some(square) = square {
            try_add(board_repr, from, square, which_half, moves);
        }
    }
}
