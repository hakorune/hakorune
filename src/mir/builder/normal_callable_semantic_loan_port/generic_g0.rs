//! Normal-package consumer for the source-bound Generic G0 function arm.

use crate::mir::compiler::capability::CanonicalGenericG0PlanV1;
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_route_policy::GenericG0PolicyModeV1;
use crate::mir::MirBuilder;

use super::canonical_route::{
    try_classify_generic_g0_route_v1, CanonicalCallableRouteV1,
};
use super::super::callable_declaration_catalog::SelectedNormalCallableKeyV1;
use super::super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;
use super::NormalCallableSemanticPackagePortAdapterV1;

#[allow(clippy::too_many_arguments)]
pub(super) fn lower_normal_top_level_function(
    adapter: &mut NormalCallableSemanticPackagePortAdapterV1<'_, '_, '_, '_, '_>,
    builder: &mut MirBuilder,
    admission: NormalTopLevelFunctionDraftAdmissionV1,
    params: Vec<String>,
    param_decls: Vec<ParamDecl>,
    return_type_name: Option<String>,
    body: Vec<ASTNode>,
    uses: Vec<String>,
    attrs: DeclarationAttrs,
) -> Result<(), String> {
    let key = SelectedNormalCallableKeyV1::TopLevel(admission.source_key().clone());
    let lineage = super::super::raw_invocation_source_transport::RawInvocationRootLineageV1::
        TopLevel(admission.source_key().clone());
    let inner = &mut *adapter.inner;
    let ordinary_new_claim_ledger = adapter.package.ordinary_new_claim_ledger();
    let mode = builder.comp_ctx.emit_debug_policy().generic_g0_policy_mode_v1();
    adapter
        .package
        .with_selected_lowering_input(&key, |selected| {
            match try_select_top_level_generic_g0_plan_v1(selected.source(), mode)? {
                Some(plan) => inner
                    .lower_normal_top_level_function_with_canonical_generic_g0_plan_v1(
                        builder, admission, plan,
                    )
                    .map_err(|error| error.to_string()),
                None => super::with_selected_source_scope(
                    inner,
                    lineage,
                    selected,
                    std::rc::Rc::clone(&ordinary_new_claim_ledger),
                    |inner, transport| {
                        inner
                            .lower_normal_top_level_function_with_source_v1(
                                builder,
                                admission,
                                params,
                                param_decls,
                                return_type_name,
                                body,
                                uses,
                                attrs,
                                transport,
                            )
                            .map_err(|error| error.to_string())
                    },
                ),
            }
        })
        .map_err(super::package_issue)?
}

pub(super) fn try_select_top_level_generic_g0_plan_v1<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    mode: Option<GenericG0PolicyModeV1>,
) -> Result<Option<CanonicalGenericG0PlanV1<'source>>, String> {
    match try_classify_generic_g0_route_v1(input, mode)? {
        Some(CanonicalCallableRouteV1::GenericG0(plan)) => Ok(Some(plan)),
        None => Ok(None),
        Some(_) => Err(
            "[freeze:contract][mir/callable-generic-g0-preflight/unexpected-route]".to_owned(),
        ),
    }
}
