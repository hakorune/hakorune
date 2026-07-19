use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::try_build_no_exit_block_recipe;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    ExitKind, IfContractKind, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::plan::LocatedLoopPlanExpressionPortV1;
use crate::mir::callable_result_representation::{LegacyBodyInputV1, LegacyStmtInputV1};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};

use super::error::LocatedGenericLoopRepresentationErrorV1 as Error;
use super::product::{
    VerifiedLocatedJoinIfRootV1, VerifiedLocatedRecipeBlockV1, VerifiedLocatedRecipeItemV1,
    VerifiedStmtWrappedJoinIfV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RecipeSealDomainV1 {
    ExitAllowed,
    NoExit,
}

pub(super) fn reject_unsupported_nested_statements(body: &[ASTNode]) -> Result<(), Error> {
    let mut pending = vec![body];
    while let Some(statements) = pending.pop() {
        for statement in statements {
            match statement {
                ASTNode::Program { .. }
                | ASTNode::ScopeBox { .. }
                | ASTNode::Loop { .. }
                | ASTNode::LoopRange { .. } => return Err(Error::UnsupportedNestedStatement),
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    pending.push(then_body);
                    if let Some(else_body) = else_body.as_deref() {
                        pending.push(else_body);
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub(super) fn seal_recipe_block<'plan>(
    port: &LocatedLoopPlanExpressionPortV1<'plan>,
    arena: &RecipeBodies,
    block: &RecipeBlock,
    body: &LegacyBodyInputV1<'plan>,
    exact_len: usize,
    domain: RecipeSealDomainV1,
) -> Result<VerifiedLocatedRecipeBlockV1<'plan>, Error> {
    port.require_exact_body(body)?;
    if exact_len > body.statements().len() {
        return Err(Error::RecipeItemCountMismatch {
            exact: exact_len,
            recipe: block.items.len(),
        });
    }
    let recipe_body = arena.get(block.body_id).ok_or(Error::MissingRecipeBody)?;
    if recipe_body.len() != exact_len {
        return Err(Error::RecipeBodyLengthMismatch {
            exact: exact_len,
            recipe: recipe_body.len(),
        });
    }
    if block.items.len() != exact_len {
        return Err(Error::RecipeItemCountMismatch {
            exact: exact_len,
            recipe: block.items.len(),
        });
    }

    let mut verified = Vec::with_capacity(exact_len);
    for (expected, item) in block.items.iter().enumerate() {
        verified.push(seal_recipe_item(port, arena, item, body, expected, domain)?);
    }
    Ok(VerifiedLocatedRecipeBlockV1 {
        items: verified.into_boxed_slice(),
    })
}

fn seal_recipe_item<'plan>(
    port: &LocatedLoopPlanExpressionPortV1<'plan>,
    arena: &RecipeBodies,
    item: &RecipeItem,
    body: &LegacyBodyInputV1<'plan>,
    expected: usize,
    domain: RecipeSealDomainV1,
) -> Result<VerifiedLocatedRecipeItemV1<'plan>, Error> {
    match item {
        RecipeItem::Stmt(reference) => {
            require_ordinal(expected, reference.index())?;
            let source = port.exact_body_stmt(body, expected)?;
            match source.node() {
                ASTNode::If { .. } if domain == RecipeSealDomainV1::ExitAllowed => {
                    seal_wrapped_join_if(port, source)
                }
                ASTNode::If { .. } => Err(Error::RecipeContractMismatch),
                ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. } => {
                    Err(Error::RecipeSourceKindMismatch)
                }
                ASTNode::Program { .. }
                | ASTNode::ScopeBox { .. }
                | ASTNode::Loop { .. }
                | ASTNode::LoopRange { .. } => Err(Error::UnsupportedNestedStatement),
                _ => Ok(VerifiedLocatedRecipeItemV1::OpaqueStmt { source }),
            }
        }
        RecipeItem::Exit { kind, stmt } => {
            require_ordinal(expected, stmt.index())?;
            let source = port.exact_body_stmt(body, expected)?;
            require_exit_kind(source.node(), *kind)?;
            Ok(VerifiedLocatedRecipeItemV1::OpaqueExit {
                source,
                kind: *kind,
            })
        }
        RecipeItem::IfV2 {
            if_stmt,
            contract,
            then_block,
            else_block,
            ..
        } => {
            require_ordinal(expected, if_stmt.index())?;
            require_contract(domain, contract)?;
            let source = port.exact_body_stmt(body, expected)?;
            if !matches!(source.node(), ASTNode::If { .. }) {
                return Err(Error::RecipeSourceKindMismatch);
            }
            let condition =
                port.exact_child_expr_from_stmt(&source, ExprChildRoleV1::IfCondition)?;
            let then_body = port.exact_child_body_from_stmt(&source, BodyChildRoleV1::IfThen)?;
            let else_body = match source.node() {
                ASTNode::If { else_body, .. } if else_body.is_some() => {
                    Some(port.exact_child_body_from_stmt(&source, BodyChildRoleV1::IfElse)?)
                }
                ASTNode::If { .. } => None,
                _ => unreachable!(),
            };
            if else_body.is_some() != else_block.is_some() {
                return Err(Error::IfElsePresenceMismatch);
            }
            let then_verified = Box::new(seal_recipe_block(
                port,
                arena,
                then_block,
                &then_body,
                then_body.statements().len(),
                domain,
            )?);
            let else_verified = match (else_block, else_body.as_ref()) {
                (Some(block), Some(body)) => Some(Box::new(seal_recipe_block(
                    port,
                    arena,
                    block,
                    body,
                    body.statements().len(),
                    domain,
                )?)),
                (None, None) => None,
                _ => return Err(Error::IfElsePresenceMismatch),
            };
            Ok(VerifiedLocatedRecipeItemV1::ExplicitIfV2 {
                source,
                condition,
                then_body,
                else_body,
                contract: *contract,
                then_block: then_verified,
                else_block: else_verified,
            })
        }
        RecipeItem::LoopV0 { .. } => Err(Error::RecipeLoopUnsupported),
    }
}

fn seal_wrapped_join_if<'plan>(
    port: &LocatedLoopPlanExpressionPortV1<'plan>,
    source_if: LegacyStmtInputV1<'plan>,
) -> Result<VerifiedLocatedRecipeItemV1<'plan>, Error> {
    let condition = port.exact_child_expr_from_stmt(&source_if, ExprChildRoleV1::IfCondition)?;
    let then_body = port.exact_child_body_from_stmt(&source_if, BodyChildRoleV1::IfThen)?;
    let else_body = match source_if.node() {
        ASTNode::If { else_body, .. } if else_body.is_some() => {
            Some(port.exact_child_body_from_stmt(&source_if, BodyChildRoleV1::IfElse)?)
        }
        ASTNode::If { .. } => None,
        _ => return Err(Error::RecipeSourceKindMismatch),
    };
    let singleton_recipe =
        try_build_no_exit_block_recipe(std::slice::from_ref(source_if.node()), true)
            .ok_or(Error::WrappedJoinIfRecipeRejected)?;
    if singleton_recipe.block.items.len() != 1 {
        return Err(Error::WrappedJoinIfRootCardinality);
    }
    let singleton_body = singleton_recipe
        .arena
        .get(singleton_recipe.block.body_id)
        .ok_or(Error::MissingRecipeBody)?;
    if singleton_body.len() != 1 {
        return Err(Error::RecipeBodyLengthMismatch {
            exact: 1,
            recipe: singleton_body.len(),
        });
    }
    let RecipeItem::IfV2 {
        if_stmt,
        contract: IfContractKind::Join,
        then_block,
        else_block,
        ..
    } = &singleton_recipe.block.items[0]
    else {
        return Err(Error::WrappedJoinIfRootNotJoin);
    };
    require_ordinal(0, if_stmt.index())?;
    if else_body.is_some() != else_block.is_some() {
        return Err(Error::IfElsePresenceMismatch);
    }
    let then_verified = Box::new(seal_recipe_block(
        port,
        &singleton_recipe.arena,
        then_block,
        &then_body,
        then_body.statements().len(),
        RecipeSealDomainV1::NoExit,
    )?);
    let else_verified = match (else_block, else_body.as_ref()) {
        (Some(block), Some(body)) => Some(Box::new(seal_recipe_block(
            port,
            &singleton_recipe.arena,
            block,
            body,
            body.statements().len(),
            RecipeSealDomainV1::NoExit,
        )?)),
        (None, None) => None,
        _ => return Err(Error::IfElsePresenceMismatch),
    };
    let singleton_root = VerifiedLocatedJoinIfRootV1 {
        then_block: then_verified,
        else_block: else_verified,
    };
    Ok(VerifiedLocatedRecipeItemV1::StmtWrappedJoinIf {
        bridge: VerifiedStmtWrappedJoinIfV1 {
            source_if,
            condition,
            then_body,
            else_body,
            singleton_recipe,
            singleton_root,
        },
    })
}

pub(super) fn require_contract(
    domain: RecipeSealDomainV1,
    contract: &IfContractKind,
) -> Result<(), Error> {
    let admitted = match (domain, contract) {
        (
            RecipeSealDomainV1::ExitAllowed,
            IfContractKind::ExitOnly { .. } | IfContractKind::ExitAllowed { .. },
        ) => true,
        (RecipeSealDomainV1::NoExit, IfContractKind::Join) => true,
        _ => false,
    };
    if admitted {
        Ok(())
    } else {
        Err(Error::RecipeContractMismatch)
    }
}

fn require_ordinal(expected: usize, actual: usize) -> Result<(), Error> {
    if expected == actual {
        Ok(())
    } else {
        Err(Error::RecipeOrdinalMismatch { expected, actual })
    }
}

fn require_exit_kind(source: &ASTNode, expected: ExitKind) -> Result<(), Error> {
    let matches = match (source, expected) {
        (ASTNode::Return { .. }, ExitKind::Return) => true,
        (ASTNode::Break { .. }, ExitKind::Break { depth: 1 }) => true,
        (ASTNode::Continue { .. }, ExitKind::Continue { depth: 1 }) => true,
        _ => false,
    };
    if matches {
        Ok(())
    } else {
        Err(Error::ExitKindMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn leaf() -> ASTNode {
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }
    }

    #[test]
    fn reject_unsupported_nested_statements_rejects_scopebox_program_and_nested_loop() {
        let cases = [
            ASTNode::ScopeBox {
                body: vec![leaf()],
                span: Span::unknown(),
            },
            ASTNode::Program {
                statements: vec![leaf()],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(leaf()),
                body: vec![leaf()],
                span: Span::unknown(),
            },
        ];
        for nested in cases {
            assert!(matches!(
                reject_unsupported_nested_statements(&[nested]),
                Err(Error::UnsupportedNestedStatement)
            ));
        }
    }
}
