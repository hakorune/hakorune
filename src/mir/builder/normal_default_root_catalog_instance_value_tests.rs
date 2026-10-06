//! Instance-value lifecycle cases borrowing the root-catalog harness.
use super::{
    callable_source, session, CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1,
    ParserBuildConfig,
};

/// `local h = me.fetch(..)` where `fetch` claims `NullableObject` lowers
/// through the nullable lifecycle lane: the call site emits the
/// `NullableHandle` invoke result and the caller's exit chain owes the
/// received value a checked `HomeReleaseIfLive` — never an unconditional
/// `HomeRelease` on a value that may carry the `Void` sentinel.
#[test]
fn nullable_receiver_call_emits_lifecycle_invoke_and_checked_release() {
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
    run(flag: i64) {
        local h = me.fetch(flag)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run(0)
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
        .expect("nullable receiver call must lower");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "Probe.run/1")
        .map(|(_, function)| function)
        .expect("lowered Probe.run function");
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
        "nullable receiver call must emit the NullableHandle invoke: {invokes:?}"
    );
    assert!(
        invokes.iter().any(|operation| matches!(
            operation,
            crate::mir::instruction::InvokeOperation::HomeReleaseIfLive { .. }
        )),
        "the received nullable owes a checked release at exit: {invokes:?}"
    );
    assert!(
        invokes.iter().all(|operation| !matches!(
            operation,
            crate::mir::instruction::InvokeOperation::HomeRelease { .. }
        )),
        "a nullable result must never emit an unconditional release: {invokes:?}"
    );
}

/// An unannotated parameter argument rides the scalar call edge under
/// the sealed untyped admission (`check_call_edge`): `flag` records
/// `MirType::Unknown` and the nullable invoke still emits — the wire
/// carries the binding's slot, never a guessed carrier.
#[test]
fn nullable_receiver_call_admits_untyped_parameter_argument() {
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
    run(flag) {
        local h = me.fetch(flag)
        return 0
    }
}
static box Main {
    main() {
        local p = new Probe(1)
        return p.run(0)
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
        .expect("untyped parameter argument must ride the scalar edge");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "Probe.run/1")
        .map(|(_, function)| function)
        .expect("lowered Probe.run function");
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
        "the untyped parameter argument must emit the NullableHandle invoke: {invokes:?}"
    );
}

/// A `Local` argument whose recorded wire type is a concrete non-i64
/// carrier stays rejected: the sealed scalar edge corroborates that
/// record as `call-argument-type-drift`, so emission freezes closed
/// rather than letting a non-scalar value ride the i64 corridor. A
/// `Bool` local keeps no Home obligation, so the argument carrier is
/// the first gate the site reaches.
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
        .expect_err("a concrete box carrier must not ride the scalar edge");
    assert!(
        rejected
            .error()
            .to_string()
            .contains("nullable-argument-carrier"),
        "unexpected rejection: {}",
        rejected.error()
    );
    rejected.discard();
}

/// A `new` argument naming a live owned binding moves the lease into the
/// constructed object: `return new Holder(h)` consumes `h` on that exit
/// path, so the tail exit emits no `HomeReleaseIfLive`, while the sibling
/// `h == null` exit still owes its checked release. `Holder`'s duplicate
/// birth store stays construction-unsupported, so the `new` rides the raw
/// lane and no fault-unwind operand enters the count — exactly one
/// checked release survives: one per owed exit, never one per function.
#[test]
fn nullable_result_moved_into_new_releases_only_on_the_owed_exit() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        r#"
box OwMoveHolder {
    init { v }
    birth(v) {
        me.v = v
        me.v = v
    }
}
box OwMoveProbe {
    init { v }
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
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("a moved nullable argument must keep the owed-exit release only");
    let (_, module, _) = completed.into_parts();
    let run = module
        .functions
        .iter()
        .find(|(_, function)| function.signature.name == "OwMoveProbe.run/1")
        .map(|(_, function)| function)
        .expect("lowered OwMoveProbe.run function");
    let checked_releases = run
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| {
            matches!(
                instruction,
                crate::mir::MirInstruction::Invoke {
                    operation: crate::mir::instruction::InvokeOperation::HomeReleaseIfLive { .. },
                    ..
                }
            )
        })
        .count();
    assert_eq!(
        checked_releases, 1,
        "the moved `new` argument owes a release only on the sibling exit"
    );
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
