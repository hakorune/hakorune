//! HEADERPORT0-I0-MODULEFINAL0-SPLIT0: post-drain finalization input.
//!
//! This is the disconnected boundary between root/function completion and
//! the later module finalizer.  It keeps the drained module candidate and the
//! sealed module declaration facts together without exposing Builder,
//! collector, function-local facts, or a bare `MirModule`.

use super::drained_module_candidate::DrainedModuleCandidateV1;
use super::module_declaration_facts::SealedModuleDeclarationFactsV1;

/// The only input admitted by the future post-drain module finalizer.
#[derive(Debug)]
pub(in crate::mir::builder) struct DrainedModuleFinalizationInputV1 {
    candidate: DrainedModuleCandidateV1,
    declaration_facts: SealedModuleDeclarationFactsV1,
    _seal: DrainedModuleFinalizationInputSealV1,
}

#[derive(Debug)]
struct DrainedModuleFinalizationInputSealV1;

impl DrainedModuleFinalizationInputV1 {
    /// Co-seal the drained candidate and the module declaration snapshot.
    ///
    /// The future finalizer consumes this product exactly once.  No Builder,
    /// collector, fallback, or external publication capability is accepted
    /// at this boundary.
    pub(in crate::mir::builder) fn new(
        candidate: DrainedModuleCandidateV1,
        declaration_facts: SealedModuleDeclarationFactsV1,
    ) -> Self {
        Self {
            candidate,
            declaration_facts,
            _seal: DrainedModuleFinalizationInputSealV1,
        }
    }

    pub(in crate::mir::builder) fn candidate(&self) -> &DrainedModuleCandidateV1 {
        &self.candidate
    }

    pub(in crate::mir::builder) fn declaration_facts(&self) -> &SealedModuleDeclarationFactsV1 {
        &self.declaration_facts
    }

    /// Consume both owners together for the future finalizer stage.
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (DrainedModuleCandidateV1, SealedModuleDeclarationFactsV1) {
        (self.candidate, self.declaration_facts)
    }
}
