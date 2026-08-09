//! Canonical Header-current opener for the source-backed Dynamic Loop canary.
//!
//! This migration-private owner consumes the exact prepared ingress, creates
//! one bounded physical placement through the canonical CFG session, adopts
//! the already materialized local value, and asks canonical Binding SSA for
//! the unsealed Header current. It never exposes a PHI token or predecessor
//! list and retires when common Recipe physical layout owns this caller.

use crate::mir::builder::normal_callable_dynamic_loop_prepare::PreparedSourceBackedDynamicLoopIngressV1;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, ValueId};

use super::canonical_ssa::{CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicLoopPhysicalPlacementV1 {
    enter: BasicBlockId,
    header: BasicBlockId,
    body_path: BasicBlockId,
    terminal_backedge: BasicBlockId,
    after: BasicBlockId,
}

impl DynamicLoopPhysicalPlacementV1 {
    pub(super) const fn enter(self) -> BasicBlockId {
        self.enter
    }

    pub(super) const fn header(self) -> BasicBlockId {
        self.header
    }

    pub(super) const fn body_path(self) -> BasicBlockId {
        self.body_path
    }

    pub(super) const fn terminal_backedge(self) -> BasicBlockId {
        self.terminal_backedge
    }

    pub(super) const fn after(self) -> BasicBlockId {
        self.after
    }
}

/// Move-only proof that canonical Binding SSA owns the Dynamic Header current.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct OpenSourceBackedDynamicLoopCarrierPhiV1 {
    ingress: PreparedSourceBackedDynamicLoopIngressV1,
    placement: DynamicLoopPhysicalPlacementV1,
    header_current: CanonicalBindingReadReceiptV1,
}

impl OpenSourceBackedDynamicLoopCarrierPhiV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.ingress.owner()
    }

    pub(super) const fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.ingress.loop_site()
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.header_current.binding()
    }

    pub(super) const fn entry(&self) -> ValueId {
        self.ingress.carrier().entry()
    }

    pub(super) const fn header_current(&self) -> CanonicalBindingReadReceiptV1 {
        self.header_current
    }

    pub(super) const fn placement(&self) -> DynamicLoopPhysicalPlacementV1 {
        self.placement
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        PreparedSourceBackedDynamicLoopIngressV1,
        DynamicLoopPhysicalPlacementV1,
        CanonicalBindingReadReceiptV1,
    ) {
        (self.ingress, self.placement, self.header_current)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DynamicLoopPhiOpenIssueV1 {
    OwnerMismatch,
    CarrierRelationMismatch,
    MissingCurrentFunction,
    MissingCurrentBlock,
    EnterAlreadyTerminated,
    EnterAlreadySealed,
    EntryDefinitionMissing,
    EntryDefinitionDuplicate,
    EntryDefinitionBlockMismatch,
    CanonicalIdentity(String),
    CanonicalCfg(String),
    HeaderRead(String),
}

impl CanonicalSsaFunctionSessionV2<'_> {
    pub(super) fn open_source_backed_dynamic_loop_header(
        &mut self,
        builder: &mut MirBuilder,
        ingress: PreparedSourceBackedDynamicLoopIngressV1,
    ) -> Result<OpenSourceBackedDynamicLoopCarrierPhiV1, DynamicLoopPhiOpenIssueV1> {
        if ingress.owner() != self.owner() {
            return Err(DynamicLoopPhiOpenIssueV1::OwnerMismatch);
        }
        let enter_definition = ingress.enter_definition();
        let carrier = ingress.carrier();
        let Some(origin) = carrier.representation().dynamic_origin() else {
            return Err(DynamicLoopPhiOpenIssueV1::CarrierRelationMismatch);
        };
        if enter_definition.binding() != carrier.binding()
            || enter_definition.entry() != carrier.entry()
            || enter_definition.origin() != origin
        {
            return Err(DynamicLoopPhiOpenIssueV1::CarrierRelationMismatch);
        }

        let enter = builder
            .function_state
            .current_block
            .ok_or(DynamicLoopPhiOpenIssueV1::MissingCurrentBlock)?;
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(DynamicLoopPhiOpenIssueV1::MissingCurrentFunction)?;
        let enter_block = function
            .get_block(enter)
            .ok_or(DynamicLoopPhiOpenIssueV1::MissingCurrentBlock)?;
        if enter_block.terminator.is_some() {
            return Err(DynamicLoopPhiOpenIssueV1::EnterAlreadyTerminated);
        }
        if enter_block.is_sealed() {
            return Err(DynamicLoopPhiOpenIssueV1::EnterAlreadySealed);
        }

        let definition_blocks = function
            .block_ids()
            .into_iter()
            .flat_map(|block| {
                function.get_block(block).into_iter().flat_map(move |row| {
                    row.instructions
                        .iter()
                        .filter(move |instruction| {
                            instruction.dst_value() == Some(enter_definition.entry())
                        })
                        .map(move |_| block)
                })
            })
            .collect::<Vec<_>>();
        let [definition_block] = definition_blocks.as_slice() else {
            return Err(if definition_blocks.is_empty() {
                DynamicLoopPhiOpenIssueV1::EntryDefinitionMissing
            } else {
                DynamicLoopPhiOpenIssueV1::EntryDefinitionDuplicate
            });
        };
        if *definition_block != enter {
            return Err(DynamicLoopPhiOpenIssueV1::EntryDefinitionBlockMismatch);
        }

        self.identity
            .publish_declaration_exact(
                enter_definition.declaration(),
                enter_definition.binding(),
                enter,
                enter_definition.entry(),
            )
            .map_err(DynamicLoopPhiOpenIssueV1::CanonicalIdentity)?;

        let header = builder.next_block_id();
        let body_path = builder.next_block_id();
        let terminal_backedge = builder.next_block_id();
        let after = builder.next_block_id();
        {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or(DynamicLoopPhiOpenIssueV1::MissingCurrentFunction)?;
            for block in [header, body_path, terminal_backedge, after] {
                self.cfg
                    .create_block(function, block)
                    .map_err(|error| DynamicLoopPhiOpenIssueV1::CanonicalCfg(error.to_string()))?;
            }
            self.cfg
                .emit_jump(function, enter, header)
                .map_err(|error| DynamicLoopPhiOpenIssueV1::CanonicalCfg(error.to_string()))?;
        }
        self.cfg
            .select_block(builder, header)
            .map_err(|error| DynamicLoopPhiOpenIssueV1::CanonicalCfg(error.to_string()))?;
        let header_current = self
            .identity
            .read_entry_receipt(builder, &mut self.phis, header, carrier.binding())
            .map_err(DynamicLoopPhiOpenIssueV1::HeaderRead)?;
        let placement = DynamicLoopPhysicalPlacementV1 {
            enter,
            header,
            body_path,
            terminal_backedge,
            after,
        };
        Ok(OpenSourceBackedDynamicLoopCarrierPhiV1 {
            ingress,
            placement,
            header_current,
        })
    }
}
