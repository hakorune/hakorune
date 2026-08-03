//! Caller-zero Binding-SSA-first DirectAccum session proof.
//!
//! The test owns one canonical CFG session, one Binding SSA builder, and one
//! PhiTxn, matching the production authority boundary without adding a caller.

#![cfg(test)]

use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBindingKeyV1, LoopCompareI64OpV1, LoopConditionV1,
    LoopJoinSigElaboratorV1, LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeItemV1,
    LoopRecipeVerifierV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1};
use crate::mir::{
    BasicBlockId, BinaryOp, BindingId, CompareOp, ConstValue, MirInstruction, MirType, ValueId,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PhysicalRoleV1 {
    Preheader,
    Header,
    Body,
    Step,
    After,
}

fn bb(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

fn role_block(role: PhysicalRoleV1) -> BasicBlockId {
    match role {
        PhysicalRoleV1::Preheader => bb(0),
        PhysicalRoleV1::Header => bb(1),
        PhysicalRoleV1::Body => bb(2),
        PhysicalRoleV1::Step => bb(3),
        PhysicalRoleV1::After => bb(4),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedLoopBindingProjectionV1 {
    owner: FunctionOwnerIdV1,
    rows: Box<[(LoopBindingKeyV1, BindingRefV1)]>,
}

impl VerifiedLoopBindingProjectionV1 {
    fn try_new(
        owner: FunctionOwnerIdV1,
        mut rows: Vec<(LoopBindingKeyV1, BindingRefV1)>,
    ) -> Result<Self, String> {
        rows.sort_by_key(|(key, _)| *key);
        if rows.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            return Err("duplicate portable binding key".to_string());
        }
        if rows.iter().any(|(_, binding)| binding.owner() != owner) {
            return Err("foreign binding owner".to_string());
        }
        Ok(Self {
            owner,
            rows: rows.into_boxed_slice(),
        })
    }

    fn resolve(&self, key: LoopBindingKeyV1) -> BindingRefV1 {
        self.rows
            .iter()
            .find(|(candidate, _)| *candidate == key)
            .map(|(_, binding)| *binding)
            .expect("verified binding projection key")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LoopSsaEmissionReceiptV1 {
    reads: Box<[(LoopBindingKeyV1, PhysicalRoleV1, ValueId)]>,
    defines: Box<[(LoopBindingKeyV1, PhysicalRoleV1, ValueId)]>,
    sealed_predecessors: Box<[(PhysicalRoleV1, Box<[BasicBlockId]>)]>,
    header_phi_inputs: Box<[Box<[(BasicBlockId, ValueId)]>]>,
}

#[derive(Debug, PartialEq, Eq)]
enum OperationScheduleErrorV1 {
    CarrierReadMissing(LoopBindingKeyV1),
    CarrierReadDuplicated(LoopBindingKeyV1),
    UnexpectedCarrierRead(LoopBindingKeyV1),
}

/// Builder-free operation schedule projected from one verified recipe/join sig.
#[derive(Debug, PartialEq, Eq)]
struct VerifiedLoopOperationScheduleV1 {
    condition: Box<[LoopOperationV1]>,
    body: Box<[LoopOperationV1]>,
    header_reads: Box<[LoopBindingKeyV1]>,
    condition_result: LoopValueKeyV1,
}

impl VerifiedLoopOperationScheduleV1 {
    fn from_direct_fixture(header_reads: Vec<LoopBindingKeyV1>) -> Result<Self, OperationScheduleErrorV1> {
        let artifact: LoopRecipeArtifactV1 =
            serde_json::from_str(super::super::DIRECT_GOLDEN).expect("direct recipe golden");
        let verified = LoopRecipeVerifierV1::verify(artifact.recipe().clone())
            .expect("direct recipe verification");
        let sig = LoopJoinSigElaboratorV1::elaborate(&verified).expect("direct join sig");
        let recipe = verified.as_recipe();
        let root = recipe
            .loops
            .iter()
            .find(|row| row.key == recipe.root_loop)
            .expect("root loop");
        let (condition_block, condition_result) = match root.condition {
            LoopConditionV1::Predicate { block, value } => (block, value),
            LoopConditionV1::Always => panic!("direct fixture requires predicate"),
        };
        let condition = operations_for_block(recipe, condition_block);
        let body = operations_for_block(recipe, root.body);
        let expected = sig
            .as_sig()
            .loops
            .iter()
            .find(|row| row.key.raw() == recipe.root_loop.raw())
            .expect("root join row")
            .carriers
            .iter()
            .map(|payload| payload.binding)
            .collect::<BTreeSet<_>>();
        let mut actual = BTreeSet::new();
        for binding in header_reads.iter().copied() {
            if !actual.insert(binding) {
                return Err(OperationScheduleErrorV1::CarrierReadDuplicated(binding));
            }
            if !expected.contains(&binding) {
                return Err(OperationScheduleErrorV1::UnexpectedCarrierRead(binding));
            }
        }
        if let Some(binding) = expected.difference(&actual).next().copied() {
            return Err(OperationScheduleErrorV1::CarrierReadMissing(binding));
        }
        Ok(Self {
            condition,
            body,
            header_reads: header_reads.into_boxed_slice(),
            condition_result,
        })
    }
}

fn operations_for_block(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV1,
    key: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
) -> Box<[LoopOperationV1]> {
    recipe
        .blocks
        .iter()
        .find(|block| block.key == key)
        .expect("recipe block")
        .items
        .iter()
        .map(|item_key| {
            let item = recipe
                .items
                .iter()
                .find(|row| row.key == *item_key)
                .expect("recipe item");
            match item.item {
                LoopRecipeItemV1::Operation { operation } => operation,
                _ => panic!("direct operation schedule contains control item"),
            }
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct OperationEmissionReceiptV1 {
    emitted: Box<[LoopOperationV1]>,
    values: Box<[(LoopValueKeyV1, ValueId)]>,
}

/// Candidate-only block creation owner used by this proof fixture.
struct TestBlockOwnerV1;

impl TestBlockOwnerV1 {
    fn install(builder: &mut MirBuilder) -> BTreeMap<PhysicalRoleV1, BasicBlockId> {
        let mut blocks = BTreeMap::new();
        for role in [
            PhysicalRoleV1::Preheader,
            PhysicalRoleV1::Header,
            PhysicalRoleV1::Body,
            PhysicalRoleV1::Step,
            PhysicalRoleV1::After,
        ] {
            let block = role_block(role);
            builder.ensure_block_exists(block).expect("candidate block");
            blocks.insert(role, block);
        }
        blocks
    }
}

struct TestExitOwnerV1;

impl TestExitOwnerV1 {
    fn emit_unit_return(builder: &mut MirBuilder, block: BasicBlockId) {
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("candidate function")
            .get_block_mut(block)
            .expect("return block")
            .set_terminator(MirInstruction::Return { value: None });
    }
}

struct CanonicalLoopSsaSessionV1 {
    builder: MirBuilder,
    cfg: CanonicalCfgSessionV1,
    ssa: BindingSsaBuilderV1<PhiToken>,
    phis: PhiTxn,
    projection: VerifiedLoopBindingProjectionV1,
    blocks: BTreeMap<PhysicalRoleV1, BasicBlockId>,
    entry_values: BTreeMap<LoopValueKeyV1, ValueId>,
    reads: Vec<(LoopBindingKeyV1, PhysicalRoleV1, ValueId)>,
    defines: Vec<(LoopBindingKeyV1, PhysicalRoleV1, ValueId)>,
    sealed_predecessors: BTreeMap<PhysicalRoleV1, Box<[BasicBlockId]>>,
}

impl CanonicalLoopSsaSessionV1 {
    fn new() -> Self {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("loop_binding_ssa_session/0".to_owned());
        let blocks = TestBlockOwnerV1::install(&mut builder);
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("owner issuer")
            .issue()
            .expect("function owner");
        let projection = VerifiedLoopBindingProjectionV1::try_new(
            owner,
            vec![
                (
                    LoopBindingKeyV1::new(0),
                    BindingRefV1::new(owner, BindingId::new(0)),
                ),
                (
                    LoopBindingKeyV1::new(1),
                    BindingRefV1::new(owner, BindingId::new(1)),
                ),
            ],
        )
        .expect("binding projection");
        let mut session = Self {
            builder,
            cfg: CanonicalCfgSessionV1::new(),
            ssa: BindingSsaBuilderV1::new(projection.owner),
            phis: PhiTxn::begin("loop_binding_ssa_session"),
            projection,
            blocks,
            entry_values: BTreeMap::new(),
            reads: Vec::new(),
            defines: Vec::new(),
            sealed_predecessors: BTreeMap::new(),
        };
        let preheader = session.block(PhysicalRoleV1::Preheader);
        let initial_i = session.emit_const_i64(preheader, 0);
        let initial_sum = session.emit_const_i64(preheader, 0);
        session.entry_values.insert(LoopValueKeyV1::new(0), initial_i);
        session
            .entry_values
            .insert(LoopValueKeyV1::new(1), initial_sum);
        session.define_at(
            LoopBindingKeyV1::new(0),
            PhysicalRoleV1::Preheader,
            initial_i,
        );
        session.define_at(
            LoopBindingKeyV1::new(1),
            PhysicalRoleV1::Preheader,
            initial_sum,
        );
        session
    }

    fn block(&self, role: PhysicalRoleV1) -> BasicBlockId {
        self.blocks[&role]
    }

    fn emit_const_i64(&mut self, block: BasicBlockId, value: i64) -> ValueId {
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

    fn emit_const_bool(&mut self, block: BasicBlockId, value: bool) -> ValueId {
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

    fn emit_add(&mut self, block: BasicBlockId, left: ValueId, right: ValueId) -> ValueId {
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

    fn emit_compare_less(
        &mut self,
        block: BasicBlockId,
        left: ValueId,
        right: ValueId,
    ) -> ValueId {
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

    fn emit_header_carriers(
        &mut self,
        schedule: &VerifiedLoopOperationScheduleV1,
        values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    ) {
        for binding in schedule.header_reads.iter().copied() {
            let result = self.read_at(binding, PhysicalRoleV1::Header);
            let entry = LoopValueKeyV1::new(binding.raw());
            values.insert(entry, result);
        }
    }

    fn emit_operations(
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
                    let value_id = self.emit_const_i64(self.block(role), value);
                    values.insert(result, value_id);
                }
                LoopOperationV1::BinaryI64 {
                    op,
                    left,
                    right,
                    result,
                } => {
                    let left = *values
                        .get(&left)
                        .ok_or_else(|| format!("missing binary lhs {left:?}"))?;
                    let right = *values
                        .get(&right)
                        .ok_or_else(|| format!("missing binary rhs {right:?}"))?;
                    let value_id = match op {
                        LoopBinaryI64OpV1::Add => self.emit_add(self.block(role), left, right),
                        LoopBinaryI64OpV1::Sub => {
                            return Err("DirectAccum schedule requires add".to_owned())
                        }
                    };
                    values.insert(result, value_id);
                }
                LoopOperationV1::CompareI64 {
                    op: LoopCompareI64OpV1::Less,
                    left,
                    right,
                    result,
                } => {
                    let left = *values
                        .get(&left)
                        .ok_or_else(|| format!("missing compare lhs {left:?}"))?;
                    let right = *values
                        .get(&right)
                        .ok_or_else(|| format!("missing compare rhs {right:?}"))?;
                    let value_id = self.emit_compare_less(self.block(role), left, right);
                    values.insert(result, value_id);
                }
                LoopOperationV1::CompareI64 { .. } => {
                    return Err("DirectAccum schedule requires less".to_owned())
                }
                LoopOperationV1::WriteBinding { binding, value } => {
                    let value_id = *values
                        .get(&value)
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

    fn define_at(&mut self, key: LoopBindingKeyV1, role: PhysicalRoleV1, value: ValueId) {
        let binding = self.projection.resolve(key);
        self.ssa
            .define(binding, self.block(role), value)
            .expect("SSA define");
        self.defines.push((key, role, value));
    }

    fn read_at(&mut self, key: LoopBindingKeyV1, role: PhysicalRoleV1) -> ValueId {
        let binding = self.projection.resolve(key);
        let block = self.block(role);
        let mut adapter = MirBindingSsaAdapterV1::new(&mut self.builder, &mut self.phis);
        let value = self.ssa.read(&mut adapter, binding, block).expect("SSA read");
        self.reads.push((key, role, value));
        value
    }

    fn emit_jump(&mut self, from: PhysicalRoleV1, to: PhysicalRoleV1) {
        let source = self.block(from);
        let target = self.block(to);
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .expect("candidate function");
        self.cfg
            .emit_jump(function, source, target)
            .expect("canonical jump");
    }

    fn emit_branch(&mut self, from: PhysicalRoleV1, condition: ValueId) {
        let source = self.block(from);
        let body = self.block(PhysicalRoleV1::Body);
        let after = self.block(PhysicalRoleV1::After);
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .expect("candidate function");
        self.cfg
            .emit_branch(
                function,
                source,
                condition,
                body,
                after,
            )
            .expect("canonical branch");
    }

    fn seal(&mut self, role: PhysicalRoleV1) {
        let block = self.block(role);
        let witness = {
            let function = self
                .builder
                .function_state
                .current_function
                .as_mut()
                .expect("candidate function");
            self.cfg.seal_block(function, block).expect("CFG seal")
        };
        self.sealed_predecessors
            .insert(role, witness.predecessors().into());
        let mut adapter = MirBindingSsaAdapterV1::new(&mut self.builder, &mut self.phis);
        self.ssa
            .seal(&mut adapter, block, &witness)
            .expect("SSA seal");
    }

    fn emit_return(&mut self, role: PhysicalRoleV1) {
        let block = self.block(role);
        TestExitOwnerV1::emit_unit_return(&mut self.builder, block);
    }

    fn finish(self) -> LoopSsaEmissionReceiptV1 {
        let CanonicalLoopSsaSessionV1 {
            mut builder,
            cfg,
            ssa,
            phis,
            reads,
            defines,
            sealed_predecessors,
            ..
        } = self;
        ssa.finish().expect("SSA finish");
        cfg.finish(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("candidate function"),
        )
        .expect("CFG finish");
        let header = builder
            .function_state
            .current_function
            .as_ref()
            .expect("candidate function")
            .get_block(bb(1))
            .expect("header");
        let header_phi_inputs = header
            .instructions
            .iter()
            .filter_map(|instruction| match instruction {
                MirInstruction::Phi { inputs, .. } => Some(inputs.clone().into_boxed_slice()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        phis.commit(&mut builder).expect("PhiTxn commit");
        LoopSsaEmissionReceiptV1 {
            reads: reads.into_boxed_slice(),
            defines: defines.into_boxed_slice(),
            sealed_predecessors: sealed_predecessors.into_iter().collect(),
            header_phi_inputs,
        }
    }
}

#[test]
fn direct_accum_uses_one_canonical_cfg_and_binding_ssa_owner() {
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);

    let header_i = session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Header);
    let header_sum = session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Header);
    let condition = session.emit_const_bool(session.block(PhysicalRoleV1::Header), true);
    session.emit_branch(PhysicalRoleV1::Header, condition);

    session.seal(PhysicalRoleV1::Body);
    let body_i = session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Body);
    let body_sum = session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Body);
    assert_eq!(body_i, header_i);
    assert_eq!(body_sum, header_sum);
    let one = session.emit_const_i64(session.block(PhysicalRoleV1::Body), 1);
    let next_sum = session.emit_add(session.block(PhysicalRoleV1::Body), body_sum, one);
    session.define_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Body, next_sum);
    let next_i = session.emit_add(session.block(PhysicalRoleV1::Body), body_i, one);
    session.define_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Body, next_i);
    session.emit_jump(PhysicalRoleV1::Body, PhysicalRoleV1::Step);

    session.seal(PhysicalRoleV1::Step);
    session.emit_jump(PhysicalRoleV1::Step, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::After);
    session.emit_return(PhysicalRoleV1::After);
    session.seal(PhysicalRoleV1::Header);

    let receipt = session.finish();
    assert_eq!(receipt.reads.len(), 4);
    assert_eq!(receipt.defines.len(), 4);
    assert_eq!(receipt.sealed_predecessors.len(), 5);
    assert_eq!(receipt.header_phi_inputs.len(), 2);
    assert!(receipt
        .header_phi_inputs
        .iter()
        .all(|inputs| inputs.len() == 2));
}

#[test]
fn binding_projection_rejects_duplicate_and_foreign_identity() {
    let owner = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("owner issuer")
        .issue()
        .expect("owner");
    let foreign = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("foreign issuer")
        .issue()
        .expect("foreign owner");
    let duplicate = VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![
            (LoopBindingKeyV1::new(0), BindingRefV1::new(owner, BindingId::new(0))),
            (LoopBindingKeyV1::new(0), BindingRefV1::new(owner, BindingId::new(1))),
        ],
    );
    assert!(duplicate.is_err());
    let foreign_row = VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![(LoopBindingKeyV1::new(0), BindingRefV1::new(foreign, BindingId::new(0)))],
    );
    assert!(foreign_row.is_err());
}

#[test]
fn canonical_session_has_no_unsealed_predecessor_vectors() {
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);
    let condition = session.emit_const_bool(session.block(PhysicalRoleV1::Header), true);
    session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Header);
    session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Header);
    session.emit_branch(PhysicalRoleV1::Header, condition);
    session.seal(PhysicalRoleV1::Body);
    session.emit_jump(PhysicalRoleV1::Body, PhysicalRoleV1::Step);
    session.seal(PhysicalRoleV1::Step);
    session.emit_jump(PhysicalRoleV1::Step, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::After);
    session.emit_return(PhysicalRoleV1::After);
    session.seal(PhysicalRoleV1::Header);
    let receipt = session.finish();
    let expected = BTreeSet::from([
        PhysicalRoleV1::Preheader,
        PhysicalRoleV1::Header,
        PhysicalRoleV1::Body,
        PhysicalRoleV1::Step,
        PhysicalRoleV1::After,
    ]);
    let actual = receipt
        .sealed_predecessors
        .iter()
        .map(|(role, _)| *role)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn operation_schedule_rejects_missing_carrier_header_read() {
    let error = VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![
        LoopBindingKeyV1::new(0),
    ])
    .expect_err("missing carrier read must reject before emission");
    assert_eq!(
        error,
        OperationScheduleErrorV1::CarrierReadMissing(LoopBindingKeyV1::new(1))
    );
}

#[test]
fn direct_operation_schedule_emits_through_one_binding_ssa_owner() {
    let schedule = VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![
        LoopBindingKeyV1::new(0),
        LoopBindingKeyV1::new(1),
    ])
    .expect("verified direct schedule");
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);

    let mut values = session.entry_values.clone();
    session.emit_header_carriers(&schedule, &mut values);
    let repeated_i = session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Header);
    assert_eq!(repeated_i, values[&LoopValueKeyV1::new(0)]);
    let header_receipt = session
        .emit_operations(PhysicalRoleV1::Header, &schedule.condition, &mut values)
        .expect("condition operations");
    let condition = values[&schedule.condition_result];
    session.emit_branch(PhysicalRoleV1::Header, condition);

    session.seal(PhysicalRoleV1::Body);
    let body_receipt = session
        .emit_operations(PhysicalRoleV1::Body, &schedule.body, &mut values)
        .expect("body operations");
    let visible_sum = session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Body);
    assert_eq!(visible_sum, values[&LoopValueKeyV1::new(7)]);
    session.emit_jump(PhysicalRoleV1::Body, PhysicalRoleV1::Step);
    session.seal(PhysicalRoleV1::Step);
    session.emit_jump(PhysicalRoleV1::Step, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::After);
    session.emit_return(PhysicalRoleV1::After);
    session.seal(PhysicalRoleV1::Header);

    assert_eq!(header_receipt.emitted.len(), 3);
    assert_eq!(body_receipt.emitted.len(), 8);
    assert_eq!(body_receipt.values.len(), 11);
    let receipt = session.finish();
    assert_eq!(receipt.reads.len(), 7);
    assert_eq!(receipt.defines.len(), 4);
    assert_eq!(receipt.header_phi_inputs.len(), 2);
    assert!(receipt
        .header_phi_inputs
        .iter()
        .all(|inputs| inputs.len() == 2));
}
