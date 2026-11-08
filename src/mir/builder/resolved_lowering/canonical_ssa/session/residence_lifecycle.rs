//! Canonical-session owner for the physical pinned-Text lifecycle pair.
//!
//! This child keeps the parent session thin.  It owns only one function-local
//! lifecycle state so a second Enter or Finish cannot be admitted by a second
//! caller.  Source meaning, runtime status, and backend lowering remain
//! outside this module.

use crate::mir::builder::MirBuilder;
use crate::mir::pinned_text_residence_lifecycle::{
    PinnedTextResidenceFinishCapabilityV1, PreparedPinnedTextResidenceLifecycleV1,
    TextFormalResidenceIdV1,
};
use crate::mir::BasicBlockId;
use std::collections::BTreeSet;

use super::CanonicalSsaFunctionSessionV2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PinnedTextResidenceLifecycleStateV1 {
    residence: TextFormalResidenceIdV1,
    finish_blocks: BTreeSet<BasicBlockId>,
}

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    /// Consume the one private Enter carrier and install its Normal/Trap CFG
    /// edges through the canonical CFG owner.  The returned capability is a
    /// function-local finish obligation; the DraftSeal exit-set projector
    /// consumes it once and chooses each already-validated explicit exit.
    pub(in crate::mir::builder::resolved_lowering) fn emit_pinned_text_residence_enter(
        &mut self,
        builder: &mut MirBuilder,
        carrier: PreparedPinnedTextResidenceLifecycleV1,
    ) -> Result<PinnedTextResidenceFinishCapabilityV1, String> {
        if self.s6c_state.pinned_text_residence().is_some() {
            return Err("pinned-Text Residence Enter was already emitted".to_owned());
        }
        if carrier.plan().owner() != self.owner {
            return Err("Residence Enter carrier owner differs from canonical session".to_owned());
        }
        let boundary = self
            .physical_entry_execution
            .ok_or_else(|| "Residence Enter requires a physical entry boundary".to_owned())?;
        let source = builder
            .function_state
            .current_block
            .ok_or_else(|| "Residence Enter requires a selected canonical block".to_owned())?;
        if source != boundary.function_entry() {
            return Err("Residence Enter source differs from physical function entry".to_owned());
        }
        if carrier.normal_landing() != boundary.execution_entry() {
            return Err("Residence Enter normal landing differs from execution entry".to_owned());
        }
        let residence = carrier.residence();
        let capability = self
            .cfg()?
            .emit_pinned_text_residence_enter(
                builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| "Residence Enter requires a current function".to_owned())?,
                source,
                carrier,
            )
            .map_err(|error| error.to_string())?;
        self.s6c_state
            .set_pinned_text_residence(PinnedTextResidenceLifecycleStateV1 {
                residence,
                finish_blocks: BTreeSet::new(),
            });
        Ok(capability)
    }
}
