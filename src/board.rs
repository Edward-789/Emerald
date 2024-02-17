use crate::{
    moves::{Move, MoveList},
    utils::{
        pop_lsb, Castling, Colour, Pieces
    },
    attacks::Attacks
};

#[derive(Clone, Copy)]
pub struct Board {
    bitboards: [u64; 8],
    colour_to_move: Colour,
    castle_rights: u8,
    en_passant_sq: Option<u8>,
    king_squares: [usize; 2]
}

impl Board {

    // utilities
    fn square_is_occupied(&self, square: usize) -> bool {
        ((self.bitboards[0] | self.bitboards[1]) & (1 << square)) > 0
    }

    fn enemy_colour(&self) -> Colour {
        Self::reverse_colour(self.colour_to_move)
    }

    fn reverse_colour(colour : Colour) -> Colour {
        match colour {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White
        }
    }

    fn get_piece_colour_bitboard(&self, piece: Pieces, colour: Colour) -> u64 {
        self.bitboards[piece as usize] & self.bitboards[colour as usize]
    }

    fn set_piece(&mut self, square: usize, colour: usize, piece: usize) {
        let i = 1 << square;
        self.bitboards[colour] |= i;
        self.bitboards[piece] |= i;
    }

    fn remove_piece(&mut self, square: usize) {
        let bit = 1 << square;

        self.bitboards[self.find_colour(square) as usize] &= !bit;
        for i in 2..=7 {
            if self.bitboards[i] & bit > 0 {
                self.bitboards[i] &= !bit
            }
        }
    }

    fn find_piece_type(&self, square: usize) -> Option<Pieces> {
        let bit = 1 << square;

        for i in 2..=7 {
            if self.bitboards[i] & bit > 0 {
                return Some(Pieces::convert_num_to_piece(i));

            }
        }

        None
    }

    fn find_colour(&self, square: usize) -> Colour {
        let i = 1 << square;

        if self.bitboards[Colour::White as usize] & i > 0 {
            Colour::White
        } else {
            Colour::Black
        }
    }

    fn in_check(&self) -> bool {
        self.square_is_attacked(self.king_squares[self.colour_to_move as usize], self.colour_to_move)
    }

    fn square_is_attacked(&self, square : usize, stm : Colour) -> bool{
        let enemy = Self::reverse_colour(stm);
        let occupied = self.bitboards[0] | self.bitboards[1];
        let queen = self.get_piece_colour_bitboard(Pieces::Queen, enemy);

        Attacks::rook_attacks(square, occupied) & (self.get_piece_colour_bitboard(Pieces::Rook, enemy)| queen) |
        Attacks::bishop_attacks(square, occupied) & ( self.get_piece_colour_bitboard(Pieces::Bishop, enemy) | queen) |
        Attacks::knight_attacks(square) & self.get_piece_colour_bitboard(Pieces::Knight, enemy) | 
        Attacks::pawn_attacks(square, enemy) & self.get_piece_colour_bitboard(Pieces::Pawn, enemy) |
        Attacks::king_attacks(square) & self.get_piece_colour_bitboard(Pieces::King, enemy)  > 0
    }

    fn blank() -> Self {
        Board {
            bitboards: [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64],
            colour_to_move: Colour::White,
            castle_rights: 0u8,
            en_passant_sq: None,
            king_squares: [0, 0]
        }
    }

    fn find_square(file : usize, rank : usize) -> usize {
        rank * 8 + file
    }

    fn find_rank(square : usize) -> usize {
        square / 8
    }

    // FEN stuff

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
                let square = Self::find_square(file as usize, rank as usize);
                let colour = if symbol.is_uppercase() { Colour::White } else { Colour::Black } as usize;
                let piece =
                    if symbol == 'p' || symbol == 'P' { Pieces::Pawn } else
                    if symbol == 'n' || symbol == 'N' { Pieces::Knight } else
                    if symbol == 'b' || symbol == 'B' { Pieces::Bishop } else
                    if symbol == 'r' || symbol == 'R' { Pieces::Rook } else
                    if symbol == 'q' || symbol == 'Q' { Pieces::Queen } else
                    if symbol == 'k' || symbol == 'K' { Pieces::King } else { panic!("invalid FEN"); } as usize;

                if symbol == 'k' { board.king_squares[0] = square} else
                if symbol == 'K' { board.king_squares[1] = square}
                board.set_piece(square, colour, piece);

                file += 1;
            }
        }

        // stm
        board.colour_to_move = if fen_split[1].chars().collect::<Vec<char>>()[0] == 'w' {Colour::White} else {Colour::Black};

        // castling
        let castle_symbols = fen_split[2].chars().collect::<Vec<char>>();

        for symbol in castle_symbols {
            board.castle_rights +=
                if symbol == 'K' { Castling::WHITE_KING } else
                if symbol == 'Q' { Castling::WHITE_QUEEN } else
                if symbol == 'k' { Castling::BLACK_KING } else
                if symbol == 'q' { Castling::BLACK_QUEEN } else {0}
                                
        }

        // en passant

        board.en_passant_sq = if fen_split[3] == "-" {None} else {
            let square_chars = fen_split[3].chars().collect::<Vec<char>>();
            Some(8 * square_chars[1].to_digit(10).unwrap_or(0) as u8 + square_chars[0] as u8 - 105)
        };

        board
    }

    //movegen and makemove

    pub fn psuedolegal_movegen(&self) -> MoveList {
        let mut list = MoveList::EMPTY;

        let enemy_colour = self.enemy_colour();
        let opposite =  self.bitboards[enemy_colour as usize];
        let us = self.bitboards[self.colour_to_move as usize];
        let all_pieces = opposite | us;
        let is_white = self.colour_to_move == Colour::White;

        //en passant stuff
        // if self.en_passant_sq != None {
        //     let mut en_passant_attack = Attacks::pawn_attacks(self.en_passant_sq.unwrap() as usize, enemy_colour) & us;    
        //     while en_passant_attack > 0 {

        //         let from_square = pop_lsb(&mut en_passant_attack);
        //         list.push(from_square, self.en_passant_sq.unwrap() as usize, Move::EN_PASSANT);
        //     }
        // }

        let mut our_pawns = self.get_piece_colour_bitboard(Pieces::Pawn, self.colour_to_move);

        while our_pawns > 0 {

            let from_square = pop_lsb(&mut our_pawns);               // pawns only attack squares an enemy piece is located on, so and with enemy pieces
            let mut attacks = Attacks::pawn_attacks(from_square, self.colour_to_move) & opposite;

            while attacks > 0 {
                let attack_square = pop_lsb(&mut attacks);
                list.push(from_square, attack_square, Move::NO_FLAG);;
            }

            let push_square = if is_white {from_square + 8} else {from_square - 8};
            if self.square_is_occupied(push_square) {
                continue;
            }
            list.push(from_square, push_square, Move::NO_FLAG);;

            let rank = Self::find_rank(from_square);
            if rank != 1 && is_white || rank != 6 && !is_white {
                continue;
            }

            let double_push_square = if is_white {push_square + 8} else {push_square - 8};

            if self.square_is_occupied(double_push_square)  {
                continue;
            }

            list.push(from_square, double_push_square, Move::DOUBLE_PAWN_PUSH);

        }    

        for i in 2..=7 {
            let piece = Pieces::convert_num_to_piece(i);
            let mut our_pieces = self.get_piece_colour_bitboard(piece, self.colour_to_move);

            while our_pieces > 0 {
                let from_square = pop_lsb(&mut our_pieces);
    
                let mut attacks = Attacks::get_piece_attacks(from_square, all_pieces, piece) & !us;
    
                while attacks > 0 {
                    let attack_square = pop_lsb(&mut attacks);
                    list.push(from_square, attack_square, Move::NO_FLAG);
                }
            }
    
        }
        list
    }

    pub fn apply(&mut self, mov : Move) -> bool {
        let from_square = mov.from_square();
        let to_square = mov.to_square();
        let side = self.colour_to_move;

        self.remove_piece(to_square);
        self.set_piece(to_square, self.colour_to_move as usize, self.find_piece_type(from_square).unwrap() as usize);
        // change ksqs
        if self.find_piece_type(from_square).unwrap() == Pieces::King {
            self.king_squares[side as usize] = to_square;
        }

        self.remove_piece(from_square);
        self.colour_to_move = self.enemy_colour();
        
        // check legal move
        !self.square_is_attacked(self.king_squares[side as usize], self.colour_to_move)
    }

     // debug stuff
     #[allow(dead_code)]

     pub fn print_board_info(&self) {
         println!("Bitboards");
 
         let labels = ["White", "Black", "Pawns", "Knights", "Bishop", "Rooks", "Queens", "Kings"];
         for i in 0..=7 {
             println!();
             println!("{}", labels[i]);
             Self::print_bitboard(self.bitboards[i]);
             println!();
         }
 
         self.print_castle_rights();
 
         print!("En Passant Index : ");
         println!("{:?}", self.en_passant_sq);
     }
 
     pub fn print_bitboard(bitboard : u64) {
         for rank in (0..8).rev() {
             for file in 0..8 {
                 let square = rank * 8 + file;
                 if (bitboard >> square) & 1u64 == 1u64 {
                     print!("X ");
                 } else {
                     print!(". ");
                 }
             }
             println!();
         }
     }
 
     fn print_castle_rights(&self) {
         println!();
         println!("Castling rights");
         let labels = ["BK", "BQ", "WK", "WQ"];
 
         for i in 0..4 {
             if (self.castle_rights >> i) & 1 == 1 {
                 print!("{}", labels[i]);
                 print!(", ");
             }
         }
         println!();
         println!();
 
     }
}


