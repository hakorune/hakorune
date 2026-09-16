use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::parser::ParserBuildConfig;

#[test]
fn source_backed_map_consumer_child_receives_and_releases_its_lease() {
    // C8 acceptance, production route: `main -> use_map -> make_map`. The
    // non-AppMain `use_map` owner consumes its own sealed loan row, installs
    // the received lease, and releases it once on its exit chain.
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"static box Main {
            main() { local r = use_map(10) return 30 }
            use_map(seed: i64): i64 { local m = make_map() return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("map consumer source must lower through the package route");
    let (_, module, validate) = completed.into_artifact_parts();
    let use_map = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("use_map"))
        .expect("cataloged use_map definition");
    // One Map-result invoke; its normal projection binds the received lease.
    let received: Vec<crate::mir::ValueId> = use_map
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::InvokeNormalResult { invoke_block, dst }
                if use_map.blocks[invoke_block].all_instructions().any(|i| {
                    matches!(
                        i,
                        crate::mir::MirInstruction::Invoke {
                            operation:
                                crate::mir::instruction::InvokeOperation::Call {
                                    result:
                                        crate::mir::instruction::InvokeCallResultKind::Map,
                                    ..
                                },
                            ..
                        }
                    )
                }) =>
            {
                Some(*dst)
            }
            _ => None,
        })
        .collect();
    let [received] = received.as_slice() else {
        panic!("exactly one map-result projection expected")
    };
    let received = *received;
    // Callee Fault: the receive invoke's fault landing returns the fault
    // frame without installing a result.
    let fault_landing = use_map
        .blocks
        .values()
        .find_map(|block| match &block.terminator {
            Some(crate::mir::MirInstruction::Invoke {
                operation:
                    crate::mir::instruction::InvokeOperation::Call {
                        result: crate::mir::instruction::InvokeCallResultKind::Map,
                        ..
                    },
                fault_landing,
                ..
            }) => Some(*fault_landing),
            _ => None,
        })
        .expect("map-result invoke fault landing");
    assert!(
        matches!(
            use_map.blocks[&fault_landing].terminator,
            Some(crate::mir::MirInstruction::ReturnFault { .. })
        ),
        "callee fault returns the borrowed frame"
    );
    // Normal cleanup: the caller's exit releases the received lease once.
    let ends = use_map
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| {
            matches!(
                instruction,
                crate::mir::MirInstruction::Invoke {
                    operation:
                        crate::mir::instruction::InvokeOperation::Map(
                            crate::mir::instruction::MapInvokeOperation::End { map },
                        ),
                    ..
                } if *map == received
            )
        })
        .count();
    assert_eq!(ends, 1, "use_map releases the received lease exactly once");
    let main = module
        .functions
        .values()
        .find(|function| function.signature.name == "main")
        .expect("lowered main");
    assert_eq!(
        main.blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, crate::mir::MirInstruction::Call(_)))
            .count(),
        1,
        "main's scalar use_map call stays on the sealed row"
    );
    // Artifact validation covers every lifecycle site of every owner.
    validate(&module).expect("artifact lifecycle coverage");
}

#[test]
fn source_backed_map_consumer_fault_suffix_still_releases_pending_homes() {
    // Caller Fault: with a second pending home behind the received lease,
    // a fault anywhere the lease is live still frees it — the literal's
    // fallible construction drains live homes on its fault paths, and the
    // exit keeps LIFO releases (the received lease ends last on the clean
    // chain and again on the pending chain).
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"static box Main {
            main() { return use_map(10) }
            use_map(seed: i64): i64 { local m = make_map() local n = %{"b" => 2} return 42 }
            make_map() { return %{"a" => 1} }
        }"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("map consumer source must lower through the package route");
    let (_, module, validate) = completed.into_artifact_parts();
    let use_map = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("use_map"))
        .expect("cataloged use_map definition");
    let follow_jumps = |mut id: crate::mir::BasicBlockId| {
        loop {
            match &use_map.blocks[&id].terminator {
                Some(crate::mir::MirInstruction::Jump {
                    target,
                    edge_args: None,
                }) => id = *target,
                _ => return id,
            }
        }
    };
    // The map-result invoke binds the received lease through its normal
    // projection.
    let received = use_map
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .find_map(|instruction| match instruction {
            crate::mir::MirInstruction::InvokeNormalResult { invoke_block, dst }
                if use_map.blocks[invoke_block].all_instructions().any(|i| {
                    matches!(
                        i,
                        crate::mir::MirInstruction::Invoke {
                            operation:
                                crate::mir::instruction::InvokeOperation::Call {
                                    result:
                                        crate::mir::instruction::InvokeCallResultKind::Map,
                                    ..
                                },
                            ..
                        }
                    )
                }) =>
            {
                Some(*dst)
            }
            _ => None,
        })
        .expect("one map-result projection binds the received lease");
    let ends: Vec<(crate::mir::ValueId, crate::mir::BasicBlockId, crate::mir::BasicBlockId)> = use_map
        .blocks
        .values()
        .filter_map(|block| match &block.terminator {
            Some(crate::mir::MirInstruction::Invoke {
                operation:
                    crate::mir::instruction::InvokeOperation::Map(
                        crate::mir::instruction::MapInvokeOperation::End { map },
                    ),
                normal_landing,
                fault_landing,
                ..
            }) => Some((*map, *normal_landing, *fault_landing)),
            _ => None,
        })
        .collect();
    // Caller Fault: every fault path that runs while the received lease is
    // live releases it — the literal's own fallible construction (New /
    // PrepareKey / InstallValue / EndOutcome) drains the live lease, and
    // the exit's pending chain releases it behind the outermost clean
    // release. Exactly one End(m) sits on the clean chain ahead of
    // `return 42`; every other End(m) drains to ReturnFault.
    let received_ends: Vec<_> = ends
        .iter()
        .filter(|(map, ..)| *map == received)
        .collect();
    assert!(
        received_ends.len() >= 2,
        "the received lease must release on the clean chain and on fault"
    );
    let clean_releases = received_ends
        .iter()
        .filter(|(_, normal, _)| {
            matches!(
                use_map.blocks[&follow_jumps(*normal)].terminator,
                Some(crate::mir::MirInstruction::Return {
                    value: Some(_)
                })
            )
        })
        .count();
    assert_eq!(
        clean_releases, 1,
        "exactly one received-lease release reaches the normal Return"
    );
    for (map, normal, fault) in &received_ends {
        if matches!(
            use_map.blocks[&follow_jumps(*normal)].terminator,
            Some(crate::mir::MirInstruction::Return {
                value: Some(_)
            })
        ) {
            continue;
        }
        for landing in [normal, fault] {
            assert!(
                matches!(
                    use_map.blocks[&follow_jumps(*landing)].terminator,
                    Some(crate::mir::MirInstruction::ReturnFault { .. })
                ),
                "fault-path release of {map:?} must drain into ReturnFault"
            );
        }
    }
    // The second pending home keeps the same contract: one clean release
    // ahead of Return and at least one fault-path release.
    let literal_ends: Vec<_> = ends
        .iter()
        .filter(|(map, ..)| *map != received)
        .collect();
    assert!(
        literal_ends.len() >= 2,
        "the pending literal home must release on clean and fault paths"
    );
    validate(&module).expect("artifact lifecycle coverage");
}

#[test]
fn source_backed_map_consumer_release_drift_rejects_at_artifact_validation() {
    // Missing/moved lifecycle evidence rejects at the artifact boundary,
    // not at process exit.
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = || {
        callable_source(
            r#"static box Main {
                main() { local r = use_map(10) return 30 }
                use_map(seed: i64): i64 { local m = make_map() return 42 }
                make_map() { return %{"a" => 1} }
            }"#,
            ParserBuildConfig::default(),
        )
    };
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source(),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("map consumer source must lower");
    let (_, mut module, validate) = completed.into_artifact_parts();
    let use_map = module
        .functions
        .values_mut()
        .find(|function| function.signature.name.contains("use_map"))
        .expect("cataloged use_map definition");
    let mut drifted = false;
    for block in use_map.blocks.values_mut() {
        if let Some(crate::mir::MirInstruction::Invoke {
            operation:
                crate::mir::instruction::InvokeOperation::Map(
                    crate::mir::instruction::MapInvokeOperation::End { map },
                ),
            ..
        }) = &mut block.terminator
        {
            *map = crate::mir::ValueId(999);
            drifted = true;
            break;
        }
    }
    assert!(drifted, "the exit release exists to be drifted");
    let error = validate(&module).expect_err("release drift must reject");
    assert!(!error.is_empty());
}
