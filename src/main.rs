mod board;
mod utils;
mod attacks;
mod magics;
mod moves;
mod perft;
mod perftsuite;
mod uci;

use crate::
    uci::uci_loop;

fn main() {
    uci_loop();
}   