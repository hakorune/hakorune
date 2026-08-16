//! Caller-zero opener for one common V2 canonical session.
//!
//! This is intentionally a thin transport wrapper.  The admission owns the
//! source/cohort checks; the canonical session owns the mutable CFG/SSA/PHI
//! state.  No operation or control placement is emitted here.

use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableParameterDescriptorV1;
use crate::mir::compiler::common_v2_session_admission::LoopV2CanonicalSessionAdmissionRefV1;
use crate::mir::loop_recipe_contract::PreparedLoopV2PreSessionEnvelopeV1;

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;

/// One callback-scoped session plus the exact envelope it consumed.  The
/// envelope is retained as a sibling view so a later physicalizer cannot
/// reacquire a second Port loan.
pub(in crate::mir) struct CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    session: CanonicalSsaFunctionSessionV2<'source>,
    envelope: &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope>,
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    pub(in crate::mir) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.session.owner()
    }

    pub(in crate::mir) const fn completion_is_implicit(&self) -> bool {
        self.session.completion_is_implicit()
    }

    pub(in crate::mir) const fn envelope(
        &self,
    ) -> &'envelope PreparedLoopV2PreSessionEnvelopeV1<'envelope, 'envelope> {
        self.envelope
    }

    pub(in crate::mir) fn adopt_physical_entry_lanes(
        &mut self,
        builder: &mut crate::mir::builder::MirBuilder,
        descriptors: &[PhysicalCallableParameterDescriptorV1],
    ) -> Result<(), String> {
        self.session
            .adopt_physical_entry_lanes(builder, descriptors)
    }

    #[cfg(test)]
    pub(in crate::mir) fn physical_entry_sidecar_row_count(&self) -> usize {
        self.session.physical_entry_sidecar_row_count()
    }
}

/// Consume one common admission and open one canonical session owner for the
/// duration of the nested callback.  The caller-zero canary deliberately
/// exposes no lowerer, DraftSeal, or physical placement API yet.
pub(in crate::mir) fn with_common_v2_canonical_session<R>(
    admission: LoopV2CanonicalSessionAdmissionRefV1<'_, '_, '_>,
    callback: impl for<'source, 'envelope> FnOnce(
        CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    ) -> R,
) -> Result<R, String> {
    admission.consume_for_canonical_session(|parts| {
        let envelope = parts.envelope();
        let session = CanonicalSsaFunctionSessionV2::new_common_v2(parts)?;
        Ok(callback(CommonV2CanonicalSessionRefV1 {
            session,
            envelope,
        }))
    })
}
