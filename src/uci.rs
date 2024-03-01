use std::{io, time::Instant};

use crate::{
    board::Board, moves::MoveList, perft::{perft, run_perft_suite}
};

pub fn uci_loop() {
    let mut board = Board::read_fen(Board::STARTPOS);
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        let split_command = command.split(' ').collect::<Vec<&str>>();

        if command == "quit" {
            break;
        } else if command == "perftsuite" {
            run_perft_suite();
        } else if command == "uci" {
            println!("uciok")
        } else if command == "isready" {
            println!("readyok");
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
            board = load_position(split_command)
        } else if split_command[0] == "go" {
            go(split_command, &board);
        } 
    }
}

fn go(split_command : Vec<&str>, board : &Board) {
    let moves = board.psuedolegal_movegen();
    let filtered = {
        let mut list = MoveList::EMPTY;
        for i in 0..moves.length {
            let mut tmp = *board;

            if tmp.apply(moves.moves[i]) {
                list.push(moves.moves[i].from_square(), moves.moves[i].to_square(), moves.moves[i].flag());
            }
        }

        list
    };

    println!("{}{}", "bestmove ", filtered.moves[(split_command[2].parse::<usize>().unwrap()) % filtered.length].to_uci())
}

fn load_position(split_command : Vec<&str>) -> Board {
    let mut board = Board::read_fen(Board::STARTPOS);

    if split_command[1] == "fen" {
        let mut fen = String::new();
        let mut move_start_index = 0;
        for i in 2..split_command.len() {
            move_start_index = i;
            if split_command[i] == "moves" {
                break;
            };
            fen += split_command[i];
            fen += " ";
        }

        board = Board::read_fen(&fen);

        for i in (move_start_index + 1)..split_command.len() {
            let moves = board.psuedolegal_movegen();

            for j in 0..moves.length {
                if moves.moves[j].to_uci() == split_command[i] {
                    board.apply(moves.moves[j]);
                }
            }
        }
    }

    board
}