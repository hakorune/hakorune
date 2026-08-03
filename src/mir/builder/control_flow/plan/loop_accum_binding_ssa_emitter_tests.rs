//! Shared caller-zero DirectAccum emitter used by owned and borrowed fixtures.
//!
//! The emitter borrows a candidate `MirBuilder`; it owns no session, commit,
//! route, or publication capability.  Keeping this implementation in one
//! test-only module prevents the candidate observer from becoming a second
//! operation lowerer before parity is measured.

#![cfg(test)]

use super::{
    CanonicalLoopSsaStateV1, LoopBindingKeyV1, LoopOperationV1, LoopSsaEmissionReceiptV1,
    LoopValueKeyV1, OperationEmissionReceiptV1, PhysicalRoleV1, VerifiedLoopBindingProjectionV1,
    VerifiedLoopOperationScheduleV1,
};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, BinaryOp, CompareOp, ConstValue, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

pub(super) fn new_state(builder: &mut MirBuilder) -> CanonicalLoopSsaStateV1 {
    let owner = super::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("function owner");
    let projection = VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![
            (
                LoopBindingKeyV1::new(0),
                super::BindingRefV1::new(owner, super::BindingId::new(0)),
            ),
            (
                LoopBindingKeyV1::new(1),
                super::BindingRefV1::new(owner, super::BindingId::new(1)),
            ),
        ],
    )
    .expect("binding projection");
    let blocks = super::TestBlockOwnerV1::install(builder);
    CanonicalLoopSsaStateV1 {
        cfg: Some(CanonicalCfgSessionV1::new()),
        ssa: Some(BindingSsaBuilderV1::new(projection.owner)),
        phis: Some(PhiTxn::begin("loop_binding_ssa_session")),
        projection,
        blocks,
        entry_values: BTreeMap::new(),
        reads: Vec::new(),
        defines: Vec::new(),
        sealed_predecessors: BTreeMap::new(),
    }
}

pub(super) struct CanonicalLoopSsaEmitterV1<'a> {
    builder: &'a mut MirBuilder,
    state: &'a mut CanonicalLoopSsaStateV1,
}

impl<'a> CanonicalLoopSsaEmitterV1<'a> {
    pub(super) fn new(builder: &'a mut MirBuilder, state: &'a mut CanonicalLoopSsaStateV1) -> Self {
        Self { builder, state }
    }

    pub(super) fn seed_entries(&mut self) {
        let preheader = self.block(PhysicalRoleV1::Preheader);
        let initial_i = self.emit_const_i64(preheader, 0);
        let initial_sum = self.emit_const_i64(preheader, 0);
        self.state
            .entry_values
            .insert(LoopValueKeyV1::new(0), initial_i);
        self.state
            .entry_values
            .insert(LoopValueKeyV1::new(1), initial_sum);
        self.define_at(
            LoopBindingKeyV1::new(0),
            PhysicalRoleV1::Preheader,
            initial_i,
        );
        self.define_at(
            LoopBindingKeyV1::new(1),
            PhysicalRoleV1::Preheader,
            initial_sum,
        );
    }

    pub(super) fn block(&self, role: PhysicalRoleV1) -> BasicBlockId {
        self.state.blocks[&role]
    }

    pub(super) fn entry_values(&self) -> &BTreeMap<LoopValueKeyV1, ValueId> {
        &self.state.entry_values
    }

    pub(super) fn emit_const_i64(&mut self, block: BasicBlockId, value: i64) -> ValueId {
        self.builder
            .start_new_block(block)
            .expect("select const block");
        let dst = self.builder.alloc_value_for_test();
        self.builder
            .emit_for_test(MirInstruction::Const {
                dst,
                value: ConstValue::Integer(value),
            })
            .expect("emit integer const");
        self.builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Integer);
        dst
    }

    pub(super) fn emit_const_bool(&mut self, block: BasicBlockId, value: bool) -> ValueId {
        self.builder
            .start_new_block(block)
            .expect("select condition block");
        let dst = self.builder.alloc_value_for_test();
        self.builder
            .emit_for_test(MirInstruction::Const {
                dst,
                value: ConstValue::Bool(value),
            })
            .expect("emit bool const");
        self.builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Bool);
        dst
    }

    pub(super) fn emit_add(
        &mut self,
        block: BasicBlockId,
        left: ValueId,
        right: ValueId,
    ) -> ValueId {
        self.builder
            .start_new_block(block)
            .expect("select binary block");
        let dst = self.builder.alloc_value_for_test();
        self.builder
            .emit_for_test(MirInstruction::BinOp {
                dst,
                op: BinaryOp::Add,
                lhs: left,
                rhs: right,
            })
            .expect("emit binary add");
        self.builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Integer);
        dst
    }

    fn emit_compare_less(&mut self, block: BasicBlockId, left: ValueId, right: ValueId) -> ValueId {
        self.builder
            .start_new_block(block)
            .expect("select compare block");
        let dst = self.builder.alloc_value_for_test();
        self.builder
            .emit_for_test(MirInstruction::Compare {
                dst,
                op: CompareOp::Lt,
                lhs: left,
                rhs: right,
            })
            .expect("emit compare");
        self.builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Bool);
        dst
    }

    pub(super) fn emit_header_carriers(
        &mut self,
        schedule: &VerifiedLoopOperationScheduleV1,
        values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    ) {
        for binding in schedule.header_reads.iter().copied() {
            let result = self.read_at(binding, PhysicalRoleV1::Header);
            values.insert(LoopValueKeyV1::new(binding.raw()), result);
        }
    }

    pub(super) fn emit_operations(
        &mut self,
        role: PhysicalRoleV1,
        operations: &[LoopOperationV1],
        values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    ) -> Result<OperationEmissionReceiptV1, String> {
        match self.emit_operations_inner(role, operations, values) {
            Ok(receipt) => Ok(receipt),
            Err(error) => self.abort(error),
        }
    }

    fn emit_operations_inner(
        &mut self,
        role: PhysicalRoleV1,
        operations: &[LoopOperationV1],
        values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    ) -> Result<OperationEmissionReceiptV1, String> {
        let mut emitted = Vec::with_capacity(operations.len());
        for operation in operations.iter().copied() {
            match operation {
                LoopOperationV1::ReadBinding { binding, result } => {
                    let value = self.read_at(binding, role);
                    values.insert(result, value);
                }
                LoopOperationV1::ConstI64 { result, value } => {
                    values.insert(result, self.emit_const_i64(self.block(role), value));
                }
                LoopOperationV1::BinaryI64 {
                    op,
                    left,
                    right,
                    result,
                } => {
                    let left = values
                        .get(&left)
                        .copied()
                        .ok_or_else(|| format!("missing binary lhs {left:?}"))?;
                    let right = values
                        .get(&right)
                        .copied()
                        .ok_or_else(|| format!("missing binary rhs {right:?}"))?;
                    let value = match op {
                        super::LoopBinaryI64OpV1::Add => {
                            self.emit_add(self.block(role), left, right)
                        }
                        super::LoopBinaryI64OpV1::Sub => {
                            return Err("DirectAccum schedule requires add".to_owned())
                        }
                    };
                    values.insert(result, value);
                }
                LoopOperationV1::CompareI64 {
                    op,
                    left,
                    right,
                    result,
                } => {
                    let left = values
                        .get(&left)
                        .copied()
                        .ok_or_else(|| format!("missing compare lhs {left:?}"))?;
                    let right = values
                        .get(&right)
                        .copied()
                        .ok_or_else(|| format!("missing compare rhs {right:?}"))?;
                    if !matches!(op, super::LoopCompareI64OpV1::Less) {
                        return Err("DirectAccum schedule requires less".to_owned());
                    }
                    values.insert(
                        result,
                        self.emit_compare_less(self.block(role), left, right),
                    );
                }
                LoopOperationV1::WriteBinding { binding, value } => {
                    let value_id = values
                        .get(&value)
                        .copied()
                        .ok_or_else(|| format!("missing write value {value:?}"))?;
                    self.define_at(binding, role, value_id);
                }
            }
            emitted.push(operation);
        }
        Ok(OperationEmissionReceiptV1 {
            emitted: emitted.into_boxed_slice(),
            values: values.iter().map(|(key, value)| (*key, *value)).collect(),
        })
    }

    pub(super) fn define_at(
        &mut self,
        key: LoopBindingKeyV1,
        role: PhysicalRoleV1,
        value: ValueId,
    ) {
        let binding = self.state.projection.resolve(key);
        let block = self.block(role);
        self.state
            .ssa
            .as_mut()
            .expect("active SSA builder")
            .define(binding, block, value)
            .expect("SSA define");
        self.state.defines.push((key, role, value));
    }

    pub(super) fn read_at(&mut self, key: LoopBindingKeyV1, role: PhysicalRoleV1) -> ValueId {
        let binding = self.state.projection.resolve(key);
        let block = self.block(role);
        let state = &mut *self.state;
        let phis = state.phis.as_mut().expect("active PHI transaction");
        let mut adapter = MirBindingSsaAdapterV1::new(&mut *self.builder, phis);
        let value = state
            .ssa
            .as_mut()
            .expect("active SSA builder")
            .read(&mut adapter, binding, block)
            .expect("SSA read");
        self.state.reads.push((key, role, value));
        value
    }

    pub(super) fn emit_jump(&mut self, from: PhysicalRoleV1, to: PhysicalRoleV1) {
        let source = self.block(from);
        let target = self.block(to);
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .expect("candidate function");
        self.state
            .cfg
            .as_mut()
            .expect("active CFG session")
            .emit_jump(function, source, target)
            .expect("canonical jump");
    }

    pub(super) fn emit_branch(&mut self, from: PhysicalRoleV1, condition: ValueId) {
        let source = self.block(from);
        let body = self.block(PhysicalRoleV1::Body);
        let after = self.block(PhysicalRoleV1::After);
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .expect("candidate function");
        self.state
            .cfg
            .as_mut()
            .expect("active CFG session")
            .emit_branch(function, source, condition, body, after)
            .expect("canonical branch");
    }

    pub(super) fn seal(&mut self, role: PhysicalRoleV1) {
        let block = self.block(role);
        let witness = {
            let function = self
                .builder
                .function_state
                .current_function
                .as_mut()
                .expect("candidate function");
            self.state
                .cfg
                .as_mut()
                .expect("active CFG session")
                .seal_block(function, block)
                .expect("CFG seal")
        };
        self.state
            .sealed_predecessors
            .insert(role, witness.predecessors().into());
        let state = &mut *self.state;
        let phis = state.phis.as_mut().expect("active PHI transaction");
        let mut adapter = MirBindingSsaAdapterV1::new(&mut *self.builder, phis);
        state
            .ssa
            .as_mut()
            .expect("active SSA builder")
            .seal(&mut adapter, block, &witness)
            .expect("SSA seal");
    }

    pub(super) fn emit_return(&mut self, role: PhysicalRoleV1) {
        super::TestExitOwnerV1::emit_unit_return(self.builder, self.block(role));
    }

    fn abort<T>(&mut self, error: String) -> Result<T, String> {
        Err(self.abort_with_error(error))
    }

    pub(super) fn abort_with_error(&mut self, error: String) -> String {
        match self.state.phis.take() {
            Some(phis) => phis.abort_on_err(self.builder, error).to_string(),
            None => error,
        }
    }

    pub(super) fn finish(&mut self) -> Result<LoopSsaEmissionReceiptV1, String> {
        let ssa = self.state.ssa.take().expect("active SSA builder");
        if let Err(error) = ssa.finish() {
            return self.abort(format!("{error:?}"));
        }
        let function = self
            .builder
            .function_state
            .current_function
            .as_ref()
            .expect("candidate function");
        let cfg = self.state.cfg.take().expect("active CFG session");
        if let Err(error) = cfg.finish(function) {
            return self.abort(format!("{error:?}"));
        }
        let header = function.get_block(super::bb(1)).expect("header");
        let header_phi_inputs = header
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                MirInstruction::Phi { inputs, .. } => Some(inputs.clone().into_boxed_slice()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let phis = self.state.phis.take().expect("active PHI transaction");
        if let Err(error) = phis.commit(self.builder) {
            return Err(error.to_string());
        }
        Ok(LoopSsaEmissionReceiptV1 {
            reads: std::mem::take(&mut self.state.reads).into_boxed_slice(),
            defines: std::mem::take(&mut self.state.defines).into_boxed_slice(),
            sealed_predecessors: std::mem::take(&mut self.state.sealed_predecessors)
                .into_iter()
                .collect(),
            header_phi_inputs,
        })
    }
}
