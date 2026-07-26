//! Receipt-required request boundary for unified Call emission.
//!
//! Ordinary callers retain the compatibility facade. The typed sibling
//! requires the existing generic physical Call terminal and never retries
//! through legacy emission or accepts an alternate route.

use crate::mir::builder::calls::CallTarget;
use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::{MirBuilder, ValueId};

use super::{
    CompletedUnifiedValueCallEmissionV1, UnifiedCallAlternateRouteV1, UnifiedCallEmitterBox,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnifiedCompatibilityDispositionV1 {
    PermitLegacy,
    RequireGenericReceipt,
}

pub(super) enum UnifiedCallAttemptErrorV1 {
    Emission(String),
    UnifiedDisabledForReceipt,
}

impl UnifiedCallAttemptErrorV1 {
    pub(super) fn into_ordinary_string(self) -> String {
        match self {
            Self::Emission(detail) => detail,
            Self::UnifiedDisabledForReceipt => {
                "[freeze:contract][unified_call/physical_receipt_disabled] generic physical Call receipt requires unified emission"
                    .to_owned()
            }
        }
    }

    fn into_receipt_error(self) -> UnifiedValueCallReceiptErrorV1 {
        match self {
            Self::Emission(detail) => UnifiedValueCallReceiptErrorV1::Emission {
                detail: detail.into_boxed_str(),
            },
            Self::UnifiedDisabledForReceipt => UnifiedValueCallReceiptErrorV1::UnifiedDisabled,
        }
    }
}

/// Typed rejection returned by the receipt-required unified-call sibling.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum UnifiedValueCallReceiptErrorV1 {
    UnifiedDisabled,
    Emission { detail: Box<str> },
    AlternateRoute { route: UnifiedCallAlternateRouteV1 },
    FinalDestinationMissing,
}

impl UnifiedCallEmitterBox {
    /// Emit one generic value-producing physical Call and return its exact
    /// post-success destination receipt.
    pub(in crate::mir::builder) fn emit_unified_value_call_with_lookup_receipt_v1(
        builder: &mut MirBuilder,
        destination: ValueId,
        target: CallTarget,
        args: Vec<ValueId>,
        lookup: Option<&dyn FunctionSignatureLookupV1>,
    ) -> Result<CompletedUnifiedValueCallEmissionV1, UnifiedValueCallReceiptErrorV1> {
        Self::emit_unified_call_outcome_with_lookup_and_map_replay(
            builder,
            Some(destination),
            target,
            args,
            lookup,
            None,
            UnifiedCompatibilityDispositionV1::RequireGenericReceipt,
        )
        .map_err(UnifiedCallAttemptErrorV1::into_receipt_error)?
        .into_required_value_receipt()
    }
}
