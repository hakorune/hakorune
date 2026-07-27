use std::ptr;

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::VerifiedStaticExactI64RequirementV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;

use super::{
    PreloopOuterCarrierResultContractErrorV1, PreloopOuterCarrierResultContractStageV1,
    RejectedPreloopOuterCarrierResultContractV1,
};

const SELECTED_PRELOOP_ARGUMENT_INDEX: u32 = 1;

/// One bounded Integer result contract for the selected outer carrier Call.
///
/// The private seal records that the exact static target requires precisely
/// the structural nested Integer argument selected by the same catalog.
#[derive(Debug)]
pub(crate) struct SealedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog> {
    requirement: VerifiedStaticExactI64RequirementV1<'result, 'catalog>,
    prepared: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    _seal: SealedPreloopOuterCarrierResultContractSealV1,
}

#[derive(Debug)]
struct SealedPreloopOuterCarrierResultContractSealV1(());

impl SealedPreloopOuterCarrierResultContractSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

impl<'result, 'site, 'view, 'catalog>
    SealedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>
{
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.requirement.caller()
    }

    pub(crate) const fn outer_site(&self) -> &SourceExprSiteV1 {
        self.requirement.site()
    }

    pub(crate) const fn selected_argument_index(&self) -> u32 {
        self.prepared.selected().index()
    }

    pub(crate) const fn inner_site(&self) -> &SourceExprSiteV1 {
        self.prepared.selected().child().site()
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.requirement.target()
    }

    pub(crate) const fn result_is_integer(&self) -> bool {
        true
    }

    pub(crate) fn is_branded_by(
        &self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        self.requirement.is_branded_by(declarations)
    }

    pub(super) const fn prepared_source(
        &self,
    ) -> &PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
        &self.prepared
    }

    pub(crate) fn discard(self) {
        let Self {
            requirement,
            prepared,
            ..
        } = self;
        let _ = (requirement, prepared);
    }
}

pub(crate) fn seal_preloop_outer_carrier_result_v1<'result, 'site, 'view, 'catalog>(
    requirement: VerifiedStaticExactI64RequirementV1<'result, 'catalog>,
    prepared: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
) -> Result<
    SealedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>,
    RejectedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>,
> {
    let catalog_matches = requirement.is_branded_by(prepared.selected().parent().view().catalog());
    let caller_matches = ptr::eq(requirement.caller(), prepared.selected().parent().caller());
    let outer_site_matches = requirement.site() == prepared.selected().parent().site();
    let selected_index = prepared.selected().index();
    let required_arguments_match = requirement.required_i64_arguments() == [selected_index];
    let actual_required_arguments: Box<[u32]> = requirement.required_i64_arguments().into();
    let inner_catalog_matches =
        requirement.is_branded_by(prepared.association().contract().target().call().catalog());

    if !catalog_matches {
        return Err(reject(
            requirement,
            prepared,
            PreloopOuterCarrierResultContractStageV1::CatalogAllocation,
            PreloopOuterCarrierResultContractErrorV1::ForeignCatalog,
        ));
    }
    if !caller_matches {
        return Err(reject(
            requirement,
            prepared,
            PreloopOuterCarrierResultContractStageV1::Caller,
            PreloopOuterCarrierResultContractErrorV1::CallerMismatch,
        ));
    }
    if !outer_site_matches {
        return Err(reject(
            requirement,
            prepared,
            PreloopOuterCarrierResultContractStageV1::OuterSite,
            PreloopOuterCarrierResultContractErrorV1::OuterSiteMismatch,
        ));
    }
    if selected_index != SELECTED_PRELOOP_ARGUMENT_INDEX {
        return Err(reject(
            requirement,
            prepared,
            PreloopOuterCarrierResultContractStageV1::SelectedArgument,
            PreloopOuterCarrierResultContractErrorV1::SelectedArgumentMismatch {
                expected: SELECTED_PRELOOP_ARGUMENT_INDEX,
                actual: selected_index,
            },
        ));
    }
    if !required_arguments_match {
        return Err(reject(
            requirement,
            prepared,
            PreloopOuterCarrierResultContractStageV1::RequiredArguments,
            PreloopOuterCarrierResultContractErrorV1::RequiredArgumentsMismatch {
                selected: selected_index,
                actual: actual_required_arguments,
            },
        ));
    }
    if !inner_catalog_matches {
        return Err(reject(
            requirement,
            prepared,
            PreloopOuterCarrierResultContractStageV1::InnerContract,
            PreloopOuterCarrierResultContractErrorV1::InnerContractCatalogMismatch,
        ));
    }

    Ok(SealedPreloopOuterCarrierResultContractV1 {
        requirement,
        prepared,
        _seal: SealedPreloopOuterCarrierResultContractSealV1::new(),
    })
}

fn reject<'result, 'site, 'view, 'catalog>(
    requirement: VerifiedStaticExactI64RequirementV1<'result, 'catalog>,
    prepared: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    stage: PreloopOuterCarrierResultContractStageV1,
    cause: PreloopOuterCarrierResultContractErrorV1,
) -> RejectedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog> {
    RejectedPreloopOuterCarrierResultContractV1::new(requirement, prepared, stage, cause)
}
