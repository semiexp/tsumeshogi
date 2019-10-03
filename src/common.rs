use crate::{D, P};

pub const BOARD_SIZE: i32 = 9;
pub const BOARD_CELLS: i32 = BOARD_SIZE * BOARD_SIZE;

pub const PIECE_TYPES: usize = 8;

/// There are 8 kinds of pieces, 6 of which can promote.
/// Non-promoted pieces are numbered by 0 through 7.
/// The promoted piece of piece i is represented by i+8.
/// English names of pieces are as in https://en.wikipedia.org/wiki/Shogi.
pub const PIECE_NAMES: [char; PIECE_TYPES * 2] = [
    '歩', // pawn
    '香', // lance
    '桂', // knight
    '銀', // silver
    '金', // gold
    '角', // bishop
    '飛', // rook
    '玉', // king
    'と', // promoted pawn
    '杏', // promoted lance
    '圭', // promoted knight
    '全', // promoted silver
    ' ',   // gold has no promotion
    '馬', // promoted bishop
    '龍', // promoted rook
    ' ',   // king has no promotion
];

pub const PIECE_MOVES_COUNT: usize = 8;

/// Possible non-ranging moves for each piece from the view of the first player.
pub const PIECE_MOVES: [[D; PIECE_MOVES_COUNT]; PIECE_TYPES * 2] = [
    [
        D(-1, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-2, -1),
        D(-2, 1),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(1, -1),
        D(1, 1),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(0, -1),
        D(0, 1),
        D(1, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(0, -1),
        D(0, 1),
        D(1, -1),
        D(1, 0),
        D(1, 1),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(0, -1),
        D(0, 1),
        D(1, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(0, -1),
        D(0, 1),
        D(1, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(0, -1),
        D(0, 1),
        D(1, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 0),
        D(-1, 1),
        D(0, -1),
        D(0, 1),
        D(1, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, 0),
        D(0, -1),
        D(0, 1),
        D(1, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(-1, -1),
        D(-1, 1),
        D(1, -1),
        D(1, 1),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
    [
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
        D(0, 0),
    ],
];

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Piece(pub i8);

impl Piece {
    pub fn promote(self) -> Piece {
        assert!(self.0 < 7 && self.0 != 4);
        Piece(self.0 + 8)
    }
    pub fn has_promotion(self) -> bool {
        self.0 != 4 && self.0 < 7
    }
    pub fn capture(self) -> Piece {
        Piece(self.0 & 7)
    }
    pub fn as_first(self) -> SidedPiece {
        SidedPiece(self.0)
    }
    pub fn as_second(self) -> SidedPiece {
        SidedPiece(!self.0)
    }
    pub fn get_name(self) -> char {
        PIECE_NAMES[self.0 as usize]
    }
}

pub const PIECE_PAWN: Piece = Piece(0);
pub const PIECE_LANCE: Piece = Piece(1);
pub const PIECE_KNIGHT: Piece = Piece(2);
pub const PIECE_SILVER: Piece = Piece(3);
pub const PIECE_GOLD: Piece = Piece(4);
pub const PIECE_BISHOP: Piece = Piece(5);
pub const PIECE_ROOK: Piece = Piece(6);
pub const PIECE_KING: Piece = Piece(7);
pub const PIECE_PROMOTED_BISHOP: Piece = Piece(13);
pub const PIECE_PROMOTED_ROOK: Piece = Piece(14);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    NoPromotion(P, P),
    Promotion(P, P),
    FromHand(P, SidedPiece),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct SidedPiece(pub i8);

impl SidedPiece {
    pub fn to_piece(self) -> Piece {
        if self.0 >= 0 {
            Piece(self.0)
        } else {
            Piece(!self.0)
        }
    }
    pub fn promote(self) -> SidedPiece {
        if self.0 >= 0 {
            SidedPiece(self.0 + 8)
        } else {
            SidedPiece(self.0 - 8)
        }
    }
    pub fn first(piece: Piece) -> SidedPiece {
        SidedPiece(piece.0)
    }
    pub fn second(piece: Piece) -> SidedPiece {
        SidedPiece(!piece.0)
    }
    pub fn is_empty(self) -> bool {
        self == EMPTY_CELL
    }
    pub fn is_first(self) -> bool {
        0 <= self.0 && self.0 < 16
    }
    pub fn is_second(self) -> bool {
        self.0 < 0
    }
}

pub const EMPTY_CELL: SidedPiece = SidedPiece(16);
pub const SECOND_KING: SidedPiece = SidedPiece(!7);
