//! Stack-local reconstruction of one selected Stage-B function ingress.
//!
//! The owned activation row is split into a body recipe and the existing
//! nested-result rebind witness. All borrowed source products live only inside
//! the HRTB callback; no Builder context, retry, or lowering authority is held.

use std::sync::Arc;

use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::source_call_target::{
    RawSourceCursorErrorV1, SourceMethodCallSiteErrorV1, VerifiedRawCallableSourceViewV1,
    VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    rebind_nested_instance_result_contract_v1, NestedInstanceResultRebindErrorV1,
    NestedInstanceResultRebindStageV1, OwnedNestedInstanceResultRebindWitnessV1,
    PreloopLocatedArgumentErrorV1, PreloopLocatedArgumentStageV1,
    PreloopNestedResultAssociationErrorV1, PreloopNestedResultAssociationStageV1,
    PreparedPreloopLocatedArgumentV1, RetainedNestedInstanceResultRebindAuthorityV1,
};

use super::activation::{
    OwnedPreloopStageBCarrierRowV1, PreparedPreloopStageBFunctionBodyRecipeV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreloopStageBFunctionIngressStageV1 {
    SourceView,
    SelectedStatement,
    OuterCall,
    SelectedArgument,
    InnerCall,
    NestedRebind,
    SourceAssociation,
    LocatedArgument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreloopStageBFunctionIngressCauseV1 {
    SourceCursor(RawSourceCursorErrorV1),
    SelectedStatementSiteMismatch,
    OuterCallSiteMismatch,
    SelectedArgumentSiteMismatch,
    SourceMethodCall(SourceMethodCallSiteErrorV1),
    NestedRebind {
        stage: NestedInstanceResultRebindStageV1,
        cause: NestedInstanceResultRebindErrorV1,
    },
    SourceAssociation {
        stage: PreloopNestedResultAssociationStageV1,
        cause: PreloopNestedResultAssociationErrorV1,
    },
    LocatedArgument {
        stage: PreloopLocatedArgumentStageV1,
        cause: PreloopLocatedArgumentErrorV1,
    },
}

#[derive(Debug)]
pub(crate) struct PreparedPreloopStageBFunctionIngressV1 {
    catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    nested_result_rebind: OwnedNestedInstanceResultRebindWitnessV1,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
}

#[derive(Debug)]
pub(crate) struct RejectedPreloopStageBFunctionIngressV1 {
    owner: RetainedPreloopStageBFunctionIngressOwnerV1,
    stage: PreloopStageBFunctionIngressStageV1,
    cause: PreloopStageBFunctionIngressCauseV1,
}

#[derive(Debug)]
enum RetainedPreloopStageBFunctionIngressOwnerV1 {
    Prepared(PreparedPreloopStageBFunctionIngressV1),
    Consumed {
        catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
        nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
        recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    },
}

impl PreparedPreloopStageBFunctionIngressV1 {
    pub(super) fn new(
        catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
        row: OwnedPreloopStageBCarrierRowV1,
    ) -> Self {
        let parts = row.into_function_ingress_parts();
        Self {
            catalog,
            nested_result_rebind: parts.nested_result_rebind,
            recipe: parts.recipe,
        }
    }

    pub(crate) const fn recipe(&self) -> &PreparedPreloopStageBFunctionBodyRecipeV1 {
        &self.recipe
    }

    pub(crate) fn with_prepared_located_argument<R>(
        self,
        consume: impl for<'scope> FnOnce(
            PreparedPreloopLocatedArgumentV1<'scope, 'scope, 'scope>,
            PreparedPreloopStageBFunctionBodyRecipeV1,
        ) -> R,
    ) -> Result<R, RejectedPreloopStageBFunctionIngressV1> {
        let Self {
            catalog,
            nested_result_rebind,
            recipe,
        } = self;

        let view = match VerifiedRawCallableSourceViewV1::verify(&catalog, recipe.caller()) {
            Ok(view) => view,
            Err(cause) => {
                return Err(reject(
                    catalog,
                    nested_result_rebind,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::SourceView,
                    PreloopStageBFunctionIngressCauseV1::SourceCursor(cause),
                ))
            }
        };
        let body = view.root_body();
        let statement_index = recipe.body_handoff().prefix_statement_count() as usize;
        let statement = match view.body_stmt(&body, statement_index) {
            Ok(statement) => statement,
            Err(cause) => {
                return Err(reject(
                    catalog,
                    nested_result_rebind,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::SelectedStatement,
                    PreloopStageBFunctionIngressCauseV1::SourceCursor(cause),
                ))
            }
        };
        if statement.site() != recipe.body_handoff().selected_statement() {
            return Err(reject(
                catalog,
                nested_result_rebind,
                recipe,
                PreloopStageBFunctionIngressStageV1::SelectedStatement,
                PreloopStageBFunctionIngressCauseV1::SelectedStatementSiteMismatch,
            ));
        }
        let outer_expression =
            match view.child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue) {
                Ok(expression) => expression,
                Err(cause) => {
                    return Err(reject(
                        catalog,
                        nested_result_rebind,
                        recipe,
                        PreloopStageBFunctionIngressStageV1::OuterCall,
                        PreloopStageBFunctionIngressCauseV1::SourceCursor(cause),
                    ))
                }
            };
        if outer_expression.site() != recipe.outer_call_site() {
            return Err(reject(
                catalog,
                nested_result_rebind,
                recipe,
                PreloopStageBFunctionIngressStageV1::OuterCall,
                PreloopStageBFunctionIngressCauseV1::OuterCallSiteMismatch,
            ));
        }
        let outer = match view.method_call_input(&outer_expression) {
            Ok(outer) => outer,
            Err(cause) => {
                return Err(reject(
                    catalog,
                    nested_result_rebind,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::OuterCall,
                    PreloopStageBFunctionIngressCauseV1::SourceCursor(cause),
                ))
            }
        };
        let selected =
            match view.method_call_argument(outer, recipe.selected_argument_index() as usize) {
                Ok(selected) => selected,
                Err(rejected) => {
                    let cause = rejected.cause().clone();
                    rejected.discard();
                    return Err(reject(
                        catalog,
                        nested_result_rebind,
                        recipe,
                        PreloopStageBFunctionIngressStageV1::SelectedArgument,
                        PreloopStageBFunctionIngressCauseV1::SourceCursor(cause),
                    ));
                }
            };
        if selected.child().site() != recipe.inner_call_site() {
            return Err(reject(
                catalog,
                nested_result_rebind,
                recipe,
                PreloopStageBFunctionIngressStageV1::SelectedArgument,
                PreloopStageBFunctionIngressCauseV1::SelectedArgumentSiteMismatch,
            ));
        }
        let inner = match view.method_call_input(selected.child()) {
            Ok(inner) => inner,
            Err(cause) => {
                return Err(reject(
                    catalog,
                    nested_result_rebind,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::InnerCall,
                    PreloopStageBFunctionIngressCauseV1::SourceCursor(cause),
                ))
            }
        };
        let source_call = match VerifiedSourceMethodCallSiteV1::verify(
            &catalog,
            recipe.caller(),
            recipe.inner_call_site().clone(),
        ) {
            Ok(call) => call,
            Err(cause) => {
                return Err(reject(
                    catalog,
                    nested_result_rebind,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::InnerCall,
                    PreloopStageBFunctionIngressCauseV1::SourceMethodCall(cause),
                ))
            }
        };
        let contract = match rebind_nested_instance_result_contract_v1(
            nested_result_rebind,
            &catalog,
            &source_call,
        ) {
            Ok(contract) => contract,
            Err(rejected) => {
                let stage = rejected.stage();
                let cause = rejected.cause().clone();
                let nested_result = rejected.into_retained_authority();
                return Err(reject_consumed(
                    catalog,
                    nested_result,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::NestedRebind,
                    PreloopStageBFunctionIngressCauseV1::NestedRebind { stage, cause },
                ));
            }
        };
        let association = match prepare_preloop_nested_result_association_v1(contract, inner) {
            Ok(association) => association,
            Err(rejected) => {
                let stage = rejected.stage();
                let cause = rejected.cause();
                let nested_result = rejected.into_retained_rebind_authority();
                return Err(reject_consumed(
                    catalog,
                    nested_result,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::SourceAssociation,
                    PreloopStageBFunctionIngressCauseV1::SourceAssociation { stage, cause },
                ));
            }
        };
        let located = match prepare_preloop_located_argument_v1(selected, association) {
            Ok(located) => located,
            Err(rejected) => {
                let stage = rejected.stage();
                let cause = rejected.cause();
                let nested_result = rejected.into_retained_rebind_authority();
                return Err(reject_consumed(
                    catalog,
                    nested_result,
                    recipe,
                    PreloopStageBFunctionIngressStageV1::LocatedArgument,
                    PreloopStageBFunctionIngressCauseV1::LocatedArgument { stage, cause },
                ));
            }
        };

        Ok(consume(located, recipe))
    }
}

impl RejectedPreloopStageBFunctionIngressV1 {
    pub(crate) const fn stage(&self) -> PreloopStageBFunctionIngressStageV1 {
        self.stage
    }

    pub(crate) const fn cause(&self) -> &PreloopStageBFunctionIngressCauseV1 {
        &self.cause
    }

    pub(crate) fn bounded_report(&self) -> Box<str> {
        format!(
            "[mir/preloop-stageb/function-ingress/{:?}] {:?}",
            self.stage, self.cause
        )
        .into_boxed_str()
    }

    pub(crate) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBFunctionIngressOwnerV1::Prepared(owner) => {
                let _ = owner;
            }
            RetainedPreloopStageBFunctionIngressOwnerV1::Consumed {
                catalog,
                nested_result,
                recipe,
            } => {
                let _ = (catalog, recipe);
                nested_result.discard();
            }
        }
    }
}

fn reject(
    catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    nested_result_rebind: OwnedNestedInstanceResultRebindWitnessV1,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    stage: PreloopStageBFunctionIngressStageV1,
    cause: PreloopStageBFunctionIngressCauseV1,
) -> RejectedPreloopStageBFunctionIngressV1 {
    RejectedPreloopStageBFunctionIngressV1 {
        owner: RetainedPreloopStageBFunctionIngressOwnerV1::Prepared(
            PreparedPreloopStageBFunctionIngressV1 {
                catalog,
                nested_result_rebind,
                recipe,
            },
        ),
        stage,
        cause,
    }
}

fn reject_consumed(
    catalog: Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    nested_result: RetainedNestedInstanceResultRebindAuthorityV1,
    recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    stage: PreloopStageBFunctionIngressStageV1,
    cause: PreloopStageBFunctionIngressCauseV1,
) -> RejectedPreloopStageBFunctionIngressV1 {
    RejectedPreloopStageBFunctionIngressV1 {
        owner: RetainedPreloopStageBFunctionIngressOwnerV1::Consumed {
            catalog,
            nested_result,
            recipe,
        },
        stage,
        cause,
    }
}
