//! Canonical session close and DraftSeal handoff.
//!
//! This child keeps the parent session facade below the source-size trigger.
//! Source, semantic, If and Completion checks close the profile. The function
//! control retains SSA and PHI state until DraftSeal verifies the final Return.

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
            .finish(builder)
            .map_err(CanonicalFunctionFinishErrorV1::Identity)?;
        let control = builder
            .function_control_for_owner_v1(owner)
            .map_err(CanonicalFunctionFinishErrorV1::Phi)?;
        control
            .borrow_mut()
            .require_transaction_settled(builder)
            .map_err(CanonicalFunctionFinishErrorV1::Phi)?;
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
