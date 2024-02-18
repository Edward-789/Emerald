use crate::board::Board;


pub fn perft<const ROOT : bool>(board : &Board, depth : u8) -> u64 {
    let movelist = board.psuedolegal_movegen();

    let mut positions = 0;
    let leaf = depth == 1;

    for i in 0..movelist.length {
        let mut next_board = *board;

        if next_board.apply(movelist.moves[i]) {

            let num = if leaf {1} else {
                perft::<false>(&next_board, depth - 1)
            };

            positions += num;

            
            if ROOT {
                println!("{}: {num}", movelist.moves[i].to_uci());
            }
        }
    }

    positions
}