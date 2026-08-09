//! Canonical Header close for the first source-backed Dynamic Loop.
//!
//! This owner consumes the exact operation receipts, defines the source
//! assignment through canonical Binding SSA, completes the real CFG path,
//! and asks the existing Binding SSA / PhiTxn owners to close the already
//! provisional Header current. It never constructs or patches a PHI itself.

use crate::mir::builder::normal_callable_dynamic_loop_prepare::PreparedLoopIncomingRoleV1;
use crate::mir::builder::normal_callable_dynamic_loop_rebind::CompletedSourceBackedDynamicLoopOperationsV1;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, ValueId};

use super::canonical_ssa::CanonicalSsaFunctionSessionV2;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ClosedSourceBackedDynamicLoopCarrierPhiV1 {
    operations: CompletedSourceBackedDynamicLoopOperationsV1,
}

impl ClosedSourceBackedDynamicLoopCarrierPhiV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.operations.carrier().opened().owner()
    }

    pub(super) const fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.operations.carrier().opened().loop_site()
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.operations.carrier().binding()
    }

    pub(super) const fn origin(&self) -> BindingRefV1 {
        self.operations.carrier().origin()
    }

    pub(super) fn placement(&self) -> super::dynamic_loop_phi::DynamicLoopPhysicalPlacementV1 {
        self.operations.carrier().opened().placement()
    }

    pub(super) const fn entry(&self) -> ValueId {
        self.operations.carrier().enter()
    }

    pub(super) const fn header_current(&self) -> ValueId {
        self.operations.carrier().header_current()
    }

    pub(super) const fn backedge(&self) -> ValueId {
        self.operations.carrier().backedge()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DynamicLoopPhiCloseIssueV1 {
    OwnerMismatch,
    SourceRelationMismatch,
    PhysicalRelationMismatch,
    IncomingRoleMismatch,
    MissingCurrentFunction,
    CanonicalIdentity(String),
    CanonicalCfg(String),
    HeaderPredecessorMismatch,
    CanonicalIdentitySeal(String),
}

impl CanonicalSsaFunctionSessionV2<'_> {
    pub(super) fn close_source_backed_dynamic_loop_header(
        &mut self,
        builder: &mut MirBuilder,
        operations: CompletedSourceBackedDynamicLoopOperationsV1,
    ) -> Result<ClosedSourceBackedDynamicLoopCarrierPhiV1, DynamicLoopPhiCloseIssueV1> {
        let predicate = operations.predicate();
        let carrier = operations.carrier();
        let opened = carrier.opened();
        let placement = opened.placement();

        if opened.owner() != self.owner() || predicate.owner() != self.owner() {
            return Err(DynamicLoopPhiCloseIssueV1::OwnerMismatch);
        }
        if predicate.loop_site() != opened.loop_site()
            || carrier.binding() != opened.binding()
            || carrier.origin()
                != opened
                    .ingress()
                    .carrier()
                    .representation()
                    .dynamic_origin()
                    .ok_or(DynamicLoopPhiCloseIssueV1::SourceRelationMismatch)?
        {
            return Err(DynamicLoopPhiCloseIssueV1::SourceRelationMismatch);
        }
        if predicate.block() != placement.header()
            || predicate.lhs() != opened.header_current_value()
            || carrier.header() != placement.header()
            || carrier.header_current() != opened.header_current_value()
            || !roles_are_distinct(placement)
        {
            return Err(DynamicLoopPhiCloseIssueV1::PhysicalRelationMismatch);
        }
        if carrier.expected_roles()
            != [
                PreparedLoopIncomingRoleV1::Enter,
                PreparedLoopIncomingRoleV1::Backedge,
            ]
        {
            return Err(DynamicLoopPhiCloseIssueV1::IncomingRoleMismatch);
        }

        self.identity
            .define_assignment_exact(
                carrier.assignment(),
                carrier.binding(),
                carrier.definition_block(),
                carrier.backedge(),
            )
            .map_err(DynamicLoopPhiCloseIssueV1::CanonicalIdentity)?;

        let (enter_witness, terminal_witness, header_witness) = {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or(DynamicLoopPhiCloseIssueV1::MissingCurrentFunction)?;
            self.cfg
                .emit_branch(
                    function,
                    placement.header(),
                    predicate.result(),
                    placement.body_path(),
                    placement.after(),
                )
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            self.cfg
                .emit_jump(
                    function,
                    placement.body_path(),
                    placement.terminal_backedge(),
                )
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            self.cfg
                .emit_jump(function, placement.terminal_backedge(), placement.header())
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            let enter_witness = self
                .cfg
                .seal_block(function, placement.enter())
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            self.cfg
                .seal_block(function, placement.body_path())
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            let terminal_witness = self
                .cfg
                .seal_block(function, placement.terminal_backedge())
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            let header_witness = self
                .cfg
                .seal_block(function, placement.header())
                .map_err(|error| DynamicLoopPhiCloseIssueV1::CanonicalCfg(error.to_string()))?;
            (enter_witness, terminal_witness, header_witness)
        };
        if header_witness.block() != placement.header()
            || header_witness.predecessors().len() != 2
            || !header_witness.predecessors().contains(&placement.enter())
            || !header_witness
                .predecessors()
                .contains(&placement.terminal_backedge())
        {
            return Err(DynamicLoopPhiCloseIssueV1::HeaderPredecessorMismatch);
        }
        self.identity
            .seal_block(builder, &mut self.phis, placement.enter(), &enter_witness)
            .map_err(DynamicLoopPhiCloseIssueV1::CanonicalIdentitySeal)?;
        self.identity
            .seal_block(
                builder,
                &mut self.phis,
                placement.terminal_backedge(),
                &terminal_witness,
            )
            .map_err(DynamicLoopPhiCloseIssueV1::CanonicalIdentitySeal)?;
        self.identity
            .seal_block(builder, &mut self.phis, placement.header(), &header_witness)
            .map_err(DynamicLoopPhiCloseIssueV1::CanonicalIdentitySeal)?;

        Ok(ClosedSourceBackedDynamicLoopCarrierPhiV1 { operations })
    }
}

fn roles_are_distinct(placement: super::dynamic_loop_phi::DynamicLoopPhysicalPlacementV1) -> bool {
    let roles = [
        placement.enter(),
        placement.header(),
        placement.body_path(),
        placement.terminal_backedge(),
        placement.after(),
    ];
    roles
        .iter()
        .enumerate()
        .all(|(index, block)| !roles[..index].contains(block))
}
