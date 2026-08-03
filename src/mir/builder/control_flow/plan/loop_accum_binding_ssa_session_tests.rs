//! Caller-zero Binding-SSA-first DirectAccum session proof.
//!
//! The fixture owns one candidate function and delegates all emission to the
//! borrowed emitter in `loop_accum_binding_ssa_emitter_tests.rs`.  The same
//! emitter is used by the unpublished-candidate observer.

#![cfg(test)]

use crate::mir::builder::emission::phi_lifecycle::{PhiToken, PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::ssa::binding::BindingSsaBuilderV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBindingKeyV1, LoopCompareI64OpV1, LoopConditionV1,
    LoopJoinSigElaboratorV1, LoopOperationV1, LoopRecipeArtifactV1, LoopRecipeItemV1,
    LoopRecipeVerifierV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1};
use crate::mir::{BasicBlockId, BindingId, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[path = "loop_accum_binding_ssa_failure_tests.rs"]
mod failure_tests;

#[path = "loop_accum_binding_ssa_operation_tests.rs"]
mod operation_tests;

#[path = "loop_accum_binding_ssa_candidate_tests.rs"]
mod candidate_tests;

#[path = "loop_accum_binding_ssa_emitter_tests.rs"]
mod emitter_tests;

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
    final_values: Box<[(LoopBindingKeyV1, LoopValueKeyV1)]>,
}

impl VerifiedLoopOperationScheduleV1 {
    fn from_direct_fixture(
        header_reads: Vec<LoopBindingKeyV1>,
    ) -> Result<Self, OperationScheduleErrorV1> {
        let artifact: LoopRecipeArtifactV1 =
            serde_json::from_str(super::DIRECT_GOLDEN).expect("direct recipe golden");
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
        let root_join = sig
            .as_sig()
            .loops
            .iter()
            .find(|row| row.key.raw() == recipe.root_loop.raw())
            .expect("root join row");
        let expected = root_join
            .carriers
            .iter()
            .map(|payload| payload.binding)
            .collect::<BTreeSet<_>>();
        let final_values = root_join
            .carriers
            .iter()
            .map(|payload| (payload.binding, payload.value))
            .collect::<Vec<_>>()
            .into_boxed_slice();
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
            final_values,
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

pub(super) struct CanonicalLoopSsaStateV1 {
    cfg: Option<CanonicalCfgSessionV1>,
    ssa: Option<BindingSsaBuilderV1<PhiToken>>,
    phis: Option<PhiTxn>,
    projection: VerifiedLoopBindingProjectionV1,
    blocks: BTreeMap<PhysicalRoleV1, BasicBlockId>,
    entry_values: BTreeMap<LoopValueKeyV1, ValueId>,
    reads: Vec<(LoopBindingKeyV1, PhysicalRoleV1, ValueId)>,
    defines: Vec<(LoopBindingKeyV1, PhysicalRoleV1, ValueId)>,
    sealed_predecessors: BTreeMap<PhysicalRoleV1, Box<[BasicBlockId]>>,
}

struct CanonicalLoopSsaSessionV1 {
    builder: MirBuilder,
    state: CanonicalLoopSsaStateV1,
}

impl CanonicalLoopSsaSessionV1 {
    fn new() -> Self {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("loop_binding_ssa_session/0".to_owned());
        let mut state = emitter_tests::new_state(&mut builder);
        {
            let mut emitter =
                emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut builder, &mut state);
            emitter.seed_entries();
        }
        Self { builder, state }
    }

    fn entry_values(&self) -> &BTreeMap<LoopValueKeyV1, ValueId> {
        &self.state.entry_values
    }

    fn emit_jump(&mut self, from: PhysicalRoleV1, to: PhysicalRoleV1) {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_jump(from, to);
    }

    fn emit_branch(&mut self, from: PhysicalRoleV1, condition: ValueId) {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_branch(from, condition);
    }

    fn seal(&mut self, role: PhysicalRoleV1) {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .seal(role);
    }

    fn emit_return(&mut self, role: PhysicalRoleV1) {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_return(role);
    }

    fn emit_const_bool(&mut self, block: BasicBlockId, value: bool) -> ValueId {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_const_bool(block, value)
    }

    fn emit_const_i64(&mut self, block: BasicBlockId, value: i64) -> ValueId {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_const_i64(block, value)
    }

    fn emit_add(&mut self, block: BasicBlockId, left: ValueId, right: ValueId) -> ValueId {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_add(block, left, right)
    }

    fn define_at(&mut self, key: LoopBindingKeyV1, role: PhysicalRoleV1, value: ValueId) {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .define_at(key, role, value);
    }

    fn read_at(&mut self, key: LoopBindingKeyV1, role: PhysicalRoleV1) -> ValueId {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .read_at(key, role)
    }

    fn emit_header_carriers(
        &mut self,
        schedule: &VerifiedLoopOperationScheduleV1,
        values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    ) {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_header_carriers(schedule, values);
    }

    fn emit_operations(
        &mut self,
        role: PhysicalRoleV1,
        operations: &[LoopOperationV1],
        values: &mut BTreeMap<LoopValueKeyV1, ValueId>,
    ) -> Result<OperationEmissionReceiptV1, String> {
        emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
            .emit_operations(role, operations, values)
    }

    fn finish(self) -> LoopSsaEmissionReceiptV1 {
        let (_builder, receipt) = self.finish_with_builder();
        receipt
    }

    fn finish_with_builder(mut self) -> (MirBuilder, LoopSsaEmissionReceiptV1) {
        let receipt =
            emitter_tests::CanonicalLoopSsaEmitterV1::new(&mut self.builder, &mut self.state)
                .finish()
                .expect("finish canonical session");
        (self.builder, receipt)
    }

    fn into_builder(self) -> MirBuilder {
        self.builder
    }
}

#[test]
fn direct_accum_uses_one_canonical_cfg_and_binding_ssa_owner() {
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);

    let header_i = session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Header);
    let header_sum = session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Header);
    let condition = session.emit_const_bool(role_block(PhysicalRoleV1::Header), true);
    session.emit_branch(PhysicalRoleV1::Header, condition);

    session.seal(PhysicalRoleV1::Body);
    let body_i = session.read_at(LoopBindingKeyV1::new(0), PhysicalRoleV1::Body);
    let body_sum = session.read_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Body);
    assert_eq!(body_i, header_i);
    assert_eq!(body_sum, header_sum);
    let one = session.emit_const_i64(role_block(PhysicalRoleV1::Body), 1);
    let next_sum = session.emit_add(role_block(PhysicalRoleV1::Body), body_sum, one);
    session.define_at(LoopBindingKeyV1::new(1), PhysicalRoleV1::Body, next_sum);
    let next_i = session.emit_add(role_block(PhysicalRoleV1::Body), body_i, one);
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
            (
                LoopBindingKeyV1::new(0),
                BindingRefV1::new(owner, BindingId::new(0)),
            ),
            (
                LoopBindingKeyV1::new(0),
                BindingRefV1::new(owner, BindingId::new(1)),
            ),
        ],
    );
    assert!(duplicate.is_err());
    let foreign_row = VerifiedLoopBindingProjectionV1::try_new(
        owner,
        vec![(
            LoopBindingKeyV1::new(0),
            BindingRefV1::new(foreign, BindingId::new(0)),
        )],
    );
    assert!(foreign_row.is_err());
}

#[test]
fn canonical_session_has_no_unsealed_predecessor_vectors() {
    let mut session = CanonicalLoopSsaSessionV1::new();
    session.emit_jump(PhysicalRoleV1::Preheader, PhysicalRoleV1::Header);
    session.seal(PhysicalRoleV1::Preheader);
    let condition = session.emit_const_bool(role_block(PhysicalRoleV1::Header), true);
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
    let error =
        VerifiedLoopOperationScheduleV1::from_direct_fixture(vec![LoopBindingKeyV1::new(0)])
            .expect_err("missing carrier read must reject before emission");
    assert_eq!(
        error,
        OperationScheduleErrorV1::CarrierReadMissing(LoopBindingKeyV1::new(1))
    );
}
