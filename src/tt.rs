use std::mem::size_of;

use crate::moves::Move;

#[derive(Clone, Copy)]
pub struct TranspositionEntry {
    pub hash : u64,
    pub best_move : Move
}

impl TranspositionEntry {
    pub const NULL_ENTRY: Self  = Self {
        hash : 0, best_move : Move::NULL
    };
}

pub struct TTable {
    entries : Vec<TranspositionEntry>
}

impl TTable {
    pub fn new(size_mb : usize) -> Self {
        let length = size_mb * 1024 * 1024 / size_of::<TranspositionEntry>();
        Self {
            entries : vec![TranspositionEntry::NULL_ENTRY; length]
        }
    }

    pub fn get_entry(&self, hash : u64) -> TranspositionEntry {
        self.entries[hash as usize % self.entries.len()]
    }

    pub fn store(&mut self, best_move : Move, hash : u64) {
        let entry_length = self.entries.len();
        self.entries[hash as usize % entry_length] = TranspositionEntry {
            best_move,
            hash
        }
    }

    pub fn clear(&mut self) {
        let len = self.entries.len();

        self.entries = Self::new(len).entries;
    }
}