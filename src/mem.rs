use std::fmt::{Debug, Formatter};

use crate::cpu::{Byte, Word};

pub const MAX_MEMORY: Word = Word::MAX;

pub struct Memory {
    data: [u8; MAX_MEMORY as usize],
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Memory").finish()
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; MAX_MEMORY as usize],
        }
    }

    pub fn read(&self, address: Word) -> Byte {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: Word, data: Byte) {
        self.data[address as usize] = data;
    }
}
