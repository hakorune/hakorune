//! Located callable-loop provider for the neutral associated-source dispatcher.
//!
//! This sibling provider pairs an already-issued `RecipeBlock` with the
//! located statement/body carriers of `CallableLoopSourceExpressionPortV1`.
//! Recipe block contracts remain the structural packaging authority; this
//! module only projects `StmtRef` indices and child roles through the source
//! port and rejects when the recipe body no longer aligns 1:1 with the
//! located body carrier. It classifies no new semantics and never falls back
//! to a name or AST rescan.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::expression_port::LoopPlanExpressionPortV1;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, LoopKindV0, LoopV0Features, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::builder::normal_callable_loop_source_port::{
    CallableLoopSourceBodyInputV1, CallableLoopSourceExprInputV1,
    CallableLoopSourceExpressionPortV1, CallableLoopSourceStmtInputV1,
};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use std::convert::Infallible;

use super::{
    sealed, PartsAssociatedRecipeItemV1, PartsAssociatedSourceErrorV1, PartsAssociatedSourceV1,
    VerifiedPartsAssociatedItemV1,
};

/// Located body carrier paired with one recipe block.
///
/// `Singleton` covers `from_ref(stmt)` recipe roots (ProgramBlock/GeneralIf
/// re-derivation and nested container recipes): the registered recipe body is
/// exactly the one already-located statement. `Located` covers branch/loop
/// bodies resolved through `child_body_from_stmt` on the located parent.
pub(in crate::mir::builder::control_flow::plan::parts) enum CallableLoopSourcePartsBlockBodyV1<
    'view,
> {
    Singleton(CallableLoopSourceStmtInputV1<'view>),
    Located(CallableLoopSourceBodyInputV1<'view>),
}

/// One recipe block co-sealed with its located body carrier.
pub(in crate::mir::builder::control_flow::plan::parts) struct CallableLoopSourcePartsBlockV1<'view>
{
    arena: &'view RecipeBodies,
    block: &'view RecipeBlock,
    body: CallableLoopSourcePartsBlockBodyV1<'view>,
}

impl<'view> CallableLoopSourcePartsBlockV1<'view> {
    /// Pair a `from_ref(stmt)` singleton recipe with its located statement.
    /// The recipe body must be exactly one verbatim clone of the statement.
    pub(in crate::mir::builder::control_flow::plan::parts) fn singleton(
        arena: &'view RecipeBodies,
        block: &'view RecipeBlock,
        stmt: CallableLoopSourceStmtInputV1<'view>,
        port: &CallableLoopSourceExpressionPortV1<'_>,
    ) -> Result<Self, PartsAssociatedSourceErrorV1> {
        if matches!(stmt, CallableLoopSourceStmtInputV1::Synthetic(_)) {
            return Err(PartsAssociatedSourceErrorV1::SyntheticCarrier);
        }
        let body = arena
            .get(block.body_id)
            .ok_or(PartsAssociatedSourceErrorV1::MissingRecipeBody)?;
        if body.as_ref().len() != 1 || &body.as_ref()[0] != port.stmt_syntax(&stmt) {
            return Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch);
        }
        Ok(Self {
            arena,
            block,
            body: CallableLoopSourcePartsBlockBodyV1::Singleton(stmt),
        })
    }

    /// Pair a recipe block with the located child body it was registered
    /// from. The recipe body must be a verbatim clone of the same statement
    /// list; per-statement equality is re-checked at item projection time.
    pub(in crate::mir::builder::control_flow::plan::parts) fn located_body(
        arena: &'view RecipeBodies,
        block: &'view RecipeBlock,
        body: CallableLoopSourceBodyInputV1<'view>,
        port: &CallableLoopSourceExpressionPortV1<'_>,
    ) -> Result<Self, PartsAssociatedSourceErrorV1> {
        if matches!(body, CallableLoopSourceBodyInputV1::Synthetic(_)) {
            return Err(PartsAssociatedSourceErrorV1::SyntheticCarrier);
        }
        let recipe_body = arena
            .get(block.body_id)
            .ok_or(PartsAssociatedSourceErrorV1::MissingRecipeBody)?;
        if recipe_body.as_ref().len() != port.body_statements(&body).len() {
            return Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch);
        }
        Ok(Self {
            arena,
            block,
            body: CallableLoopSourcePartsBlockBodyV1::Located(body),
        })
    }

    /// The issued recipe body backing this block. Carriers read names off the
    /// co-sealed AST; no fresh source lookup is performed.
    pub(in crate::mir::builder::control_flow::plan::parts) fn recipe_body(
        &self,
    ) -> Result<&'view RecipeBody, PartsAssociatedSourceErrorV1> {
        self.arena
            .get(self.block.body_id)
            .ok_or(PartsAssociatedSourceErrorV1::MissingRecipeBody)
    }
}

/// `RecipeItem::LoopV0` payload carried beside the located loop statement.
pub(in crate::mir::builder::control_flow::plan::parts) struct CallableLoopSourcePartsLoopV0V1<
    'view,
> {
    pub(in crate::mir::builder::control_flow::plan::parts) source:
        CallableLoopSourceStmtInputV1<'view>,
    pub(in crate::mir::builder::control_flow::plan::parts) kind: LoopKindV0,
    pub(in crate::mir::builder::control_flow::plan::parts) condition:
        CallableLoopSourceExprInputV1<'view>,
    pub(in crate::mir::builder::control_flow::plan::parts) body_block:
        CallableLoopSourcePartsBlockV1<'view>,
    pub(in crate::mir::builder::control_flow::plan::parts) body_contract: BlockContractKind,
    pub(in crate::mir::builder::control_flow::plan::parts) features: LoopV0Features,
}

/// Source provider: projects recipe items into located port carriers.
///
/// The provider borrows one recipe arena for the block it drives; child
/// blocks minted by `item()` always share that arena, so a foreign block can
/// only arrive through the constructor checks above.
pub(in crate::mir::builder::control_flow::plan::parts) struct CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger> {
    arena: &'view RecipeBodies,
    port: CallableLoopSourceExpressionPortV1<'ledger>,
}

impl<'view, 'ledger: 'view> CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger> {
    pub(in crate::mir::builder::control_flow::plan::parts) const fn new(
        arena: &'view RecipeBodies,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
    ) -> Self {
        Self { arena, port }
    }

    /// Bind the provider to the arena that issued `block`.
    pub(in crate::mir::builder::control_flow::plan::parts) const fn for_block(
        block: &CallableLoopSourcePartsBlockV1<'view>,
        port: CallableLoopSourceExpressionPortV1<'ledger>,
    ) -> Self {
        Self::new(block.arena, port)
    }

    fn require_own_block(
        &self,
        block: &CallableLoopSourcePartsBlockV1<'view>,
    ) -> Result<(), PartsAssociatedSourceErrorV1> {
        if std::ptr::eq(self.arena, block.arena) {
            Ok(())
        } else {
            Err(PartsAssociatedSourceErrorV1::ForeignRawBlock)
        }
    }

    /// Resolve one `StmtRef` through the located carrier, then prove the
    /// recipe body entry is the verbatim clone of the located statement.
    fn statement(
        &self,
        block: &CallableLoopSourcePartsBlockV1<'view>,
        reference: StmtRef,
    ) -> Result<CallableLoopSourceStmtInputV1<'view>, PartsAssociatedSourceErrorV1> {
        self.require_own_block(block)?;
        let recipe_stmt = block
            .recipe_body()?
            .get_ref(reference)
            .ok_or(PartsAssociatedSourceErrorV1::MissingRecipeStatement {
                index: reference.index(),
            })?;
        let projected = match &block.body {
            CallableLoopSourcePartsBlockBodyV1::Singleton(stmt) => {
                if reference.index() != 0 {
                    return Err(PartsAssociatedSourceErrorV1::MissingRecipeStatement {
                        index: reference.index(),
                    });
                }
                stmt.clone()
            }
            CallableLoopSourcePartsBlockBodyV1::Located(body) => self
                .port
                .body_stmt(body, reference.index())
                .map_err(|error| {
                    PartsAssociatedSourceErrorV1::SourcePortProjection(error.render())
                })?,
        };
        if self.port.stmt_syntax(&projected) != recipe_stmt {
            return Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch);
        }
        Ok(projected)
    }

    /// The issued `CondBlockView` is the condition authority; the projected
    /// expression must carry the same prelude/tail shape.
    fn require_condition_view(
        &self,
        cond_view: &CondBlockView,
        syntax: &ASTNode,
    ) -> Result<(), PartsAssociatedSourceErrorV1> {
        let matches = if cond_view.prelude_stmts.is_empty() {
            cond_view.tail_expr == *syntax
        } else {
            matches!(
                syntax,
                ASTNode::BlockExpr {
                    prelude_stmts,
                    tail_expr,
                    ..
                } if *prelude_stmts == cond_view.prelude_stmts
                    && **tail_expr == cond_view.tail_expr
            )
        };
        if matches {
            Ok(())
        } else {
            Err(PartsAssociatedSourceErrorV1::ConditionViewMismatch)
        }
    }

    fn child_expr(
        &self,
        parent: &CallableLoopSourceStmtInputV1<'view>,
        role: ExprChildRoleV1,
    ) -> Result<CallableLoopSourceExprInputV1<'view>, PartsAssociatedSourceErrorV1> {
        self.port
            .child_expr_from_stmt(parent, role)
            .map_err(|error| PartsAssociatedSourceErrorV1::SourcePortProjection(error.render()))
    }

    fn child_body(
        &self,
        parent: &CallableLoopSourceStmtInputV1<'view>,
        role: BodyChildRoleV1,
    ) -> Result<CallableLoopSourceBodyInputV1<'view>, PartsAssociatedSourceErrorV1> {
        self.port
            .child_body_from_stmt(parent, role)
            .map_err(|error| PartsAssociatedSourceErrorV1::SourcePortProjection(error.render()))
    }

    fn child_block(
        &self,
        block: &'view RecipeBlock,
        parent: &CallableLoopSourceStmtInputV1<'view>,
        role: BodyChildRoleV1,
    ) -> Result<CallableLoopSourcePartsBlockV1<'view>, PartsAssociatedSourceErrorV1> {
        let body = self.child_body(parent, role)?;
        CallableLoopSourcePartsBlockV1::located_body(self.arena, block, body, &self.port)
    }
}

impl sealed::Sealed for CallableLoopSourcePartsAssociatedSourceV1<'_, '_> {}

impl<'view, 'ledger: 'view> PartsAssociatedSourceV1
    for CallableLoopSourcePartsAssociatedSourceV1<'view, 'ledger>
{
    type PortHandle = CallableLoopSourceExpressionPortV1<'ledger>;
    type BlockInput = CallableLoopSourcePartsBlockV1<'view>;
    type StmtInput = CallableLoopSourceStmtInputV1<'view>;
    type ConditionInput = CallableLoopSourceExprInputV1<'view>;
    type BodyInput = CallableLoopSourceBodyInputV1<'view>;
    type WrappedJoinInput = Infallible;
    type LoopInput = CallableLoopSourcePartsLoopV0V1<'view>;

    fn block_len(&self, block: &Self::BlockInput) -> Result<usize, PartsAssociatedSourceErrorV1> {
        self.require_own_block(block)?;
        Ok(block.block.items.len())
    }

    fn item(
        &self,
        block: &Self::BlockInput,
        index: usize,
    ) -> Result<
        VerifiedPartsAssociatedItemV1<
            Self::PortHandle,
            Self::StmtInput,
            Self::ConditionInput,
            Self::BodyInput,
            Self::BlockInput,
            Self::WrappedJoinInput,
            Self::LoopInput,
        >,
        PartsAssociatedSourceErrorV1,
    > {
        self.require_own_block(block)?;
        let item = block.block.items.get(index).ok_or(
            PartsAssociatedSourceErrorV1::ItemIndexOutOfBounds {
                index,
                len: block.block.items.len(),
            },
        )?;
        let item = match item {
            RecipeItem::Stmt(reference) => PartsAssociatedRecipeItemV1::OpaqueStmt {
                source: self.statement(block, *reference)?,
            },
            RecipeItem::Exit { kind, stmt } => PartsAssociatedRecipeItemV1::OpaqueExit {
                source: self.statement(block, *stmt)?,
                kind: *kind,
            },
            RecipeItem::IfV2 {
                if_stmt,
                cond_view,
                contract,
                then_block,
                else_block,
            } => {
                let source = self.statement(block, *if_stmt)?;
                let condition = self.child_expr(&source, ExprChildRoleV1::IfCondition)?;
                self.require_condition_view(cond_view, self.port.expr_syntax(&condition))?;
                let then_body = self.child_body(&source, BodyChildRoleV1::IfThen)?;
                let ASTNode::If {
                    else_body: ast_else_body,
                    ..
                } = self.port.stmt_syntax(&source)
                else {
                    return Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch);
                };
                if else_block.is_some() != ast_else_body.is_some() {
                    return Err(PartsAssociatedSourceErrorV1::RecipeBodyMismatch);
                }
                let else_body = else_block
                    .as_ref()
                    .map(|_| self.child_body(&source, BodyChildRoleV1::IfElse))
                    .transpose()?;
                let then_block = self.child_block(then_block, &source, BodyChildRoleV1::IfThen)?;
                let else_block = else_block
                    .as_deref()
                    .map(|block| self.child_block(block, &source, BodyChildRoleV1::IfElse))
                    .transpose()?;
                PartsAssociatedRecipeItemV1::ExplicitIfV2 {
                    source,
                    condition,
                    then_body,
                    else_body,
                    contract: *contract,
                    then_block,
                    else_block,
                }
            }
            RecipeItem::LoopV0 {
                loop_stmt,
                kind,
                cond_view,
                body_block,
                body_contract,
                features,
            } => {
                let source = self.statement(block, *loop_stmt)?;
                let condition = self.child_expr(&source, ExprChildRoleV1::LoopCondition)?;
                self.require_condition_view(cond_view, self.port.expr_syntax(&condition))?;
                let body_block = self.child_block(body_block, &source, BodyChildRoleV1::LoopBody)?;
                PartsAssociatedRecipeItemV1::RawLoopV0 {
                    loop_input: CallableLoopSourcePartsLoopV0V1 {
                        source,
                        kind: *kind,
                        condition,
                        body_block,
                        body_contract: *body_contract,
                        features: *features,
                    },
                }
            }
        };
        Ok(VerifiedPartsAssociatedItemV1::new(self.port, item))
    }
}
