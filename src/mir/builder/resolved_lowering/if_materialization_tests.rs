use std::collections::{BTreeMap, BTreeSet};

use hakorune_mir_core::BindingId;

use crate::mir::builder::emission::{branch, constant};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIssuerV1};
use crate::mir::{ConstValue, MirInstruction, ValueId};

use super::branch_transaction::{
    AuthorizedBranchRebindV1, BranchValueStoreV1, ResolvedBranchTransactionV1,
};
use super::if_materialization::{
    define_join_phis, DefinedJoinPublishV1, DefinedJoinValueStoreV1, IfCfgSessionV1,
};
use super::MirBuilder;

#[derive(Debug, Default)]
struct TestValueStoreV1 {
    values: BTreeMap<BindingRefV1, ValueId>,
    fail_current_for: Option<BindingRefV1>,
    fail_rebind_for: BTreeSet<BindingRefV1>,
    rebind_attempts: Vec<BindingRefV1>,
    published: usize,
}

impl BranchValueStoreV1 for TestValueStoreV1 {
    fn branch_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        if self.fail_current_for == Some(binding) {
            return Err("[injected/current_value]".to_string());
        }
        self.values
            .get(&binding)
            .copied()
            .ok_or_else(|| "[test/value_missing]".to_string())
    }

    fn branch_rebind_authorized(
        &mut self,
        authorization: AuthorizedBranchRebindV1,
    ) -> Result<ValueId, String> {
        self.rebind_attempts.push(authorization.binding());
        if self.fail_rebind_for.contains(&authorization.binding()) {
            return Err("[injected/rebind]".to_string());
        }
        let slot = self
            .values
            .get_mut(&authorization.binding())
            .ok_or_else(|| "[test/rebind_missing]".to_string())?;
        let old = *slot;
        *slot = authorization.value();
        Ok(old)
    }
}

impl DefinedJoinValueStoreV1 for TestValueStoreV1 {
    fn defined_join_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.values
            .get(&binding)
            .copied()
            .ok_or_else(|| "[test/publish_missing]".to_string())
    }

    fn publish_defined_join_batch(
        &mut self,
        publishes: Vec<DefinedJoinPublishV1>,
    ) -> Result<(), String> {
        for publish in &publishes {
            if !self.values.contains_key(&publish.binding()) {
                return Err("[test/publish_missing]".to_string());
            }
        }
        for publish in publishes {
            self.values.insert(publish.binding(), publish.value());
            self.published += 1;
        }
        Ok(())
    }
}

fn bindings(count: usize) -> Vec<BindingRefV1> {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let owner = issuer.issue().unwrap();
    (0..count)
        .map(|index| BindingRefV1::new(owner, BindingId::new(index as u32)))
        .collect()
}

fn store(entries: &[(BindingRefV1, ValueId)]) -> TestValueStoreV1 {
    TestValueStoreV1 {
        values: entries.iter().copied().collect(),
        fail_current_for: None,
        fail_rebind_for: BTreeSet::new(),
        rebind_attempts: Vec::new(),
        published: 0,
    }
}

fn builder_fixture() -> (MirBuilder, ValueId, ValueId) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("canonical_if_i1a/0".to_string());
    let condition = constant::emit_bool(&mut builder, true).unwrap();
    let entry = constant::emit_integer(&mut builder, 7).unwrap();
    (builder, condition, entry)
}

#[test]
fn ordered_domain_permit_and_first_old_restore_are_exact() {
    let domain = bindings(2);
    let first = ValueId::new(10);
    let second = ValueId::new(20);
    let mut values = store(&[(domain[0], first), (domain[1], second)]);
    let mut transaction = ResolvedBranchTransactionV1::snapshot(&values, &domain, &domain).unwrap();

    transaction
        .rebind(&mut values, domain[1], ValueId::new(21))
        .unwrap();
    transaction
        .rebind(&mut values, domain[1], ValueId::new(22))
        .unwrap();
    transaction
        .rebind(&mut values, domain[0], ValueId::new(11))
        .unwrap();
    let branch = transaction.capture_and_restore(&mut values).unwrap();
    assert_eq!(values.values[&domain[0]], first);
    assert_eq!(values.values[&domain[1]], second);

    let implicit = transaction.implicit_false_values();
    let rows = transaction.join_rows(&branch, &implicit).unwrap();
    assert_eq!(rows[0].binding(), domain[0]);
    assert_eq!(rows[0].then_value(), ValueId::new(11));
    assert_eq!(rows[1].binding(), domain[1]);
    assert_eq!(rows[1].then_value(), ValueId::new(22));
}

#[test]
fn unused_permit_preserves_entry_while_unauthorized_rebind_fails_fast() {
    let domain = bindings(2);
    let mut values = store(&[(domain[0], ValueId::new(1)), (domain[1], ValueId::new(2))]);
    let mut transaction =
        ResolvedBranchTransactionV1::snapshot(&values, &domain[..1], &domain[..1]).unwrap();
    assert!(transaction
        .rebind(&mut values, domain[1], ValueId::new(3))
        .unwrap_err()
        .contains("rebind_not_authorized"));
    let branch = transaction.capture_and_restore(&mut values).unwrap();
    let rows = transaction
        .join_rows(&branch, &transaction.implicit_false_values())
        .unwrap();
    assert_eq!(rows[0].then_value(), ValueId::new(1));
    assert_eq!(rows[0].else_value(), ValueId::new(1));

    assert!(
        ResolvedBranchTransactionV1::snapshot(&values, &domain[..1], &domain[1..])
            .unwrap_err()
            .contains("rebind_permit_outside_join_domain")
    );
}

#[test]
fn capture_error_still_restores_first_old_values() {
    let domain = bindings(1);
    let entry = ValueId::new(4);
    let mut values = store(&[(domain[0], entry)]);
    let mut transaction = ResolvedBranchTransactionV1::snapshot(&values, &domain, &domain).unwrap();
    transaction
        .rebind(&mut values, domain[0], ValueId::new(5))
        .unwrap();
    values.fail_current_for = Some(domain[0]);
    assert!(transaction.capture_and_restore(&mut values).is_err());
    assert_eq!(values.values[&domain[0]], entry);
}

#[test]
fn branch_body_error_uses_explicit_restore_without_source_side_effects() {
    let domain = bindings(1);
    let entry = ValueId::new(6);
    let mut values = store(&[(domain[0], entry)]);
    let mut transaction = ResolvedBranchTransactionV1::snapshot(&values, &domain, &domain).unwrap();
    transaction
        .rebind(&mut values, domain[0], ValueId::new(7))
        .unwrap();
    transaction.restore_error(&mut values).unwrap();
    assert_eq!(values.values[&domain[0]], entry);
}

#[test]
fn snapshot_prime_restores_entry_even_when_rhs_changes_before_rebind() {
    let binding = bindings(1)[0];
    let entry = ValueId::new(30);
    let mut values = store(&[(binding, entry)]);
    let mut transaction =
        ResolvedBranchTransactionV1::snapshot(&values, &[binding], &[binding]).unwrap();

    // Simulate a nested If published while the outer assignment RHS lowers.
    values.values.insert(binding, ValueId::new(31));
    transaction
        .rebind(&mut values, binding, ValueId::new(32))
        .unwrap();
    transaction.restore_error(&mut values).unwrap();
    assert_eq!(values.values[&binding], entry);
}

#[test]
fn nonpermit_domain_change_is_rejected_and_restore_attempts_every_permit() {
    let domain = bindings(3);
    let entries = [ValueId::new(40), ValueId::new(41), ValueId::new(42)];
    let mut values = store(&[
        (domain[0], entries[0]),
        (domain[1], entries[1]),
        (domain[2], entries[2]),
    ]);
    let mut transaction =
        ResolvedBranchTransactionV1::snapshot(&values, &domain, &domain[..2]).unwrap();
    values.values.insert(domain[0], ValueId::new(50));
    values.values.insert(domain[1], ValueId::new(51));
    values.values.insert(domain[2], ValueId::new(52));
    values.fail_rebind_for.insert(domain[1]);

    let error = transaction.capture_and_restore(&mut values).unwrap_err();
    assert!(error.contains("nonpermit_domain_changed"));
    assert!(error.contains("restore_failures"));
    assert_eq!(values.rebind_attempts, vec![domain[1], domain[0]]);
    assert_eq!(values.values[&domain[0]], entries[0]);
    assert_eq!(values.values[&domain[1]], ValueId::new(51));
}

#[test]
fn implicit_false_edge_targets_merge_without_synthetic_else() {
    let (mut builder, condition, _) = builder_fixture();
    let mut session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    let layout = session.layout();
    assert_eq!(layout.else_entry(), None);
    session.enter_then(&mut builder).unwrap();
    session.close_then(&mut builder).unwrap();
    let predecessors = session.verify_actual_predecessors(&mut builder).unwrap();

    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    assert!(matches!(
        function.get_block(layout.header()).unwrap().terminator,
        Some(MirInstruction::Branch { else_bb, .. }) if else_bb == layout.merge()
    ));
    let actual = crate::mir::verification::utils::compute_predecessors(function);
    assert_eq!(actual[&layout.merge()].len(), 2);
    assert_eq!(builder.current_block, Some(layout.merge()));
    let _ = predecessors;
}

#[test]
fn aborted_if_restores_the_saved_post_condition_header_cursor() {
    let (mut builder, condition, _) = builder_fixture();
    let session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    let header = session.layout().header();
    session.enter_then(&mut builder).unwrap();
    assert_ne!(builder.current_block, Some(header));

    session.restore_header_after_error(&mut builder).unwrap();

    assert_eq!(builder.current_block, Some(header));
    assert!(builder.scope_ctx.current_function.as_ref().unwrap().blocks[&header].is_terminated());
}

#[test]
fn nested_actual_exit_not_then_entry_is_merge_predecessor() {
    let (mut builder, condition, _) = builder_fixture();
    let mut session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    let layout = session.layout();
    session.enter_then(&mut builder).unwrap();
    let nested_exit = builder.next_block_id();
    builder.ensure_block_exists(nested_exit).unwrap();
    branch::emit_jump(&mut builder, nested_exit).unwrap();
    builder.start_new_block(nested_exit).unwrap();
    session.close_then(&mut builder).unwrap();
    session.verify_actual_predecessors(&mut builder).unwrap();

    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let actual = crate::mir::verification::utils::compute_predecessors(function);
    assert!(actual[&layout.merge()].contains(&nested_exit));
    assert!(!actual[&layout.merge()].contains(&layout.then_entry()));
}

#[test]
fn extra_actual_predecessor_is_rejected_before_phi_definition() {
    let (mut builder, condition, _) = builder_fixture();
    let mut session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    let layout = session.layout();
    session.enter_then(&mut builder).unwrap();
    session.close_then(&mut builder).unwrap();
    let rogue = builder.next_block_id();
    builder.start_new_block(rogue).unwrap();
    branch::emit_jump(&mut builder, layout.merge()).unwrap();
    assert!(session
        .verify_actual_predecessors(&mut builder)
        .unwrap_err()
        .contains("actual_predecessor_mismatch"));
    assert!(
        builder.scope_ctx.current_function.as_ref().unwrap().blocks[&layout.merge()]
            .phi_instructions()
            .next()
            .is_none()
    );
}

#[test]
fn disconnected_block_cannot_claim_the_then_exit_role() {
    let (mut builder, condition, _) = builder_fixture();
    let mut session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    let layout = session.layout();
    let disconnected = builder.next_block_id();
    builder.start_new_block(disconnected).unwrap();
    session.close_then(&mut builder).unwrap();
    assert!(session
        .verify_actual_predecessors(&mut builder)
        .unwrap_err()
        .contains("then_exit_not_reachable"));
    assert!(
        builder.scope_ctx.current_function.as_ref().unwrap().blocks[&layout.merge()]
            .phi_instructions()
            .next()
            .is_none()
    );
}

#[test]
fn same_input_still_defines_fresh_final_phi_before_batch_publish() {
    let (mut builder, condition, entry) = builder_fixture();
    let binding = bindings(1)[0];
    let mut values = store(&[(binding, entry)]);
    let mut transaction =
        ResolvedBranchTransactionV1::snapshot(&values, &[binding], &[binding]).unwrap();
    transaction.rebind(&mut values, binding, entry).unwrap();
    let then_values = transaction.capture_and_restore(&mut values).unwrap();
    let rows = transaction
        .join_rows(&then_values, &transaction.implicit_false_values())
        .unwrap();

    let mut session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    let layout = session.layout();
    session.enter_then(&mut builder).unwrap();
    session.close_then(&mut builder).unwrap();
    let predecessors = session.verify_actual_predecessors(&mut builder).unwrap();
    let defined = define_join_phis(&mut builder, predecessors, &rows).unwrap();

    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let phi = function
        .get_block(layout.merge())
        .unwrap()
        .phi_instructions()
        .next();
    let Some(MirInstruction::Phi { dst, inputs, .. }) = phi else {
        panic!("expected final PHI")
    };
    assert_ne!(*dst, entry);
    assert_eq!(inputs.len(), 2);
    for (predecessor, value) in inputs {
        if *value == entry {
            continue;
        }
        assert!(function
            .get_block(*predecessor)
            .unwrap()
            .instructions
            .iter()
            .any(|instruction| matches!(
                instruction,
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(7),
                } if dst == value
            )));
    }
    assert_eq!(values.published, 0);

    defined.publish_join_values(&mut values).unwrap();
    assert_eq!(values.published, 1);
    assert_eq!(values.values[&binding], *dst);
}

#[test]
fn later_phi_failure_cannot_publish_any_join_value() {
    let (mut builder, condition, entry) = builder_fixture();
    let domain = bindings(2);
    let invalid = ValueId::new(999_999);
    let mut values = store(&[(domain[0], entry), (domain[1], invalid)]);
    let mut transaction = ResolvedBranchTransactionV1::snapshot(&values, &domain, &domain).unwrap();
    transaction.rebind(&mut values, domain[0], entry).unwrap();
    transaction.rebind(&mut values, domain[1], invalid).unwrap();
    let then_values = transaction.capture_and_restore(&mut values).unwrap();
    let rows = transaction
        .join_rows(&then_values, &transaction.implicit_false_values())
        .unwrap();

    let mut session = IfCfgSessionV1::open_implicit_false(&mut builder, condition).unwrap();
    session.enter_then(&mut builder).unwrap();
    session.close_then(&mut builder).unwrap();
    let predecessors = session.verify_actual_predecessors(&mut builder).unwrap();
    assert!(define_join_phis(&mut builder, predecessors, &rows).is_err());
    assert_eq!(values.published, 0);
    assert_eq!(values.values[&domain[0]], entry);
    assert_eq!(values.values[&domain[1]], invalid);
}

#[test]
fn explicit_else_uses_both_actual_branch_exits() {
    let (mut builder, condition, _) = builder_fixture();
    let mut session = IfCfgSessionV1::open_explicit_else(&mut builder, condition).unwrap();
    let layout = session.layout();
    session.enter_then(&mut builder).unwrap();
    session.close_then(&mut builder).unwrap();
    session.enter_else(&mut builder).unwrap();
    let else_exit = builder.current_block.unwrap();
    session.close_else(&mut builder).unwrap();
    session.verify_actual_predecessors(&mut builder).unwrap();

    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let actual = crate::mir::verification::utils::compute_predecessors(function);
    assert!(actual[&layout.merge()].contains(&layout.then_entry()));
    assert!(actual[&layout.merge()].contains(&else_exit));
    assert_eq!(layout.else_entry(), Some(else_exit));
}
