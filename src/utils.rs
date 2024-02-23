#[derive(PartialEq, Clone, Copy)]
pub enum Colour {
    White = 0,
    Black = 1,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Pieces {
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
    King = 7,
}

impl Pieces {
    pub fn convert_num_to_piece(num : usize) -> Self {
        match num {
            2 => Pieces::Pawn,
            3 => Pieces::Knight,
            4 => Pieces::Bishop,
            5 => Pieces::Rook,
            6 => Pieces::Queen,
            7 => Pieces::King,
            _ => panic!("tried to convert to piece that doesnt exist")
        }
    }
}
pub struct Castling;
impl Castling {
    pub const WHITE_QUEEN: u8 = 8;
    pub const WHITE_KING: u8 = 4;
    pub const BLACK_QUEEN: u8 = 2;
    pub const BLACK_KING: u8 = 1;
    pub const CASTLE_FLAG_MASKS: [u8; 2] = [0b1100, 0b0011];
    pub const CASTLE_RIGHT_MASKS: [u8; 64] = [
        7, 15, 15, 15, 3, 15, 15, 11, // white
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        13, 15, 15, 15, 12, 15, 15, 14, // black
    ];
}


pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
pub struct Masks;

#[allow(dead_code)]
impl Masks {
    pub const FILE_A: u64 = 0x0101010101010101;
    pub const FILE_H: u64 = 0x8080808080808080;
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_8: u64 = 0xFF00000000000000;
}

#[macro_export]
macro_rules! pop_lsb {
    ($bitboard:expr) => {{
        let lsb = $bitboard.trailing_zeros();
        $bitboard &= $bitboard - 1;
        lsb as usize
    }};
}

pub const SQUARE_TO_STR: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];
