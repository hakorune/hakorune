//! Builder-free admission seal for the first located Parts root.
//!
//! The seal borrows one already-bound O0 lowering view and validates every
//! carrier used by the first strict profile before any lowering hook exists.
//! It owns no Builder state, recipe reconstruction, or condition grammar.

use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::{
    LoopPlanExpressionPortErrorV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::{
    VerifiedLocatedGenericLoopLoweringModeV1, VerifiedLocatedGenericLoopLoweringViewV1,
    VerifiedLocatedRecipeBlockLoweringViewV1, VerifiedStmtWrappedJoinIfLoweringViewV1,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    ExitKind, IfContractKind, IfMode, RecipeItem,
};
use crate::mir::resolved_semantics::ExprChildRoleV1;

use super::{
    LocatedPartsAssociatedSourceV1, PartsAssociatedRecipeItemV1, PartsAssociatedSourceErrorV1,
    PartsAssociatedSourceV1, VerifiedPartsAssociatedItemV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::control_flow::plan::parts) enum LocatedPartsPreflightErrorV1 {
    WrongLoweringMode,
    RootCardinality { actual: usize },
    UnexpectedRootItem { ordinal: usize },
    UnsupportedOpaqueStatement { ordinal: usize },
    TypedLocalUnsupported { ordinal: usize },
    LocalShapeMismatch { ordinal: usize },
    WrongIfContract { ordinal: usize },
    IfElsePresenceMismatch { ordinal: usize },
    ExitBranchCardinality { actual: usize },
    WrongExitKind,
    MissingReturnValue,
    WrappedJoinRecipeMismatch,
    WrappedJoinElseMissing,
    WrappedJoinBranchCardinality { branch: &'static str, actual: usize },
    WrongJoinBranchStatement { branch: &'static str },
    WrongJoinBranchTarget { branch: &'static str },
    Source(PartsAssociatedSourceErrorV1),
    Port(LoopPlanExpressionPortErrorV1),
}

impl From<PartsAssociatedSourceErrorV1> for LocatedPartsPreflightErrorV1 {
    fn from(value: PartsAssociatedSourceErrorV1) -> Self {
        Self::Source(value)
    }
}

impl From<LoopPlanExpressionPortErrorV1> for LocatedPartsPreflightErrorV1 {
    fn from(value: LoopPlanExpressionPortErrorV1) -> Self {
        Self::Port(value)
    }
}

pub(in crate::mir::builder::control_flow::plan::parts) struct VerifiedLocatedGenericLoopPartsPreflightV1<
    'seal,
    'view,
    'plan,
> {
    lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    carrier_targets: Box<[String]>,
    _seal: LocatedPartsPreflightSealV1,
}

struct LocatedPartsPreflightSealV1;

pub(in crate::mir::builder) struct VerifiedLocatedGenericLoopPartsExecutionV1<'seal, 'view, 'plan> {
    lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    carrier_targets: Box<[String]>,
    _seal: LocatedPartsExecutionSealV1,
}

struct LocatedPartsExecutionSealV1;

impl<'seal, 'view, 'plan> VerifiedLocatedGenericLoopPartsPreflightV1<'seal, 'view, 'plan> {
    pub(in crate::mir::builder::control_flow::plan::parts) fn verify(
        lowering: &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
    ) -> Result<Self, LocatedPartsPreflightErrorV1> {
        let VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe { root } = lowering.mode()
        else {
            return Err(LocatedPartsPreflightErrorV1::WrongLoweringMode);
        };

        let carrier_targets = verify_strict_root(&root)?;
        Ok(Self {
            lowering,
            carrier_targets,
            _seal: LocatedPartsPreflightSealV1,
        })
    }

    pub(in crate::mir::builder) fn into_execution(
        self,
    ) -> VerifiedLocatedGenericLoopPartsExecutionV1<'seal, 'view, 'plan> {
        VerifiedLocatedGenericLoopPartsExecutionV1 {
            lowering: self.lowering,
            carrier_targets: self.carrier_targets,
            _seal: LocatedPartsExecutionSealV1,
        }
    }
}

impl<'seal, 'view, 'plan> VerifiedLocatedGenericLoopPartsExecutionV1<'seal, 'view, 'plan> {
    pub(in crate::mir::builder) fn lower_with_parts_adapter(
        self,
        lower: impl FnOnce(
            &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
        ) -> Result<
            Vec<crate::mir::builder::control_flow::plan::LoweredRecipe>,
            String,
        >,
    ) -> Result<Vec<crate::mir::builder::control_flow::plan::LoweredRecipe>, String> {
        lower(self.lowering)
    }

    pub(super) fn into_components(
        self,
    ) -> (
        &'seal VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
        Box<[String]>,
    ) {
        (self.lowering, self.carrier_targets)
    }
}

fn verify_strict_root(
    root: &VerifiedLocatedRecipeBlockLoweringViewV1<'_, '_>,
) -> Result<Box<[String]>, LocatedPartsPreflightErrorV1> {
    let provider = LocatedPartsAssociatedSourceV1::new(root);
    let len = provider.block_len(root)?;
    if len != 5 {
        return Err(LocatedPartsPreflightErrorV1::RootCardinality { actual: len });
    }

    verify_local(&provider, root, 0)?;
    verify_local(&provider, root, 1)?;
    verify_exit_if(&provider, root, 2)?;
    verify_local(&provider, root, 3)?;
    let mut carrier_targets = BTreeSet::new();
    verify_wrapped_join(&provider, root, 4, &mut carrier_targets)?;
    Ok(carrier_targets
        .into_iter()
        .collect::<Vec<_>>()
        .into_boxed_slice())
}

fn verify_local<'view, 'plan>(
    provider: &LocatedPartsAssociatedSourceV1<'view, 'plan>,
    root: &VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    ordinal: usize,
) -> Result<(), LocatedPartsPreflightErrorV1> {
    let VerifiedPartsAssociatedItemV1 { port, item } = provider.item(root, ordinal)?;
    let PartsAssociatedRecipeItemV1::OpaqueStmt { source } = item else {
        return Err(LocatedPartsPreflightErrorV1::UnexpectedRootItem { ordinal });
    };
    let ASTNode::Local {
        variables,
        initial_values,
        declared_type_names,
        ..
    } = port.stmt_syntax(&source)
    else {
        return Err(LocatedPartsPreflightErrorV1::UnsupportedOpaqueStatement { ordinal });
    };
    if declared_type_names.iter().any(Option::is_some) {
        return Err(LocatedPartsPreflightErrorV1::TypedLocalUnsupported { ordinal });
    }
    if variables.len() != 1
        || initial_values.len() != 1
        || declared_type_names.len() != 1
        || initial_values[0].is_none()
    {
        return Err(LocatedPartsPreflightErrorV1::LocalShapeMismatch { ordinal });
    }
    port.child_expr_from_stmt(&source, ExprChildRoleV1::LocalInitializer(0))?;
    Ok(())
}

fn verify_exit_if<'view, 'plan>(
    provider: &LocatedPartsAssociatedSourceV1<'view, 'plan>,
    root: &VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    ordinal: usize,
) -> Result<(), LocatedPartsPreflightErrorV1> {
    let VerifiedPartsAssociatedItemV1 { port, item } = provider.item(root, ordinal)?;
    let PartsAssociatedRecipeItemV1::ExplicitIfV2 {
        source,
        condition,
        then_body,
        else_body,
        contract,
        then_block,
        else_block,
    } = item
    else {
        return Err(LocatedPartsPreflightErrorV1::UnexpectedRootItem { ordinal });
    };
    if !matches!(
        contract,
        IfContractKind::ExitOnly {
            mode: IfMode::ExitIf
        }
    ) {
        return Err(LocatedPartsPreflightErrorV1::WrongIfContract { ordinal });
    }
    if else_body.is_some() || else_block.is_some() {
        return Err(LocatedPartsPreflightErrorV1::IfElsePresenceMismatch { ordinal });
    }
    if !matches!(
        port.stmt_syntax(&source),
        ASTNode::If {
            else_body: None,
            ..
        }
    ) {
        return Err(LocatedPartsPreflightErrorV1::IfElsePresenceMismatch { ordinal });
    }
    let _ = port.expr_syntax(&condition);
    if port.body_statements(&then_body).len() != 1 {
        return Err(LocatedPartsPreflightErrorV1::ExitBranchCardinality {
            actual: port.body_statements(&then_body).len(),
        });
    }
    let branch_len = provider.block_len(&then_block)?;
    if branch_len != 1 {
        return Err(LocatedPartsPreflightErrorV1::ExitBranchCardinality { actual: branch_len });
    }
    let VerifiedPartsAssociatedItemV1 {
        port: branch_port,
        item: branch_item,
    } = provider.item(&then_block, 0)?;
    let PartsAssociatedRecipeItemV1::OpaqueExit {
        source: returning,
        kind,
    } = branch_item
    else {
        return Err(LocatedPartsPreflightErrorV1::WrongExitKind);
    };
    if !matches!(kind, ExitKind::Return) {
        return Err(LocatedPartsPreflightErrorV1::WrongExitKind);
    }
    if !matches!(
        branch_port.stmt_syntax(&returning),
        ASTNode::Return { value: Some(_), .. }
    ) {
        return Err(LocatedPartsPreflightErrorV1::MissingReturnValue);
    }
    branch_port.child_expr_from_stmt(&returning, ExprChildRoleV1::ReturnValue)?;
    Ok(())
}

fn verify_wrapped_join<'view, 'plan>(
    provider: &LocatedPartsAssociatedSourceV1<'view, 'plan>,
    root: &VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    ordinal: usize,
    carrier_targets: &mut BTreeSet<String>,
) -> Result<(), LocatedPartsPreflightErrorV1> {
    let VerifiedPartsAssociatedItemV1 { item, .. } = provider.item(root, ordinal)?;
    let PartsAssociatedRecipeItemV1::StmtWrappedJoinIf { bridge } = item else {
        return Err(LocatedPartsPreflightErrorV1::UnexpectedRootItem { ordinal });
    };
    verify_join_recipe(&bridge)?;
    carrier_targets.insert(verify_join_branch(provider, &bridge, true)?);
    carrier_targets.insert(verify_join_branch(provider, &bridge, false)?);
    Ok(())
}

fn verify_join_recipe(
    bridge: &VerifiedStmtWrappedJoinIfLoweringViewV1<'_, '_>,
) -> Result<(), LocatedPartsPreflightErrorV1> {
    if !matches!(
        bridge.source_syntax(),
        ASTNode::If {
            else_body: Some(_),
            ..
        }
    ) || bridge.else_body().is_none()
    {
        return Err(LocatedPartsPreflightErrorV1::WrappedJoinElseMissing);
    }
    let _ = bridge.condition();
    let recipe = bridge.singleton_recipe();
    if !matches!(
        recipe.block.items.as_slice(),
        [RecipeItem::IfV2 {
            if_stmt,
            contract: IfContractKind::Join,
            ..
        }] if if_stmt.index() == 0
    ) {
        return Err(LocatedPartsPreflightErrorV1::WrappedJoinRecipeMismatch);
    }
    Ok(())
}

fn verify_join_branch<'view, 'plan>(
    provider: &LocatedPartsAssociatedSourceV1<'view, 'plan>,
    bridge: &VerifiedStmtWrappedJoinIfLoweringViewV1<'view, 'plan>,
    then_branch: bool,
) -> Result<String, LocatedPartsPreflightErrorV1> {
    let branch = if then_branch { "then" } else { "else" };
    let root = bridge.singleton_root();
    let block = if then_branch {
        root.then_block()
    } else {
        root.else_block()
            .ok_or(LocatedPartsPreflightErrorV1::WrappedJoinElseMissing)?
    };
    let len = provider.block_len(&block)?;
    if len != 1 {
        return Err(LocatedPartsPreflightErrorV1::WrappedJoinBranchCardinality {
            branch,
            actual: len,
        });
    }
    let VerifiedPartsAssociatedItemV1 { port, item } = provider.item(&block, 0)?;
    let PartsAssociatedRecipeItemV1::OpaqueStmt { source } = item else {
        return Err(LocatedPartsPreflightErrorV1::WrongJoinBranchStatement { branch });
    };
    if !matches!(port.stmt_syntax(&source), ASTNode::Assignment { .. }) {
        return Err(LocatedPartsPreflightErrorV1::WrongJoinBranchStatement { branch });
    }
    let target = port.child_expr_from_stmt(&source, ExprChildRoleV1::AssignmentTarget)?;
    let target_name = require_variable_join_target(port.expr_syntax(&target), branch)?;
    port.child_expr_from_stmt(&source, ExprChildRoleV1::AssignmentValue)?;
    Ok(target_name)
}

fn require_variable_join_target(
    target: &ASTNode,
    branch: &'static str,
) -> Result<String, LocatedPartsPreflightErrorV1> {
    match target {
        ASTNode::Variable { name, .. } => Ok(name.clone()),
        _ => Err(LocatedPartsPreflightErrorV1::WrongJoinBranchTarget { branch }),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, Span};

    use super::{require_variable_join_target, LocatedPartsPreflightErrorV1};

    #[test]
    fn join_target_preflight_accepts_only_a_variable_without_builder_state() {
        let variable = ASTNode::Variable {
            name: "value".to_string(),
            span: Span::unknown(),
        };
        assert_eq!(
            require_variable_join_target(&variable, "then"),
            Ok("value".to_string())
        );

        let field = ASTNode::FieldAccess {
            object: Box::new(ASTNode::Variable {
                name: "me".to_string(),
                span: Span::unknown(),
            }),
            field: "value".to_string(),
            span: Span::unknown(),
        };
        assert_eq!(
            require_variable_join_target(&field, "else"),
            Err(LocatedPartsPreflightErrorV1::WrongJoinBranchTarget { branch: "else" })
        );
    }
}
