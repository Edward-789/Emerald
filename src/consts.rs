pub enum Colour {
    White = 0,
    Black = 1,
}

pub enum Pieces {
    None = 0,
    Pawn = 2,
    Knight = 3,
    Bishop = 4,
    Rook = 5,
    Queen = 6,
    King = 7,
}

pub enum Castling {
    WhiteQueen = 8,
    WhiteKing = 4,
    BlackQueen = 2,
    BlackKing = 1,
    None = 0
}