//! Success-only Integer publication for the exact pre-loop outer carrier.
//!
//! The complete assignment carrier is the sole destination authority. The
//! existing monotone type-fact decision owns conflict policy; this module only
//! retains that owner across preparation and commits `Publish` through
//! `TypeContext::set_type`.

use hakorune_mir_builder::lowering_facts::{
    PreparedTypeFactPublicationV1, TypeFactDecisionErrorV1, TypeFactDecisionV1,
};

use crate::mir::builder::type_context::TypeContext;
use crate::mir::{MirType, ValueId};

use super::preloop_outer_carrier_assignment::CompletedPreloopCarrierAssignmentV1;

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

#[derive(Debug)]
pub(super) struct RejectedPreloopOuterCarrierIntegerPublicationV1<'site, 'view, 'catalog> {
    carrier: CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
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
