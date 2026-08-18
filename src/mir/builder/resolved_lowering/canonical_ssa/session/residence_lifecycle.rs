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

use super::CanonicalSsaFunctionSessionV2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PinnedTextResidenceLifecycleStateV1 {
    residence: TextFormalResidenceIdV1,
    normal_landing: BasicBlockId,
    finish_emitted: bool,
}

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    /// Consume the one private Enter carrier and install its Normal/Trap CFG
    /// edges through the canonical CFG owner.  The returned capability is
    /// valid only for the admitted normal landing.
    pub(in crate::mir::builder::resolved_lowering) fn emit_pinned_text_residence_enter(
        &mut self,
        builder: &mut MirBuilder,
        carrier: PreparedPinnedTextResidenceLifecycleV1,
    ) -> Result<PinnedTextResidenceFinishCapabilityV1, String> {
        if self.pinned_text_residence.is_some() {
            return Err("pinned-Text Residence Enter was already emitted".to_owned());
        }
        let source = builder
            .function_state
            .current_block
            .ok_or_else(|| "Residence Enter requires a selected canonical block".to_owned())?;
        let residence = carrier.residence();
        let normal_landing = carrier.normal_landing();
        let capability = self
            .cfg
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
        self.pinned_text_residence = Some(PinnedTextResidenceLifecycleStateV1 {
            residence,
            normal_landing,
            finish_emitted: false,
        });
        Ok(capability)
    }

    /// Consume the one-shot Finish capability and install the success-only
    /// marker.  The canonical CFG writer rejects a Return-before-Finish
    /// ordering because the admitted landing must still be unterminated.
    pub(in crate::mir::builder::resolved_lowering) fn emit_pinned_text_residence_finish(
        &mut self,
        builder: &mut MirBuilder,
        capability: PinnedTextResidenceFinishCapabilityV1,
    ) -> Result<(), String> {
        let current = builder
            .function_state
            .current_block
            .ok_or_else(|| "Residence Finish requires a selected canonical block".to_owned())?;
        let state = self
            .pinned_text_residence
            .as_ref()
            .ok_or_else(|| "Residence Finish has no admitted Enter".to_owned())?;
        if state.finish_emitted {
            return Err("pinned-Text Residence Finish was already emitted".to_owned());
        }
        if state.normal_landing != current || capability.normal_landing() != current {
            return Err("Residence Finish is outside the admitted normal landing".to_owned());
        }
        if state.residence != capability.residence() {
            return Err("Residence Finish provenance differs from Enter".to_owned());
        }
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "Residence Finish requires a current function".to_owned())?;
        self.cfg
            .emit_pinned_text_residence_finish(function, current, capability)
            .map_err(|error| error.to_string())?;
        self.pinned_text_residence
            .as_mut()
            .expect("Residence state was checked above")
            .finish_emitted = true;
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn lifecycle_state_for_test(&self) -> Option<PinnedTextResidenceLifecycleStateV1> {
        self.pinned_text_residence
    }
}
