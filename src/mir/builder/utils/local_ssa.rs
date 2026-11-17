//! LocalSSA convenience wrappers
//!
//! Readability helpers that wrap ssa::local module functions.
//! These ensure values have definitions in the current block.

impl super::super::MirBuilder {
    // ---- LocalSSA convenience (readability helpers) ----
    #[inline]
    pub(crate) fn local_recv(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::recv(self, v)
    }

    #[inline]
    pub(crate) fn local_arg(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::arg(self, v)
    }

    #[inline]
    pub(crate) fn local_field_base(&mut self, v: super::super::ValueId) -> super::super::ValueId {
        super::super::ssa::local::field_base(self, v)
    }

}
