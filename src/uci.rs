use std::{io, time::Instant};

use crate::{
    board::Board, 
    perft::{perft, run_perft_suite}
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
        } else if split_command[0] == "perft" && split_command[1].parse::<u8>().is_ok() {
            let start_time = Instant::now();
            let nodes = perft::<false, false>(&board, split_command[1].parse::<u8>().unwrap());

            println!("{}{}", "Nodes : ", nodes);
            println!("{}{}", "Milliseconds : ", start_time.elapsed().as_millis());
            println!("{}{}", "NPS : ", (nodes as u128 / (start_time.elapsed().as_millis() + 1)) * 1000);
        } else if split_command[0] == "splitperft" && split_command[1].parse::<u8>().is_ok() {
            let start_time = Instant::now();
            let nodes = perft::<false, false>(&board, split_command[1].parse::<u8>().unwrap());

            println!();
            println!("{}{}", "Milliseconds : ", start_time.elapsed().as_millis());
            println!("{}{}", "NPS : ", (nodes as u128 / (start_time.elapsed().as_millis() + 1)) * 1000);
        } else if split_command[0] == "position" {
            board = load_position(split_command)
        }
    }
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

        fen = fen.trim().to_string();   // trim trailing whitespace
        board = Board::read_fen(&fen);

        for i in (move_start_index + 1)..split_command.len() {
            board.apply_uci_move(split_command[i]);
        }
    }

    board
}