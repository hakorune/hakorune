/*!
 * BasicBlockId - Control flow graph block identity
 *
 * This substrate owns the block identity and generator only. The actual basic
 * block structure still lives in `src/mir/basic_block.rs` for now.
 */

use std::fmt;

/// Unique identifier for basic blocks within a function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BasicBlockId(pub u32);

impl BasicBlockId {
    /// Create a new BasicBlockId
    pub fn new(id: u32) -> Self {
        BasicBlockId(id)
    }

    /// Get the raw ID value
    pub fn as_u32(self) -> u32 {
        self.0
    }

    /// Create BasicBlockId from usize (for array indexing)
    pub fn from_usize(id: usize) -> Self {
        BasicBlockId(id as u32)
    }

    /// Convert to usize (for array indexing)
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for BasicBlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bb{}", self.0)
    }
}

/// Basic block ID generator
#[derive(Debug, Clone)]
pub struct BasicBlockIdGenerator {
    next_id: u32,
}

impl BasicBlockIdGenerator {
    /// Create a new generator starting from 0
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    /// Generate the next unique BasicBlockId
    pub fn next(&mut self) -> BasicBlockId {
        let id = BasicBlockId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Peek at the next ID without consuming it
    pub fn peek_next(&self) -> BasicBlockId {
        BasicBlockId(self.next_id)
    }

    /// Reset the generator (for testing)
    pub fn reset(&mut self) {
        self.next_id = 0;
    }
}

impl Default for BasicBlockIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_block_id_generation() {
        let mut gen = BasicBlockIdGenerator::new();
        let bb1 = gen.next();
        let bb2 = gen.next();
        let bb3 = gen.next();

        assert_eq!(bb1, BasicBlockId(0));
        assert_eq!(bb2, BasicBlockId(1));
        assert_eq!(bb3, BasicBlockId(2));

        assert_eq!(gen.peek_next(), BasicBlockId(3));
    }
}
