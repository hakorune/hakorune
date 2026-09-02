//! Atomic owner of one canonical module Builder candidate.
//!
//! The caller's Builder is never mutated until the complete candidate module
//! has passed Builder lowering and compiler post-processing. Dropping this
//! session on any error discards all module, function, scope, and ID effects.

use crate::mir::builder::{BuilderInvocationConfigV1, MirBuilder};

pub(super) struct CanonicalModuleLoweringSessionV1 {
    candidate: MirBuilder,
}

impl CanonicalModuleLoweringSessionV1 {
    pub(super) fn open(current: &MirBuilder) -> Self {
        let source_file = current.current_source_file();
        let config =
            BuilderInvocationConfigV1::snapshot_for_canonical(current, source_file.as_deref());
        let mut candidate = MirBuilder::new();
        config.install_into(&mut candidate);
        Self { candidate }
    }

    pub(super) fn builder_mut(&mut self) -> &mut MirBuilder {
        &mut self.candidate
    }

    pub(super) fn commit(self, current: &mut MirBuilder) {
        *current = self.candidate;
    }
}
