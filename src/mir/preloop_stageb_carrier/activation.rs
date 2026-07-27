//! Owned one-row activation for the bounded pre-loop Stage-B carrier.
//!
//! Borrowed source proofs are normalized before the declaration catalog moves
//! into the plan. The resulting row carries no AST borrow or Builder authority.

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{
    SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
};
use crate::mir::source_instance_result_contract::OwnedNestedInstanceResultRebindWitnessV1;
use std::sync::Arc;

use super::outer_result::SealedPreloopOuterCarrierResultContractV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreloopOuterCarrierActivationShapeErrorV1 {
    OuterCallMustBeRootAssignmentValue,
    SelectedStatementIndexOverflow,
    SelectedStatementUnavailable,
    SelectedStatementMustBeAssignment,
    OuterCallSyntaxMismatch,
    BodyStatementCountOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopStageBCarrierActivationStageV1 {
    BodyHandoff,
    CatalogAllocation,
    Caller,
    Target,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreloopStageBCarrierActivationErrorV1 {
    BodyHandoff(PreloopOuterCarrierActivationShapeErrorV1),
    CatalogAllocationMismatch,
    CallerOutsideCatalog,
    TargetOutsideCatalog,
}

/// Owned prefix/selected/suffix schedule for one root assignment.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PreloopStageBBodyHandoffV1 {
    prefix_statement_count: u32,
    selected_statement: SourceStmtSiteV1,
    suffix_statement_start: u32,
    body_statement_count: u32,
}

impl PreloopStageBBodyHandoffV1 {
    pub(crate) const fn prefix_statement_count(&self) -> u32 {
        self.prefix_statement_count
    }

    pub(crate) const fn selected_statement(&self) -> &SourceStmtSiteV1 {
        &self.selected_statement
    }

    pub(crate) const fn suffix_statement_start(&self) -> u32 {
        self.suffix_statement_start
    }

    pub(crate) const fn body_statement_count(&self) -> u32 {
        self.body_statement_count
    }
}

/// Owned exact-Integer disposition derived from the sealed borrowed contract.
#[derive(Debug)]
pub(crate) struct SealedPreloopOuterCarrierOwnedResultV1 {
    _seal: SealedPreloopOuterCarrierOwnedResultSealV1,
}

#[derive(Debug)]
struct SealedPreloopOuterCarrierOwnedResultSealV1(());

impl SealedPreloopOuterCarrierOwnedResultV1 {
    const fn new() -> Self {
        Self {
            _seal: SealedPreloopOuterCarrierOwnedResultSealV1(()),
        }
    }

    pub(crate) const fn is_integer(&self) -> bool {
        true
    }
}

/// One owned activation row. Its existence proves the complete bounded source
/// relation; it does not select or lower a production function.
#[derive(Debug)]
pub(crate) struct OwnedPreloopStageBCarrierRowV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    body_handoff: PreloopStageBBodyHandoffV1,
    outer_call_site: SourceExprSiteV1,
    selected_argument_index: u32,
    inner_call_site: SourceExprSiteV1,
    nested_result_rebind: OwnedNestedInstanceResultRebindWitnessV1,
    outer_target: CanonicalSameModuleCallableKeyV1,
    result: SealedPreloopOuterCarrierOwnedResultV1,
}

/// Owned source schedule retained while the nested-result witness is rebound
/// into stack-local source products.
#[derive(Debug)]
pub(crate) struct PreparedPreloopStageBFunctionBodyRecipeV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    body_handoff: PreloopStageBBodyHandoffV1,
    outer_call_site: SourceExprSiteV1,
    selected_argument_index: u32,
    inner_call_site: SourceExprSiteV1,
    outer_target: CanonicalSameModuleCallableKeyV1,
    result: SealedPreloopOuterCarrierOwnedResultV1,
}

#[derive(Debug)]
pub(super) struct PreparedPreloopStageBFunctionIngressSourcePartsV1 {
    pub(super) nested_result_rebind: OwnedNestedInstanceResultRebindWitnessV1,
    pub(super) recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
}

impl OwnedPreloopStageBCarrierRowV1 {
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn body_handoff(&self) -> &PreloopStageBBodyHandoffV1 {
        &self.body_handoff
    }

    pub(crate) const fn outer_call_site(&self) -> &SourceExprSiteV1 {
        &self.outer_call_site
    }

    pub(crate) const fn selected_argument_index(&self) -> u32 {
        self.selected_argument_index
    }

    pub(crate) const fn inner_call_site(&self) -> &SourceExprSiteV1 {
        &self.inner_call_site
    }

    pub(crate) const fn nested_result_rebind(&self) -> &OwnedNestedInstanceResultRebindWitnessV1 {
        &self.nested_result_rebind
    }

    pub(crate) const fn outer_target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.outer_target
    }

    pub(crate) const fn result(&self) -> &SealedPreloopOuterCarrierOwnedResultV1 {
        &self.result
    }

    pub(super) fn into_function_ingress_parts(
        self,
    ) -> PreparedPreloopStageBFunctionIngressSourcePartsV1 {
        PreparedPreloopStageBFunctionIngressSourcePartsV1 {
            nested_result_rebind: self.nested_result_rebind,
            recipe: PreparedPreloopStageBFunctionBodyRecipeV1 {
                caller: self.caller,
                body_handoff: self.body_handoff,
                outer_call_site: self.outer_call_site,
                selected_argument_index: self.selected_argument_index,
                inner_call_site: self.inner_call_site,
                outer_target: self.outer_target,
                result: self.result,
            },
        }
    }
}

impl PreparedPreloopStageBFunctionBodyRecipeV1 {
    pub(crate) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn body_handoff(&self) -> &PreloopStageBBodyHandoffV1 {
        &self.body_handoff
    }

    pub(crate) const fn outer_call_site(&self) -> &SourceExprSiteV1 {
        &self.outer_call_site
    }

    pub(crate) const fn selected_argument_index(&self) -> u32 {
        self.selected_argument_index
    }

    pub(crate) const fn inner_call_site(&self) -> &SourceExprSiteV1 {
        &self.inner_call_site
    }

    pub(crate) const fn outer_target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.outer_target
    }

    pub(crate) const fn result(&self) -> &SealedPreloopOuterCarrierOwnedResultV1 {
        &self.result
    }
}

/// Prepared owned normalization. The pointer identity is construction-only and
/// is consumed when the exact shared catalog allocation seals the final plan.
#[derive(Debug)]
pub(crate) struct PreparedPreloopStageBCarrierRowsV1 {
    catalog_identity: usize,
    row: OwnedPreloopStageBCarrierRowV1,
}

impl PreparedPreloopStageBCarrierRowsV1 {
    pub(super) const fn catalog_identity(&self) -> usize {
        self.catalog_identity
    }

    pub(super) const fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.row.caller()
    }

    pub(super) const fn outer_call_site(&self) -> &SourceExprSiteV1 {
        self.row.outer_call_site()
    }

    pub(super) const fn selected_argument_index(&self) -> u32 {
        self.row.selected_argument_index()
    }

    pub(super) const fn inner_call_site(&self) -> &SourceExprSiteV1 {
        self.row.inner_call_site()
    }

    pub(super) const fn outer_target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.row.outer_target()
    }
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopStageBCarrierRowsV1<'result, 'site, 'view, 'catalog> {
    contract: SealedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>,
    stage: PreloopStageBCarrierActivationStageV1,
    cause: PreloopStageBCarrierActivationErrorV1,
}

impl RejectedPreloopStageBCarrierRowsV1<'_, '_, '_, '_> {
    pub(crate) const fn stage(&self) -> PreloopStageBCarrierActivationStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &PreloopStageBCarrierActivationErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        let Self { contract, .. } = self;
        contract.discard();
    }
}

pub(crate) fn prepare_preloop_stageb_carrier_rows_v1<'result, 'site, 'view, 'catalog>(
    contract: SealedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>,
) -> Result<
    PreparedPreloopStageBCarrierRowsV1,
    RejectedPreloopStageBCarrierRowsV1<'result, 'site, 'view, 'catalog>,
> {
    let outer_call_site = contract.outer_site().clone();
    let [SourcePathSegmentV1::Body(selected_statement_index), SourcePathSegmentV1::Value] =
        outer_call_site.node().segments()
    else {
        return Err(reject_rows(
            contract,
            PreloopOuterCarrierActivationShapeErrorV1::OuterCallMustBeRootAssignmentValue,
        ));
    };
    let selected_statement_index = *selected_statement_index;
    let selected_statement_index_usize = match usize::try_from(selected_statement_index) {
        Ok(index) => index,
        Err(_) => {
            return Err(reject_rows(
                contract,
                PreloopOuterCarrierActivationShapeErrorV1::SelectedStatementIndexOverflow,
            ))
        }
    };
    let prepared = contract.prepared_source();
    let view = prepared.selected().parent().view();
    let Some(statement) = view
        .declaration()
        .body()
        .get(selected_statement_index_usize)
    else {
        return Err(reject_rows(
            contract,
            PreloopOuterCarrierActivationShapeErrorV1::SelectedStatementUnavailable,
        ));
    };
    let ASTNode::Assignment { value, .. } = statement else {
        return Err(reject_rows(
            contract,
            PreloopOuterCarrierActivationShapeErrorV1::SelectedStatementMustBeAssignment,
        ));
    };
    if !std::ptr::eq(value.as_ref(), prepared.selected().parent().node()) {
        return Err(reject_rows(
            contract,
            PreloopOuterCarrierActivationShapeErrorV1::OuterCallSyntaxMismatch,
        ));
    }
    let body_statement_count = match u32::try_from(view.declaration().body().len()) {
        Ok(count) => count,
        Err(_) => {
            return Err(reject_rows(
                contract,
                PreloopOuterCarrierActivationShapeErrorV1::BodyStatementCountOverflow,
            ))
        }
    };
    let Some(suffix_statement_start) = selected_statement_index.checked_add(1) else {
        return Err(reject_rows(
            contract,
            PreloopOuterCarrierActivationShapeErrorV1::SelectedStatementIndexOverflow,
        ));
    };
    let catalog_identity = view.catalog() as *const _ as usize;
    let caller = contract.caller().clone();
    let selected_statement = SourcePathV1::root_body(selected_statement_index_usize).stmt();
    let selected_argument_index = contract.selected_argument_index();
    let inner_call_site = contract.inner_site().clone();
    let outer_target = contract.target().clone();

    let nested_result_rebind = contract.into_owned_nested_result_rebind_witness();
    Ok(PreparedPreloopStageBCarrierRowsV1 {
        catalog_identity,
        row: OwnedPreloopStageBCarrierRowV1 {
            caller,
            body_handoff: PreloopStageBBodyHandoffV1 {
                prefix_statement_count: selected_statement_index,
                selected_statement,
                suffix_statement_start,
                body_statement_count,
            },
            outer_call_site,
            selected_argument_index,
            inner_call_site,
            nested_result_rebind,
            outer_target,
            result: SealedPreloopOuterCarrierOwnedResultV1::new(),
        },
    })
}

fn reject_rows<'result, 'site, 'view, 'catalog>(
    contract: SealedPreloopOuterCarrierResultContractV1<'result, 'site, 'view, 'catalog>,
    cause: PreloopOuterCarrierActivationShapeErrorV1,
) -> RejectedPreloopStageBCarrierRowsV1<'result, 'site, 'view, 'catalog> {
    RejectedPreloopStageBCarrierRowsV1 {
        contract,
        stage: PreloopStageBCarrierActivationStageV1::BodyHandoff,
        cause: PreloopStageBCarrierActivationErrorV1::BodyHandoff(cause),
    }
}

/// Owned, non-Clone, single-use activation plan.
#[derive(Debug)]
pub(crate) struct VerifiedPreloopStageBCarrierActivationPlanV1 {
    declaration_catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    row: OwnedPreloopStageBCarrierRowV1,
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopStageBCarrierActivationPlanV1 {
    declaration_catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    rows: PreparedPreloopStageBCarrierRowsV1,
    stage: PreloopStageBCarrierActivationStageV1,
    cause: PreloopStageBCarrierActivationErrorV1,
}

impl VerifiedPreloopStageBCarrierActivationPlanV1 {
    pub(crate) fn seal(
        declaration_catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
        rows: PreparedPreloopStageBCarrierRowsV1,
    ) -> Result<Self, RejectedPreloopStageBCarrierActivationPlanV1> {
        let stage_and_cause = if rows.catalog_identity != Arc::as_ptr(&declaration_catalog) as usize
        {
            Some((
                PreloopStageBCarrierActivationStageV1::CatalogAllocation,
                PreloopStageBCarrierActivationErrorV1::CatalogAllocationMismatch,
            ))
        } else if declaration_catalog.declaration(rows.row.caller()).is_none() {
            Some((
                PreloopStageBCarrierActivationStageV1::Caller,
                PreloopStageBCarrierActivationErrorV1::CallerOutsideCatalog,
            ))
        } else if declaration_catalog
            .declaration(rows.row.outer_target())
            .is_none()
        {
            Some((
                PreloopStageBCarrierActivationStageV1::Target,
                PreloopStageBCarrierActivationErrorV1::TargetOutsideCatalog,
            ))
        } else {
            None
        };
        if let Some((stage, cause)) = stage_and_cause {
            return Err(RejectedPreloopStageBCarrierActivationPlanV1 {
                declaration_catalog,
                rows,
                stage,
                cause,
            });
        }

        Ok(Self {
            declaration_catalog,
            row: rows.row,
        })
    }

    pub(crate) const fn row(&self) -> &OwnedPreloopStageBCarrierRowV1 {
        &self.row
    }

    pub(in crate::mir) fn into_module_install_parts_v1(
        self,
    ) -> super::module_install::PreparedPreloopStageBActivationInstallPartsV1 {
        super::module_install::PreparedPreloopStageBActivationInstallPartsV1::new(
            self.declaration_catalog,
            self.row,
        )
    }
}

impl RejectedPreloopStageBCarrierActivationPlanV1 {
    pub(crate) const fn stage(&self) -> PreloopStageBCarrierActivationStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &PreloopStageBCarrierActivationErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        let Self {
            declaration_catalog,
            rows,
            ..
        } = self;
        let _ = (declaration_catalog, rows);
    }
}
