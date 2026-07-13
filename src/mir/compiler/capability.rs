//! Whole-unit capability proof before the first Builder effect.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::resolved_semantics::{
    BindingKindV1, RegionKindV1, ResolvedAssignmentTargetV1, ResolvedExitOriginV1,
    ResolvedLexicalRefV1, ScopeKindV1, SourceBindingSiteV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::source_view::{BodyChildRoleV1, ExprChildRoleV1};

#[derive(Debug, Clone, Copy)]
pub(crate) struct CanonicalFirstFamilyPlanV1<'a> {
    function: ResolvedFunctionLoweringInputV1<'a>,
    returns_value: bool,
}

impl<'a> CanonicalFirstFamilyPlanV1<'a> {
    pub(crate) const fn function(self) -> ResolvedFunctionLoweringInputV1<'a> {
        self.function
    }

    pub(crate) const fn returns_value(self) -> bool {
        self.returns_value
    }
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
        let (returns_value, block_expr_count) =
            verify_body(function, &located_body, ReturnPolicyV1::FinalOnly)?;
        verify_product_shape(function, block_expr_count)?;
        Ok(CanonicalFirstFamilyPlanV1 {
            function,
            returns_value,
        })
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
) -> Result<(bool, usize), CanonicalLoweringErrorV1> {
    let mut returns_value = false;
    let mut block_expr_count = 0;
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(body, index)
            .map_err(source_navigation)?;
        let is_last = index + 1 == body.statements().len();
        let (statement_returns, nested_count) =
            verify_statement(input, &statement, return_policy, is_last)?;
        returns_value |= statement_returns;
        block_expr_count += nested_count;
    }
    Ok((returns_value, block_expr_count))
}

fn verify_statement(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &LocatedStmtV1<'_>,
    return_policy: ReturnPolicyV1,
    is_last: bool,
) -> Result<(bool, usize), CanonicalLoweringErrorV1> {
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
            Ok((false, block_expr_count))
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
            Ok((false, 0))
        }
        ASTNode::Assignment { target, .. } => {
            if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                return unsupported(site, target, "target_is_not_binding_rebind");
            }
            let value = input
                .source()
                .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
                .map_err(source_navigation)?;
            Ok((false, verify_expression(input, &value)?))
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
                return Ok((true, verify_expression(input, &value)?));
            }
            Ok((false, 0))
        }
        ASTNode::Literal { .. }
        | ASTNode::Variable { .. }
        | ASTNode::BinaryOp { .. }
        | ASTNode::BlockExpr { .. } => {
            let expression = input
                .source()
                .statement_expression(statement)
                .map_err(source_navigation)?;
            Ok((false, verify_expression(input, &expression)?))
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
            let (_, prelude_count) = verify_body(input, &prelude, ReturnPolicyV1::Forbidden)?;
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
    if product.scope_count() != 2 + block_expr_count
        || product.region_count() != 2 + block_expr_count
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
                ScopeKindV1::Function | ScopeKindV1::LexicalBlock | ScopeKindV1::BlockExpr
            )
        })
        || product.regions().any(|(_, region)| {
            !matches!(
                region.kind(),
                RegionKindV1::Function | RegionKindV1::Sequence | RegionKindV1::BlockExpr
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
