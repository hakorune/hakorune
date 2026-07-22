//! CUT0-S0: one Builder-free post-drain finalization terminal.
//!
//! The finalizer consumes the co-sealed candidate/facts input exactly once.
//! It does not accept a Builder, collector, HeaderPort, transient context,
//! bare module, fallback, retry, or publication capability.

use super::drained_module_candidate::DrainedModuleCandidateV1;
use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_finalization_split::DrainedModuleFinalizationInputV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct FinalizedModuleCandidateV1 {
    candidate: DrainedModuleCandidateV1,
    declaration_facts: SealedModuleDeclarationFactsV1,
    _seal: FinalizedModuleCandidateSealV1,
}

#[derive(Debug)]
struct FinalizedModuleCandidateSealV1;

/// The only post-drain finalizer.  This is intentionally infallible: all
/// fallible inventory and declaration checks belong before this boundary.
pub(in crate::mir::builder) fn finalize_drained_module_once(
    input: DrainedModuleFinalizationInputV1,
) -> FinalizedModuleCandidateV1 {
    let (candidate, declaration_facts) = input.into_parts();
    FinalizedModuleCandidateV1 {
        candidate,
        declaration_facts,
        _seal: FinalizedModuleCandidateSealV1,
    }
}

impl FinalizedModuleCandidateV1 {
    pub(in crate::mir::builder) fn candidate(&self) -> &DrainedModuleCandidateV1 {
        &self.candidate
    }

    pub(in crate::mir::builder) fn declaration_facts(&self) -> &SealedModuleDeclarationFactsV1 {
        &self.declaration_facts
    }
}
