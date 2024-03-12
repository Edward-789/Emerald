use std::{io, time::Instant};

use crate::{
    board::Board, perft::{perft, run_perft_suite}, search::Searcher, utils::Colour
};

pub fn uci_loop() {
    let mut board = Board::read_fen(Board::STARTPOS);
    let mut searcher = Searcher::new(0, Vec::new());
    loop {
        let mut input = String::new();
        let mut zobrist_history = Vec::new();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        let split_command = command.split(' ').collect::<Vec<&str>>();

        if command == "quit" {
            break;
        }
        
        if command == "perftsuite" {
            run_perft_suite();
        } else if command == "uci" {
            println!("uciok")
        } else if command == "isready" {
            println!("readyok");
        } else if command == "ucinewgame" {
            searcher.tt.clear();
        } else if split_command[0] == "perft" && split_command[1].parse::<u8>().is_ok() {
            let start_time = Instant::now();
            let nodes = perft::<false, false>(&board, split_command[1].parse::<u8>().unwrap());

            println!("{}{}", "Nodes : ", nodes);
            println!("{}{}", "Milliseconds : ", start_time.elapsed().as_millis());
            println!("{}{}", "NPS : ", (nodes as u128 / (start_time.elapsed().as_millis() + 1)) * 1000);
        } else if split_command[0] == "splitperft" && split_command[1].parse::<u8>().is_ok() {
            let start_time = Instant::now();
            let nodes = perft::<true, true>(&board, split_command[1].parse::<u8>().unwrap());

            println!();
            println!("{}{}", "Milliseconds : ", start_time.elapsed().as_millis());
            println!("{}{}", "NPS : ", (nodes as u128 / (start_time.elapsed().as_millis() + 1)) * 1000);
        } else if split_command[0] == "position" {
            board = load_position(split_command, &mut zobrist_history);
        } else if split_command[0] == "go" {
            go(split_command, &board, zobrist_history, &mut searcher);
        } 
    }
}

fn go(split_command : Vec<&str>, board : &Board, zobrist_history : Vec<u64>, searcher : &mut Searcher) {
    searcher.zobrist_history = zobrist_history;

    for i in 0..split_command.len() {
        if split_command[i] == "btime" && board.colour_to_move == Colour::Black ||
           split_command[i] == "wtime" && board.colour_to_move == Colour::White {
                searcher.max_time = split_command[i + 1].parse::<u128>().unwrap();
           }
    }
    searcher.timer = Instant::now();
    searcher.iterative_deepening(board);

    println!("{}{}", "bestmove ", searcher.best_move.to_uci())
}

fn load_position(split_command : Vec<&str>, zobrist_history : &mut Vec<u64>) -> Board {
    let mut board = Board::read_fen(Board::STARTPOS);
    let mut move_start_index = 0;


    if split_command[1] == "fen" {
        let mut fen = String::new();
        for i in 2..split_command.len() {
            move_start_index = i;
            if split_command[i] == "moves" {
                break;
            };
            fen += split_command[i];
            fen += " ";
        }

        board = Board::read_fen(&fen);
    } else if split_command[1] == "startpos" {
        move_start_index = 2;
    }

    for i in (move_start_index + 1)..split_command.len() {
        zobrist_history.push(board.zobrist);
        let moves = board.psuedolegal_movegen();

        for j in 0..moves.length {
            if moves.moves[j].to_uci() == split_command[i] {
                board.apply(moves.moves[j]);
            }
        }
    }


    board
}