//! Canonical session bridge for function-local pinned-text transport plans.
//!
//! The plan table is metadata transport only.  This child keeps stamp binding
//! and row issuance behind the same canonical SSA session that owns the
//! destination `ValueId`; it does not derive source meaning or runtime state.

use crate::mir::builder::MirBuilder;
use crate::mir::pinned_text_access_plan::{PinnedTextAccessKindV1, PinnedTextAccessPlanIdV1};

use super::CanonicalSsaFunctionSessionV2;

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn issue_pinned_text_plan(
        &mut self,
        builder: &mut MirBuilder,
        stamp: u64,
        kind: PinnedTextAccessKindV1,
    ) -> Result<PinnedTextAccessPlanIdV1, String> {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "pinned text plan requires current function".to_owned())?;
        function
            .metadata
            .pinned_text_access_plans
            .bind_stamp_once(stamp)?;
        Ok(function.metadata.pinned_text_access_plans.issue(kind))
    }
}
