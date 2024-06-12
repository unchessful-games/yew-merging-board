pub mod movement;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::Index,
};

/// A single chess piece, or a combination of two pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Piece {
    Unitary(UnitaryPiece),
    Combination(CombinationPiece),
}

impl Piece {
    pub fn is_unitary(self) -> bool {
        matches!(self, Piece::Unitary(_))
    }

    pub fn is_combination(self) -> bool {
        matches!(self, Piece::Combination(_))
    }

    pub fn contains(self, piece: UnitaryPiece) -> bool {
        match self {
            Piece::Unitary(p) => p == piece,
            Piece::Combination(p) => p.contains(piece),
        }
    }

    pub fn for_components(self, mut f: impl FnMut(UnitaryPiece)) {
        match self {
            Piece::Unitary(p) => f(p),
            Piece::Combination(p) => {
                f(p.first());
                f(p.second());
            }
        }
    }
}

/// A chess piece with a color associated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ColorPiece {
    White(Piece),
    Black(Piece),
}

/// A player color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Either the left or the right half of a piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PieceHalf {
    Left,
    Right,
}

impl PieceHalf {
    pub fn opposite(self) -> Self {
        match self {
            PieceHalf::Left => PieceHalf::Right,
            PieceHalf::Right => PieceHalf::Left,
        }
    }
}

impl Display for ColorPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorPiece::White(piece) => write!(f, "white {}", piece),
            ColorPiece::Black(piece) => write!(f, "black {}", piece),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Unitary(piece) => Display::fmt(&piece, f),
            Piece::Combination(piece) => Display::fmt(&piece, f),
        }
    }
}

impl ColorPiece {
    pub fn is_white(self) -> bool {
        matches!(self, ColorPiece::White(_))
    }

    pub fn is_black(self) -> bool {
        matches!(self, ColorPiece::Black(_))
    }

    pub fn color(self) -> Color {
        match self {
            ColorPiece::White(_) => Color::White,
            ColorPiece::Black(_) => Color::Black,
        }
    }

    pub fn piece(self) -> Piece {
        match self {
            ColorPiece::White(piece) | ColorPiece::Black(piece) => piece,
        }
    }

    pub fn is_unitary(self) -> bool {
        self.piece().is_unitary()
    }
}

/// A single unitary piece. Combinations are made of two of these,
/// except for the King.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnitaryPiece {
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    King,
}

impl UnitaryPiece {
    pub const ALL: [UnitaryPiece; 6] = [
        UnitaryPiece::Queen,
        UnitaryPiece::Bishop,
        UnitaryPiece::Knight,
        UnitaryPiece::Rook,
        UnitaryPiece::Pawn,
        UnitaryPiece::King,
    ];
}

impl Display for UnitaryPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitaryPiece::Queen => write!(f, "queen"),
            UnitaryPiece::Bishop => write!(f, "bishop"),
            UnitaryPiece::Knight => write!(f, "knight"),
            UnitaryPiece::Rook => write!(f, "rook"),
            UnitaryPiece::Pawn => write!(f, "pawn"),
            UnitaryPiece::King => write!(f, "king"),
        }
    }
}

impl UnitaryPiece {
    pub fn white(self) -> ColorPiece {
        ColorPiece::White(self.into())
    }

    pub fn black(self) -> ColorPiece {
        ColorPiece::Black(self.into())
    }
}

impl From<UnitaryPiece> for Piece {
    fn from(piece: UnitaryPiece) -> Self {
        Self::Unitary(piece)
    }
}

/// A combination of two pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CombinationPiece {
    first: UnitaryPiece,
    second: UnitaryPiece,
}

impl Index<PieceHalf> for CombinationPiece {
    type Output = UnitaryPiece;

    fn index(&self, index: PieceHalf) -> &Self::Output {
        match index {
            PieceHalf::Left => &self.first,
            PieceHalf::Right => &self.second,
        }
    }
}

impl CombinationPiece {
    pub unsafe fn new_unchecked(first: UnitaryPiece, second: UnitaryPiece) -> Self {
        Self { first, second }
    }

    pub fn new(first: UnitaryPiece, second: UnitaryPiece) -> Option<Self> {
        // If either of the pieces is the King, the combination is invalid
        if first == UnitaryPiece::King || second == UnitaryPiece::King {
            return None;
        }

        // Swap the two pieces if they're in the wrong order
        if first < second {
            Some(Self { first, second })
        } else {
            Some(Self {
                first: second,
                second: first,
            })
        }
    }

    pub fn first(&self) -> UnitaryPiece {
        self.first
    }

    pub fn second(&self) -> UnitaryPiece {
        self.second
    }

    pub fn pieces(&self) -> (UnitaryPiece, UnitaryPiece) {
        (self.first, self.second)
    }

    pub fn black(self) -> ColorPiece {
        ColorPiece::Black(self.into())
    }

    pub fn white(self) -> ColorPiece {
        ColorPiece::White(self.into())
    }

    pub fn contains(self, piece: UnitaryPiece) -> bool {
        self.first == piece || self.second == piece
    }
}

impl From<CombinationPiece> for Piece {
    fn from(piece: CombinationPiece) -> Self {
        Self::Combination(piece)
    }
}

impl Display for CombinationPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.first, self.second)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_combination_piece_list() {
        let mut combo_pieces = vec![];
        for first in UnitaryPiece::ALL {
            for second in UnitaryPiece::ALL {
                if let Some(piece) = CombinationPiece::new(first, second) {
                    combo_pieces.push(piece);
                }
            }
        }

        use CombinationPiece as C;
        use UnitaryPiece as U;

        let target = unsafe {
            vec![
                C::new_unchecked(U::Queen, U::Queen),
                C::new_unchecked(U::Queen, U::Bishop),
                C::new_unchecked(U::Queen, U::Knight),
                C::new_unchecked(U::Queen, U::Rook),
                C::new_unchecked(U::Queen, U::Pawn),
                C::new_unchecked(U::Bishop, U::Bishop),
                C::new_unchecked(U::Bishop, U::Knight),
                C::new_unchecked(U::Bishop, U::Rook),
                C::new_unchecked(U::Bishop, U::Pawn),
                C::new_unchecked(U::Knight, U::Knight),
                C::new_unchecked(U::Knight, U::Rook),
                C::new_unchecked(U::Knight, U::Pawn),
                C::new_unchecked(U::Rook, U::Rook),
                C::new_unchecked(U::Rook, U::Pawn),
                C::new_unchecked(U::Pawn, U::Pawn),
            ]
        };

        let combo_pieces: HashSet<C> = HashSet::from_iter(combo_pieces);
        let target = HashSet::from_iter(target);

        assert_eq!(combo_pieces, target);
    }
}
