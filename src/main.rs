mod board;
#[macro_use]
mod utils;
mod attacks;
mod magics;
mod moves;
mod perft;
mod perftsuite;

use crate::
    perft::run_perft_suite;

fn main() {
    run_perft_suite();
}   