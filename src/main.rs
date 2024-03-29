mod board;
mod utils;
mod attacks;
mod magics;
mod moves;
mod perft;
mod perftsuite;
mod uci;
mod zobrist;
mod search;
mod eval;
mod tt;
mod eval_consts;

use crate::
    uci::uci_loop;

fn main() {
    uci_loop();
}   