//! Value ID and Block ID allocation
//!
//! Critical path: ID allocation with PHI reservation support.
//!
//! Phase 136 Step 2/7 + Phase 2-2: Uses core_ctx as SSOT (no sync needed)
//! Phase 201-A: Skips reserved ValueIds (PHI dsts from LoopHeaderPhiBuilder)

use crate::mir::MirType;

impl super::super::MirBuilder {
    // ---- Value ID allocation (function-local or module-global) ----
    /// Allocate a new ValueId in the appropriate context
    /// - Inside function: uses function-local allocator
    /// - Outside function: uses module-global allocator
    ///
    /// Phase 201-A: Skips reserved ValueIds (PHI dsts from LoopHeaderPhiBuilder)
    /// to prevent carrier value corruption in JoinIR loops.
    #[inline]
    #[allow(deprecated)]
    pub(crate) fn next_value_id(&mut self) -> super::super::ValueId {
        loop {
            let candidate = if let Some(ref mut f) = self.scope_ctx.current_function {
                f.next_value_id() // Function context
            } else {
                // Phase 136 Step 2/7 + Phase 2-2: Use core_ctx as SSOT (no sync needed)
                self.core_ctx.next_value()
            };

            // Phase 201-A: Skip reserved PHI dst ValueIds
            if !self.comp_ctx.reserved_value_ids.contains(&candidate) {
                return candidate;
            }
            // Reserved ID - try next one (loop continues)
            if crate::config::env::builder_201a_debug() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[201-A] next_value_id: Skipping reserved {:?}",
                    candidate
                ));
            }
        }
    }

    /// Allocate a new ValueId and register its MIR type (convenience helper).
    ///
    /// This is a readability helper for code that repeatedly does:
    /// `let v = next_value_id(); type_ctx.value_types.insert(v, ty);`
    #[inline]
    pub(crate) fn alloc_typed(&mut self, ty: MirType) -> super::super::ValueId {
        let value_id = self.next_value_id();
        self.type_ctx.value_types.insert(value_id, ty);
        value_id
    }

    /// Allocate a new BasicBlockId
    ///
    /// Phase 136 Step 2/7 + Phase 2-2: Uses core_ctx as SSOT (no sync needed).
    #[inline]
    pub(crate) fn next_block_id(&mut self) -> super::super::BasicBlockId {
        self.core_ctx.next_block()
    }
}
