//! Integer fact publication for one emitted pre-loop result.
//!
//! Preparation delegates the complete fact policy to `TypeFactDecisionV1`.
//! The sole terminal reads the destination fact and immediately consumes the
//! prepared owner. Only `Publish` reaches `TypeContext::set_type`; this module
//! owns no Builder, Call emission, source lookup, or direct fact-map access.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::type_context::TypeContext;
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

    fn commit(self, type_ctx: &mut TypeContext) {
        let Self {
            receipt,
            publication,
        } = self;
        let destination = receipt.final_destination();

        if let PreparedTypeFactPublicationV1::Publish(ty) = publication {
            type_ctx.set_type(destination, ty);
        }

        receipt.discard();
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

/// Publishes one exact Integer fact from a successfully emitted nested call.
///
/// The read, decision, and consuming commit stay in one terminal so callers
/// cannot retain a prepared write across another fact-store mutation.
pub(super) fn publish_preloop_nested_integer_result_v1(
    receipt: EmittedNestedInstanceCallV1,
    type_ctx: &mut TypeContext,
) -> Result<(), RejectedPreloopNestedIntegerPublicationV1> {
    let destination = receipt.final_destination();
    let prepared = PreparedPreloopNestedIntegerPublicationV1::prepare(
        receipt,
        type_ctx.get_type(destination),
    )?;
    prepared.commit(type_ctx);
    Ok(())
}
