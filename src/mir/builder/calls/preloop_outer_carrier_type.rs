//! Success-only Integer publication for the exact pre-loop outer carrier.
//!
//! The complete assignment carrier is the sole destination authority. The
//! existing monotone type-fact decision owns conflict policy; this module only
//! retains that owner across preparation and commits `Publish` through
//! `TypeContext::set_type`.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::calls::unified_emitter::CompletedUnifiedValueCallEmissionV1;
use crate::mir::builder::stmts::CompletedVariableAssignmentV1;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::preloop_stageb_carrier::PreparedPreloopStageBFunctionBodyRecipeV1;
use crate::mir::source_instance_result_contract::RetainedNestedInstanceResultRebindAuthorityV1;
use crate::mir::{MirType, ValueId};

use super::preloop_outer_carrier_assignment::{
    CompletedPreloopCarrierAssignmentV1, OwnedPreloopCarrierAssignmentPartsV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopOuterCarrierIntegerPublicationDispositionV1 {
    Published,
    Idempotent,
}

#[derive(Debug)]
pub(super) struct PreparedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog> {
    carrier: CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
    publication: PreparedTypeFactPublicationV1,
}

#[derive(Debug)]
pub(super) struct CompletedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog> {
    carrier: CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
    disposition: PreloopOuterCarrierIntegerPublicationDispositionV1,
    _seal: CompletedPreloopOuterCarrierIntegerPublicationSealV1,
}

#[derive(Debug)]
struct CompletedPreloopOuterCarrierIntegerPublicationSealV1;

/// Lifetime-free terminal for the complete selected Stage-B carrier chain.
///
/// This owner stores the actual sealed receipts. The nested-result authority
/// is inspection/discard-only, so the completed payload cannot rebind or retry
/// the consumed source association.
#[derive(Debug)]
pub(super) struct CompletedPreloopStageBCarrierV1 {
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
    inner_call: CompletedUnifiedValueCallEmissionV1,
    outer_call: CompletedUnifiedValueCallEmissionV1,
    assignment: CompletedVariableAssignmentV1,
    publication: PreloopOuterCarrierIntegerPublicationDispositionV1,
    _seal: CompletedPreloopStageBCarrierSealV1,
}

#[derive(Debug)]
struct CompletedPreloopStageBCarrierSealV1;

#[derive(Debug)]
pub(super) struct RejectedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog> {
    carrier: CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
    cause: TypeFactDecisionErrorV1,
}

#[derive(Debug)]
pub(super) struct OwnedRejectedPreloopOuterCarrierIntegerPublicationV1 {
    carrier: OwnedPreloopCarrierAssignmentPartsV1,
    cause: TypeFactDecisionErrorV1,
}

impl<'site, 'view, 'catalog>
    PreparedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog>
{
    pub(super) fn prepare(
        carrier: CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
        existing_destination: Option<&MirType>,
    ) -> Result<Self, RejectedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog>> {
        match TypeFactDecisionV1::prepare(existing_destination, Some(&MirType::Integer)) {
            Ok(publication) => Ok(Self {
                carrier,
                publication,
            }),
            Err(cause) => Err(RejectedPreloopOuterCarrierIntegerPublicationV1 { carrier, cause }),
        }
    }

    fn commit(
        self,
        type_ctx: &mut TypeContext,
    ) -> CompletedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog> {
        let Self {
            carrier,
            publication,
        } = self;
        let destination = carrier.outer_destination();
        let disposition = match publication {
            PreparedTypeFactPublicationV1::Publish(ty) => {
                type_ctx.set_type(destination, ty);
                PreloopOuterCarrierIntegerPublicationDispositionV1::Published
            }
            PreparedTypeFactPublicationV1::Idempotent(_) => {
                PreloopOuterCarrierIntegerPublicationDispositionV1::Idempotent
            }
            PreparedTypeFactPublicationV1::PreserveExisting(_)
            | PreparedTypeFactPublicationV1::NoPublication => {
                unreachable!("exact Integer proposal must publish or be idempotent")
            }
        };

        CompletedPreloopOuterCarrierIntegerPublicationV1 {
            carrier,
            disposition,
            _seal: CompletedPreloopOuterCarrierIntegerPublicationSealV1,
        }
    }
}

impl CompletedPreloopOuterCarrierIntegerPublicationV1<'_, '_, '_> {
    pub(super) const fn destination(&self) -> ValueId {
        self.carrier.outer_destination()
    }

    pub(super) const fn disposition(&self) -> PreloopOuterCarrierIntegerPublicationDispositionV1 {
        self.disposition
    }

    pub(super) fn discard(self) {
        self.carrier.discard();
    }

    pub(super) fn into_stageb_carrier_v1(self) -> CompletedPreloopStageBCarrierV1 {
        let Self {
            carrier,
            disposition,
            ..
        } = self;
        let OwnedPreloopCarrierAssignmentPartsV1 {
            carrier,
            assignment,
        } = carrier.into_owned_parts_v1();
        let physical = carrier.physical;
        CompletedPreloopStageBCarrierV1 {
            recipe: carrier.recipe,
            nested_result: physical.nested_result,
            inner_call: physical.inner_call,
            outer_call: physical.outer_call,
            assignment,
            publication: disposition,
            _seal: CompletedPreloopStageBCarrierSealV1,
        }
    }
}

impl RejectedPreloopOuterCarrierIntegerPublicationV1<'_, '_, '_> {
    pub(super) const fn destination(&self) -> ValueId {
        self.carrier.outer_destination()
    }

    pub(super) const fn cause(&self) -> &TypeFactDecisionErrorV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        format!(
            "[preloop-outer-carrier/type-publication-rejected] destination={:?} cause={}",
            self.destination(),
            self.cause
        )
        .into_boxed_str()
    }

    pub(super) fn discard(self) {
        self.carrier.discard();
    }

    pub(super) fn into_owned_rejection_v1(
        self,
    ) -> OwnedRejectedPreloopOuterCarrierIntegerPublicationV1 {
        OwnedRejectedPreloopOuterCarrierIntegerPublicationV1 {
            carrier: self.carrier.into_owned_parts_v1(),
            cause: self.cause,
        }
    }
}

impl CompletedPreloopStageBCarrierV1 {
    pub(super) const fn recipe(&self) -> &PreparedPreloopStageBFunctionBodyRecipeV1 {
        &self.recipe
    }

    pub(super) const fn inner_destination(&self) -> ValueId {
        self.inner_call.final_destination()
    }

    pub(super) const fn outer_destination(&self) -> ValueId {
        self.outer_call.final_destination()
    }

    pub(super) const fn assigned_destination(&self) -> ValueId {
        self.assignment.assigned()
    }

    pub(super) fn assignment_target(&self) -> &str {
        self.assignment.target()
    }

    pub(super) const fn publication(&self) -> PreloopOuterCarrierIntegerPublicationDispositionV1 {
        self.publication
    }

    pub(super) fn discard(self) {
        self.nested_result.discard();
        let _ = (
            self.recipe,
            self.inner_call,
            self.outer_call,
            self.assignment,
            self.publication,
        );
    }
}

impl OwnedRejectedPreloopOuterCarrierIntegerPublicationV1 {
    pub(super) const fn destination(&self) -> ValueId {
        self.carrier.assignment.assigned()
    }

    pub(super) const fn cause(&self) -> &TypeFactDecisionErrorV1 {
        &self.cause
    }

    pub(super) fn discard(self) {
        self.carrier.discard();
    }
}

pub(super) fn publish_preloop_outer_carrier_integer_v1<'site, 'view, 'catalog>(
    carrier: CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
    type_ctx: &mut TypeContext,
) -> Result<
    CompletedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog>,
    RejectedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog>,
> {
    let destination = carrier.outer_destination();
    let prepared = PreparedPreloopOuterCarrierIntegerPublicationV1::prepare(
        carrier,
        type_ctx.get_type(destination),
    )?;
    Ok(prepared.commit(type_ctx))
}
