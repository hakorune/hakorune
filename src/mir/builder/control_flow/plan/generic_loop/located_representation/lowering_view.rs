//! Lifetime-bound bridge from the O0 source seal to neutral Parts lowering.
//!
//! This module publishes borrowed views only. It does not reconstruct recipe
//! policy, source ordinals, or callable-result claims.

use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::plan::expression_port::{
    LocatedLoopPlanBodyInputV1, LocatedLoopPlanExprInputV1, LocatedLoopPlanExpressionPortV1,
    LocatedLoopPlanStmtInputV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{ExitKind, IfContractKind};

use super::product::{
    VerifiedLocatedGenericLoopBodyModeV1, VerifiedLocatedJoinIfRootV1,
    VerifiedLocatedRecipeBlockV1, VerifiedLocatedRecipeItemV1, VerifiedStmtWrappedJoinIfV1,
};
use super::{
    LocatedGenericLoopRepresentationErrorV1, VerifiedLocatedGenericLoopBodyRepresentationV1,
};

pub(in crate::mir::builder) struct VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan> {
    representation: &'view VerifiedLocatedGenericLoopBodyRepresentationV1<'plan>,
    port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
}

pub(in crate::mir::builder) enum VerifiedLocatedGenericLoopLoweringModeV1<'view, 'plan> {
    DirectRecipeOnly {
        body: VerifiedLocatedDirectBodyLoweringViewV1<'view, 'plan>,
    },
    ExitAllowedRecipe {
        root: VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    },
}

pub(in crate::mir::builder) struct VerifiedLocatedDirectBodyLoweringViewV1<'view, 'plan> {
    prefix: &'view [crate::mir::callable_result_representation::LegacyStmtInputV1<'plan>],
    port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
}

pub(in crate::mir::builder) struct VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan> {
    block: &'view VerifiedLocatedRecipeBlockV1<'plan>,
    port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
}

pub(in crate::mir::builder) enum VerifiedLocatedRecipeItemLoweringViewV1<'view, 'plan> {
    OpaqueStmt {
        source: LocatedLoopPlanStmtInputV1<'plan, 'view>,
    },
    OpaqueExit {
        source: LocatedLoopPlanStmtInputV1<'plan, 'view>,
        kind: ExitKind,
    },
    ExplicitIfV2 {
        source: LocatedLoopPlanStmtInputV1<'plan, 'view>,
        condition: LocatedLoopPlanExprInputV1<'plan, 'view>,
        then_body: LocatedLoopPlanBodyInputV1<'plan, 'view>,
        else_body: Option<LocatedLoopPlanBodyInputV1<'plan, 'view>>,
        contract: IfContractKind,
        then_block: VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
        else_block: Option<VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>>,
    },
    StmtWrappedJoinIf {
        bridge: VerifiedStmtWrappedJoinIfLoweringViewV1<'view, 'plan>,
    },
}

pub(in crate::mir::builder) struct VerifiedStmtWrappedJoinIfLoweringViewV1<'view, 'plan> {
    bridge: &'view VerifiedStmtWrappedJoinIfV1<'plan>,
    port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
}

pub(in crate::mir::builder) struct VerifiedLocatedJoinIfRootLoweringViewV1<'view, 'plan> {
    root: &'view VerifiedLocatedJoinIfRootV1<'plan>,
    port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
}

impl<'plan> VerifiedLocatedGenericLoopBodyRepresentationV1<'plan> {
    pub(in crate::mir::builder) fn bind_lowering_port<'view>(
        &'view self,
        port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
    ) -> Result<
        VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan>,
        LocatedGenericLoopRepresentationErrorV1,
    > {
        port.require_exact_stmt(&self.loop_root)?;
        Ok(VerifiedLocatedGenericLoopLoweringViewV1 {
            representation: self,
            port,
        })
    }

    pub(in crate::mir::builder) fn into_loop_statement(
        self,
    ) -> crate::mir::callable_result_representation::LegacyStmtInputV1<'plan> {
        self.loop_root
    }
}

impl<'view, 'plan> VerifiedLocatedGenericLoopLoweringViewV1<'view, 'plan> {
    pub(in crate::mir::builder) fn loop_var(&self) -> &str {
        &self.representation.extraction.facts().loop_var
    }

    pub(in crate::mir::builder) fn carrier_role(
        &self,
    ) -> crate::mir::builder::control_flow::plan::generic_loop::facts_types::GenericLoopCarrierRoleV1
    {
        self.representation.extraction.facts().carrier_role
    }

    pub(in crate::mir::builder) fn condition(&self) -> LocatedLoopPlanExprInputV1<'plan, 'view> {
        self.port.borrowed_expr(&self.representation.condition)
    }

    pub(in crate::mir::builder) fn cleanup(&self) -> LocatedLoopPlanStmtInputV1<'plan, 'view> {
        let cleanup = match &self.representation.mode {
            VerifiedLocatedGenericLoopBodyModeV1::DirectRecipeOnly { cleanup, .. }
            | VerifiedLocatedGenericLoopBodyModeV1::ExitAllowedRecipe { cleanup, .. } => cleanup,
        };
        self.port.borrowed_stmt(cleanup)
    }

    pub(in crate::mir::builder) fn mode(
        &self,
    ) -> VerifiedLocatedGenericLoopLoweringModeV1<'view, 'plan> {
        match &self.representation.mode {
            VerifiedLocatedGenericLoopBodyModeV1::DirectRecipeOnly { prefix, .. } => {
                VerifiedLocatedGenericLoopLoweringModeV1::DirectRecipeOnly {
                    body: VerifiedLocatedDirectBodyLoweringViewV1 {
                        prefix,
                        port: self.port,
                    },
                }
            }
            VerifiedLocatedGenericLoopBodyModeV1::ExitAllowedRecipe { root, .. } => {
                VerifiedLocatedGenericLoopLoweringModeV1::ExitAllowedRecipe {
                    root: VerifiedLocatedRecipeBlockLoweringViewV1 {
                        block: root,
                        port: self.port,
                    },
                }
            }
        }
    }
}

impl<'view, 'plan> VerifiedLocatedDirectBodyLoweringViewV1<'view, 'plan> {
    pub(in crate::mir::builder) fn expression_port(
        &self,
    ) -> &'view LocatedLoopPlanExpressionPortV1<'plan> {
        self.port
    }

    pub(in crate::mir::builder) fn len(&self) -> usize {
        self.prefix.len()
    }

    pub(in crate::mir::builder) fn statement(
        &self,
        index: usize,
    ) -> Option<LocatedLoopPlanStmtInputV1<'plan, 'view>> {
        self.prefix
            .get(index)
            .map(|source| self.port.borrowed_stmt(source))
    }
}

impl<'view, 'plan> VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan> {
    pub(in crate::mir::builder) fn expression_port(
        &self,
    ) -> &'view LocatedLoopPlanExpressionPortV1<'plan> {
        self.port
    }

    pub(in crate::mir::builder) fn len(&self) -> usize {
        self.block.items.len()
    }

    pub(in crate::mir::builder) fn item(
        &self,
        index: usize,
    ) -> Option<VerifiedLocatedRecipeItemLoweringViewV1<'view, 'plan>> {
        let item = self.block.items.get(index)?;
        Some(match item {
            VerifiedLocatedRecipeItemV1::OpaqueStmt { source } => {
                VerifiedLocatedRecipeItemLoweringViewV1::OpaqueStmt {
                    source: self.port.borrowed_stmt(source),
                }
            }
            VerifiedLocatedRecipeItemV1::OpaqueExit { source, kind } => {
                VerifiedLocatedRecipeItemLoweringViewV1::OpaqueExit {
                    source: self.port.borrowed_stmt(source),
                    kind: *kind,
                }
            }
            VerifiedLocatedRecipeItemV1::ExplicitIfV2 {
                source,
                condition,
                then_body,
                else_body,
                contract,
                then_block,
                else_block,
            } => VerifiedLocatedRecipeItemLoweringViewV1::ExplicitIfV2 {
                source: self.port.borrowed_stmt(source),
                condition: self.port.borrowed_expr(condition),
                then_body: self.port.borrowed_body(then_body),
                else_body: else_body.as_ref().map(|body| self.port.borrowed_body(body)),
                contract: *contract,
                then_block: VerifiedLocatedRecipeBlockLoweringViewV1 {
                    block: then_block,
                    port: self.port,
                },
                else_block: else_block.as_deref().map(|block| {
                    VerifiedLocatedRecipeBlockLoweringViewV1 {
                        block,
                        port: self.port,
                    }
                }),
            },
            VerifiedLocatedRecipeItemV1::StmtWrappedJoinIf { bridge } => {
                VerifiedLocatedRecipeItemLoweringViewV1::StmtWrappedJoinIf {
                    bridge: VerifiedStmtWrappedJoinIfLoweringViewV1 {
                        bridge,
                        port: self.port,
                    },
                }
            }
        })
    }
}

impl<'view, 'plan> VerifiedStmtWrappedJoinIfLoweringViewV1<'view, 'plan> {
    pub(in crate::mir::builder) fn source_syntax(&self) -> &'view crate::ast::ASTNode {
        let source = self.port.borrowed_stmt(&self.bridge.source_if);
        self.port.stmt_syntax(&source)
    }

    pub(in crate::mir::builder) fn source(&self) -> LocatedLoopPlanStmtInputV1<'plan, 'view> {
        self.port.borrowed_stmt(&self.bridge.source_if)
    }

    pub(in crate::mir::builder) fn condition(&self) -> LocatedLoopPlanExprInputV1<'plan, 'view> {
        self.port.borrowed_expr(&self.bridge.condition)
    }

    pub(in crate::mir::builder) fn then_body(&self) -> LocatedLoopPlanBodyInputV1<'plan, 'view> {
        self.port.borrowed_body(&self.bridge.then_body)
    }

    pub(in crate::mir::builder) fn else_body(
        &self,
    ) -> Option<LocatedLoopPlanBodyInputV1<'plan, 'view>> {
        self.bridge
            .else_body
            .as_ref()
            .map(|body| self.port.borrowed_body(body))
    }

    pub(in crate::mir::builder) fn singleton_recipe(&self) -> &'view NoExitBlockRecipe {
        &self.bridge.singleton_recipe
    }

    pub(in crate::mir::builder) fn singleton_root(
        &self,
    ) -> VerifiedLocatedJoinIfRootLoweringViewV1<'view, 'plan> {
        VerifiedLocatedJoinIfRootLoweringViewV1 {
            root: &self.bridge.singleton_root,
            port: self.port,
        }
    }
}

impl<'view, 'plan> VerifiedLocatedJoinIfRootLoweringViewV1<'view, 'plan> {
    pub(in crate::mir::builder) fn then_block(
        &self,
    ) -> VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan> {
        VerifiedLocatedRecipeBlockLoweringViewV1 {
            block: &self.root.then_block,
            port: self.port,
        }
    }

    pub(in crate::mir::builder) fn else_block(
        &self,
    ) -> Option<VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>> {
        self.root
            .else_block
            .as_deref()
            .map(|block| VerifiedLocatedRecipeBlockLoweringViewV1 {
                block,
                port: self.port,
            })
    }
}
