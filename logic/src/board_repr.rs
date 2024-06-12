use std::ops::{Index, IndexMut};

use crate::{
    pieces::{
        movement::{get_moves_from_square, Move, MovesList},
        Color, ColorPiece, CombinationPiece, Piece, UnitaryPiece,
    },
    square::{File, Rank, Square},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoardRepr {
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_pieces",
            deserialize_with = "deserialize_pieces"
        )
    )]
    pub pieces: [Option<ColorPiece>; 64],
    /// If the last move made an en passant capture possible, the square on which the pawn
    /// to be captured is located is stored here.
    pub en_passant_square: Option<Square>,

    /// The side to move is stored here
    pub side_to_move: Color,

    /// The castling rights are stored here
    /// The order is: white king side, white queen side, black king side, black queen side
    pub castling_rights: [bool; 4],

    /// The move that was just played by the opposite player.
    /// None if there is no previous move.
    pub previous_move: Option<Move>,
}

#[cfg(feature = "serde")]
fn serialize_pieces<S>(pieces: &[Option<ColorPiece>; 64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq as _;

    let mut seq = serializer.serialize_seq(Some(64))?;
    for piece in pieces {
        seq.serialize_element(piece)?;
    }
    seq.end()
}

#[cfg(feature = "serde")]
fn deserialize_pieces<'de, D>(deserializer: D) -> Result<[Option<ColorPiece>; 64], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let pieces = Vec::<Option<ColorPiece>>::deserialize(deserializer)?;
    if pieces.len() != 64 {
        return Err(serde::de::Error::custom(format!(
            "Expected 64 pieces, got {}",
            pieces.len()
        )));
    }

    let mut pieces_out = [None; 64];
    for (i, piece) in pieces_out.iter_mut().enumerate() {
        *piece = pieces[i];
    }
    Ok(pieces_out)
}

impl Index<Square> for BoardRepr {
    type Output = Option<ColorPiece>;

    fn index(&self, index: Square) -> &Self::Output {
        &self.pieces[index as usize]
    }
}

impl IndexMut<Square> for BoardRepr {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.pieces[index as usize]
    }
}

impl Index<usize> for BoardRepr {
    type Output = Option<ColorPiece>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pieces[index]
    }
}

impl IndexMut<usize> for BoardRepr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pieces[index]
    }
}

impl BoardRepr {
    pub const fn empty() -> Self {
        Self {
            pieces: [None; 64],
            en_passant_square: None,
            side_to_move: Color::White,
            castling_rights: [false; 4],
            previous_move: None,
        }
    }

    pub fn iter_pieces(&self) -> BoardPieceIter {
        BoardPieceIter {
            board: self,
            square_idx: 0,
        }
    }

    pub fn is_safe_move(&self, move_: Move, side: Color) -> bool {
        // log::debug!("Testing move: {move_:?}");
        // Make a temporary copy of the board
        let mut board = *self;
        if board.play(move_).is_err() {
            log::debug!("Failed to play {move_:?} on temp board");
            return false;
        }

        // Check if the king of the specified side is in check after the move
        // if not, the move is safe
        !board.king_in_check(side)
    }

    pub fn king_square(&self, side: Color) -> Square {
        for (square, piece) in self.iter_pieces() {
            if piece.color() == side && piece.piece().contains(UnitaryPiece::King) {
                return square;
            }
        }
        panic!("There's no king of the side {side:?} on the board");
    }

    pub fn king_in_check(&self, side: Color) -> bool {
        let side_to_move = side.opposite();
        // For every enemy piece,
        // check if it can move to the king's square
        let king_square = self.king_square(side);
        for (square, piece) in self.iter_pieces() {
            if piece.color() != side {
                let moves =
                    get_moves_from_square(MovesList::new_const(), self, side_to_move, square, None);
                let mut checking_moves = moves.iter().filter(|m| m.to == king_square);
                let first_checking_move = checking_moves.next();
                if first_checking_move.is_some() {
                    log::warn!("Found a checking move: {first_checking_move:?}, therefore {side:?} is in check");
                    return true;
                }
            }
        }
        false
    }

    pub fn play(&mut self, move_: crate::pieces::movement::Move) -> Result<(), ()> {
        fn play_inner(this: &mut BoardRepr, move_: Move) -> Result<(), ()> {
            let Move {
                from,
                to,
                which_half,
            } = move_;
            // If the source square is empty, there's nothing to do
            let src_piece = this[move_.from].ok_or(())?;
            let dst_piece = this[move_.to];

            // Generate the legal moves from the source square
            let moves = get_moves_from_square(
                MovesList::new_const(),
                this,
                this.side_to_move,
                move_.from,
                move_.which_half,
            );

            // Check if the move is legal
            if !moves.contains(&move_) {
                return Err(());
            }

            let mut did_set_en_passant = false;

            // If the piece that's moving contains a pawn,
            // if it moved two squares vertically starting at the pawn source rank
            // then set the en passant square
            if src_piece.piece().contains(UnitaryPiece::Pawn) {
                let required_start_rank = match src_piece.color() {
                    Color::White => Rank::Second,
                    Color::Black => Rank::Seventh,
                };

                if from.rank() == required_start_rank
                    && to.rank().distance(from.rank()) == 2
                    && to.file() == from.file()
                {
                    this.en_passant_square = Some(to);
                    did_set_en_passant = true;
                }
            }

            // If the half to move is unset,
            // then we're moving the entire piece
            // The following cases can happen:
            // - either the destination square is empty
            // - or the destination square is occupied by an enemy (in which case it is removed)
            // - or the destination square is occupied by a friendly (in which case both of them must be unitary, and the result is a merge)
            fn process_unitary_move(
                this: &mut BoardRepr,
                from: Square,
                to: Square,
                src_piece: ColorPiece,
                dst_piece: Option<ColorPiece>,
            ) -> Result<(), ()> {
                // If the piece that's moving is a king,
                // then it loses its castling rights
                // TODO: check that the king doesn't castle through check
                if src_piece.piece().contains(UnitaryPiece::King) {
                    match src_piece.color() {
                        Color::White => {
                            // If the king has castling rights kingside,
                            // and is moving from E1 to G1,
                            // and the rook is on H1,
                            // then move the king and the rook

                            if this.castling_rights[0]
                                && from == Square::E1
                                && to == Square::G1
                                && this[Square::H1]
                                    .is_some_and(|v| v.piece().contains(UnitaryPiece::Rook))
                                && this[Square::F1].is_none()
                                && this[Square::G1].is_none()
                            {
                                this[Square::G1] = this[Square::E1];
                                this[Square::E1] = None;
                                this[Square::F1] = this[Square::H1];
                                this[Square::H1] = None;
                                this.castling_rights[0] = false;
                                this.castling_rights[1] = false;
                                return Ok(());
                            }

                            // If the king has castling rights queenside,
                            // and is moving from E1 to C1,
                            // and the rook is on A1,
                            // then move the king and the rook
                            if this.castling_rights[1]
                                && from == Square::E1
                                && to == Square::C1
                                && this[Square::A1]
                                    .is_some_and(|v| v.piece().contains(UnitaryPiece::Rook))
                                && this[Square::B1].is_none()
                                && this[Square::C1].is_none()
                                && this[Square::D1].is_none()
                            {
                                this[Square::C1] = this[Square::E1];
                                this[Square::E1] = None;
                                this[Square::D1] = this[Square::A1];
                                this[Square::A1] = None;
                                this.castling_rights[0] = false;
                                this.castling_rights[1] = false;
                                return Ok(());
                            }

                            this.castling_rights[0] = false;
                            this.castling_rights[1] = false;
                        }
                        Color::Black => {
                            // If the king has castling rights kingside,
                            // and is moving from E8 to G8,
                            // and the rook is on H8,
                            // then move the king and the rook
                            if this.castling_rights[2]
                                && from == Square::E8
                                && to == Square::G8
                                && this[Square::H8]
                                    .is_some_and(|v| v.piece().contains(UnitaryPiece::Rook))
                                && this[Square::F8].is_none()
                                && this[Square::G8].is_none()
                            {
                                this[Square::G8] = this[Square::E8];
                                this[Square::E8] = None;
                                this[Square::F8] = this[Square::H8];
                                this[Square::H8] = None;
                                this.castling_rights[2] = false;
                                this.castling_rights[3] = false;
                                return Ok(());
                            }

                            // If the king has castling rights queenside,
                            // and is moving from E8 to C8,
                            // and the rook is on A8,
                            // then move the king and the rook
                            if this.castling_rights[3]
                                && from == Square::E8
                                && to == Square::C8
                                && this[Square::A8]
                                    .is_some_and(|v| v.piece().contains(UnitaryPiece::Rook))
                                && this[Square::B8].is_none()
                                && this[Square::C8].is_none()
                                && this[Square::D8].is_none()
                            {
                                this[Square::C8] = this[Square::E8];
                                this[Square::E8] = None;
                                this[Square::D8] = this[Square::A8];
                                this[Square::A8] = None;
                                this.castling_rights[2] = false;
                                this.castling_rights[3] = false;
                                return Ok(());
                            }

                            this.castling_rights[2] = false;
                            this.castling_rights[3] = false;
                        }
                    }
                }

                // If this piece contains a pawn,
                // and it is arriving at the final rank,
                // then replace the destination square with a queen of the color of the pawn
                if src_piece.piece().contains(UnitaryPiece::Pawn)
                    && (to.rank() == Rank::Eighth && src_piece.color() == Color::White)
                    || (to.rank() == Rank::First && src_piece.color() == Color::Black)
                {
                    this[from] = None;
                    this[to] = Some(match src_piece.color() {
                        Color::White => UnitaryPiece::Queen.white(),
                        Color::Black => UnitaryPiece::Queen.black(),
                    });
                    return Ok(());
                }

                // If the piece that's moving is a rook,
                // and it's starting from a corner,
                // then the side that's moving loses its castling rights in that direction
                if src_piece.piece().contains(UnitaryPiece::Rook) {
                    match src_piece.color() {
                        Color::White => {
                            if from == Square::A1 {
                                this.castling_rights[1] = false;
                            }
                            if from == Square::H1 {
                                this.castling_rights[0] = false;
                            }
                        }
                        Color::Black => {
                            if from == Square::A8 {
                                this.castling_rights[3] = false;
                            }
                            if from == Square::H8 {
                                this.castling_rights[2] = false;
                            }
                        }
                    }
                }

                // If the destination square is empty,
                // and this piece contains a pawn,
                // and it is to the left or right of the en passant square,
                // and it is moving to the same file as the en passant square
                if let Some(ep_square) = this.en_passant_square {
                    if from.file().distance(ep_square.file()) == 1
                        && from.rank() == ep_square.rank()
                        && to.file() == ep_square.file()
                    {
                        this[from] = None;
                        this[ep_square] = None;
                        this[to] = Some(src_piece);
                        this.en_passant_square = None;
                        return Ok(());
                    }
                }

                // If the destination square is empty
                if dst_piece.is_none() {
                    this[to] = Some(src_piece);
                    this[from] = None;
                    return Ok(());
                }

                if let Some(dst_piece) = dst_piece {
                    // If the destination square is occupied by an enemy
                    if dst_piece.color() != src_piece.color() {
                        this[from] = None;
                        this[to] = Some(src_piece);
                        return Ok(());
                    } else {
                        // If the destination square is occupied by a friendly unitary piece
                        if dst_piece.is_unitary() && dst_piece.color() == src_piece.color() {
                            let (p1, p2) = match (src_piece, dst_piece) {
                                (
                                    ColorPiece::White(Piece::Unitary(p1)),
                                    ColorPiece::White(Piece::Unitary(p2)),
                                ) => (p1, p2),
                                (
                                    ColorPiece::Black(Piece::Unitary(p1)),
                                    ColorPiece::Black(Piece::Unitary(p2)),
                                ) => (p1, p2),
                                _ => {
                                    log::warn!("Fell through match statement in merging step in process_unitary_move, declaring illegal");
                                    return Err(());
                                }
                            };
                            // Merge the pieces
                            let color_constructor = match src_piece.color() {
                                Color::White => ColorPiece::White,
                                Color::Black => ColorPiece::Black,
                            };
                            let combo = CombinationPiece::new(p1, p2);
                            if combo.is_none() {
                                log::warn!("While merging, a combination was invalid, declaring illegal. Tried merging p1: {p1:?}, p2: {p2:?}");
                                return Err(());
                            }
                            this[from] = None;
                            this[to] = Some(color_constructor(Piece::Combination(combo.unwrap())));
                            return Ok(());
                        }
                    }
                }

                // Some unexpected combination of conditions occurred,
                // the move is probably illegal
                log::warn!("Fell through conditions in process_unitary_move, declaring illegal");
                Err(())
            }

            if which_half.is_none() {
                process_unitary_move(this, from, to, src_piece, dst_piece)?;
            } else {
                // The half to move is set;
                // the old square will now contain the remaining half.
                let src_combo_piece = match src_piece.piece() {
                    crate::pieces::Piece::Unitary(_) => {
                        // We already checked for this above,
                        // so this should never happen
                        unreachable!()
                    }
                    crate::pieces::Piece::Combination(p) => p,
                };
                let which_half = which_half.unwrap();
                let color_constructor = match src_piece.color() {
                    Color::White => ColorPiece::White,
                    Color::Black => ColorPiece::Black,
                };

                let unitary_to_remain = src_combo_piece[which_half.opposite()];
                let unitary_to_move = src_combo_piece[which_half];
                let color_to_move = color_constructor(Piece::Unitary(unitary_to_move));
                this[from] = Some(color_constructor(Piece::Unitary(
                    src_combo_piece[which_half],
                )));

                // Now, the source square contains only the half that's moving.
                // The piece that's staying behind is in temporary memory.

                // Check that the move is legal for this half.
                let current_move_without_half = Move {
                    from,
                    to,
                    which_half: None,
                };
                let moves_for_moving_half = get_moves_from_square(
                    MovesList::new_const(),
                    this,
                    this.side_to_move,
                    from,
                    None,
                );

                if !moves_for_moving_half.contains(&current_move_without_half) {
                    return Err(());
                }

                process_unitary_move(this, from, to, color_to_move, dst_piece)?;

                // Finally, the temporary piece is stored back in the source square
                this[from] = Some(color_constructor(Piece::Unitary(unitary_to_remain)));
            }

            // If we didn't set en-passant on this move,
            // then unset it
            if !did_set_en_passant {
                this.en_passant_square = None;
            }

            // Swap the side to move
            this.side_to_move = this.side_to_move.opposite();

            Ok(())
        }

        let old_self = *self;
        if let Err(why) = play_inner(self, move_) {
            *self = old_self;
            return Err(why);
        }

        // The move was played successfully,
        // so update the previous move
        self.previous_move = Some(move_);
        Ok(())
    }

    pub fn has_insufficient_material(&self, side: Color) -> bool {
        // TODO: implement real check for this
        // right now, only insufficient material is when there is only kings on the board.
        !self
            .iter_pieces()
            .filter(|(_, piece)| piece.color() == side)
            .any(|(_, piece)| piece.piece() != Piece::Unitary(UnitaryPiece::King))
    }
}

impl Default for BoardRepr {
    fn default() -> Self {
        let mut b = BoardRepr::empty();
        for file in File::ALL {
            b[Square::from_coords(file, Rank::Second)] = Some(UnitaryPiece::Pawn.white());
            b[Square::from_coords(file, Rank::Seventh)] = Some(UnitaryPiece::Pawn.black());
        }

        for (rank, color) in [
            (Rank::First, ColorPiece::White as fn(Piece) -> ColorPiece),
            (Rank::Eighth, ColorPiece::Black as fn(Piece) -> ColorPiece),
        ] {
            b[Square::from_coords(File::A, rank)] = Some(color(UnitaryPiece::Rook.into()));
            b[Square::from_coords(File::B, rank)] = Some(color(UnitaryPiece::Knight.into()));
            b[Square::from_coords(File::C, rank)] = Some(color(UnitaryPiece::Bishop.into()));
            b[Square::from_coords(File::D, rank)] = Some(color(UnitaryPiece::Queen.into()));
            b[Square::from_coords(File::E, rank)] = Some(color(UnitaryPiece::King.into()));
            b[Square::from_coords(File::F, rank)] = Some(color(UnitaryPiece::Bishop.into()));
            b[Square::from_coords(File::G, rank)] = Some(color(UnitaryPiece::Knight.into()));
            b[Square::from_coords(File::H, rank)] = Some(color(UnitaryPiece::Rook.into()));
        }

        b.castling_rights = [true; 4];
        b
    }
}

pub struct BoardPieceIter<'a> {
    board: &'a BoardRepr,
    square_idx: u8,
}

impl<'a> Iterator for BoardPieceIter<'a> {
    type Item = (Square, ColorPiece);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.square_idx..64 {
            self.square_idx = i + 1;
            if let Some(piece) = self.board.pieces[i as usize] {
                return Some((Square::new(i as u32), piece));
            }
        }
        None
    }
}
