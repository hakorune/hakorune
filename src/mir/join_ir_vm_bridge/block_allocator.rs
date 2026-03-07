//! Block Allocator - Unified block ID allocation
//!
//! Phase 260 P0.2: Extracted from joinir_block_converter.rs
//! Eliminates repeated block allocation shapes (4x duplication).
//!
//! ## Structure Before
//!
//! ```ignore
//! let then_block = BasicBlockId(self.next_block_id);
//! self.next_block_id += 1;
//! let else_block = BasicBlockId(self.next_block_id);
//! self.next_block_id += 1;
//! let merge_block = BasicBlockId(self.next_block_id);
//! self.next_block_id += 1;
//! ```
//!
//! ## Structure After
//!
//! ```ignore
//! let (then_block, else_block, merge_block) = allocator.allocate_three();
//! ```

use crate::mir::BasicBlockId;

/// Block ID allocator for JoinIR conversion
///
/// Provides deterministic, sequential block ID allocation with
/// convenience methods for common patterns (2-block, 3-block allocation).
#[derive(Debug)]
pub struct BlockAllocator {
    next_id: u32,
}

impl BlockAllocator {
    /// Create a new allocator starting from specified ID
    ///
    /// # Arguments
    ///
    /// * `start_id` - First block ID to allocate (typically 1 since 0 is entry)
    pub fn new(start_id: u32) -> Self {
        Self { next_id: start_id }
    }

    /// Allocate a single block ID
    pub fn allocate_one(&mut self) -> BasicBlockId {
        let id = BasicBlockId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Allocate two block IDs (e.g., exit + continue)
    ///
    /// Returns: (first, second)
    pub fn allocate_two(&mut self) -> (BasicBlockId, BasicBlockId) {
        let first = self.allocate_one();
        let second = self.allocate_one();
        (first, second)
    }

    /// Allocate three block IDs (e.g., then + else + merge)
    ///
    /// Returns: (first, second, third)
    pub fn allocate_three(&mut self) -> (BasicBlockId, BasicBlockId, BasicBlockId) {
        let first = self.allocate_one();
        let second = self.allocate_one();
        let third = self.allocate_one();
        (first, second, third)
    }

    /// Allocate N block IDs
    ///
    /// Returns: Vec of N allocated IDs
    pub fn allocate_n(&mut self, n: usize) -> Vec<BasicBlockId> {
        (0..n).map(|_| self.allocate_one()).collect()
    }

    /// Get current next ID without allocating
    pub fn peek_next(&self) -> u32 {
        self.next_id
    }

    /// Get current next ID and update internal state
    /// Used for compatibility with existing converter code
    pub fn next_id_mut(&mut self) -> &mut u32 {
        &mut self.next_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_one() {
        let mut allocator = BlockAllocator::new(1);
        assert_eq!(allocator.allocate_one(), BasicBlockId(1));
        assert_eq!(allocator.allocate_one(), BasicBlockId(2));
        assert_eq!(allocator.allocate_one(), BasicBlockId(3));
    }

    #[test]
    fn test_allocate_two() {
        let mut allocator = BlockAllocator::new(5);
        let (a, b) = allocator.allocate_two();
        assert_eq!(a, BasicBlockId(5));
        assert_eq!(b, BasicBlockId(6));
        assert_eq!(allocator.peek_next(), 7);
    }

    #[test]
    fn test_allocate_three() {
        let mut allocator = BlockAllocator::new(10);
        let (a, b, c) = allocator.allocate_three();
        assert_eq!(a, BasicBlockId(10));
        assert_eq!(b, BasicBlockId(11));
        assert_eq!(c, BasicBlockId(12));
        assert_eq!(allocator.peek_next(), 13);
    }

    #[test]
    fn test_allocate_n() {
        let mut allocator = BlockAllocator::new(0);
        let blocks = allocator.allocate_n(5);
        assert_eq!(blocks.len(), 5);
        assert_eq!(blocks[0], BasicBlockId(0));
        assert_eq!(blocks[4], BasicBlockId(4));
        assert_eq!(allocator.peek_next(), 5);
    }

    #[test]
    fn test_peek_next() {
        let allocator = BlockAllocator::new(42);
        assert_eq!(allocator.peek_next(), 42);
    }
}
