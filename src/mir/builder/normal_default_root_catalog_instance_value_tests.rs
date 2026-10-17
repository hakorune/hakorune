//! Instance-value lifecycle cases borrowing the root-catalog harness.
use super::{
    callable_source, session, CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1,
    ParserBuildConfig,
};

/// The original receiver Call requires a completed source object contract.
/// Keep both old untyped fixtures as unavailable, and exercise typed/untyped
/// caller arguments with declared fields through the common packet writer.
#[test]
fn nullable_receiver_arguments_require_completed_source_object_contract() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    for formal in ["flag: i64", "flag"] {
        for (fields, admitted) in [("init { v }", false), ("v: i64", true)] {
            let text = format!(
                r#"
box Probe {{
    {fields}
    birth(v) {{ me.v = v }}
    fetch(flag) {{
        if flag == 0 {{ return null }}
        return new Probe(7)
    }}
    run({formal}) {{
        local h = me.fetch(flag)
        return 0
    }}
}}
static box Main {{
    main() {{
        local p = new Probe(1)
        return p.run(0)
    }}
}}
"#
            );
            let result = session().complete_normal_default_program_root_catalog_lifecycle(
                callable_source(&text, ParserBuildConfig::default()),
                CallableMainMaterializationPolicyV1::Omitted,
                NormalRuntimeInputSnapshotV1::empty(),
            );
            if !admitted {
                let rejected =
                    result.expect_err("untyped fields cannot supply a completed object packet");
                assert!(
                    rejected
                        .error()
                        .to_string()
                        .contains("lexical-instance-call/source-pending"),
                    "{formal}: unexpected source refusal: {}",
                    rejected.error()
                );
                rejected.discard();
                continue;
            }
            let (_, module, _) = result
                .expect("declared fields must admit the original nullable Call")
                .into_parts();
            let run = module
                .functions
                .values()
                .find(|function| function.signature.name == "Probe.run/1")
                .expect("original Probe.run");
            let invokes: Vec<_> = run
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter_map(|instruction| match instruction {
                    crate::mir::MirInstruction::Invoke { operation, .. } => Some(operation),
                    _ => None,
                })
                .collect();
            assert!(
                invokes.iter().any(|operation| matches!(
                    operation,
                    crate::mir::instruction::InvokeOperation::Call {
                        result: crate::mir::instruction::InvokeCallResultKind::NullableHandle,
                        ..
                    }
                )),
                "{formal}: nullable invoke missing: {invokes:?}"
            );
            assert!(
                invokes.iter().any(|operation| matches!(
                    operation,
                    crate::mir::instruction::InvokeOperation::HomeReleaseIfLive { .. }
                )),
                "{formal}: nullable result owes checked release: {invokes:?}"
            );
            assert!(
                invokes.iter().all(|operation| !matches!(
                    operation,
                    crate::mir::instruction::InvokeOperation::HomeRelease { .. }
                )),
                "{formal}: nullable result cannot owe unconditional release: {invokes:?}"
            );
        }
    }
}

/// A Bool actual cannot establish the whole original formal's integer
/// domain for `flag == 0`. Reject at that source contract before emitting
/// a nullable call; wire representation alone cannot authorize comparison.
#[test]
fn nullable_receiver_call_rejects_concrete_non_i64_local_argument() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Probe {
    init { v }
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new Probe(7)
    }
    run() {
        local b = true
        local h = me.fetch(b)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run()
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("a Bool actual must not establish an integer comparison domain");
    assert!(
        rejected
            .error()
            .to_string()
            .contains("ordinary-new/borrowed-compare/integer-source-unproved"),
        "unexpected rejection: {}",
        rejected.error()
    );
    rejected.discard();
}

/// Duplicate Birth stores keep construction unavailable. A retained raw
/// diagnostic must not become a completed artifact or receive a result End.
/// Valid nullable-to-New moves are covered by the original child-handle
/// publication tests and the received-Birth final-consumer mutations.
#[test]
fn nullable_result_into_unsupported_new_stays_unavailable() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box OwMoveHolder {
    v: i64
    birth(v) {
        me.v = v
        me.v = v
    }
}
box OwMoveProbe {
    v: i64
    birth(v) { me.v = v }
    fetch(flag) {
        if flag == 0 {
            return null
        }
        return new OwMoveProbe(7)
    }
    run(flag: i64) {
        local h = me.fetch(flag)
        if h == null {
            return new OwMoveHolder(0)
        }
        return new OwMoveHolder(h)
    }
}
static box OwMoveMain {
    main() {
        local p = new OwMoveProbe(1)
        return p.run(0)
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let rejected = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect_err("duplicate Birth stores must not complete through retained diagnostics");
    assert!(
        rejected
            .error()
            .to_string()
            .contains("IncompleteOrdinaryNewCoverage"),
        "unexpected rejection: {}",
        rejected.error()
    );
    rejected.discard();
}

/// `local r = pool.allocate(8)` — a claim-local lexical receiver call
/// whose callee returns an exact-`i64` scalar — lowers through the
/// lexical lifecycle lane: the Standard-route gate corroborates the
/// sealed observation with the minted disposition row and emits
/// `Invoke{SameModuleInstance, I64}` — the receiver rides the callee's
/// `me` slot outside the source arguments, the literal `8` materializes
/// inside them, and the bound result is registered `Integer` so the
/// terminal `return r` type-checks.
#[test]
fn lexical_i64_instance_call_emits_lifecycle_invoke() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Pool {
    birth() { }
    allocate(size: i64): i64 { return size }
}
static box Main {
    main() {
        local pool = new Pool()
        local r = pool.allocate(8)
        return r
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("lexical i64 instance call must lower");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered main function");
    let invokes: Vec<_> = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Invoke {
                operation: crate::mir::instruction::InvokeOperation::Call { call, result },
                ..
            } => Some((call, *result)),
            _ => None,
        })
        .collect();
    let (call, result) = invokes
        .iter()
        .find(|(call, _)| {
            matches!(
                &call.callee,
                crate::mir::Callee::SameModuleInstance { key, .. }
                    if key.namespace()
                        == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                        && key.owner() == "Pool"
                        && key.name() == "allocate"
                        && key.arity() == 1
            )
        })
        .expect("lexical i64 instance invoke");
    assert_eq!(*result, crate::mir::instruction::InvokeCallResultKind::I64);
    assert_eq!(
        call.args.len(),
        1,
        "the literal argument materializes inside the source args"
    );
}

/// `local r = pool.give(pool.allocate(8))` — a proven-i64 call nested in
/// direct argument position of another proven-i64 lexical call — lowers
/// as two ordered `Invoke{SameModuleInstance, I64}` instructions: the
/// inner `allocate` result feeds the outer `give` argument slot, and
/// both invocations fold into the outer statement's single recorded
/// binding group (the inner call owns no destination binding).
#[test]
fn lexical_i64_call_result_argument_emits_ordered_invokes() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Pool {
    birth() { }
    allocate(size: i64): i64 { return size }
    give(p: i64): i64 { return p }
}
static box Main {
    main() {
        local pool = new Pool()
        local r = pool.give(pool.allocate(8))
        return r
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("nested call-result argument must lower");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered main function");
    let invoke_at = |name: &str| {
        main.blocks
            .iter()
            .find_map(|(id, block)| {
                block
                    .all_instructions()
                    .find(|instruction| {
                        matches!(
                            instruction,
                            crate::mir::MirInstruction::Invoke {
                                operation:
                                    crate::mir::instruction::InvokeOperation::Call {
                                        call,
                                        result
                                    },
                                ..
                            } if *result == crate::mir::instruction::InvokeCallResultKind::I64
                                && matches!(
                                    &call.callee,
                                    crate::mir::Callee::SameModuleInstance { key, .. }
                                        if key.namespace()
                                            == hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                                            && key.owner() == "Pool"
                                            && key.name() == name
                                )
                        )
                    })
                    .map(|instruction| (*id, instruction))
            })
            .unwrap_or_else(|| panic!("{name} i64 instance invoke"))
    };
    let (allocate_block, allocate_invoke) = invoke_at("allocate");
    let (give_block, give_invoke) = invoke_at("give");
    let (
        crate::mir::MirInstruction::Invoke {
            normal_landing: allocate_landing,
            ..
        },
        crate::mir::MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::Call { call, .. },
            ..
        },
    ) = (allocate_invoke, give_invoke)
    else {
        panic!("invoke shapes")
    };
    // The outer call emits inside the inner call's normal-landing block —
    // the CFG edge is the ordering authority, not block-map iteration.
    assert_eq!(
        give_block, *allocate_landing,
        "the consuming invoke seats on the argument call's normal edge"
    );
    // The inner projection names the inner result value, which feeds the
    // outer argument slot.
    let inner_result = main
        .blocks
        .get(allocate_landing)
        .and_then(|block| {
            block
                .all_instructions()
                .find_map(|instruction| match instruction {
                    crate::mir::MirInstruction::InvokeNormalResult { invoke_block, dst }
                        if *invoke_block == allocate_block =>
                    {
                        Some(*dst)
                    }
                    _ => None,
                })
        })
        .expect("inner invoke normal projection");
    assert!(
        call.args.contains(&inner_result),
        "the inner call result feeds the outer argument slot: {:?}",
        call.args
    );
}

/// `local n = pool.size` — a proven local-initializer field read on a
/// claim-local `new` Home — lowers as one `ObjectFieldGet` against the
/// receiver's materialized value, typed `Integer` from the sealed field
/// declaration (never a layout guess).
#[test]
fn local_field_read_emits_object_field_get_scalar() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Pool {
    size: i64 = 0
    birth() { }
}
static box Main {
    main() {
        local pool = new Pool()
        local n = pool.size
        return n
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("local scalar field read must lower");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered main function");
    let reads: Vec<_> = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::ObjectFieldGet { dst, base, field } => {
                Some((*dst, *base, *field))
            }
            _ => None,
        })
        .collect();
    let [(dst, _, field)] = reads.as_slice() else {
        panic!("exactly one ObjectFieldGet, got {reads:?}")
    };
    assert_eq!(
        field.declaration_ordinal(),
        0,
        "the read names Pool's `size` declaration"
    );
    assert_eq!(
        main.metadata.value_types.get(dst),
        Some(&crate::mir::MirType::Integer),
        "the scalar read destination is typed Integer from the declaration"
    );
}

/// `local page = pool.page; local n = page.alloc` — a proven
/// ordinary-box field read binds a borrowed alias, and a later read on
/// that alias lowers as a second `ObjectFieldGet` whose base is exactly
/// the first read's destination. The alias destination is typed
/// `Box("Page")` from the sealed declaration — never `Integer`.
#[test]
fn local_field_read_emits_chained_alias_object_field_get() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box Page {
    alloc: i64 = 0
    birth() { }
}
box Pool {
    page: Page = new Page()
    birth() { }
}
static box Main {
    main() {
        local pool = new Pool()
        local page = pool.page
        local n = page.alloc
        return n
    }
}
"#,
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("chained field read must lower");
    let (_, module, _) = completed.into_parts();
    let main = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "main")
        .map(|(_, function)| function)
        .expect("lowered main function");
    let reads: Vec<_> = main
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::ObjectFieldGet { dst, base, field } => {
                Some((*dst, *base, *field))
            }
            _ => None,
        })
        .collect();
    // `pool.page` first, `page.alloc` second — the alias read must sit on
    // the alias binding's materialized value (a Copy of the first read's
    // destination), never on the receiver root.
    let [(alias_dst, pool_base, _), (scalar_dst, alias_base, _)] = reads.as_slice() else {
        panic!("two ObjectFieldGet instructions, got {reads:?}")
    };
    assert!(
        main.blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|instruction| matches!(
                instruction,
                crate::mir::MirInstruction::Copy { dst, src }
                    if *dst == *alias_base && *src == *alias_dst
            )),
        "the second read bases on the alias binding materialized from the first read: {reads:?}"
    );
    assert_ne!(
        *alias_base, *pool_base,
        "the alias read must not re-read the receiver root"
    );
    assert_eq!(
        main.metadata.value_types.get(alias_dst),
        Some(&crate::mir::MirType::Box("Page".to_owned())),
        "the alias destination keeps the declared Page class"
    );
    assert_eq!(
        main.metadata.value_types.get(scalar_dst),
        Some(&crate::mir::MirType::Integer),
        "the scalar read destination is typed Integer"
    );
}
