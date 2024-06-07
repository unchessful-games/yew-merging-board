use std::ops::{Index, IndexMut};

use crate::{
    pieces::{Color, ColorPiece, Piece, UnitaryPiece},
    square::{File, Rank, Square},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardRepr {
    pub pieces: [Option<ColorPiece>; 64],
    /// If the last move made an en passant capture possible, the square on which the pawn
    /// to be captured is located is stored here.
    pub en_passant_square: Option<Square>,

    /// The side to move is stored here
    pub side_to_move: Color,

    /// The castling rights are stored here
    /// The order is: white king side, white queen side, black king side, black queen side
    pub castling_rights: [bool; 4],
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

impl BoardRepr {
    pub const fn empty() -> Self {
        Self {
            pieces: [None; 64],
            en_passant_square: None,
            side_to_move: Color::White,
            castling_rights: [false; 4],
        }
    }

    pub fn iter_pieces(&self) -> BoardPieceIter {
        BoardPieceIter {
            board: self,
            square_idx: 0,
        }
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
