//! LocalSSA convenience wrappers
//!
//! Readability helpers that wrap ssa::local module functions.
//! These ensure values have definitions in the current block.

impl super::super::MirBuilder {
    // ---- LocalSSA convenience (readability helpers) ----
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn local_recv(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::recv(self, v)
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn local_arg(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::arg(self, v)
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn local_cmp_operand(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::cmp_operand(self, v)
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn local_field_base(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::field_base(self, v)
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn local_cond(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::cond(self, v)
    }

    /// Local SSA: ensure a value has a definition in the current block and cache it per-block.
    /// kind: 0 = recv (reserved for args in future)
    pub(crate) fn local_ssa_ensure(
        &mut self,
        v: super::super::ValueId,
        kind: u8,
    ) -> super::super::ValueId {
        use super::super::ssa::local::{ensure, LocalKind};
        let lk = match kind {
            0 => LocalKind::Recv,
            1 => LocalKind::Arg,
            2 => LocalKind::CompareOperand,
            4 => LocalKind::Cond,
            x => LocalKind::Other(x),
        };
        ensure(self, v, lk)
    }
}
