/*!
 * BindingId - Lexical variable binding identity (Phase 74)
 *
 * Phase 74: BindingId Infrastructure
 * Provides a parallel identifier system alongside ValueId for tracking
 * lexical variable bindings independently from SSA values.
 *
 * ## What is a BindingId?
 *
 * A `BindingId` represents a unique lexical variable binding in the source code,
 * separate from SSA `ValueId`s. While ValueId tracks SSA values that may be
 * renamed through PHI nodes and shadowing, BindingId tracks the original
 * variable binding identity.
 *
 * ### Relationship with ValueId
 *
 * - **ValueId**: SSA value identity (may change through PHI nodes, renaming)
 * - **BindingId**: Lexical binding identity (stable across shadowing)
 *
 * Example:
 * ```nyash
 * local x = 1;      // BindingId(0), ValueId(10)
 * {
 *   local x = 2;    // BindingId(1), ValueId(20) <- new binding, shadows outer x
 * }                  // BindingId(1) goes out of scope, restore BindingId(0)
 * ```
 *
 * ### Relationship with Shadowing
 *
 * Shadowing creates a **new BindingId** for the inner scope:
 * - Outer `x`: BindingId(0) -> ValueId(10)
 * - Inner `x`: BindingId(1) -> ValueId(20)
 *
 * On scope exit, BindingId(1) is discarded and BindingId(0) is restored.
 *
 * ## Design Goals
 *
 * 1. **Parallel Allocation**: BindingId and ValueId allocate independently
 * 2. **Zero Runtime Cost**: Infrastructure layer only, no production impact
 * 3. **Incremental Adoption**: Can be added without changing existing behavior
 * 4. **Future-Ready**: Supports Phase 75+ ScopeManager migration
 */

/// Unique identifier for a lexical variable binding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BindingId(pub u32);

impl BindingId {
    /// Create a new BindingId (raw constructor, prefer `allocate_binding_id()` in MirBuilder)
    pub fn new(id: u32) -> Self {
        debug_assert!(
            id < u32::MAX,
            "BindingId overflow: attempted to create BindingId({})",
            id
        );
        BindingId(id)
    }

    /// Get the next sequential BindingId
    pub fn next(self) -> Self {
        debug_assert!(
            self.0 < u32::MAX - 1,
            "BindingId overflow: next() called on BindingId({})",
            self.0
        );
        BindingId(self.0 + 1)
    }

    /// Get the raw u32 value
    pub fn raw(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for BindingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BindingId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_id_creation() {
        let id = BindingId::new(0);
        assert_eq!(id.0, 0);
        assert_eq!(id.raw(), 0);
    }

    #[test]
    fn test_binding_id_next() {
        let id = BindingId::new(0);
        let next = id.next();
        assert_eq!(next.0, 1);
        assert_eq!(next.raw(), 1);
    }

    #[test]
    fn test_binding_id_display() {
        let id = BindingId::new(42);
        assert_eq!(format!("{}", id), "BindingId(42)");
    }

    #[test]
    fn test_binding_id_ordering() {
        let id0 = BindingId::new(0);
        let id1 = BindingId::new(1);
        assert!(id0 < id1);
        assert!(id1 > id0);
        assert_eq!(id0, id0);
    }

    #[test]
    #[should_panic(expected = "BindingId overflow")]
    #[cfg(debug_assertions)]
    fn test_binding_id_overflow() {
        let id = BindingId::new(u32::MAX - 1);
        let _ = id.next(); // Should panic in debug build
    }
}
