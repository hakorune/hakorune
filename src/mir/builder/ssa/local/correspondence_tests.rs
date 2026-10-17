//! Same-frame provenance and consumer handoff failure boundaries.
use super::*;
use crate::mir::builder::function_state_transaction::{
    FunctionOwnedStateTransactionV1, FunctionStateCaptureModeV1,
};
use crate::mir::builder::MirBuilder;

fn fixture() -> (MirBuilder, ValueId) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("correspondence/0".into());
    let value = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    (builder, value)
}

#[test]
fn derived_coverage_rejects_missing_observation_without_losing_source_loan() {
    let (mut builder, left) = fixture();
    let original = OriginalScalarAppendV1::fixture_for_correspondence_test(&mut builder, left);
    let value = original.original().1.dst_value().unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    builder
        .function_state
        .local_ssa_correspondence
        .install_scalar_original(function, original)
        .unwrap();
    // Force the existing writer into another block, rather than exercising
    // an identity/cache loan that correctly creates no new observation.
    let block = builder.core_ctx.next_block();
    let source_block = builder.function_state.current_block.unwrap();
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .blocks
        .get_mut(&source_block)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: block,
            edge_args: None,
        });
    builder.start_new_block(block).unwrap();
    let (derived, _) = super::super::checked_scalar_arg(&mut builder, value).unwrap();
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks[&block]
        .instructions
        .iter()
        .any(|row| matches!(row, MirInstruction::Copy { dst, src }
            if *dst == derived && *src == value)));
    let index = &mut builder.function_state.local_ssa_correspondence;
    let targets = BTreeSet::from([derived]);
    index.validate_scalar_consumer_coverage(&targets).unwrap();
    assert!(index
        .validate_scalar_consumer_coverage(&BTreeSet::new())
        .is_err());
    index.scalar_appends.remove(&derived).unwrap();
    assert!(index.scalar_source_for(derived).is_some());
    assert!(index
        .validate_scalar_consumer_coverage(&targets)
        .unwrap_err()
        .contains("scalar-consumer-coverage"));
}

#[test]
fn ordinary_arithmetic_propagates_selected_reentry_failure_before_consumer_append() {
    let (mut builder, left) = fixture();
    let original = OriginalScalarAppendV1::fixture_for_correspondence_test(&mut builder, left);
    let value = original.original().1.dst_value().unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    builder
        .function_state
        .local_ssa_correspondence
        .install_scalar_original(function, original)
        .unwrap();
    for block in builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .blocks
        .values_mut()
    {
        block
            .instructions
            .retain(|row| row.dst_value() != Some(value));
    }
    let block = builder.core_ctx.next_block();
    builder.start_new_block(block).unwrap();
    let error = builder
        .build_binary_op_from_values_recorded(crate::ast::BinaryOperator::Add, value, left)
        .unwrap_err();
    assert!(error.contains("local-ssa-correspondence") || error.contains("scalar-original"));
    assert!(!builder
        .current_function_instructions()
        .iter()
        .any(|row| matches!(
            row,
            MirInstruction::BinOp {
                op: crate::mir::BinaryOp::Add,
                ..
            }
        )));
    assert!(builder
        .function_state
        .local_ssa_correspondence
        .scalar_observations()
        .next()
        .is_none());
}

#[test]
fn cache_and_identity_keep_exact_index_membership_and_detect_mutation() {
    let (mut builder, raw) = fixture();
    let (returned, relation) = super::super::checked_scalar_arg(&mut builder, raw).unwrap();
    let (cached, cached_relation) = super::super::checked_scalar_arg(&mut builder, raw).unwrap();
    assert_eq!(returned, cached);
    assert!(Rc::ptr_eq(&relation, &cached_relation));
    let (_, identity) = super::super::checked_scalar_arg(&mut builder, returned).unwrap();
    let (_, same_identity) = super::super::checked_scalar_arg(&mut builder, returned).unwrap();
    assert!(Rc::ptr_eq(&identity, &same_identity));
    builder
        .function_state
        .local_ssa_correspondence
        .require_member(&identity)
        .unwrap();
    let (mut foreign, foreign_raw) = fixture();
    let (_, foreign_relation) =
        super::super::checked_scalar_arg(&mut foreign, foreign_raw).unwrap();
    assert_eq!(relation.values(), foreign_relation.values());
    assert!(builder
        .function_state
        .local_ssa_correspondence
        .require_member(&foreign_relation)
        .unwrap_err()
        .contains("foreign-index"));
    let mut changed = builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .clone();
    for block in changed.blocks.values_mut() {
        for row in &mut block.instructions {
            if let MirInstruction::Const { dst, value } = row {
                if *dst == returned {
                    *value = crate::mir::ConstValue::Integer(8);
                }
            }
        }
    }
    assert!(relation
        .verify(&changed)
        .unwrap_err()
        .contains("materialized-producer"));

    // The same LocalSSA writer must retain ordered Compare children through
    // the pin Copy used by short-circuit conditions, and reject later edits.
    for op in [crate::mir::CompareOp::Eq, crate::mir::CompareOp::Lt] {
        let (mut builder, lhs) = fixture();
        let rhs = crate::mir::builder::emission::constant::emit_integer(&mut builder, 9).unwrap();
        let original = builder.next_value_id();
        builder.emit_instruction(MirInstruction::Compare { dst: original, op, lhs, rhs }).unwrap();
        let pinned = builder.pin_to_slot(original, "condition").unwrap();
        let returned = super::super::try_cond(&mut builder, pinned).unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        let relation = builder.function_state.local_ssa_correspondence.loan(function, pinned, returned)
            .expect("same writer retains Compare under pin Copy");
        relation.verify(function).unwrap();
        let Step::Append(children) = &relation.step else { panic!("Copy append required") };
        let Definition::Instruction(compare) = &children[0].source else { panic!("original Compare required") };
        for change_op in [false, true] {
            let mut changed = function.clone();
            let block = changed.blocks.get_mut(&compare.0).unwrap();
            let row = block.instructions.iter_mut().find(|row| **row == compare.1).unwrap();
            let MirInstruction::Compare { op, lhs, rhs, .. } = row else { panic!("Compare required") };
            if change_op { *op = crate::mir::CompareOp::Ne; } else { std::mem::swap(lhs, rhs); }
            assert!(relation.verify(&changed).is_err(), "changed Compare must reject");
        }
    }
}

#[test]
fn nested_function_restores_caller_index_and_refuses_child_loans() {
    let (mut builder, raw) = fixture();
    let (_, outer) = super::super::checked_scalar_arg(&mut builder, raw).unwrap();
    let transaction = FunctionOwnedStateTransactionV1::begin(
        &mut builder.function_state,
        FunctionStateCaptureModeV1::LegacyRestore,
    );
    assert!(builder.function_state.local_ssa_correspondence.is_empty());
    builder.enter_function_for_test("child/0".into());
    let child_raw = crate::mir::builder::emission::constant::emit_integer(&mut builder, 7).unwrap();
    let (_, child) = super::super::checked_scalar_arg(&mut builder, child_raw).unwrap();
    transaction.restore(&mut builder.function_state);
    builder
        .function_state
        .local_ssa_correspondence
        .require_member(&outer)
        .unwrap();
    assert!(builder
        .function_state
        .local_ssa_correspondence
        .require_member(&child)
        .is_err());
    outer
        .verify(builder.function_state.current_function.as_ref().unwrap())
        .unwrap();

    // An original loop-body loan keeps its identity while the sole Phi writer
    // completes the definition. Final capture requires that completion.
    let header = builder.core_ctx.next_block();
    builder.ensure_block_exists(header).unwrap();
    let backedge = builder.core_ctx.next_block();
    builder.ensure_block_exists(backedge).unwrap();
    let initial = builder.function_state.current_block.unwrap();
    let phi = builder.next_value_id();
    let lifecycle = crate::mir::builder::emission::phi_lifecycle::define_provisional_phi;
    lifecycle(&mut builder, header, phi, "correspondence-test").unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    let loan = builder.function_state.local_ssa_correspondence.loan(function, phi, phi).unwrap();
    loan.verify(function).unwrap();
    assert!(loan.require_completed_v1().unwrap_err().contains("phi-incomplete"));
    let mut provisional = Vec::new();
    loan.append_observed_bindings_v1(&mut provisional);
    let inputs = vec![(initial, raw), (backedge, raw)];
    crate::mir::builder::emission::phi_lifecycle::patch_phi_inputs(
        &mut builder, header, phi, inputs.clone(), "correspondence-test").unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    let again = builder.function_state.local_ssa_correspondence.loan(function, phi, phi).unwrap();
    assert!(Rc::ptr_eq(&loan, &again));
    loan.require_completed_v1().unwrap();
    loan.verify(function).unwrap();
    loan.verify_projected(function, &mut |binding| Ok(Some(binding.clone()))).unwrap();
    let completed = loan.resolve_observed_binding_v1(&provisional[0], true).unwrap().unwrap();
    assert!(matches!(&completed.1, MirInstruction::Phi { inputs: actual, .. } if actual == &inputs));
    let mut changed = function.clone();
    let row = changed.blocks.get_mut(&header).unwrap().instructions.iter_mut()
        .find(|row| row.dst_value() == Some(phi)).unwrap();
    let MirInstruction::Phi { inputs, .. } = row else { panic!("Phi required") };
    inputs.pop();
    assert!(loan.verify(&changed).is_err());
    assert!(loan.verify_projected(&changed, &mut |binding| Ok(Some(binding.clone()))).is_err());
    let before = function.blocks[&header].instructions.clone();
    assert!(crate::mir::builder::emission::phi_lifecycle::patch_phi_inputs(
        &mut builder, header, phi, vec![(initial, raw)], "duplicate-completion").is_err());
    assert_eq!(builder.function_state.current_function.as_ref().unwrap().blocks[&header].instructions, before);
    let before_spans = builder.function_state.current_function.as_ref().unwrap().blocks[&header].instruction_spans.clone();
    assert!(lifecycle(&mut builder, header, phi, "duplicate-definition").is_err());
    assert_eq!(builder.function_state.current_function.as_ref().unwrap().blocks[&header].instructions, before);
    assert_eq!(builder.function_state.current_function.as_ref().unwrap().blocks[&header].instruction_spans, before_spans);
    let (foreign, _) = fixture();
    assert!(foreign.function_state.local_ssa_correspondence.require_member(&loan).is_err());

    let rolled = builder.next_value_id();
    lifecycle(&mut builder, header, rolled, "rollback-test").unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    let pending = builder.function_state.local_ssa_correspondence.loan(function, rolled, rolled).unwrap();
    assert!(crate::mir::builder::emission::phi_lifecycle::rollback_provisional_phi(
        &mut builder, header, rolled, "rollback-test").unwrap());
    assert!(pending.require_completed_v1().is_err());
    assert!(pending.verify(builder.function_state.current_function.as_ref().unwrap()).is_err());
}

#[test]
fn copy_of_binary_retains_actual_append_and_ordered_children() {
    let (mut builder, lhs) = fixture();
    let rhs = crate::mir::builder::emission::constant::emit_integer(&mut builder, 4).unwrap();
    let binary = builder.next_value_id();
    builder
        .emit_instruction(MirInstruction::BinOp {
            dst: binary,
            op: crate::mir::BinaryOp::Sub,
            lhs,
            rhs,
        })
        .unwrap();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(binary, crate::mir::MirType::Integer);
    let copy = builder.next_value_id();
    builder
        .emit_instruction(MirInstruction::Copy {
            dst: copy,
            src: binary,
        })
        .unwrap();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(copy, crate::mir::MirType::Integer);
    // Exercise the cross-block writer: same-block BinOp children now lend
    // their original producer through Identity instead of replaying it.
    let target = builder.core_ctx.next_block();
    crate::mir::builder::emission::branch::emit_jump(&mut builder, target).unwrap();
    builder.start_new_block(target).unwrap();
    let (returned, relation) = super::super::checked_scalar_arg(&mut builder, copy).unwrap();
    assert_ne!(copy, returned);
    relation
        .verify(builder.function_state.current_function.as_ref().unwrap())
        .unwrap();
    let Step::Append(children) = &relation.step else {
        panic!("actual Copy append required")
    };
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].values().0, binary);
    assert!(matches!(children[0].step, Step::Append(_)));
}
#[test]
fn selected_child_emission_failure_publishes_no_parent_cache_or_relation() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(
        &[
            ("HAKO_JOINIR_STRICT", Some("1")),
            ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
            ("HAKO_JOINIR_DEBUG", Some("1")),
        ],
        || {
            let (mut builder, lhs) = fixture();
            // A defined, non-rematerialized child in an orphan block reaches
            // the actual Copy writer, whose dominance guard rejects it.
            let non_dominating = builder.next_value_id();
            let orphan = builder.core_ctx.next_block();
            let mut child_block = crate::mir::BasicBlock::new(orphan);
            child_block.add_instruction(MirInstruction::UnaryOp {
                dst: non_dominating,
                op: crate::mir::UnaryOp::Neg,
                operand: lhs,
            });
            builder
                .function_state
                .current_function
                .as_mut()
                .unwrap()
                .add_block(child_block);
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(non_dominating, crate::mir::MirType::Integer);
            let parent = builder.next_value_id();
            builder
                .emit_instruction(MirInstruction::BinOp {
                    dst: parent,
                    op: crate::mir::BinaryOp::Sub,
                    lhs: non_dominating,
                    rhs: lhs,
                })
                .unwrap();
            builder
                .function_state
                .type_ctx
                .value_types
                .insert(parent, crate::mir::MirType::Integer);
            builder.comp_ctx.emit_debug_policy = crate::mir::builder::emit_debug_policy::BuilderEmitDebugPolicySnapshotV1::from_environment();
            let target = builder.core_ctx.next_block();
            crate::mir::builder::emission::branch::emit_jump(&mut builder, target).unwrap();
            builder.start_new_block(target).unwrap();
            let before = builder.current_function_instructions();
            let error = super::super::checked_scalar_arg(&mut builder, parent).unwrap_err();
            assert!(
                error.contains("builder/non_dominating_copy"),
                "actual child Copy emission must fail: {error}"
            );
            assert_eq!(builder.current_function_instructions(), before);
            assert!(builder.function_state.local_ssa_map.is_empty());
            assert!(builder.function_state.local_ssa_correspondence.is_empty());
        },
    );
}

#[test]
fn consumer_handoff_is_atomic_and_rejects_resend_without_replacing_loans() {
    let (ledger, sources) = OriginalScalarAppendV1::source_inventory_for_correspondence_test();
    assert_eq!(sources.len(), 3, "complete original source inventory");
    let owner = sources[0].owner();
    assert!(sources.iter().all(|source| source.owner() == owner));
    let (mut builder, left) = fixture();
    let mut values = Vec::new();
    for source in sources {
        let right = crate::mir::builder::emission::constant::emit_integer(&mut builder, 4).unwrap();
        let operator = if source.positive_divisor().is_some() {
            crate::ast::BinaryOperator::Divide
        } else {
            crate::ast::BinaryOperator::Subtract
        };
        let (completed, relations) = builder
            .build_source_scalar_from_values_recorded(operator, left, right)
            .unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        let original = ledger
            .record_scalar_operation_append_v1(
                source,
                (left, right),
                relations,
                &completed,
                &builder.function_state.local_ssa_correspondence,
                function,
            )
            .unwrap();
        builder
            .function_state
            .local_ssa_correspondence
            .install_scalar_original(function, original)
            .unwrap();
        values.push(completed.value());
    }
    let missing = builder
        .function_state
        .local_ssa_correspondence
        .scalar_loans
        .remove(&values[0])
        .unwrap();
    let error = ledger
        .record_scalar_operation_consumers_v1(
            owner,
            builder.function_state.current_function.as_ref().unwrap(),
            &builder.function_state.local_ssa_correspondence,
        )
        .unwrap_err();
    assert!(error.contains("original-coverage"), "{error}");
    builder
        .function_state
        .local_ssa_correspondence
        .scalar_loans
        .insert(values[0], missing);
    let block = builder.core_ctx.next_block();
    let source_block = builder.function_state.current_block.unwrap();
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .blocks
        .get_mut(&source_block)
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: block,
            edge_args: None,
        });
    builder.start_new_block(block).unwrap();
    for value in values {
        super::super::checked_scalar_arg(&mut builder, value).unwrap();
    }
    let index = &builder.function_state.local_ssa_correspondence;
    let function = builder.function_state.current_function.as_ref().unwrap();
    let observations: Vec<_> = index.scalar_observations().collect();
    assert!(
        observations.len() >= 2,
        "failure follows a valid derived row"
    );
    let late_target = observations.last().unwrap().1.values().1;
    let mut changed = function.clone();
    let mut edits = 0;
    for block in changed.blocks.values_mut() {
        for row in &mut block.instructions {
            if let MirInstruction::Copy { dst, src } = row {
                if *dst == late_target {
                    *src = *dst;
                    edits += 1;
                }
            }
        }
    }
    assert_eq!(edits, 1, "mutate the last actual derived producer only");
    assert!(ledger
        .record_scalar_operation_consumers_v1(owner, &changed, index)
        .is_err());
    let error = ledger
        .visit_scalar_operation_consumers_v1(owner, |_, _| Ok(()))
        .unwrap_err();
    assert!(
        error.contains("consumer-handoff-missing"),
        "failed handoff published no rows: {error}"
    );
    ledger
        .record_scalar_operation_consumers_v1(owner, function, index)
        .unwrap();
    let mut accepted = Vec::new();
    ledger
        .visit_scalar_operation_consumers_v1(owner, |original, relation| {
            accepted.push((Rc::clone(original), Rc::clone(relation)));
            Ok(())
        })
        .unwrap();
    assert_eq!(accepted.len(), observations.len());
    for (accepted, original) in accepted.iter().zip(&observations) {
        assert!(Rc::ptr_eq(&accepted.0, &original.0));
        assert!(Rc::ptr_eq(&accepted.1, &original.1));
    }
    let error = ledger
        .record_scalar_operation_consumers_v1(owner, function, index)
        .unwrap_err();
    assert!(error.contains("duplicate-consumer-handoff"), "{error}");
    let mut preserved = Vec::new();
    ledger
        .visit_scalar_operation_consumers_v1(owner, |original, relation| {
            preserved.push((Rc::clone(original), Rc::clone(relation)));
            Ok(())
        })
        .unwrap();
    assert_eq!(preserved.len(), accepted.len());
    for (before, after) in accepted.iter().zip(&preserved) {
        assert!(Rc::ptr_eq(&before.0, &after.0));
        assert!(Rc::ptr_eq(&before.1, &after.1));
    }
}

#[test]
fn same_block_selected_scalar_reuses_original_producer_and_facts() {
    let (mut builder, left) = fixture();
    let original = OriginalScalarAppendV1::fixture_for_correspondence_test(&mut builder, left);
    let value = original.original().1.dst_value().unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    builder
        .function_state
        .local_ssa_correspondence
        .install_scalar_original(function, Rc::clone(&original))
        .unwrap();
    let instructions = builder.current_function_instructions();
    let types = builder.function_state.type_ctx.value_types.clone();
    let literals = builder.function_state.type_ctx.string_literals.clone();
    let (actual, relation) = super::super::checked_scalar_arg(&mut builder, value).unwrap();
    assert_eq!(actual, value);
    assert_eq!(relation.values(), (value, value));
    assert!(relation.target_binding().is_none());
    relation
        .verify(builder.function_state.current_function.as_ref().unwrap())
        .unwrap();
    assert_eq!(builder.current_function_instructions(), instructions);
    assert_eq!(builder.function_state.type_ctx.value_types, types);
    assert_eq!(builder.function_state.type_ctx.string_literals, literals);
    let index = &builder.function_state.local_ssa_correspondence;
    assert!(Rc::ptr_eq(
        index.scalar_source_for(value).unwrap(),
        &original
    ));
    assert!(index.scalar_observations().next().is_none());
}

#[test]
fn selected_scalar_rejects_unreachable_cross_block_reuse() {
    let (mut builder, left) = fixture();
    let original = OriginalScalarAppendV1::fixture_for_correspondence_test(&mut builder, left);
    let value = original.original().1.dst_value().unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    builder
        .function_state
        .local_ssa_correspondence
        .install_scalar_original(function, original)
        .unwrap();
    let target = builder.core_ctx.next_block();
    builder.start_new_block(target).unwrap();
    let error = super::super::checked_scalar_arg(&mut builder, value).unwrap_err();
    assert!(error.contains("scalar-non-dominating-producer"), "{error}");
    assert!(builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks[&target]
        .instructions
        .is_empty());
}

#[test]
fn selected_scalar_rejects_reachable_nondominating_and_duplicate_producers_before_copy() {
    for duplicate in [false, true] {
        let (mut builder, left) = fixture();
        let original = OriginalScalarAppendV1::fixture_for_correspondence_test(&mut builder, left);
        let value = original.original().1.dst_value().unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        builder
            .function_state
            .local_ssa_correspondence
            .install_scalar_original(function, original)
            .unwrap();
        let source = builder.function_state.current_block.unwrap();
        let entry = builder.core_ctx.next_block();
        let bypass = builder.core_ctx.next_block();
        let target = builder.core_ctx.next_block();
        let condition = builder.alloc_value_for_test();
        {
            let function = builder.function_state.current_function.as_mut().unwrap();
            function.entry_block = entry;
            let mut entry_block = crate::mir::BasicBlock::new(entry);
            entry_block.instructions.push(MirInstruction::Const {
                dst: condition,
                value: crate::mir::ConstValue::Bool(true),
            });
            entry_block.set_terminator(MirInstruction::Branch {
                condition,
                then_bb: source,
                else_bb: bypass,
                then_edge_args: None,
                else_edge_args: None,
            });
            function.blocks.insert(entry, entry_block);
            let mut bypass_block = crate::mir::BasicBlock::new(bypass);
            if duplicate {
                bypass_block.instructions.push(MirInstruction::Const {
                    dst: value,
                    value: crate::mir::ConstValue::Integer(9),
                });
            }
            bypass_block.set_terminator(MirInstruction::Jump {
                target,
                edge_args: None,
            });
            function.blocks.insert(bypass, bypass_block);
            function
                .blocks
                .get_mut(&source)
                .unwrap()
                .set_terminator(MirInstruction::Jump {
                    target,
                    edge_args: None,
                });
        }
        builder.start_new_block(target).unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        let dominators = crate::mir::verification::utils::compute_dominators(function);
        assert!(dominators.is_reachable(source) && dominators.is_reachable(target));
        assert!(!dominators.dominates(source, target));
        let error = super::super::checked_scalar_arg(&mut builder, value).unwrap_err();
        let expected = if duplicate {
            "scalar-original-producer"
        } else {
            "scalar-non-dominating-producer"
        };
        assert!(error.contains(expected), "{error}");
        assert!(builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks[&target]
            .instructions
            .is_empty());
        assert!(builder
            .function_state
            .local_ssa_correspondence
            .scalar_observations()
            .next()
            .is_none());
    }
}
