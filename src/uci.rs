use std::io;

use crate::perft::run_perft_suite;

pub fn uci_loop() {
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    match command {
        "uci" => println!("uciok"),
        "ready" => println!("readyok"),
        "perftsuite" => run_perft_suite(),
        _ => {}
    }
}