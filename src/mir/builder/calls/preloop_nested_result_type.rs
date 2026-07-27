//! Builder-free Integer fact preparation for one emitted pre-loop result.
//!
//! This box consumes the source-qualified emitted receipt and delegates the
//! complete fact policy to `TypeFactDecisionV1`. S0 owns no Builder,
//! `TypeContext`, Call emission, source lookup, or fact-store write.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::{MirType, ValueId};

use super::preloop_nested_result_receipt::EmittedNestedInstanceCallV1;

/// One emitted nested result paired with a prepared exact-Integer decision.
#[derive(Debug)]
pub(super) struct PreparedPreloopNestedIntegerPublicationV1 {
    receipt: EmittedNestedInstanceCallV1,
    publication: PreparedTypeFactPublicationV1,
}

/// Typed fact conflict retaining the one-shot emitted receipt.
#[derive(Debug)]
pub(super) struct RejectedPreloopNestedIntegerPublicationV1 {
    receipt: EmittedNestedInstanceCallV1,
    cause: TypeFactDecisionErrorV1,
}

impl PreparedPreloopNestedIntegerPublicationV1 {
    pub(super) fn prepare(
        receipt: EmittedNestedInstanceCallV1,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, RejectedPreloopNestedIntegerPublicationV1> {
        match TypeFactDecisionV1::prepare(existing_destination, Some(&MirType::Integer)) {
            Ok(publication) => Ok(Self {
                receipt,
                publication,
            }),
            Err(cause) => Err(RejectedPreloopNestedIntegerPublicationV1 { receipt, cause }),
        }
    }

    pub(super) const fn destination(&self) -> ValueId {
        self.receipt.final_destination()
    }

    #[cfg(test)]
    pub(super) const fn publication(&self) -> &PreparedTypeFactPublicationV1 {
        &self.publication
    }

    pub(super) fn discard(self) {
        self.receipt.discard();
    }
}

impl RejectedPreloopNestedIntegerPublicationV1 {
    pub(super) const fn destination(&self) -> ValueId {
        self.receipt.final_destination()
    }

    pub(super) const fn cause(&self) -> &TypeFactDecisionErrorV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        format!(
            "[preloop-nested/type-publication-rejected] destination={:?} cause={}",
            self.destination(),
            self.cause
        )
        .into_boxed_str()
    }

    pub(super) fn discard(self) {
        self.receipt.discard();
    }
}
