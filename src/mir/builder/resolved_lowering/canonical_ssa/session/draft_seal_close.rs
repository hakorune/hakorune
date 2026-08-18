//! Canonical session close and DraftSeal handoff.
//!
//! This child keeps the parent session facade below the source-size trigger.
//! It owns no new authority: the parent session still owns CFG, SSA, PHI, and
//! Completion, while DraftSeal remains the only Return projection owner.

use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::MirBuilder;

use super::{
    CanonicalFunctionFinishErrorV1, CanonicalSsaFunctionSessionV2, ReadyCanonicalProfileCloseV1,
};

impl CanonicalSsaFunctionSessionV2<'_> {
    pub(in crate::mir::builder::resolved_lowering) fn finish_for_draft_seal(
        self,
        builder: &mut MirBuilder,
        profile_close: ReadyCanonicalProfileCloseV1,
    ) -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1> {
        let (profile_owner, terminal_block) = profile_close.parts();
        if profile_owner != self.owner {
            return Err(CanonicalFunctionFinishErrorV1::ProfileOwnerMismatch);
        }
        if builder.function_state.current_block != Some(terminal_block) {
            return Err(CanonicalFunctionFinishErrorV1::TerminalBlockMismatch);
        }
        let Self {
            owner,
            root_body,
            root_body_end,
            target_function,
            identity,
            semantics,
            if_control,
            completion,
            cfg,
            phis,
            ..
        } = self;
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(CanonicalFunctionFinishErrorV1::FunctionMissing)?;
        let checked_callout_census = function
            .metadata
            .verify_checked_callout_function(function)
            .map_err(|error| {
                CanonicalFunctionFinishErrorV1::CheckedCallOut(format!("{error:?}"))
            })?;
        cfg.finish(function)
            .map_err(|error| CanonicalFunctionFinishErrorV1::Cfg(error.to_string()))?;
        semantics
            .finish()
            .map_err(CanonicalFunctionFinishErrorV1::Semantic)?;
        if_control
            .finish()
            .map_err(|error| CanonicalFunctionFinishErrorV1::IfControl(format!("{error:?}")))?;
        identity
            .finish()
            .map_err(CanonicalFunctionFinishErrorV1::Identity)?;
        phis.commit(builder)
            .map_err(|error| CanonicalFunctionFinishErrorV1::Phi(error.to_string()))?;
        builder
            .function_state
            .resolved_binding_state
            .finish(owner)
            .map_err(CanonicalFunctionFinishErrorV1::Binding)?;
        let completion = completion
            .finish(&root_body, root_body_end, target_function)
            .map_err(CanonicalFunctionFinishErrorV1::Completion)?;
        if completion.returns_value() && completion.explicit_claims().is_empty() {
            return Err(CanonicalFunctionFinishErrorV1::ReturnOperandMissing);
        }
        Ok(ReadyFunctionDraftSealV1::from_v2_finish(
            completion,
            terminal_block,
            checked_callout_census,
        ))
    }
}
