//! Whole-unit capability proof before the first Builder effect.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::resolved_control_flow::if_control::verify_resolved_function_if_control_v1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_region_flow::{
    analyze_resolved_function_flow_v1, VerifiedResolvedFunctionFlowV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, RegionKindV1, ResolvedAssignmentTargetV1, ResolvedExitOriginV1,
    ResolvedLexicalRefV1, ScopeKindV1, SourceBindingSiteV1,
};
use crate::mir::resolved_value_profile::{
    analyze_trivial_canonical_owner_v1, product::VerifiedTrivialCanonicalOwnerV1,
    TrivialCanonicalOwnerAnalysisV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1};

#[derive(Debug)]
pub(crate) struct CanonicalCurrentAPlusPlanV1<'a> {
    function: ResolvedFunctionLoweringInputV1<'a>,
    flow: VerifiedResolvedFunctionFlowV1,
    completion: VerifiedFunctionCompletionV1,
    block_expr_count: usize,
}

impl<'a> CanonicalCurrentAPlusPlanV1<'a> {
    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'a>,
        VerifiedResolvedFunctionFlowV1,
        VerifiedFunctionCompletionV1,
        usize,
    ) {
        (
            self.function,
            self.flow,
            self.completion,
            self.block_expr_count,
        )
    }
}

#[derive(Debug)]
pub(crate) struct CanonicalTrivialBindingSsaPlanV1<'a> {
    function: ResolvedFunctionLoweringInputV1<'a>,
    if_control: crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
}

impl<'a> CanonicalTrivialBindingSsaPlanV1<'a> {
    pub(crate) fn into_parts(
        self,
    ) -> (
        ResolvedFunctionLoweringInputV1<'a>,
        crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1,
        VerifiedFunctionCompletionV1,
        VerifiedTrivialCanonicalOwnerV1,
        usize,
    ) {
        (
            self.function,
            self.if_control,
            self.completion,
            self.profile,
            self.block_expr_count,
        )
    }
}

/// One whole-unit canonical value-authority selection.
///
/// The variant is sealed before the module candidate is opened. A later
/// lowering failure cannot be reclassified as the temporary A+ route.
#[derive(Debug)]
pub(crate) enum CanonicalFirstFamilyPlanV1<'a> {
    TrivialBindingSsa(CanonicalTrivialBindingSsaPlanV1<'a>),
    CurrentCanonicalAPlus(CanonicalCurrentAPlusPlanV1<'a>),
}

pub(crate) struct CanonicalLoweringPreflightV1;

impl CanonicalLoweringPreflightV1 {
    pub(crate) fn verify(
        unit: &VerifiedResolvedSourceUnitV1,
    ) -> Result<CanonicalFirstFamilyPlanV1<'_>, CanonicalLoweringErrorV1> {
        if unit.forest().owner_count() != 1 || !unit.forest().upvars().is_empty() {
            return unsupported("source_unit", unit.syntax_root(), "owner_family_not_closed");
        }
        let function = unit.root_function_input()?;
        let ASTNode::FunctionDeclaration {
            name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            contracts,
            is_static,
            is_override,
            attrs,
            ..
        } = function.source().root()
        else {
            return unsupported("root", function.source().root(), "root_is_not_function");
        };
        if !*is_static || *is_override || name == "main" {
            return unsupported(
                "root",
                function.source().root(),
                "owner_kind_not_first_family",
            );
        }
        if !uses.is_empty() || !contracts.is_empty() || !attrs.is_empty() {
            return unsupported(
                "root",
                function.source().root(),
                "function_metadata_not_activated",
            );
        }
        if return_type_name.is_some()
            || param_decls
                .iter()
                .any(|decl| decl.declared_type_name.is_some())
            || (!param_decls.is_empty() && param_decls.len() != params.len())
            || (!param_decls.is_empty()
                && param_decls
                    .iter()
                    .zip(params)
                    .any(|(decl, name)| decl.name != *name))
        {
            return unsupported(
                "root",
                function.source().root(),
                "typed_signature_not_activated",
            );
        }

        let located_body = function.source().root_body().map_err(source_navigation)?;
        debug_assert_eq!(body.len(), located_body.statements().len());
        let block_expr_count = verify_body(function, &located_body, ReturnPolicyV1::FinalOnly)?;
        let completion = verify_function_completion_v1(function).map_err(|error| {
            CanonicalLoweringErrorV1::ResolvedFunctionCompletion {
                detail: format!("{error:?}"),
            }
        })?;
        let if_control =
            verify_resolved_function_if_control_v1(function, &completion).map_err(|error| {
                CanonicalLoweringErrorV1::ResolvedRegionFlow {
                    detail: format!("if_control_contract={error:?}"),
                }
            })?;
        verify_product_shape(
            function,
            if_control.row_count(),
            if_control.explicit_else_count(),
            block_expr_count,
        )?;

        let profile = analyze_trivial_canonical_owner_v1(function, &completion, &if_control)
            .map_err(|error| CanonicalLoweringErrorV1::ResolvedRegionFlow {
                detail: format!("trivial_profile_contract={error:?}"),
            })?;
        match profile {
            TrivialCanonicalOwnerAnalysisV1::Admitted(profile) => {
                return Ok(CanonicalFirstFamilyPlanV1::TrivialBindingSsa(
                    CanonicalTrivialBindingSsaPlanV1 {
                        function,
                        if_control,
                        completion,
                        profile,
                        block_expr_count,
                    },
                ));
            }
            TrivialCanonicalOwnerAnalysisV1::NotAdmitted(_) => {}
        }

        // Temporary A+ is selected only from an explicit whole-owner profile
        // stop. Contract failures above are canonical errors and never reach
        // this branch. The legacy RegionFlow analysis is therefore absent
        // from every admitted Binding-SSA route.
        let flow = analyze_resolved_function_flow_v1(function, &completion).map_err(|error| {
            CanonicalLoweringErrorV1::ResolvedRegionFlow {
                detail: format!("{error:?}"),
            }
        })?;
        Ok(CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(
            CanonicalCurrentAPlusPlanV1 {
                function,
                flow,
                completion,
                block_expr_count,
            },
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReturnPolicyV1 {
    FinalOnly,
    Forbidden,
}

fn verify_body(
    input: ResolvedFunctionLoweringInputV1<'_>,
    body: &LocatedBodyV1<'_>,
    return_policy: ReturnPolicyV1,
) -> Result<usize, CanonicalLoweringErrorV1> {
    let mut block_expr_count = 0;
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(body, index)
            .map_err(source_navigation)?;
        let is_last = index + 1 == body.statements().len();
        block_expr_count += verify_statement(input, &statement, return_policy, is_last)?;
    }
    Ok(block_expr_count)
}

fn verify_statement(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    return_policy: ReturnPolicyV1,
    is_last: bool,
) -> Result<usize, CanonicalLoweringErrorV1> {
    let site = format!("{:?}", statement.site());
    match statement.node() {
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } => {
            if variables.is_empty()
                || initial_values.len() != variables.len()
                || declared_type_names.len() != variables.len()
                || declared_type_names.iter().any(Option::is_some)
            {
                return unsupported(site, statement.node(), "local_shape_not_closed");
            }
            let mut block_expr_count = 0;
            for (index, initial) in initial_values.iter().enumerate() {
                if initial.is_some() {
                    let initial = input
                        .source()
                        .child_expr_from_stmt(
                            statement,
                            ExprChildRoleV1::LocalInitializer(index as u32),
                        )
                        .map_err(source_navigation)?;
                    block_expr_count += verify_expression(input, &initial)?;
                }
            }
            Ok(block_expr_count)
        }
        ASTNode::Outbox {
            variables,
            initial_values,
            ..
        } => {
            if variables.is_empty()
                || initial_values.len() != variables.len()
                || initial_values.iter().any(Option::is_some)
            {
                return unsupported(site, statement.node(), "outbox_shape_not_closed");
            }
            Ok(0)
        }
        ASTNode::Assignment { target, .. } => {
            if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                return unsupported(site, target, "target_is_not_binding_rebind");
            }
            let value = input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
                .map_err(source_navigation)?;
            verify_expression(input, &value)
        }
        ASTNode::If { else_body, .. } => {
            let condition = input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
                .map_err(source_navigation)?;
            let mut block_expr_count = verify_expression(input, &condition)?;
            let then_body = input
                .source()
                .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
                .map_err(source_navigation)?;
            block_expr_count += verify_body(input, &then_body, ReturnPolicyV1::Forbidden)?;
            if else_body.is_some() {
                let else_body = input
                    .source()
                    .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                    .map_err(source_navigation)?;
                block_expr_count += verify_body(input, &else_body, ReturnPolicyV1::Forbidden)?;
            }
            Ok(block_expr_count)
        }
        ASTNode::Return { value, .. } => {
            if return_policy == ReturnPolicyV1::Forbidden || !is_last {
                return unsupported(site, statement.node(), "return_not_allowed_here");
            }
            if value.is_some() {
                let value = input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                    .map_err(source_navigation)?;
                return verify_expression(input, &value);
            }
            Ok(0)
        }
        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::BlockExpr { .. } => {
            let expression = input
                .source()
                .statement_expression(statement)
                .map_err(source_navigation)?;
            verify_expression(input, &expression)
        }
        _ => unsupported(site, statement.node(), "statement_not_in_first_family"),
    }
}

fn verify_expression(
    input: ResolvedFunctionLoweringInputV1<'_>,
    expression: &LocatedExprV1<'_>,
) -> Result<usize, CanonicalLoweringErrorV1> {
    let site = format!("{:?}", expression.site());
    match expression.node() {
        ASTNode::Literal { .. } | ASTNode::Variable { .. } => Ok(0),
        ASTNode::BinaryOp { operator, .. }
            if !matches!(operator, BinaryOperator::And | BinaryOperator::Or) =>
        {
            let left = input
                .source()
                .child_expr_from_expr(expression, ExprChildRoleV1::BinaryLeft)
                .map_err(source_navigation)?;
            let right = input
                .source()
                .child_expr_from_expr(expression, ExprChildRoleV1::BinaryRight)
                .map_err(source_navigation)?;
            Ok(verify_expression(input, &left)? + verify_expression(input, &right)?)
        }
        ASTNode::BlockExpr { .. } => {
            input
                .function()
                .block_expr_scope_region_pair(expression.owner(), expression.site())
                .map_err(|_| CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
                    site: site.clone(),
                    actual: expression.node().node_type(),
                    reason: "blockexpr_pair_not_closed",
                })?;
            let prelude = input
                .source()
                .child_body_from_expr(expression, BodyChildRoleV1::BlockExprPrelude)
                .map_err(source_navigation)?;
            let prelude_count = verify_body(input, &prelude, ReturnPolicyV1::Forbidden)?;
            let tail = input
                .source()
                .child_expr_from_expr(expression, ExprChildRoleV1::BlockExprTail)
                .map_err(source_navigation)?;
            Ok(1 + prelude_count + verify_expression(input, &tail)?)
        }
        _ => unsupported(site, expression.node(), "expression_not_in_first_family"),
    }
}

fn verify_product_shape(
    input: ResolvedFunctionLoweringInputV1<'_>,
    if_count: usize,
    explicit_else_count: usize,
    block_expr_count: usize,
) -> Result<(), CanonicalLoweringErrorV1> {
    let product = input.function();
    if product.owner() != input.owner()
        || input.forest().owner(input.owner()).is_none()
        || product
            .declaration_binding(&SourceBindingSiteV1::Receiver)
            .is_some()
    {
        return unsupported("product", input.source().root(), "owner_product_mismatch");
    }
    for (_, binding) in product.bindings() {
        if !matches!(
            binding.kind(),
            BindingKindV1::Parameter { .. }
                | BindingKindV1::Local { .. }
                | BindingKindV1::Outbox { .. }
        ) {
            return unsupported(
                "product.binding",
                input.source().root(),
                "binding_kind_not_closed",
            );
        }
    }
    let product_block_expr_scopes = product
        .scopes()
        .filter(|(_, scope)| scope.kind() == ScopeKindV1::BlockExpr)
        .count();
    let product_block_expr_regions = product
        .regions()
        .filter(|(_, region)| region.kind() == RegionKindV1::BlockExpr)
        .count();
    let expected_scope_count = 2 + block_expr_count + if_count + explicit_else_count;
    let expected_region_count = 2 + block_expr_count + (2 * if_count) + explicit_else_count;
    if product.scope_count() != expected_scope_count
        || product.region_count() != expected_region_count
        || product_block_expr_scopes != block_expr_count
        || product_block_expr_regions != block_expr_count
        || product
            .variable_refs()
            .any(|(_, reference)| !matches!(reference, ResolvedLexicalRefV1::Local(_)))
        || product
            .assignment_targets()
            .any(|(_, target)| !matches!(target, ResolvedAssignmentTargetV1::BindingRebind(_)))
        || product
            .resolved_exits()
            .any(|(_, exit)| exit.origin() != ResolvedExitOriginV1::ExplicitReturn)
        || product.scopes().any(|(_, scope)| {
            !matches!(
                scope.kind(),
                ScopeKindV1::Function
                    | ScopeKindV1::LexicalBlock
                    | ScopeKindV1::BlockExpr
                    | ScopeKindV1::IfThen
                    | ScopeKindV1::IfElse
            )
        })
        || product.regions().any(|(_, region)| {
            !matches!(
                region.kind(),
                RegionKindV1::Function
                    | RegionKindV1::Sequence
                    | RegionKindV1::BlockExpr
                    | RegionKindV1::If
                    | RegionKindV1::IfThen
                    | RegionKindV1::IfElse
            )
        })
    {
        return unsupported(
            "product",
            input.source().root(),
            "semantic_shape_not_closed",
        );
    }
    Ok(())
}

fn source_navigation(error: impl ToString) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceNavigation {
        detail: error.to_string(),
    }
}

fn unsupported<T>(
    site: impl Into<String>,
    node: &ASTNode,
    reason: &'static str,
) -> Result<T, CanonicalLoweringErrorV1> {
    Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
        site: site.into(),
        actual: node.node_type(),
        reason,
    })
}
