//! A selected source call requires the existing physical Array alternate.
//! No Call receipt is forged, and no observation survives on Builder state.
use super::*;
use crate::mir::builder::CompletedArrayElementWriteV1;

impl UnifiedCallEmitterBox {
    pub(in crate::mir::builder) fn emit_array_write_with_lookup_receipt_v1(
        builder: &mut MirBuilder, dst: Option<ValueId>, target: CallTarget, args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
    ) -> Result<CompletedArrayElementWriteV1, String> {
        let mut observation = None;
        let outcome = Self::emit_unified_call_outcome_with_array_observation_v1(builder,
            dst, target, args, lookup, None, UnifiedCallSignaturePublicationV1::Existing,
            Some(&mut observation)).map_err(UnifiedCallAttemptErrorV1::into_ordinary_string)?;
        if !matches!(outcome, UnifiedCallEmissionOutcomeV1::Alternate(UnifiedCallAlternateRouteV1::KnownArrayWrite)) {
            return Err("[freeze:contract][array-write/selected-alternate-missing]".into());
        }
        observation.ok_or_else(|| "[freeze:contract][array-write/observation-missing]".into())
    }
}
