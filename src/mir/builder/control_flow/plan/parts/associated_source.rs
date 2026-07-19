//! Behavior-neutral recipe/source association vocabulary for Parts.
//!
//! Providers in this module project either the existing raw recipe arena or
//! the O0 located lowering view into one neutral item shape. They do not lower
//! statements, select new recipe semantics, mutate a Builder, or rebuild
//! recipes.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::expression_port::{
    LocatedLoopPlanBodyInputV1, LocatedLoopPlanExprInputV1, LocatedLoopPlanExpressionPortV1,
    LocatedLoopPlanStmtInputV1, RawLoopPlanExpressionPortV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::{
    VerifiedLocatedRecipeBlockLoweringViewV1, VerifiedLocatedRecipeItemLoweringViewV1,
    VerifiedStmtWrappedJoinIfLoweringViewV1,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, ExitKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBlock,
    RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::RecipeBody;
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum PartsAssociatedSourceErrorV1 {
    ItemIndexOutOfBounds { index: usize, len: usize },
    MissingRecipeBody,
    MissingRecipeStatement { index: usize },
    ForeignRawBlock,
    ForeignLocatedBlock,
}

mod sealed {
    pub trait Sealed {}
}

pub(super) mod block_driver;
pub(in crate::mir::builder) mod direct_if;
pub(super) mod dispatch;
#[cfg(test)]
mod dispatch_tests;
#[cfg(test)]
mod located_hook_tests;
pub(super) mod located_lowering;
#[cfg(test)]
mod located_parity_tests;
pub(super) mod located_preflight;
pub(super) mod raw_lowering;
#[cfg(test)]
mod raw_parity_tests;

/// One representation-neutral item vocabulary.
///
/// The type parameters are source carriers only. This enum intentionally owns
/// no lowering policy or Builder state.
pub(in crate::mir::builder) enum PartsAssociatedRecipeItemV1<
    StmtInput,
    ConditionInput,
    BodyInput,
    BlockInput,
    WrappedJoinInput,
    LoopInput,
> {
    OpaqueStmt {
        source: StmtInput,
    },
    OpaqueExit {
        source: StmtInput,
        kind: ExitKind,
    },
    ExplicitIfV2 {
        source: StmtInput,
        condition: ConditionInput,
        then_body: BodyInput,
        else_body: Option<BodyInput>,
        contract: IfContractKind,
        then_block: BlockInput,
        else_block: Option<BlockInput>,
    },
    StmtWrappedJoinIf {
        bridge: WrappedJoinInput,
    },
    RawLoopV0 {
        loop_input: LoopInput,
    },
}

/// One item and the exact expression port that issued its source carriers.
///
/// Construction stays private so later Parts lowering cannot pair a located
/// carrier with a different callable-result source view.
pub(in crate::mir::builder) struct VerifiedPartsAssociatedItemV1<
    PortHandle,
    StmtInput,
    ConditionInput,
    BodyInput,
    BlockInput,
    WrappedJoinInput,
    LoopInput,
> {
    port: PortHandle,
    item: PartsAssociatedRecipeItemV1<
        StmtInput,
        ConditionInput,
        BodyInput,
        BlockInput,
        WrappedJoinInput,
        LoopInput,
    >,
}

impl<PortHandle, StmtInput, ConditionInput, BodyInput, BlockInput, WrappedJoinInput, LoopInput>
    VerifiedPartsAssociatedItemV1<
        PortHandle,
        StmtInput,
        ConditionInput,
        BodyInput,
        BlockInput,
        WrappedJoinInput,
        LoopInput,
    >
{
    fn new(
        port: PortHandle,
        item: PartsAssociatedRecipeItemV1<
            StmtInput,
            ConditionInput,
            BodyInput,
            BlockInput,
            WrappedJoinInput,
            LoopInput,
        >,
    ) -> Self {
        Self { port, item }
    }

    #[cfg(test)]
    pub(super) fn test_parts(
        self,
    ) -> (
        PortHandle,
        PartsAssociatedRecipeItemV1<
            StmtInput,
            ConditionInput,
            BodyInput,
            BlockInput,
            WrappedJoinInput,
            LoopInput,
        >,
    ) {
        (self.port, self.item)
    }
}

/// Closed provider contract consumed by the later Parts dispatcher cutover.
pub(super) trait PartsAssociatedSourceV1: sealed::Sealed {
    type PortHandle;
    type BlockInput;
    type StmtInput;
    type ConditionInput;
    type BodyInput;
    type WrappedJoinInput;
    type LoopInput;

    fn block_len(&self, block: &Self::BlockInput) -> Result<usize, PartsAssociatedSourceErrorV1>;

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
    >;
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::builder) struct RawPartsAssociatedBlockV1<'source> {
    arena: &'source RecipeBodies,
    block: &'source RecipeBlock,
}

impl<'source> RawPartsAssociatedBlockV1<'source> {
    const fn new(arena: &'source RecipeBodies, block: &'source RecipeBlock) -> Self {
        Self { arena, block }
    }

    pub(super) const fn recipe_block(&self) -> &'source RecipeBlock {
        self.block
    }
}

pub(in crate::mir::builder) struct RawPartsAssociatedLoopV0V1<'source> {
    pub(in crate::mir::builder) source: &'source ASTNode,
    pub(in crate::mir::builder) kind: LoopKindV0,
    pub(in crate::mir::builder) condition: &'source CondBlockView,
    pub(in crate::mir::builder) body_block: RawPartsAssociatedBlockV1<'source>,
    pub(in crate::mir::builder) body_contract: BlockContractKind,
    pub(in crate::mir::builder) features: LoopV0Features,
}

pub(in crate::mir::builder) struct RawPartsAssociatedSourceV1<'source> {
    arena: &'source RecipeBodies,
}

impl<'source> RawPartsAssociatedSourceV1<'source> {
    pub(in crate::mir::builder) const fn new(arena: &'source RecipeBodies) -> Self {
        Self { arena }
    }

    pub(in crate::mir::builder) const fn root(
        &self,
        block: &'source RecipeBlock,
    ) -> RawPartsAssociatedBlockV1<'source> {
        RawPartsAssociatedBlockV1::new(self.arena, block)
    }

    fn require_own_block(
        &self,
        block: &RawPartsAssociatedBlockV1<'source>,
    ) -> Result<(), PartsAssociatedSourceErrorV1> {
        if std::ptr::eq(self.arena, block.arena) {
            Ok(())
        } else {
            Err(PartsAssociatedSourceErrorV1::ForeignRawBlock)
        }
    }

    fn body(
        &self,
        block: &RawPartsAssociatedBlockV1<'source>,
    ) -> Result<&'source RecipeBody, PartsAssociatedSourceErrorV1> {
        self.require_own_block(block)?;
        self.arena
            .get(block.block.body_id)
            .ok_or(PartsAssociatedSourceErrorV1::MissingRecipeBody)
    }

    fn statement(
        &self,
        block: &RawPartsAssociatedBlockV1<'source>,
        reference: crate::mir::builder::control_flow::recipes::refs::StmtRef,
    ) -> Result<&'source ASTNode, PartsAssociatedSourceErrorV1> {
        self.body(block)?.get_ref(reference).ok_or(
            PartsAssociatedSourceErrorV1::MissingRecipeStatement {
                index: reference.index(),
            },
        )
    }

    fn block_body(
        &self,
        block: &RawPartsAssociatedBlockV1<'source>,
    ) -> Result<&'source [ASTNode], PartsAssociatedSourceErrorV1> {
        Ok(self.body(block)?.as_ref())
    }
}

impl sealed::Sealed for RawPartsAssociatedSourceV1<'_> {}

impl<'source> PartsAssociatedSourceV1 for RawPartsAssociatedSourceV1<'source> {
    type PortHandle = RawLoopPlanExpressionPortV1;
    type BlockInput = RawPartsAssociatedBlockV1<'source>;
    type StmtInput = &'source ASTNode;
    type ConditionInput = &'source CondBlockView;
    type BodyInput = &'source [ASTNode];
    type WrappedJoinInput = Infallible;
    type LoopInput = RawPartsAssociatedLoopV0V1<'source>;

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
                let then_block = RawPartsAssociatedBlockV1::new(self.arena, then_block);
                let else_block = else_block
                    .as_deref()
                    .map(|block| RawPartsAssociatedBlockV1::new(self.arena, block));
                PartsAssociatedRecipeItemV1::ExplicitIfV2 {
                    source: self.statement(block, *if_stmt)?,
                    condition: cond_view,
                    then_body: self.block_body(&then_block)?,
                    else_body: else_block
                        .as_ref()
                        .map(|block| self.block_body(block))
                        .transpose()?,
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
            } => PartsAssociatedRecipeItemV1::RawLoopV0 {
                loop_input: RawPartsAssociatedLoopV0V1 {
                    source: self.statement(block, *loop_stmt)?,
                    kind: *kind,
                    condition: cond_view,
                    body_block: RawPartsAssociatedBlockV1::new(self.arena, body_block),
                    body_contract: *body_contract,
                    features: *features,
                },
            },
        };
        Ok(VerifiedPartsAssociatedItemV1::new(
            RawLoopPlanExpressionPortV1::new(),
            item,
        ))
    }
}

pub(in crate::mir::builder) struct LocatedPartsAssociatedSourceV1<'view, 'plan> {
    port: &'view LocatedLoopPlanExpressionPortV1<'plan>,
}

impl<'view, 'plan> LocatedPartsAssociatedSourceV1<'view, 'plan> {
    pub(in crate::mir::builder) fn new(
        root: &VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    ) -> Self {
        Self {
            port: root.expression_port(),
        }
    }

    fn require_own_block(
        &self,
        block: &VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>,
    ) -> Result<(), PartsAssociatedSourceErrorV1> {
        if std::ptr::eq(self.port, block.expression_port()) {
            Ok(())
        } else {
            Err(PartsAssociatedSourceErrorV1::ForeignLocatedBlock)
        }
    }
}

impl sealed::Sealed for LocatedPartsAssociatedSourceV1<'_, '_> {}

impl<'view, 'plan: 'view> PartsAssociatedSourceV1 for LocatedPartsAssociatedSourceV1<'view, 'plan> {
    type PortHandle = &'view LocatedLoopPlanExpressionPortV1<'plan>;
    type BlockInput = VerifiedLocatedRecipeBlockLoweringViewV1<'view, 'plan>;
    type StmtInput = LocatedLoopPlanStmtInputV1<'plan, 'view>;
    type ConditionInput = LocatedLoopPlanExprInputV1<'plan, 'view>;
    type BodyInput = LocatedLoopPlanBodyInputV1<'plan, 'view>;
    type WrappedJoinInput = VerifiedStmtWrappedJoinIfLoweringViewV1<'view, 'plan>;
    type LoopInput = Infallible;

    fn block_len(&self, block: &Self::BlockInput) -> Result<usize, PartsAssociatedSourceErrorV1> {
        self.require_own_block(block)?;
        Ok(block.len())
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
        let item = block
            .item(index)
            .ok_or(PartsAssociatedSourceErrorV1::ItemIndexOutOfBounds {
                index,
                len: block.len(),
            })?;
        let item = match item {
            VerifiedLocatedRecipeItemLoweringViewV1::OpaqueStmt { source } => {
                PartsAssociatedRecipeItemV1::OpaqueStmt { source }
            }
            VerifiedLocatedRecipeItemLoweringViewV1::OpaqueExit { source, kind } => {
                PartsAssociatedRecipeItemV1::OpaqueExit { source, kind }
            }
            VerifiedLocatedRecipeItemLoweringViewV1::ExplicitIfV2 {
                source,
                condition,
                then_body,
                else_body,
                contract,
                then_block,
                else_block,
            } => PartsAssociatedRecipeItemV1::ExplicitIfV2 {
                source,
                condition,
                then_body,
                else_body,
                contract,
                then_block,
                else_block,
            },
            VerifiedLocatedRecipeItemLoweringViewV1::StmtWrappedJoinIf { bridge } => {
                PartsAssociatedRecipeItemV1::StmtWrappedJoinIf { bridge }
            }
        };
        Ok(VerifiedPartsAssociatedItemV1::new(self.port, item))
    }
}
