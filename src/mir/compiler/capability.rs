//! Whole-unit capability proof before the first Builder effect.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::resolved_semantics::{
    BindingKindV1, RegionKindV1, ResolvedAssignmentTargetV1, ResolvedExitOriginV1,
    ResolvedLexicalRefV1, ScopeKindV1, SourceBindingSiteV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};

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

        let mut returns_value = false;
        for (index, statement) in body.iter().enumerate() {
            let site = format!("body[{index}]");
            let is_last = index + 1 == body.len();
            returns_value |= verify_statement(statement, &site, is_last)?;
        }
        verify_product_shape(function)?;
        Ok(CanonicalFirstFamilyPlanV1 {
            function,
            returns_value,
        })
    }
}

fn verify_statement(
    statement: &ASTNode,
    site: &str,
    is_last: bool,
) -> Result<bool, CanonicalLoweringErrorV1> {
    match statement {
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
                return unsupported(site, statement, "local_shape_not_closed");
            }
            for (index, initial) in initial_values.iter().enumerate() {
                if let Some(initial) = initial {
                    verify_expression(initial, &format!("{site}.initializer[{index}]"))?;
                }
            }
            Ok(false)
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
                return unsupported(site, statement, "outbox_shape_not_closed");
            }
            Ok(false)
        }
        ASTNode::Assignment { target, value, .. } => {
            if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                return unsupported(
                    &format!("{site}.target"),
                    target,
                    "target_is_not_binding_rebind",
                );
            }
            verify_expression(value, &format!("{site}.value"))?;
            Ok(false)
        }
        ASTNode::Return { value, .. } => {
            if !is_last {
                return unsupported(site, statement, "return_must_be_final_statement");
            }
            if let Some(value) = value {
                verify_expression(value, &format!("{site}.value"))?;
            }
            Ok(value.is_some())
        }
        expression => {
            verify_expression(expression, site)?;
            Ok(false)
        }
    }
}

fn verify_expression(expression: &ASTNode, site: &str) -> Result<(), CanonicalLoweringErrorV1> {
    match expression {
        ASTNode::Literal { .. } | ASTNode::Variable { .. } => Ok(()),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } if !matches!(operator, BinaryOperator::And | BinaryOperator::Or) => {
            verify_expression(left, &format!("{site}.lhs"))?;
            verify_expression(right, &format!("{site}.rhs"))
        }
        _ => unsupported(site, expression, "expression_not_in_first_family"),
    }
}

fn verify_product_shape(
    input: ResolvedFunctionLoweringInputV1<'_>,
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
    if product.scope_count() != 2
        || product.region_count() != 2
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
                ScopeKindV1::Function | ScopeKindV1::LexicalBlock
            )
        })
        || product.regions().any(|(_, region)| {
            !matches!(
                region.kind(),
                RegionKindV1::Function | RegionKindV1::Sequence
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
