const fn rand(mut seed : u64) -> u64 {
    seed ^= seed << 13;
    seed ^= seed >> 6;
    seed ^= seed << 21;
    seed
}

const fn create_zobrist_hashes() -> ([[[u64; 64];8] ; 2], [u64; 16], u64, [u64; 64]) {
    let mut seed = 111_111_111;
    seed = rand(seed);

    let mut piece_table = [[[0; 64];8] ; 2];

    let mut colour = 0;
    let mut piece = 2; 
    let mut square = 0;
    
    while colour < 2 {
        while piece < 8 {
            while square < 64 {
                seed = rand(seed);
                piece_table[colour][piece][square] = seed;
                square += 1
            }
            piece += 1;
            square = 0;
        }

        piece = 2;
        colour += 1;
    }


    let mut castle_table = [0; 16];

    let mut i = 0;

    while i < 16 {
        seed = rand(seed);
        castle_table[i] = seed;
        i += 1;
    }

    i = 0;
    let mut en_passant_table = [0;64];

    while i < 64 {
        seed = rand(seed);
        en_passant_table[i] = seed;
        i += 1
    }

    let stm_hash = rand(seed);

    (piece_table, castle_table, stm_hash, en_passant_table)
}

pub const PIECE_HASHES: [[[u64; 64];8] ; 2] = create_zobrist_hashes().0;
pub const CASTLE_HASHES: [u64; 16] = create_zobrist_hashes().1;
pub const STM_HASH: u64 = create_zobrist_hashes().2;
pub const EN_PASSANT_HASHES: [u64; 64] = create_zobrist_hashes().3;