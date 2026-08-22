//! Canonical-session bridge issuer for the installed S6C ExactText sidecar.
//!
//! The session lends the already adopted sidecar to the mechanical bridge
//! issuer.  No source role or runtime pair is reconstructed here.

use super::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::common_v2_s6c_textref_entry_bridge::{
    issue_common_v2_s6c_textref_entry_bridge_plan_v1, CommonV2S6CTextRefEntryBridgePlanV1,
    CommonV2S6CTextRefEntryBridgeRejectV1,
};

impl<'source> CanonicalSsaFunctionSessionV2<'source> {
    pub(in crate::mir::builder::resolved_lowering) fn issue_s6c_textref_entry_bridge_plan(
        &self,
    ) -> Result<CommonV2S6CTextRefEntryBridgePlanV1, CommonV2S6CTextRefEntryBridgeRejectV1> {
        let sidecar = self
            .physical_entry_sidecar
            .as_ref()
            .ok_or(CommonV2S6CTextRefEntryBridgeRejectV1::EmptySidecar)?;
        issue_common_v2_s6c_textref_entry_bridge_plan_v1(sidecar, self.owner.compilation_brand())
    }
}
