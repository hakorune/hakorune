//! Atomic owner of one canonical module Builder candidate.
//!
//! The caller's Builder is never mutated until the complete candidate module
//! has passed Builder lowering and compiler post-processing. Dropping this
//! session on any error discards all module, function, scope, and ID effects.

use crate::mir::builder::MirBuilder;

pub(super) struct CanonicalModuleLoweringSessionV1 {
    candidate: MirBuilder,
}

impl CanonicalModuleLoweringSessionV1 {
    pub(super) fn open(current: &MirBuilder) -> Self {
        let mut candidate = MirBuilder::new();
        candidate.comp_ctx.quiet_internal_logs = current.comp_ctx.quiet_internal_logs;
        Self { candidate }
    }

    pub(super) fn builder_mut(&mut self) -> &mut MirBuilder {
        &mut self.candidate
    }

    pub(super) fn commit(self, current: &mut MirBuilder) {
        *current = self.candidate;
    }
}
