use arrayvec::ArrayVec;

use crate::{
    board_repr::BoardRepr,
    square::{Rank, Square},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Color, ColorPiece, Piece, PieceHalf, UnitaryPiece};

pub type MovesList = ArrayVec<Move, 1024>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub which_half: Option<PieceHalf>,
}

pub fn find_any_legal_move(board_repr: &BoardRepr, side_to_move: Color) -> Option<Move> {
    let mut moves = MovesList::new();
    for (square, piece) in board_repr.iter_pieces() {
        if piece.color() == side_to_move {
            moves = get_moves_from_square(moves, board_repr, side_to_move, square, None);
            moves.retain(|x| board_repr.is_safe_move(*x, side_to_move));
            if !moves.is_empty() {
                return Some(moves[0]);
            }
        }
    }

    None
}

pub fn get_all_legal_moves(board_repr: &BoardRepr, side_to_move: Color) -> MovesList {
    let mut moves = MovesList::new();
    for (square, piece) in board_repr.iter_pieces() {
        if piece.color() == side_to_move {
            moves = get_moves_from_square(moves, board_repr, side_to_move, square, None);
        }
    }

    // log::debug!("All semi-legal moves of position: {moves:?}");
    moves.retain(|x| board_repr.is_safe_move(*x, side_to_move));
    // log::debug!("Only legal moves of position: {moves:?}");
    moves
}

pub fn get_legal_moves_from_square(
    moves: MovesList,
    board_repr: &BoardRepr,
    side_to_move: Color,
    from: Square,
    which_half: Option<PieceHalf>,
) -> MovesList {
    let mut moves = get_moves_from_square(moves, board_repr, side_to_move, from, which_half);

    // log::debug!("All moves: {moves:?}");
    moves.retain(|x| board_repr.is_safe_move(*x, side_to_move));

    // log::debug!("Legal moves: {moves:?}");
    moves
}

pub fn get_moves_from_square(
    mut moves: MovesList,
    board_repr: &BoardRepr,
    side_to_move: Color,
    from: Square,
    which_half: Option<PieceHalf>,
) -> MovesList {
    // If there is no piece of the current color on the square, there are no moves
    let piece = if let Some(piece) = board_repr[from] {
        if piece.color() == side_to_move {
            piece
        } else {
            return moves;
        }
    } else {
        return moves;
    };
    let color = piece.color();
    let piece = piece.piece();

    // If the color of the piece is not the same as the side to move, there are no moves
    if color != side_to_move {
        return MovesList::new();
    }

    // If the piece is unitary, then it has no halves
    // If a half is specified in such a case, then there are no moves
    if which_half.is_some() && piece.is_unitary() {
        return MovesList::new();
    }

    let mut add_unitary = |piece: UnitaryPiece| match piece {
        super::UnitaryPiece::Pawn => {
            get_pawn_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves)
        }
        super::UnitaryPiece::King => {
            get_king_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves)
        }
        super::UnitaryPiece::Queen => {
            get_rook_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves);
            get_bishop_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves);
        }
        super::UnitaryPiece::Rook => {
            get_rook_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves)
        }
        super::UnitaryPiece::Bishop => {
            get_bishop_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves)
        }
        super::UnitaryPiece::Knight => {
            get_knight_moves_from_square(board_repr, side_to_move, from, which_half, &mut moves)
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
    side_to_move: Color,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    match side_to_move {
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
                    if board_repr[up].is_none()
                        && board_repr[up_up].is_none()
                        && from.rank() == Rank::Second
                    {
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
                        if piece.color() != side_to_move {
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
                        if piece.color() != side_to_move {
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
                if let Some(en_passant_square) = board_repr.en_passant_square {
                    if let Some(upleft) = up.left() {
                        if let Some(left) = from.left() {
                            if left == en_passant_square && board_repr[upleft].is_none() {
                                moves.push(Move {
                                    from,
                                    to: upleft,
                                    which_half,
                                });
                            }
                        }
                    }

                    if let Some(upright) = up.right() {
                        if let Some(right) = from.right() {
                            if right == en_passant_square && board_repr[upright].is_none() {
                                moves.push(Move {
                                    from,
                                    to: upright,
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
                    if board_repr[down].is_none()
                        && board_repr[down_down].is_none()
                        && from.rank() == Rank::Seventh
                    {
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
                        if piece.color() != side_to_move {
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
                        if piece.color() != side_to_move {
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
                if let Some(en_passant_square) = board_repr.en_passant_square {
                    if let Some(downleft) = down.left() {
                        if let Some(left) = from.left() {
                            if left == en_passant_square && board_repr[downleft].is_none() {
                                moves.push(Move {
                                    from,
                                    to: downleft,
                                    which_half,
                                });
                            }
                        }
                    }

                    if let Some(downright) = down.right() {
                        if let Some(right) = from.right() {
                            if right == en_passant_square && board_repr[downright].is_none() {
                                moves.push(Move {
                                    from,
                                    to: downright,
                                    which_half,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

fn try_add(
    board_repr: &BoardRepr,
    side_to_move: Color,
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
            if dst_piece.color() != side_to_move {
                moves.push(Move {
                    from,
                    to: square,
                    which_half,
                });
            } else {
                // If the square up is occupied by a friendly unitary piece,
                // and the source piece is also unitary,
                // the move is also valid
                // (But not if that piece is the king)
                let src_piece = board_repr[from].unwrap();
                if dst_piece.is_unitary() && src_piece.is_unitary() {
                    if let Piece::Unitary(UnitaryPiece::King) = dst_piece.piece() {
                        return false;
                    }
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
    side_to_move: Color,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    // Try all 4 directions from the square

    let mut current_square = from;

    while let Some(left) = current_square.left() {
        current_square = left;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }

    let mut current_square = from;

    while let Some(right) = current_square.right() {
        current_square = right;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }

    let mut current_square = from;

    while let Some(up) = current_square.up() {
        current_square = up;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }

    let mut current_square = from;

    while let Some(down) = current_square.down() {
        current_square = down;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }
}

pub fn get_bishop_moves_from_square(
    board_repr: &BoardRepr,
    side_to_move: Color,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    let mut current_square = from;
    while let Some(upright) = current_square.up().and_then(|x| x.right()) {
        current_square = upright;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }

    let mut current_square = from;
    while let Some(upleft) = current_square.up().and_then(|x| x.left()) {
        current_square = upleft;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }

    let mut current_square = from;
    while let Some(downright) = current_square.down().and_then(|x| x.right()) {
        current_square = downright;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }

    let mut current_square = from;
    while let Some(downleft) = current_square.down().and_then(|x| x.left()) {
        current_square = downleft;
        if !try_add(
            board_repr,
            side_to_move,
            from,
            current_square,
            which_half,
            moves,
        ) {
            break;
        }
    }
}

pub fn get_knight_moves_from_square(
    board_repr: &BoardRepr,
    side_to_move: Color,
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
            try_add(board_repr, side_to_move, from, square, which_half, moves);
        }
    }
}

pub fn get_king_moves_from_square(
    board_repr: &BoardRepr,
    side_to_move: Color,
    from: Square,
    which_half: Option<PieceHalf>,
    moves: &mut MovesList,
) {
    // The king cannot merge with any other piece
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
            // If the square is occupied by a friendly piece, the move is invalid
            // (king cannot merge)
            // If it is empty, or it is occupied by an enemy piece, the move is valid
            if let Some(piece) = board_repr[square] {
                if piece.color() == side_to_move {
                    continue;
                }
            }
            moves.push(Move {
                from,
                to: square,
                which_half,
            });
        }
    }

    // If the king has castling rights, and it is in its starting position,
    // and the squares between the king and the rook are empty,
    // then the move is valid
    // TODO: check that the king doesn't castle through check

    match side_to_move {
        Color::White => {
            if from == Square::E1 {
                if board_repr.castling_rights[0] {
                    // can castle kingside
                    if board_repr[Square::F1].is_none() && board_repr[Square::G1].is_none() {
                        moves.push(Move {
                            from,
                            to: Square::G1,
                            which_half,
                        });
                    }
                }

                if board_repr.castling_rights[1] {
                    // can castle queenside
                    if board_repr[Square::B1].is_none()
                        && board_repr[Square::C1].is_none()
                        && board_repr[Square::D1].is_none()
                    {
                        moves.push(Move {
                            from,
                            to: Square::C1,
                            which_half,
                        });
                    }
                }
            }
        }
        Color::Black => {
            if from == Square::E8 {
                if board_repr.castling_rights[2] {
                    // can castle kingside
                    if board_repr[Square::F8].is_none() && board_repr[Square::G8].is_none() {
                        moves.push(Move {
                            from,
                            to: Square::G8,
                            which_half,
                        });
                    }
                }

                if board_repr.castling_rights[3] {
                    // can castle queenside
                    if board_repr[Square::B8].is_none()
                        && board_repr[Square::C8].is_none()
                        && board_repr[Square::D8].is_none()
                    {
                        moves.push(Move {
                            from,
                            to: Square::C8,
                            which_half,
                        });
                    }
                }
            }
        }
    }
}
