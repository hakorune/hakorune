//! Actual source body -> Instance draft -> diagnostic finishing; no artifact grant.
use super::*;

#[test]
fn direct_object_return_actual_instance_draft_keeps_normal_result_and_prior_home() {
    let package = issue("box Token {} box Maker { make(size: i64) { return new Token() } relay() { local spare = new Token() return me.make(7) } } static box Main { main() { return 0 } }").unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let declaration = package.batch().declarations().find(|row| {
        matches!(package.selected.key_for_batch_slot(row.batch_slot()), Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "relay")
    }).unwrap();
    let owner = declaration.owner();
    let key = package
        .selected
        .key_for_batch_slot(declaration.batch_slot())
        .unwrap()
        .clone();
    let exit = ledger
        .sole_terminal_relation_for_owner(owner)
        .unwrap()
        .return_site();
    let source = ledger
        .verified_direct_object_return_source_v1(owner, exit)
        .unwrap()
        .unwrap();
    let expected_target = source.target().target().clone();
    let mut builder = MirBuilder::new();
    let consumed = std::cell::RefCell::new(std::collections::BTreeSet::new());
    let locator =
        crate::mir::normal_callable_semantic_package::DeclaredInstanceCallLocatorScopeV1::new(
            crate::mir::normal_callable_semantic_package::DeclaredInstanceCallLocatorViewV1::new(
                package.declared_instance_call_locators(),
                package.batch().declared_instance_call_source(),
            ),
            &consumed,
        );
    let function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, syntax| {
            builder.lower_instance_dependency_for_test(
                input,
                key,
                syntax.method_source_observation().unwrap().clone(),
                std::rc::Rc::clone(ledger),
                locator,
            )
        })
        .unwrap()
        .expect("same-source Instance relay reaches sole physical owner");
    let results: Vec<_> = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| {
            let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction else {
                return None;
            };
            function.blocks[invoke_block]
                .all_instructions()
                .any(|i| {
                    matches!(
                        i,
                        MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                result: InvokeCallResultKind::Handle,
                                call,
                            },
                            ..
                        } if matches!(&call.callee, hakorune_mir_defs::Callee::SameModuleInstance { key, .. } if key == &expected_target)
                    )
                })
                .then_some((*invoke_block, *dst))
        })
        .collect();
    let [(invoke_origin, result)] = results.as_slice() else {
        panic!("exactly one original acquired result: {results:?}");
    };
    assert!(function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|i| matches!(i, MirInstruction::Return { value: Some(actual) } if actual == result)));
    let (normal, acquisition_fault) = function.blocks[invoke_origin]
        .all_instructions()
        .find_map(|i| match i {
            MirInstruction::Invoke {
                normal_landing,
                fault_landing,
                ..
            } => Some((*normal_landing, *fault_landing)),
            _ => None,
        })
        .unwrap();
    let cleanup_head = follow_jumps(&function, normal);
    let Some(MirInstruction::Invoke {
        operation: InvokeOperation::HomeRelease {
            value: prior_home, ..
        },
        fault_landing: cleanup_fault,
        ..
    }) = function.blocks[&cleanup_head].terminator.as_ref()
    else {
        panic!("original call Normal must first release the prior Home");
    };
    assert_ne!(prior_home, result);
    assert_eq!(cleanup_path_releases(&function, normal), vec![*prior_home]);
    assert_eq!(
        cleanup_path_releases(&function, acquisition_fault),
        vec![*prior_home]
    );
    assert_eq!(
        cleanup_path_releases(&function, *cleanup_fault),
        vec![*result]
    );
    assert!(
        !reachable_release(&function, acquisition_fault, *result),
        "acquisition Fault cannot own the result"
    );
    assert!(
        reachable_release(&function, *cleanup_fault, *result),
        "cleanup Fault owns the acquired result"
    );
    let mut changed = function.clone();
    let foreign_origin = function.entry_block;
    assert_ne!(*invoke_origin, foreign_origin);
    let mut mutations = 0;
    for block in changed.blocks.values_mut() {
        for instruction in &mut block.instructions {
            if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                if dst == result {
                    *invoke_block = foreign_origin;
                    mutations += 1;
                }
            }
        }
    }
    assert_eq!(mutations, 1);
    let error = ledger
        .validate_finalized_child_emissions(owner, &changed)
        .unwrap_err();
    assert!(error.contains("[freeze:contract]"), "{error}");
    ledger
        .validate_finalized_child_emissions(owner, &function)
        .unwrap();
    let mut module = MirModule::new("object-return-diagnostic".into());
    module
        .functions
        .insert(function.signature.name.clone(), function);
    crate::mir::passes::simplify_cfg::simplify(&mut module);
    ledger
        .validate_finalized_child_functions(&module, false)
        .unwrap();
    assert!(ledger.validate_no_pending_object_returns_v1().is_err());
}

fn reachable_release(
    function: &crate::mir::MirFunction,
    start: crate::mir::BasicBlockId,
    result: ValueId,
) -> bool {
    let mut pending = vec![start];
    let mut visited = std::collections::BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        let block = &function.blocks[&id];
        if block.all_instructions().any(|i| matches!(i,
            MirInstruction::Invoke { operation: InvokeOperation::HomeRelease { value, .. } | InvokeOperation::HomeReleaseIfLive { value, .. }, .. } if *value == result)) {
            return true;
        }
        pending.extend(block.out_edges().iter().map(|edge| edge.target));
    }
    false
}

#[path = "object_return_physical_matrix_tests.rs"]
mod matrix;
