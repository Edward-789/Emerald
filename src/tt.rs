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
        let mut tt = TTable {
            entries : vec![]
        };

        tt.resize(size_mb);
        tt
    }

    pub fn resize(&mut self, size_mb : usize) {
        let length = size_mb * 1024 * 1024 / size_of::<TranspositionEntry>();
        self.entries.resize(length, TranspositionEntry::NULL_ENTRY);
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
        for i in 0..self.entries.len() {
            self.entries[i] = TranspositionEntry::NULL_ENTRY;
        }
    }    
}