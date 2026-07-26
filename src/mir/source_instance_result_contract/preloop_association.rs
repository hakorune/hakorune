//! Source-only pairing for the selected pre-loop nested instance result.
//!
//! This box deliberately stops before Builder descent. It proves that the
//! sealed Integer contract and the catalog-backed located MethodCall are the
//! same source occurrence; a later physical-call row consumes this owner.

use std::ptr;

use crate::mir::source_call_target::RawLocatedMethodCallInputV1;

use super::SealedNestedInstanceResultContractV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopNestedResultAssociationStageV1 {
    CatalogAllocation,
    Declaration,
    Caller,
    Site,
    Syntax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopNestedResultAssociationErrorV1 {
    ForeignCatalog,
    DeclarationMismatch,
    CallerMismatch,
    SiteMismatch,
    SyntaxMismatch,
}

/// A one-shot source association with no lowering or result-publication
/// authority. Its private fields prevent construction from equal-looking
/// catalog data that was allocated separately.
#[derive(Debug)]
pub(crate) struct PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog> {
    contract: SealedNestedInstanceResultContractV1<'site, 'catalog>,
    input: RawLocatedMethodCallInputV1<'view, 'catalog>,
}

impl<'site, 'view, 'catalog> PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog> {
    pub(crate) const fn contract(&self) -> &SealedNestedInstanceResultContractV1<'site, 'catalog> {
        &self.contract
    }

    pub(crate) const fn input(&self) -> &RawLocatedMethodCallInputV1<'view, 'catalog> {
        &self.input
    }

    pub(crate) fn discard(self) {}
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopNestedResultAssociationV1<'site, 'view, 'catalog> {
    contract: SealedNestedInstanceResultContractV1<'site, 'catalog>,
    input: RawLocatedMethodCallInputV1<'view, 'catalog>,
    stage: PreloopNestedResultAssociationStageV1,
    cause: PreloopNestedResultAssociationErrorV1,
}

impl<'site, 'view, 'catalog> RejectedPreloopNestedResultAssociationV1<'site, 'view, 'catalog> {
    pub(crate) const fn stage(&self) -> PreloopNestedResultAssociationStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> PreloopNestedResultAssociationErrorV1 {
        self.cause
    }

    pub(crate) fn discard(self) {
        let Self {
            contract, input, ..
        } = self;
        let _ = (contract, input);
    }
}

pub(crate) fn prepare_preloop_nested_result_association_v1<'site, 'view, 'catalog>(
    contract: SealedNestedInstanceResultContractV1<'site, 'catalog>,
    input: RawLocatedMethodCallInputV1<'view, 'catalog>,
) -> Result<
    PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
    RejectedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
> {
    let call = contract.target().call();
    let rejection = if !ptr::eq(call.catalog(), input.view().catalog()) {
        Some((
            PreloopNestedResultAssociationStageV1::CatalogAllocation,
            PreloopNestedResultAssociationErrorV1::ForeignCatalog,
        ))
    } else if !ptr::eq(call.declaration(), input.view().declaration()) {
        Some((
            PreloopNestedResultAssociationStageV1::Declaration,
            PreloopNestedResultAssociationErrorV1::DeclarationMismatch,
        ))
    } else if call.caller() != input.caller() {
        Some((
            PreloopNestedResultAssociationStageV1::Caller,
            PreloopNestedResultAssociationErrorV1::CallerMismatch,
        ))
    } else if call.site() != input.site() {
        Some((
            PreloopNestedResultAssociationStageV1::Site,
            PreloopNestedResultAssociationErrorV1::SiteMismatch,
        ))
    } else if !ptr::eq(call.expression(), input.node()) {
        Some((
            PreloopNestedResultAssociationStageV1::Syntax,
            PreloopNestedResultAssociationErrorV1::SyntaxMismatch,
        ))
    } else {
        None
    };

    if let Some((stage, cause)) = rejection {
        return Err(RejectedPreloopNestedResultAssociationV1 {
            contract,
            input,
            stage,
            cause,
        });
    }
    Ok(PreparedPreloopNestedResultAssociationV1 { contract, input })
}
