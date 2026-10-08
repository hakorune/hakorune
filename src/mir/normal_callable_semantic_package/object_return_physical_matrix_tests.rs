//! Actual source -> Instance draft -> finishing for the admitted Object result matrix.
use super::*;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;

fn package(
    received: bool,
    opaque: bool,
    nullable: bool,
    owned: bool,
    zero: bool,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let formal = if zero {
        ""
    } else if opaque {
        "size"
    } else {
        "size: i64"
    };
    let actual = if zero { "" } else { "7" };
    let fields = if owned {
        "items: ArrayBox = new ArrayBox() tail: ArrayBox = new ArrayBox() birth() {}"
    } else {
        ""
    };
    let make = if nullable {
        format!(
            "if {} > 0 {{ return new Token() }} return null",
            if zero { "1" } else { "size" }
        )
    } else {
        "return new Token()".to_owned()
    };
    let relay = if received {
        format!("local spare = new Spare() local item = me.make({actual}) return item")
    } else {
        format!("local spare = new Spare() return me.make({actual})")
    };
    issue(&format!("box Spare {{}} box Token {{ {fields} }} box Maker {{ make({formal}) {{ {make} }} relay() {{ {relay} }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap()
}

fn lower_relay(
    package: &VerifiedNormalCallableSemanticPackageV1,
) -> Result<
    (
        crate::mir::resolved_semantics::FunctionOwnerIdV1,
        crate::mir::MirFunction,
    ),
    String,
> {
    let declaration = package.batch().declarations().find(|row| {
        matches!(package.selected.key_for_batch_slot(row.batch_slot()), Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "relay")
    }).unwrap();
    let owner = declaration.owner();
    let key = package
        .selected
        .key_for_batch_slot(declaration.batch_slot())
        .unwrap()
        .clone();
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
    package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, syntax| {
            builder.lower_instance_dependency_for_test(
                input,
                key,
                syntax.method_source_observation().unwrap().clone(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                locator,
            )
        })
        .unwrap()
        .map(|function| (owner, function))
}

#[test]
fn object_return_actual_instance_matrix_preserves_normal_fault_and_finished_bindings() {
    for received in [false, true] {
        for (opaque, zero) in [(false, false), (true, false), (false, true)] {
            for (nullable, owned) in [(false, false), (true, false), (false, true)] {
                let label = format!(
                    "received={received} opaque={opaque} nullable={nullable} owned={owned} zero={zero}"
                );
                let package = package(received, opaque, nullable, owned, zero);
                let ledger = &package.ordinary_new_claim_ledger;
                let object = ledger
                    .pending_result_claims_for_test()
                    .values()
                    .find(|claim| claim.class() == "Token")
                    .unwrap()
                    .object();
                let fields: Vec<_> = ledger
                    .owned_field_children_for(object)
                    .flatten()
                    .unwrap_or(&[])
                    .iter()
                    .rev()
                    .map(|child| child.field)
                    .collect();
                assert_eq!(fields.len(), if owned { 2 } else { 0 }, "{label}");
                let expected_target = package
                    .batch()
                    .declarations()
                    .find_map(
                        |row| match package.selected.key_for_batch_slot(row.batch_slot()) {
                            Some(SelectedNormalCallableKeyV1::Cataloged(key))
                                if key.name() == "make" =>
                            {
                                Some(key.clone())
                            }
                            _ => None,
                        },
                    )
                    .unwrap();
                let expected_kind = if nullable {
                    InvokeCallResultKind::NullableHandle
                } else {
                    InvokeCallResultKind::Handle
                };
                let (owner, function) =
                    lower_relay(&package).unwrap_or_else(|e| panic!("{label}: {e}"));
                let calls: Vec<_> = function.blocks.iter().filter_map(|(id, block)| match block.terminator.as_ref() {
                    Some(MirInstruction::Invoke { operation: InvokeOperation::Call { result, call }, normal_landing, fault_landing, .. })
                        if matches!(&call.callee, hakorune_mir_defs::Callee::SameModuleInstance { key, .. } if key == &expected_target) => {
                        assert_eq!(*result, expected_kind, "{label}");
                        Some((*id, *normal_landing, *fault_landing))
                    }
                    _ => None,
                }).collect();
                let [(origin, normal, acquisition_fault)] = calls.as_slice() else {
                    panic!("{label}: original call {calls:?}")
                };
                let results: Vec<_> = function
                    .blocks
                    .values()
                    .flat_map(|block| block.all_instructions())
                    .filter_map(|instruction| match instruction {
                        MirInstruction::InvokeNormalResult { invoke_block, dst }
                            if invoke_block == origin =>
                        {
                            Some(*dst)
                        }
                        _ => None,
                    })
                    .collect();
                let [result] = results.as_slice() else {
                    panic!("{label}: original projection {results:?}")
                };
                assert!(function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Return { value: Some(value) } if value == result)), "{label}");
                let cleanup_head = follow_jumps(&function, *normal);
                let Some(MirInstruction::Invoke {
                    operation: InvokeOperation::HomeRelease { value: prior, .. },
                    fault_landing: cleanup_fault,
                    ..
                }) = function.blocks[&cleanup_head].terminator.as_ref()
                else {
                    panic!("{label}: Normal prior Home")
                };
                assert_ne!(prior, result, "{label}");
                assert_eq!(home_values(&function, *normal), vec![*prior], "{label}");
                assert_eq!(
                    home_values(&function, *acquisition_fault),
                    vec![*prior],
                    "{label}"
                );
                assert!(
                    !reachable_release(&function, *acquisition_fault, *result),
                    "{label}"
                );
                let fault = operations(&function, *cleanup_fault);
                let actual_fields: Vec<_> = fault
                    .iter()
                    .filter_map(|operation| match operation {
                        InvokeOperation::OwnedFieldResidenceRelease { field, base }
                            if base == result =>
                        {
                            Some(*field)
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(actual_fields, fields, "{label}");
                assert_eq!(
                    home_values(&function, *cleanup_fault),
                    vec![*result],
                    "{label}"
                );
                assert!(
                    reachable_release(&function, *cleanup_fault, *result),
                    "{label}"
                );
                assert!(
                    matches!(fault.last(), Some(InvokeOperation::HomeReleaseIfLive { value, .. }) if nullable && value == result)
                        || matches!(fault.last(), Some(InvokeOperation::HomeRelease { value, .. }) if !nullable && value == result),
                    "{label}: {fault:?}"
                );
                ledger
                    .validate_finalized_child_emissions(owner, &function)
                    .unwrap_or_else(|e| panic!("{label}: {e}"));
                if received {
                    ledger
                        .validate_new_emissions_after_local_pool_move_for_test(owner, &function)
                        .unwrap_or_else(|error| {
                            panic!("{label}: same packet after pool move: {error}")
                        });
                }
                crate::mir::verification::MirVerifier::new_strict()
                    .verify_function(&function)
                    .unwrap_or_else(|errors| panic!("{label}: strict draft {errors:?}"));
                let mut changed = function.clone();
                let Some(MirInstruction::Invoke {
                    operation: InvokeOperation::Call { result: kind, .. },
                    ..
                }) = &mut changed.blocks.get_mut(origin).unwrap().terminator
                else {
                    panic!("{label}")
                };
                *kind = InvokeCallResultKind::I64;
                assert!(
                    ledger
                        .validate_finalized_child_emissions(owner, &changed)
                        .is_err(),
                    "{label}: wrong physical kind"
                );
                let mut module = MirModule::new(label.clone());
                module
                    .functions
                    .insert(function.signature.name.clone(), function);
                crate::mir::passes::simplify_cfg::simplify(&mut module);
                for function in module.functions.values() {
                    crate::mir::verification::MirVerifier::new_strict()
                        .verify_function(function)
                        .unwrap_or_else(|errors| panic!("{label}: strict finishing {errors:?}"));
                }
                ledger
                    .validate_finalized_child_functions(&module, false)
                    .unwrap_or_else(|e| panic!("{label}: {e}"));
                assert_eq!(
                    ledger.validate_finalized_child_functions(&module, true).unwrap_err(),
                    "[freeze:contract][ordinary-new/local-commit/object-return-handoff-unavailable]",
                    "{label}: artifact guard"
                );
            }
        }
    }
}

#[test]
fn object_return_actual_instance_nullable_owned_fields_keep_the_physical_stop() {
    for received in [false, true] {
        for opaque in [false, true] {
            let package = package(received, opaque, true, true, false);
            let label = format!("received={received} opaque={opaque}");
            let ledger = std::rc::Rc::clone(&package.ordinary_new_claim_ledger);
            let owner = package.batch().declarations().find(|row| {
                matches!(package.selected.key_for_batch_slot(row.batch_slot()), Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key.name() == "relay")
            }).unwrap().owner();
            let exit = ledger
                .sole_terminal_relation_for_owner(owner)
                .unwrap()
                .return_site();
            if received {
                assert_eq!(
                    lower_relay(&package).unwrap_err(),
                    "[freeze:contract][ordinary-new/local-commit/handle-release-unproven]",
                    "{label}: nullable owned-field physical refusal"
                );
                assert_eq!(
                    ledger
                        .prepare_root_home_exit(owner, exit.node())
                        .unwrap_err(),
                    "[freeze:contract][ordinary-new/local-commit/duplicate-root-exit-prepare]",
                    "{label}: refused lowering already attempted this affine exit prepare"
                );
            } else {
                let source = ledger
                    .verified_direct_object_return_source_v1(owner, exit)
                    .unwrap()
                    .unwrap();
                assert!(
                    source.teardown().is_none(),
                    "{label}: no nullable field teardown envelope"
                );
                assert!(
                    !ledger.object_return_construction_ready_v1(owner).unwrap(),
                    "{label}"
                );
                assert!(
                    !ledger.prepare_root_home_exit(owner, exit.node()).unwrap(),
                    "{label}"
                );
                let call = source.target();
                assert_eq!(
                    ledger
                        .take_receiver_object_packet_v1(
                            owner,
                            call.call_site().site(),
                            call.target()
                        )
                        .unwrap_err(),
                    "[freeze:contract][ordinary-new/receiver-object/local-source-missing]",
                    "{label}: terminal is not local"
                );
            }
            assert_eq!(
                package
                    .ordinary_new_claim_ledger
                    .validate_no_pending_object_returns_v1()
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/local-commit/object-return-handoff-unavailable]"
            );
        }
    }
}

fn operations(
    function: &crate::mir::MirFunction,
    mut head: crate::mir::BasicBlockId,
) -> Vec<InvokeOperation> {
    let mut result = Vec::new();
    let mut visited = std::collections::BTreeSet::new();
    loop {
        assert!(visited.insert(head), "cyclic cleanup");
        match function.blocks[&head].terminator.as_ref() {
            Some(MirInstruction::Jump { target, .. }) => head = *target,
            Some(MirInstruction::Invoke {
                operation,
                normal_landing,
                ..
            }) => {
                result.push(operation.clone());
                head = *normal_landing;
            }
            Some(MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. }) => {
                return result
            }
            other => panic!("unexpected cleanup terminal: {other:?}"),
        }
    }
}

fn home_values(function: &crate::mir::MirFunction, head: crate::mir::BasicBlockId) -> Vec<ValueId> {
    operations(function, head)
        .iter()
        .filter_map(|operation| match operation {
            InvokeOperation::HomeRelease { value, .. }
            | InvokeOperation::HomeReleaseIfLive { value, .. } => Some(*value),
            _ => None,
        })
        .collect()
}
