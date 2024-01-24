#[derive(PartialEq)]
pub enum Colour {
    White = 0,
    Black = 1,
}

#[derive(PartialEq)]
pub enum Pieces {
    None = 0,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
    King = 7,
}

pub struct Castling;
impl Castling {
    pub const WHITE_QUEEN: u8 = 8;
    pub const WHITE_KING: u8 = 4;
    pub const BLACK_QUEEN: u8 = 2;
    pub const BLACK_KING: u8 = 1;
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
impl Masks {
    pub const FILE_A: u64 = 0x0101010101010101;
    pub const FILE_H: u64 = 0x8080808080808080;
    pub const RANK_1: u64 = 0x00000000000000FF;
    pub const RANK_8: u64 = 0xFF00000000000000;
}