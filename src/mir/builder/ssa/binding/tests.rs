use super::{BindingSsaBuilderV1, BindingSsaErrorV1, BindingSsaIrV1};
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, FunctionOwnerIssuerV1};
use crate::mir::{BasicBlockId, BindingId, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
struct FakePhiV1 {
    block: BasicBlockId,
    inputs: Vec<(BasicBlockId, ValueId)>,
}

#[derive(Debug, Default)]
struct FakeIrV1 {
    next_value: u32,
    next_token: u32,
    phis: BTreeMap<u32, FakePhiV1>,
    rollback_attempts: Vec<u32>,
    fail_patch: bool,
    fail_verify: BTreeSet<(BasicBlockId, ValueId)>,
    fail_rollback: BTreeSet<u32>,
}

impl FakeIrV1 {
    fn new() -> Self {
        Self {
            next_value: 1_000,
            ..Self::default()
        }
    }

    fn phi_at(&self, block: BasicBlockId) -> Option<(ValueId, &FakePhiV1)> {
        self.phis.iter().find_map(|(token, phi)| {
            (phi.block == block).then_some((ValueId::new(1_000 + *token), phi))
        })
    }
}

impl BindingSsaIrV1 for FakeIrV1 {
    type PhiToken = u32;

    fn define_provisional_phi(
        &mut self,
        block: BasicBlockId,
    ) -> Result<(ValueId, Self::PhiToken), String> {
        let token = self.next_token;
        self.next_token += 1;
        let value = ValueId::new(self.next_value);
        self.next_value += 1;
        self.phis.insert(
            token,
            FakePhiV1 {
                block,
                inputs: Vec::new(),
            },
        );
        Ok((value, token))
    }

    fn patch_phi_inputs(
        &mut self,
        token: Self::PhiToken,
        inputs: &[(BasicBlockId, ValueId)],
    ) -> Result<(), String> {
        if self.fail_patch {
            return Err(format!("injected patch failure token={token}"));
        }
        self.phis
            .get_mut(&token)
            .ok_or_else(|| format!("missing phi token={token}"))?
            .inputs = inputs.to_vec();
        Ok(())
    }

    fn verify_phi_input(&self, predecessor: BasicBlockId, value: ValueId) -> Result<(), String> {
        if self.fail_verify.contains(&(predecessor, value)) {
            Err(format!(
                "value {value} does not dominate predecessor {predecessor}"
            ))
        } else {
            Ok(())
        }
    }

    fn rollback_phi(&mut self, token: Self::PhiToken) -> Result<(), String> {
        self.rollback_attempts.push(token);
        if self.fail_rollback.contains(&token) {
            return Err(format!("injected rollback failure token={token}"));
        }
        self.phis.remove(&token);
        Ok(())
    }
}

fn make_owner() -> FunctionOwnerIdV1 {
    FunctionOwnerIssuerV1::new_for_compilation()
        .unwrap()
        .issue()
        .unwrap()
}

fn binding_ref(owner: FunctionOwnerIdV1, slot: u32) -> BindingRefV1 {
    BindingRefV1::new(owner, BindingId::new(slot))
}

fn bb(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

fn value(id: u32) -> ValueId {
    ValueId::new(id)
}

fn witness(block: u32, predecessors: &[u32]) -> VerifiedPredecessorsV1 {
    VerifiedPredecessorsV1::from_test_parts(
        bb(block),
        predecessors.iter().copied().map(bb).collect(),
    )
}

#[test]
fn entry_definition_and_same_block_overwrite() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();

    ssa.define(binding, bb(0), value(10)).unwrap();
    ssa.define(binding, bb(0), value(11)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();

    assert_eq!(ssa.read(&mut ir, binding, bb(0)).unwrap(), value(11));
    assert!(ir.phis.is_empty());
    ssa.finish().unwrap();
}

#[test]
fn single_predecessor_forwards_without_phi() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(20)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    ssa.seal(&mut ir, bb(1), &witness(1, &[0])).unwrap();

    assert_eq!(ssa.read(&mut ir, binding, bb(1)).unwrap(), value(20));
    assert!(ir.phis.is_empty());
    ssa.finish().unwrap();
}

#[test]
fn diamond_keeps_same_input_and_one_sided_phis() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(30)).unwrap();
    for (block, predecessors) in [(0, vec![]), (1, vec![0]), (2, vec![0])] {
        ssa.seal(&mut ir, bb(block), &witness(block, &predecessors))
            .unwrap();
    }
    ssa.seal(&mut ir, bb(3), &witness(3, &[1, 2])).unwrap();
    let same_phi = ssa.read(&mut ir, binding, bb(3)).unwrap();
    assert_eq!(
        ir.phi_at(bb(3)).unwrap().1.inputs,
        vec![(bb(1), value(30)), (bb(2), value(30))]
    );
    assert_eq!(same_phi, ir.phi_at(bb(3)).unwrap().0);

    let binding2 = binding_ref(owner, 1);
    ssa.define(binding2, bb(0), value(31)).unwrap();
    ssa.define(binding2, bb(1), value(32)).unwrap();
    let one_sided = ssa.read(&mut ir, binding2, bb(3)).unwrap();
    let (token, phi) = ir
        .phis
        .iter()
        .find(|(_, phi)| phi.inputs.contains(&(bb(1), value(32))))
        .unwrap();
    assert_eq!(one_sided, ValueId::new(1_000 + *token));
    assert_eq!(phi.inputs, vec![(bb(1), value(32)), (bb(2), value(31))]);
}

#[test]
fn two_sided_and_nested_diamonds_use_exact_predecessors() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(1), value(41)).unwrap();
    ssa.define(binding, bb(2), value(42)).unwrap();
    for (block, predecessors) in [
        (1, vec![]),
        (2, vec![]),
        (3, vec![1, 2]),
        (4, vec![3]),
        (5, vec![3]),
        (6, vec![4, 5]),
    ] {
        ssa.seal(&mut ir, bb(block), &witness(block, &predecessors))
            .unwrap();
    }
    let inner = ssa.read(&mut ir, binding, bb(3)).unwrap();
    ssa.define(binding, bb(4), value(44)).unwrap();
    let outer = ssa.read(&mut ir, binding, bb(6)).unwrap();

    assert_ne!(inner, outer);
    let outer_phi = ir.phi_at(bb(6)).unwrap().1;
    assert_eq!(outer_phi.inputs, vec![(bb(4), value(44)), (bb(5), inner)]);
}

#[test]
fn open_loop_header_completes_zero_and_backedge_inputs() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(50)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    let header = ssa.read(&mut ir, binding, bb(1)).unwrap();
    ssa.define(binding, bb(2), value(52)).unwrap();
    ssa.seal(&mut ir, bb(2), &witness(2, &[1])).unwrap();
    ssa.seal(&mut ir, bb(1), &witness(1, &[0, 2])).unwrap();
    ssa.seal(&mut ir, bb(3), &witness(3, &[1])).unwrap();

    assert_eq!(ssa.read(&mut ir, binding, bb(3)).unwrap(), header);
    assert_eq!(
        ir.phi_at(bb(1)).unwrap().1.inputs,
        vec![(bb(0), value(50)), (bb(2), value(52))]
    );
    ssa.finish().unwrap();
}

#[test]
fn open_loop_retains_self_phi_when_backedge_has_no_redefinition() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(55)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    let header = ssa.read(&mut ir, binding, bb(1)).unwrap();
    ssa.seal(&mut ir, bb(2), &witness(2, &[1])).unwrap();
    assert_eq!(ssa.read(&mut ir, binding, bb(2)).unwrap(), header);
    ssa.seal(&mut ir, bb(1), &witness(1, &[0, 2])).unwrap();

    assert_eq!(
        ir.phi_at(bb(1)).unwrap().1.inputs,
        vec![(bb(0), value(55)), (bb(2), header)]
    );
}

#[test]
fn multiple_backedges_are_ordered_and_exact() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(60)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    ssa.read(&mut ir, binding, bb(1)).unwrap();
    for (block, value_id) in [(2, 62), (3, 63)] {
        ssa.define(binding, bb(block), value(value_id)).unwrap();
        ssa.seal(&mut ir, bb(block), &witness(block, &[1])).unwrap();
    }
    ssa.seal(&mut ir, bb(1), &witness(1, &[3, 0, 2])).unwrap();

    assert_eq!(
        ir.phi_at(bb(1)).unwrap().1.inputs,
        vec![(bb(0), value(60)), (bb(2), value(62)), (bb(3), value(63))]
    );
}

#[test]
fn missing_definition_and_foreign_owner_are_typed() {
    let owner = make_owner();
    let foreign = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    assert!(matches!(
        ssa.read(&mut ir, binding, bb(0)),
        Err(BindingSsaErrorV1::MissingDefinition { .. })
    ));

    let mut other: BindingSsaBuilderV1<u32> = BindingSsaBuilderV1::new(owner);
    assert!(matches!(
        other.define(binding_ref(foreign, 0), bb(0), value(1)),
        Err(BindingSsaErrorV1::ForeignBinding { .. })
    ));
}

#[test]
fn witness_mismatch_and_second_seal_fail_fast() {
    let owner = make_owner();
    let mut ssa: BindingSsaBuilderV1<u32> = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    assert!(matches!(
        ssa.seal(&mut ir, bb(1), &witness(2, &[])),
        Err(BindingSsaErrorV1::WitnessBlockMismatch { .. })
    ));
    ssa.seal(&mut ir, bb(1), &witness(1, &[])).unwrap();
    assert!(matches!(
        ssa.seal(&mut ir, bb(1), &witness(1, &[])),
        Err(BindingSsaErrorV1::BlockSealedTwice { .. })
    ));
}

#[test]
fn finish_rejects_open_and_incomplete_blocks() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut open: BindingSsaBuilderV1<u32> = BindingSsaBuilderV1::new(owner);
    open.define(binding, bb(0), value(70)).unwrap();
    assert!(matches!(
        open.finish(),
        Err(BindingSsaErrorV1::UnsealedAtFinish { .. })
    ));

    let mut incomplete = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    incomplete.read(&mut ir, binding, bb(1)).unwrap();
    assert!(matches!(
        incomplete.finish(),
        Err(BindingSsaErrorV1::IncompleteAtFinish { count: 1 })
    ));
}

#[test]
fn patch_failure_rolls_back_phi_and_poisons_builder() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(80)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    ssa.read(&mut ir, binding, bb(1)).unwrap();
    ssa.define(binding, bb(2), value(82)).unwrap();
    ssa.seal(&mut ir, bb(2), &witness(2, &[1])).unwrap();
    ir.fail_patch = true;

    assert!(matches!(
        ssa.seal(&mut ir, bb(1), &witness(1, &[0, 2])),
        Err(BindingSsaErrorV1::PhiOperation {
            operation: "patch",
            ..
        })
    ));
    assert_eq!(ir.rollback_attempts, vec![0]);
    assert!(ir.phis.is_empty());
    assert!(matches!(
        ssa.read(&mut ir, binding, bb(0)),
        Err(BindingSsaErrorV1::Poisoned)
    ));
}

#[test]
fn rollback_failures_are_retained_after_input_verification_failure() {
    let owner = make_owner();
    let binding = binding_ref(owner, 0);
    let mut ssa = BindingSsaBuilderV1::new(owner);
    let mut ir = FakeIrV1::new();
    ssa.define(binding, bb(0), value(90)).unwrap();
    ssa.seal(&mut ir, bb(0), &witness(0, &[])).unwrap();
    ssa.read(&mut ir, binding, bb(1)).unwrap();
    ssa.define(binding, bb(2), value(92)).unwrap();
    ssa.seal(&mut ir, bb(2), &witness(2, &[1])).unwrap();
    ir.fail_verify.insert((bb(2), value(92)));
    ir.fail_rollback.insert(0);

    let error = ssa.seal(&mut ir, bb(1), &witness(1, &[0, 2])).unwrap_err();
    assert!(matches!(error, BindingSsaErrorV1::DuringPhiCleanup { .. }));
    assert_eq!(ir.rollback_attempts, vec![0]);
}
