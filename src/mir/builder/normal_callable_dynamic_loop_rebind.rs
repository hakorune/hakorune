//! Exact-once physical terminal for the first source-backed Dynamic Loop.
//!
//! This owner consumes the whole prepared ingress. It delegates instruction
//! insertion to the existing Builder/Compare/Const writers, commits one
//! atomic callable-current transition, and returns a move-only handoff for the
//! canonical Binding SSA / PHI transaction owner. It is not a route or a
//! PHI writer.

use crate::mir::builder::emission::{compare, constant};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1, SourceNodeSiteV1,
};
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, MirBuilder, MirInstruction, MirType, ValueId};

use super::normal_callable_dynamic_loop_prepare::{
    PreparedLoopIncomingRoleV1, PreparedSourceBackedDynamicLoopIngressV1,
};
use super::normal_callable_dynamic_operation_source::{
    DynamicLoopComparisonKindV1, DynamicLoopOperationResultClassV1,
};
use super::normal_callable_dynamic_origin::CurrentDynamicBindingReceiptV1;
use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CompletedDynamicLoopCompareBoolReceiptV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    operation: SourceExprSiteV1,
    block: BasicBlockId,
    lhs: ValueId,
    rhs: ValueId,
    result: ValueId,
}

impl CompletedDynamicLoopCompareBoolReceiptV1 {
    pub(super) const fn result(&self) -> ValueId {
        self.result
    }

    pub(super) const fn block(&self) -> BasicBlockId {
        self.block
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct ReadySourceBackedDynamicLoopCarrierForPhiV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    binding: BindingRefV1,
    origin: BindingRefV1,
    enter: ValueId,
    backedge: ValueId,
    assignment: SourceExprSiteV1,
    definition_block: BasicBlockId,
    expected_roles: [PreparedLoopIncomingRoleV1; 2],
    current: CurrentDynamicBindingReceiptV1,
}

impl ReadySourceBackedDynamicLoopCarrierForPhiV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn origin(&self) -> BindingRefV1 {
        self.origin
    }

    pub(super) const fn enter(&self) -> ValueId {
        self.enter
    }

    pub(super) const fn backedge(&self) -> ValueId {
        self.backedge
    }

    pub(super) const fn definition_block(&self) -> BasicBlockId {
        self.definition_block
    }

    pub(super) const fn expected_roles(&self) -> [PreparedLoopIncomingRoleV1; 2] {
        self.expected_roles
    }

    pub(super) const fn assignment(&self) -> &SourceExprSiteV1 {
        &self.assignment
    }

    pub(super) const fn current(&self) -> &CurrentDynamicBindingReceiptV1 {
        &self.current
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CompletedSourceBackedDynamicLoopOperationsV1 {
    predicate: CompletedDynamicLoopCompareBoolReceiptV1,
    carrier: ReadySourceBackedDynamicLoopCarrierForPhiV1,
}

impl CompletedSourceBackedDynamicLoopOperationsV1 {
    pub(super) const fn predicate(&self) -> &CompletedDynamicLoopCompareBoolReceiptV1 {
        &self.predicate
    }

    pub(super) const fn carrier(&self) -> &ReadySourceBackedDynamicLoopCarrierForPhiV1 {
        &self.carrier
    }
}

pub(super) struct DynamicLoopOperationExecutionV1;

impl DynamicLoopOperationExecutionV1 {
    pub(super) fn execute(
        ingress: PreparedSourceBackedDynamicLoopIngressV1,
        state: &mut CallableSemanticLoweringState,
        builder: &mut MirBuilder,
        comparison_block: BasicBlockId,
        add_block: BasicBlockId,
    ) -> Result<CompletedSourceBackedDynamicLoopOperationsV1, String> {
        require_block(builder, comparison_block)?;
        require_block(builder, add_block)?;

        let comparison = ingress.operations().comparison();
        if comparison.kind() != DynamicLoopComparisonKindV1::Less
            || comparison.result() != DynamicLoopOperationResultClassV1::Bool
        {
            return Err(freeze("comparison-contract"));
        }
        let carrier = ingress.carrier();
        let origin = carrier
            .representation()
            .dynamic_origin()
            .ok_or_else(|| freeze("carrier-not-dynamic"))?;
        let comparison_carrier = ingress
            .entry_binding(comparison.carrier())
            .ok_or_else(|| freeze("comparison-carrier-entry"))?;
        let comparison_operand = ingress
            .entry_binding(comparison.operand())
            .ok_or_else(|| freeze("comparison-operand-entry"))?;
        if comparison_carrier.current() != carrier.entry()
            || comparison_carrier.representation().dynamic_origin() != Some(origin)
            || comparison_operand
                .representation()
                .dynamic_origin()
                .is_none()
        {
            return Err(freeze("comparison-lineage"));
        }

        let add = ingress.operations().add_rebind();
        if add.carrier() != carrier.binding()
            || add.result() != DynamicLoopOperationResultClassV1::Dynamic
        {
            return Err(freeze("add-contract"));
        }
        let add_result = builder.next_value_id();
        let prepared_rebind = state.prepare_source_backed_dynamic_rebind(
            add.target().node(),
            carrier.binding(),
            carrier.entry(),
            add_result,
            origin,
        )?;

        let predicate_result = builder.next_value_id();
        compare::emit_to_at(
            builder,
            comparison_block,
            predicate_result,
            CompareOp::Lt,
            comparison_carrier.current(),
            comparison_operand.current(),
        )?;
        let predicate = CompletedDynamicLoopCompareBoolReceiptV1 {
            owner: ingress.owner(),
            loop_site: ingress.loop_site().clone(),
            operation: comparison.operation().clone(),
            block: comparison_block,
            lhs: comparison_carrier.current(),
            rhs: comparison_operand.current(),
            result: predicate_result,
        };

        let delta = constant::emit_integer_at(builder, add_block, add.delta())?;
        builder.emit_instruction_at(
            add_block,
            MirInstruction::BinOp {
                dst: add_result,
                op: BinaryOp::Add,
                lhs: carrier.entry(),
                rhs: delta,
            },
        )?;
        if builder.function_state.type_ctx.get_type(add_result) == Some(&MirType::Integer) {
            return Err(freeze("dynamic-add-published-integer"));
        }
        let current = state.commit_source_backed_dynamic_rebind(prepared_rebind);
        let carrier = ReadySourceBackedDynamicLoopCarrierForPhiV1 {
            owner: ingress.owner(),
            loop_site: ingress.loop_site().clone(),
            binding: carrier.binding(),
            origin,
            enter: carrier.entry(),
            backedge: add_result,
            assignment: add.target().clone(),
            definition_block: add_block,
            expected_roles: carrier.expected_roles(),
            current,
        };
        Ok(CompletedSourceBackedDynamicLoopOperationsV1 { predicate, carrier })
    }
}

fn require_block(builder: &MirBuilder, block: BasicBlockId) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("current-function"))?;
    if function.get_block(block).is_none() {
        return Err(freeze("physical-block"));
    }
    Ok(())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][dynamic-loop-rebind/{reason}]")
}

#[cfg(test)]
#[path = "normal_callable_dynamic_loop_rebind_tests.rs"]
mod tests;
