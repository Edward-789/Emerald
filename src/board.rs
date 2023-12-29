use crate::consts::{
    Pieces,
    Colour,
    Castling,
};

pub struct Board {
    bitboards: [u64; 8],
    whites_move: bool,
    castle_rights: u8,
    en_passant_sq: u8,
}

impl Board {
    fn set_piece(&mut self, square: usize, colour: usize, piece: usize) {
        let i = 1 << square;
        self.bitboards[colour] ^= i;
        self.bitboards[piece] ^= i;
    }

    fn blank() -> Self {
        Board {
            bitboards: [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64],
            whites_move: true,
            castle_rights: 0u8,
            en_passant_sq: 0u8
        }
    }

    pub fn read_fen(fen: &str) -> Self {
        let fen_split = fen.split(' ').collect::<Vec<&str>>();
        let pieces = fen_split[0].chars().collect::<Vec<char>>();

        let mut rank = 7u8;
        let mut file = 0u8;
        let mut board = Self::blank();

        // pieces
        for symbol in pieces {
            if symbol == '/' {
                rank -= 1;
                file = 0;
            } else if symbol.is_numeric() {
                file += symbol.to_digit(10).unwrap_or(0) as u8;
            } else {
                let square = (file + rank * 8) as usize;
                let colour = if symbol.is_uppercase() { Colour::White } else { Colour::Black } as usize;
                let piece =
                    if symbol == 'p' || symbol == 'P' { Pieces::Pawn } else
                    if symbol == 'n' || symbol == 'N' { Pieces::Knight } else
                    if symbol == 'b' || symbol == 'B' { Pieces::Bishop } else
                    if symbol == 'r' || symbol == 'R' { Pieces::Rook } else
                    if symbol == 'q' || symbol == 'Q' { Pieces::Queen } else
                    if symbol == 'k' || symbol == 'K' { Pieces::King }  else {Pieces::None} as usize;

                board.set_piece(square, colour, piece);

                file += 1;
            }
        }

        // stm
        board.whites_move = fen_split[1].chars().collect::<Vec<char>>()[0] == 'w';

        // castling
        let castle_symbols = fen_split[2].chars().collect::<Vec<char>>();

        for symbol in castle_symbols {
            board.castle_rights +=
                if symbol == 'K' { Castling::WhiteKing } else
                if symbol == 'Q' { Castling::WhiteQueen } else
                if symbol == 'k' { Castling::BlackKing } else
                if symbol == 'q' { Castling::BlackQueen } else {Castling::None} as u8
                                
        }

        // en passant

        board.en_passant_sq = if fen_split[3] == "-" {0} else {
            let square_chars = fen_split[3].chars().collect::<Vec<char>>();
            8 * square_chars[1].to_digit(10).unwrap_or(0) as u8 + square_chars[0] as u8 - 105
        };

        board
    }

    // debug stuff

    pub fn print_board_info(&self) {
        println!("Bitboards");

        self.print_bitboards();

        print!("Whites move : ");
        println!("{}", self.whites_move);

        self.print_castle_rights();

        print!("En Passant Index : ");
        println!("{}", self.en_passant_sq);
    }

    fn print_bitboards(&self) {
        let labels = ["White", "Black", "Pawns", "Knights", "Bishop", "Rooks", "Queens", "Kings"];
        for i in 0..7 {
            println!();
            println!("{}", labels[i]);

            for rank in (0..8).rev() {
                for file in 0..8 {
                    let square = rank * 8 + file;
                    if (self.bitboards[i] >> square) & 1u64 == 1u64 {
                        print!("X");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }

    fn print_castle_rights(&self) {
        println!();
        println!("Castling rights");
        let labels = ["BK", "BQ", "WK", "WQ"];

        for i in 0..4 {
            if (self.castle_rights >> i) & 1u8 == 1u8 {
                print!("{}", labels[i]);
                print!(", ");
            }
        }
        println!();
        println!();

    }
}

