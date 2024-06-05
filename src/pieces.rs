use std::fmt::{Debug, Display, Formatter};

/// A single chess piece, or a combination of two pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Unitary(UnitaryPiece),
    Combination(CombinationPiece),
}

/// A chess piece with a color associated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorPiece {
    White(Piece),
    Black(Piece),
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

    pub fn piece(self) -> Piece {
        match self {
            ColorPiece::White(piece) | ColorPiece::Black(piece) => piece,
        }
    }
}

/// A single unitary piece. Combinations are made of two of these,
/// except for the King.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnitaryPiece {
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    King,
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
pub struct CombinationPiece {
    first: UnitaryPiece,
    second: UnitaryPiece,
}

impl CombinationPiece {
    pub fn new(first: UnitaryPiece, second: UnitaryPiece) -> Option<Self> {
        // If either of the pieces is the King, the combination is invalid
        if first == UnitaryPiece::King || second == UnitaryPiece::King {
            return None;
        }

        // Swap the two pieces if they're in the wrong order
        if first > second {
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
}

impl From<CombinationPiece> for Piece {
    fn from(piece: CombinationPiece) -> Self {
        Self::Combination(piece)
    }
}

impl Display for CombinationPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}", self.first, self.second)
    }
}
