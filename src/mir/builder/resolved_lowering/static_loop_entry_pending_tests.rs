//! Original selected batch checks before collector or ledger publication.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;

impl PreparedSelectedStaticLoopEntryV1<'_, '_> {
    pub(in crate::mir::builder) fn check_unpublished_mutations_for_test(
        self,
        claims: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        let Self {
            pending,
            entry_group,
            body_retention,
            header_route,
            body_route,
            tail_route,
            publication,
            early_root_pending,
            tail_root_pending,
            ..
        } = self;
        let owner = entry_group.source_site().owner();
        let routes = [
            header_route.site().clone(),
            body_route.site().clone(),
            tail_route.site().clone(),
        ];
        let view = PendingSelectedStaticLoopViewV1::new(
            &entry_group,
            body_retention.packet_for_test(),
            [&early_root_pending, &tail_root_pending],
            &routes,
        );
        let unpublished = || {
            assert_eq!(claims.selected_static_entry_group_count_for_test(), 0);
            assert_eq!(claims.emitted_root_exit_count_for_test(), 0);
            assert_eq!(claims.selected_loop_body_packet_count_for_test(), 0);
            assert_eq!(claims.child_physical_validation_count_for_test(), 0);
        };
        unpublished();
        pending.complete_before_restore(|function| {
            // Reborrow the one production validator; successful preparation is
            // a pending receipt, never a committed state.
            drop(claims.validate_selected_static_loop_child_emissions(owner, &function, &view)?);
            claims.validate_selected_loop_artifact_coverage_for_test(owner, &function, &view)?;
            let body_bindings = body_retention.packet_for_test().lifecycle_bindings();
            assert_eq!(
                body_bindings.len(),
                6,
                "header and body each retain Invoke/Normal/Fault"
            );
            let mut checked_bindings = body_bindings.to_vec();
            let entry_invoke = entry_group.original_invoke_binding();
            let MirInstruction::Invoke { normal_landing, fault_landing, .. } = &entry_invoke.1 else { unreachable!() };
            checked_bindings.push(entry_invoke.clone());
            checked_bindings.push((*normal_landing, function.blocks[normal_landing].instructions.iter()
                .find(|row| matches!(row, MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == entry_invoke.0))
                .unwrap().clone()));
            checked_bindings.push((*fault_landing, function.blocks[fault_landing].terminator.as_ref().unwrap().clone()));
            assert_eq!(checked_bindings.len(), 9, "entry/header/body coverage");
            for binding in &checked_bindings {
                let mut changed = function.clone();
                let block = changed.blocks.get_mut(&binding.0).unwrap();
                match &binding.1 {
                    MirInstruction::Invoke { .. } => {
                        let MirInstruction::Invoke { fault_frame, .. } =
                            block.terminator.as_mut().unwrap()
                        else {
                            unreachable!()
                        };
                        *fault_frame = ValueId::new(900001);
                    }
                    MirInstruction::InvokeNormalResult { .. } => {
                        let projection = block
                            .instructions
                            .iter_mut()
                            .find(|row| **row == binding.1)
                            .unwrap();
                        let MirInstruction::InvokeNormalResult { dst, .. } = projection else {
                            unreachable!()
                        };
                        *dst = ValueId::new(900002);
                    }
                    MirInstruction::ReturnFault { .. } => {
                        block.terminator = Some(MirInstruction::ReturnFault {
                            fault_frame: ValueId::new(900003),
                        });
                    }
                    _ => panic!("unexpected retained lifecycle operation"),
                }
                assert!(
                    claims
                        .validate_selected_loop_artifact_coverage_for_test(owner, &changed, &view)
                        .is_err(),
                    "foreign entry/header/body lifecycle binding admitted"
                );
                let mut missing = function.clone();
                let block = missing.blocks.get_mut(&binding.0).unwrap();
                if matches!(binding.1, MirInstruction::InvokeNormalResult { .. }) {
                    block.instructions.retain(|row| row != &binding.1);
                } else {
                    block.terminator = None;
                }
                assert!(
                    claims
                        .validate_selected_loop_artifact_coverage_for_test(owner, &missing, &view)
                        .is_err(),
                    "missing entry/header/body lifecycle binding admitted"
                );
                unpublished();
            }
            for binding in &checked_bindings {
                let mut changed = function.clone();
                match &binding.1 {
                    MirInstruction::Invoke { normal_landing, .. } => {
                        let MirInstruction::Invoke { fault_landing, .. } = changed
                            .blocks
                            .get_mut(&binding.0)
                            .unwrap()
                            .terminator
                            .as_mut()
                            .unwrap()
                        else {
                            unreachable!()
                        };
                        *fault_landing = *normal_landing;
                    }
                    MirInstruction::InvokeNormalResult { .. } => {
                        let projection = changed
                            .blocks
                            .get_mut(&binding.0)
                            .unwrap()
                            .instructions
                            .iter_mut()
                            .find(|row| **row == binding.1)
                            .unwrap();
                        let MirInstruction::InvokeNormalResult { invoke_block, .. } = projection
                        else {
                            unreachable!()
                        };
                        *invoke_block = binding.0;
                    }
                    _ => continue,
                }
                assert!(
                    claims
                        .validate_selected_loop_artifact_coverage_for_test(owner, &changed, &view)
                        .is_err(),
                    "foreign Normal linkage/Fault landing admitted"
                );
                unpublished();
            }
            let mut extra = function.clone();
            let (normal, projection) = &body_bindings[1];
            extra
                .blocks
                .get_mut(normal)
                .unwrap()
                .instructions
                .push(projection.clone());
            assert!(
                claims
                    .validate_selected_loop_artifact_coverage_for_test(owner, &extra, &view)
                    .is_err(),
                "extra lifecycle operation admitted"
            );
            unpublished();
            let reject = |changed: &MirFunction,
                          candidate: &PendingSelectedStaticLoopViewV1<'_, '_>,
                          label: &str| {
                let error = match claims
                    .validate_selected_static_loop_child_emissions(owner, changed, candidate)
                {
                    Ok(_) => panic!("{label}: malformed pending batch admitted"),
                    Err(error) => error,
                };
                assert!(error.contains("[freeze:contract]"), "{label}: {error}");
                unpublished();
            };
            let (normal, _, phis) = body_retention.packet_for_test()
                .header_normal_phi_role().expect("source-checked header Normal role");
            assert!(!phis.is_empty(), "original fixture exercises canonical forwarding PHIs");
            for mutation in 0..5 {
                let mut changed = function.clone();
                let block = changed.blocks.get_mut(&normal).unwrap();
                match mutation {
                    0 => {
                        let MirInstruction::Phi { inputs, .. } = &mut block.instructions[0] else { unreachable!() };
                        inputs[0].0 = normal;
                    }
                    1 => {
                        let MirInstruction::Phi { inputs, .. } = &mut block.instructions[0] else { unreachable!() };
                        inputs[0].1 = ValueId::new(900004);
                    }
                    2 => {
                        let phi = block.instructions.remove(0);
                        block.instructions.push(phi);
                    }
                    3 => { block.instructions.remove(0); }
                    _ => { block.instructions.retain(|row| !matches!(row, MirInstruction::Phi { .. })); }
                }
                reject(&changed, &view, "header Normal PHI prefix drift");
            }
            let returns: Vec<_> = function
                .blocks
                .iter()
                .filter_map(|(id, block)| match block.terminator.as_ref() {
                    Some(MirInstruction::Return { value: Some(value) }) => Some((*id, *value)),
                    _ => None,
                })
                .collect();
            assert_eq!(returns.len(), 2);
            for (index, (block, _)) in returns.iter().enumerate() {
                for replacement in [
                    None,
                    Some(MirInstruction::Return {
                        value: Some(returns[1 - index].1),
                    }),
                ] {
                    let mut changed = function.clone();
                    changed.blocks.get_mut(block).unwrap().terminator = replacement;
                    reject(&changed, &view, "pending Return missing/foreign");
                }
            }
            let calls: Vec<_> = function
                .blocks
                .iter()
                .filter_map(|(id, block)| match block.terminator.as_ref() {
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call { call, .. },
                        ..
                    }) => Some((*id, call)),
                    _ => None,
                })
                .collect();
            let (_, _, original_calls) = publication.into_parts();
            let original_tail = original_calls[3].1.canonical_global_target_v1()?;
            let expected_callee =
                crate::mir::definitions::MirCall::global(None, original_tail, vec![]).callee;
            let tail_calls: Vec<_> = calls
                .iter()
                .filter(|(_, call)| call.callee == expected_callee)
                .collect();
            assert_eq!(tail_calls.len(), 1, "one original source tail target");
            let tail = tail_calls[0].0;
            let other_target = calls
                .iter()
                .find(|(_, call)| !call.args.is_empty())
                .expect("original prior call")
                .1
                .callee
                .clone();
            for wrong_target in [true, false] {
                let mut changed = function.clone();
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, result },
                    ..
                } = changed
                    .blocks
                    .get_mut(&tail)
                    .unwrap()
                    .terminator
                    .as_mut()
                    .unwrap()
                else {
                    unreachable!()
                };
                if wrong_target {
                    call.callee = other_target.clone();
                } else {
                    *result = InvokeCallResultKind::Unit;
                }
                reject(&changed, &view, "pending tail target/result drift");
            }
            for missing in 0..routes.len() {
                let incomplete: Vec<_> = routes
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != missing)
                    .map(|(_, site)| site.clone())
                    .collect();
                let changed_view = PendingSelectedStaticLoopViewV1::new(
                    &entry_group,
                    body_retention.packet_for_test(),
                    [&early_root_pending, &tail_root_pending],
                    &incomplete,
                );
                reject(&function, &changed_view, "pending route exclusion missing");
            }
            unpublished();
            Ok(())
        })
    }
}
