//! Natural source through the production package, sole write, final handoff and
//! C-frame boundary. Query replies here are controlled observations, not C execution.
use super::super::tests::published_request;
use super::*;
use crate::mir::compiler::MirCompiler;
use crate::mir::{ArrayElementWriteKind, MirInstruction};

fn source(body: &str) -> String {
    format!("static box Scan {{ run(s) {{ local arr = new ArrayBox() local i = 0 loop(i < 1) {{ {body} i = i + 1 }} return i }} }}")
}

/// Source-backed lowering recurses deeply enough in debug builds to exceed
/// the default 8 MiB test-thread stack; run those pins on a widened thread
/// the same way the callable pipeline loop pins do.
fn run_on_test_thread(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name(name.to_owned())
        .stack_size(32 * 1024 * 1024)
        .spawn(body)
        .expect("spawn test thread")
        .join()
        .expect("test thread panicked");
}

#[test]
fn named_array_source_reaches_retained_typed_write_and_c_frame() {
    run_on_test_thread(
        "named-array-source-c-frame",
        named_array_source_reaches_retained_typed_write_and_c_frame_inner,
    );
}

fn named_array_source_reaches_retained_typed_write_and_c_frame_inner() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
        for body in [
            "arr.push(\"text\")",
            "arr.push(s.substring(0, 1))",
            "arr.push(\"one\") arr.push(\"two\")",
        ] {
            for optimize in [false, true] {
                let expected = if body.contains("two") { 2 } else { 1 };
                let mut calls = 0;
                MirCompiler::with_options(optimize).compile_normal_with_published(
                    published_request(&source(body)), |view, verification| -> Result<(), String> {
                        calls += 1;
                        assert!(verification.is_ok(), "{verification:?}");
                        assert_eq!(view.validated_named_arrays()?.len(), expected);
                        let writes = view.module().functions.values().flat_map(|f| f.blocks.values())
                            .flat_map(|b| b.all_instructions()).filter(|i| matches!(i,
                                MirInstruction::ArrayElementWrite { kind: ArrayElementWriteKind::Push, dst: None, index: None, .. })).count();
                        assert_eq!(writes, expected);
                        assert!(!view.has_lifecycle_instructions());
                        crate::mir::backend_capability::enforce_published_backend_supported(view, "ny-llvmc-obj")?;
                        crate::runner::mir_json_emit::emit_published_view_body(view)?;
                        let mut queries = 0;
                        c_transport_v2::PublishedStaticMethodCFrameV2::from_view_with_query(view, |_, _, _| {
                            queries += 1;
                            Ok(Some(map_named_allocations::NamedAllocationConsumer::Array))
                        })?;
                        assert_eq!(queries, 1);
                        assert!(c_transport_v2::PublishedStaticMethodCFrameV2::from_view_with_query(view, |_, _, _| {
                            Ok(Some(map_named_allocations::NamedAllocationConsumer::DirectArray))
                        }).unwrap_err().contains("array-capability-unsupported"));
                        let generic = PublishedMirBackendView::try_new(view.module()).unwrap();
                        assert!(generic.validated_named_arrays().is_err());
                        Ok(())
                    }
                ).unwrap_or_else(|error| panic!("{body}, optimize={optimize}: {error:?}"));
                assert_eq!(calls, 1);
            }
        }
    });
}

#[test]
fn named_array_value_demand_rejects_before_published_consumer() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_DEFAULT_MODE, || {
        let mut reached = false;
        let result = MirCompiler::with_options(false).compile_normal_with_published(
            published_request(&source("local result = arr.push(\"text\")")),
            |_, _| -> Result<(), String> {
                reached = true;
                Ok(())
            },
        );
        assert!(!reached);
        let error = match result {
            Err(error) => format!("{error:?}"),
            Ok(_) => panic!("value-demand must reject"),
        };
        assert!(error.contains("ValueDemand"), "{error}");
    });
}

/// `me.<field> = new ArrayBox()` residence: four Array/I64 appends share the
/// declared-initializer providers recorded inside `Holder.birth/1`.
fn holder_source(seed_body: &str) -> String {
    format!(
        "box Holder {{ init {{ free_stack, block_used, use_counts, requested_sizes }} capacity: i64 birth() {{ me.capacity = 0 me.free_stack = new ArrayBox() me.block_used = new ArrayBox() me.use_counts = new ArrayBox() me.requested_sizes = new ArrayBox() }} seed() {{ {seed_body} }} }} static box Main {{ main() {{ return 1 }} }}"
    )
}

fn holder_seed_body() -> &'static str {
    "local free_stack = me.free_stack local block_used = me.block_used local use_counts = me.use_counts local requested_sizes = me.requested_sizes local i = 0 loop(i < 4) { free_stack.push(i) block_used.push(0) use_counts.push(0) requested_sizes.push(0) i = i + 1 }"
}

/// The residence family rejects with typed `NamedArrayResidence` issues — the
/// uncovered generic route rejection is itself a valid fail-fast boundary for
/// shapes that stay outside the claim.
fn holder_variant_source(box_members: &str, seed_body: &str) -> String {
    format!(
        "box Holder {{ {box_members} seed() {{ {seed_body} }} }} static box Main {{ main() {{ return 1 }} }}"
    )
}

#[test]
fn field_resident_array_i64_push_reaches_retained_typed_writes() {
    run_on_test_thread(
        "field-resident-array-push",
        field_resident_array_i64_push_reaches_retained_typed_writes_inner,
    );
}

fn field_resident_array_i64_push_reaches_retained_typed_writes_inner() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    field_resident_env(field_resident_array_i64_push_reaches_retained_typed_writes_body);
}

/// Instance boxes without equals/toString would trigger DefaultDerive;
/// the macro gate is orthogonal to this named-array seam. One process-state
/// lock acquisition must cover both pins — nested helpers self-deadlock.
fn field_resident_env(body: impl FnOnce()) {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_MACRO_DISABLE", Some("1")),
            (crate::test_support::JOINIR_MODE_KEYS[0], None),
            (crate::test_support::JOINIR_MODE_KEYS[1], None),
            (crate::test_support::JOINIR_MODE_KEYS[2], None),
            (crate::test_support::JOINIR_MODE_KEYS[3], None),
            (crate::test_support::JOINIR_MODE_KEYS[4], None),
            (crate::test_support::JOINIR_MODE_KEYS[5], None),
        ],
        body,
    );
}

fn field_resident_array_i64_push_reaches_retained_typed_writes_body() {
        for optimize in [false, true] {
            let mut calls = 0;
            // The enclosing box's construction plan only admits i64 stored
            // fields today, so the artifact lane keeps the residence provider
            // as `RetainedUnavailable`. The MIR-JSON document lane exercises
            // the same named-array emission consumption, handoff validation
            // and coverage checks without that upstream object admission.
            MirCompiler::with_options(optimize).compile_normal_for_mir_json(
                published_request(&holder_source(holder_seed_body())),
                |view, verification| -> Result<(), String> {
                    calls += 1;
                    assert!(verification.is_ok(), "{verification:?}");
                    assert_eq!(view.validated_named_arrays()?.len(), 4);
                    let writes = view.module().functions.values().flat_map(|f| f.blocks.values())
                        .flat_map(|b| b.all_instructions()).filter(|i| matches!(i,
                            MirInstruction::ArrayElementWrite { kind: ArrayElementWriteKind::Push, dst: None, index: None, .. })).count();
                    assert_eq!(writes, 4);
                    let provider_allocations: usize = view.module().functions.values()
                        .map(|f| f.metadata.named_array_field_allocations.len()).sum();
                    assert_eq!(provider_allocations, 4, "all four field providers recorded");
                    crate::runner::mir_json_emit::emit_published_view_body(view)?;
                    let mut queries = 0;
                    c_transport_v2::PublishedStaticMethodCFrameV2::from_view_with_query(view, |_, _, _| {
                        queries += 1;
                        Ok(Some(map_named_allocations::NamedAllocationConsumer::Array))
                    })?;
                    assert!(queries > 0);
                    Ok(())
                },
            ).unwrap_or_else(|error| panic!("optimize={optimize}: {error:?}"));
            assert_eq!(calls, 1);
        }
}

#[test]
fn field_resident_push_rejects_non_array_field() {
    run_on_test_thread("field-resident-non-array", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let body = "local capacity = me.capacity local i = 0 loop(i < 4) { capacity.push(0) i = i + 1 }";
        let result = MirCompiler::with_options(false).compile_normal_with_published(
            published_request(&holder_source(body)),
            |_, _| -> Result<(), String> { Ok(()) },
        );
        assert!(result.is_err(), "non-ArrayBox field push must not silently pass");
        });
    });
}

#[test]
fn field_resident_push_rejects_non_integer_argument() {
    run_on_test_thread("field-resident-non-i64", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let body = "local free_stack = me.free_stack local i = 0 loop(i < 1) { free_stack.push(\"text\") i = i + 1 }";
        let result = MirCompiler::with_options(false).compile_normal_with_published(
            published_request(&holder_source(body)),
            |_, _| -> Result<(), String> { Ok(()) },
        );
        let error = match result {
            Err(error) => format!("{error:?}"),
            Ok(_) => panic!("non-integer argument must reject"),
        };
        assert!(
            error.contains("IntegerSourceMissing") || error.contains("route-not-front-selected"),
            "unexpected rejection: {error}"
        );
        });
    });
}

/// The residence claim must close on typed issues for every near-miss shape:
/// each variant exercises exactly one boundary of the accepted relation.
#[test]
fn field_resident_push_rejects_near_miss_family() {
    run_on_test_thread("field-resident-near-miss", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let cases: &[(&str, &str, &str)] = &[
            // Providerless declared field: the birth never stores the array.
            (
                "init { free_stack, block_used } birth() { me.block_used = new ArrayBox() }",
                "local a = me.free_stack local i = 0 loop(i < 1) { a.push(0) i = i + 1 }",
                "ProviderMissing",
            ),
            // Weak residence cannot own the array lifetime.
            (
                "weak free_stack init { block_used } birth() { me.free_stack = new ArrayBox() me.block_used = new ArrayBox() }",
                "local a = me.free_stack local i = 0 loop(i < 1) { a.push(0) i = i + 1 }",
                "WeakFieldResidence",
            ),
            // A foreign-instance field read is outside the owning receiver.
            (
                "init { free_stack, block_used } birth() { me.free_stack = new ArrayBox() me.block_used = new ArrayBox() }",
                "local o: Holder = new Holder() local a = o.free_stack local i = 0 loop(i < 1) { a.push(0) i = i + 1 }",
                "ForeignFieldOwner",
            ),
            // The alias must not be rebound between the field read and push.
            (
                "init { free_stack, block_used } birth() { me.free_stack = new ArrayBox() me.block_used = new ArrayBox() }",
                "local a = me.free_stack a = me.block_used local i = 0 loop(i < 1) { a.push(0) i = i + 1 }",
                "ReassignedReceiver",
            ),
            // `push` returns NoValue: a value-demanding position rejects.
            (
                "init { free_stack, block_used } birth() { me.free_stack = new ArrayBox() me.block_used = new ArrayBox() }",
                "local a = me.free_stack local i = 0 loop(i < 1) { local r = a.push(0) i = i + 1 }",
                "ValueDemand",
            ),
            // A same-named shadow `new ArrayBox()` alias stays in the
            // construction family, which requires a Text argument source;
            // an integer argument is a typed rejection, not a residence.
            (
                "init { free_stack, block_used } birth() { me.free_stack = new ArrayBox() me.block_used = new ArrayBox() }",
                "local a = new ArrayBox() local i = 0 loop(i < 1) { a.push(0) i = i + 1 }",
                "TextSourceMissing",
            ),
        ];
        for (members, seed, expected) in cases {
            let result = MirCompiler::with_options(false).compile_normal_for_mir_json(
                published_request(&holder_variant_source(members, seed)),
                |_, _| -> Result<(), String> { Ok(()) },
            );
            let error = match result {
                Err(error) => format!("{error:?}"),
                Ok(_) => panic!("{expected}: near-miss must reject"),
            };
            assert!(
                error.contains(expected),
                "{expected}: unexpected rejection: {error}"
            );
        }
        });
    });
}

/// Production `field: ArrayBox = new ArrayBox()` decl-init shape: the
/// generated provider stores are plan stores, so the same residence
/// family reaches the sole canonical `Invoke FieldSet` emission on the
/// artifact lane instead of stopping at `artifact-source-unavailable`.
fn holder_decl_source(seed_body: &str) -> String {
    format!(
        "box Holder {{ free_stack: ArrayBox = new ArrayBox() block_used: ArrayBox = new ArrayBox() use_counts: ArrayBox = new ArrayBox() requested_sizes: ArrayBox = new ArrayBox() capacity: i64 = 0 reserved: usize = 0 birth() {{ }} seed() {{ {seed_body} }} }} static box Main {{ main() {{ return 1 }} }}"
    )
}

#[test]
fn provider_construction_store_reaches_artifact_lane() {
    run_on_test_thread("provider-construction-store", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        for optimize in [false, true] {
            let mut calls = 0;
            MirCompiler::with_options(optimize).compile_normal_with_published(
                published_request(&holder_decl_source(holder_seed_body())),
                |view, verification| -> Result<(), String> {
                    calls += 1;
                    assert!(verification.is_ok(), "{verification:?}");
                    assert_eq!(view.validated_named_arrays()?.len(), 4);
                    let writes = view.module().functions.values().flat_map(|f| f.blocks.values())
                        .flat_map(|b| b.all_instructions()).filter(|i| matches!(i,
                            MirInstruction::ArrayElementWrite { kind: ArrayElementWriteKind::Push, dst: None, index: None, .. })).count();
                    assert_eq!(writes, 4);
                    let provider_allocations: usize = view.module().functions.values()
                        .map(|f| f.metadata.named_array_field_allocations.len()).sum();
                    assert_eq!(provider_allocations, 4, "all four field providers recorded");
                    // The six generated stores commit through the sole
                    // canonical FieldSet emission — four providers plus the
                    // declared i64 and usize scalar initializers.
                    let field_sets = view.module().functions.values().flat_map(|f| f.blocks.values())
                        .flat_map(|b| b.all_instructions()).filter(|i| matches!(i,
                            MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::FieldSet { .. }, .. })).count();
                    assert_eq!(field_sets, 6, "generated stores are plan FieldSets");
                    Ok(())
                },
            ).unwrap_or_else(|error| panic!("optimize={optimize}: {error:?}"));
            assert_eq!(calls, 1);
        }
        });
    });
}

/// Provider-store admission stays bounded: a wrong declared class, a
/// user-box provider, a scalar field carrying `new`, an argumented
/// provider, or an unsupported initializer all stay typed plan rejections
/// surfaced through the artifact validator.
#[test]
fn provider_construction_store_rejects_foreign_shapes() {
    run_on_test_thread("provider-construction-foreign", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let cases: &[(&str, &str)] = &[
            // Declared ArrayBox but the provider constructs StringBox.
            (
                "box Holder { free_stack: ArrayBox = new StringBox() birth() { } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // A user-box provider is admitted only when the child is
            // `PlainI64NoHook`; a child owning an ArrayBox field needs the
            // deeper teardown lane and still fails plan admission.
            (
                "box Inner { items: ArrayBox = new ArrayBox() birth() { } } box Holder { inner: Inner = new Inner() birth() { } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // The provider must construct exactly the declared class.
            (
                "box Inner { value: i64 = 0 birth() { } } box Other { value: i64 = 0 birth() { } } box Holder { inner: Inner = new Other() birth() { } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // A provider class outside exact object membership is foreign.
            (
                "box Holder { inner: Missing = new Missing() birth() { } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // User-provider arguments admit sealed literals only: a field
            // read argument is out of contract.
            (
                "box Inner { value: i64 = 0 birth(v) { } } box Holder { capacity: i64 = 0 inner: Inner = new Inner(me.capacity) birth() { } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // An i64 field cannot carry a provider `new` store.
            (
                "box Holder { capacity: i64 birth() { me.capacity = new ArrayBox() } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // Provider `new` must be bare: arguments are foreign.
            (
                "box Holder { free_stack: ArrayBox = new ArrayBox(4) birth() { } seed() { } } static box Main { main() { return 1 } }",
                "FieldContractUnsupported",
            ),
            // An arbitrary declared-initializer expression is not a
            // supported store shape.
            (
                "box Holder { capacity: i64 = [1] birth() { } seed() { } } static box Main { main() { return 1 } }",
                "BodyCoverageUnsupported",
            ),
        ];
        for (source, expected) in cases {
            let result = MirCompiler::with_options(false).compile_normal_with_published(
                published_request(source),
                |_, _| -> Result<(), String> { Ok(()) },
            );
            let error = match result {
                Err(error) => format!("{error:?}"),
                Ok(_) => panic!("{expected}: foreign provider shape must reject"),
            };
            assert!(
                error.contains(expected),
                "{expected}: unexpected rejection: {error}"
            );
        }
        });
    });
}

/// User-object provider: `inner: Inner = new Inner()` emits a checked
/// `NewBox`, the canonical `BirthConstructor` call and the checked
/// `ObjectFieldSet` store inside `Holder.birth/1`, with the child reclaimed
/// on birth fault and discharged on store fault.
#[test]
fn user_object_provider_construction_reaches_artifact_lane() {
    run_on_test_thread("user-object-provider-construction", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let source = "box Inner { value: i64 = 0 birth() { } } box Holder { inner: Inner = new Inner() birth() { } seed() { } } static box Main { main() { return 1 } }";
        for optimize in [false, true] {
            let mut calls = 0;
            MirCompiler::with_options(optimize).compile_normal_with_published(
                published_request(source),
                |view, verification| -> Result<(), String> {
                    calls += 1;
                    assert!(verification.is_ok(), "{verification:?}");
                    let instructions = || {
                        view.module().functions.values().flat_map(|f| f.blocks.values())
                            .flat_map(|b| b.all_instructions())
                    };
                    let new_boxes = instructions().filter(|i| matches!(i,
                        MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::NewBox { .. }, .. })).count();
                    assert_eq!(new_boxes, 1, "provider emits the checked NewBox");
                    let birth_calls = instructions().filter(|i| matches!(i,
                        MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. }
                            if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }))).count();
                    assert_eq!(birth_calls, 1, "provider emits the canonical BirthConstructor call");
                    let field_sets = instructions().filter(|i| matches!(i,
                        MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ObjectFieldSet { .. }, .. })).count();
                    assert_eq!(field_sets, 1, "provider store is the checked ObjectFieldSet");
                    let reclaims = instructions().filter(|i| matches!(i,
                        MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished { .. }, .. })).count();
                    assert_eq!(reclaims, 1, "birth fault reclaims the unpublished child");
                    let discharges = instructions().filter(|i| matches!(i,
                        MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::HomeRelease { .. }, .. })).count();
                    assert_eq!(discharges, 1, "store fault discharges the child");
                    Ok(())
                },
            ).unwrap_or_else(|error| panic!("optimize={optimize}: {error:?}"));
            assert_eq!(calls, 1);
        }
        });
    });
}

/// D2 lane disposition: claimed loop pushes are contract-owned — the plain
/// `compile_normal` finishing deliberately never discharges retained markers
/// (`let _callables`, `5a2dea9b3c`) and must stop at the typed rejection,
/// never silently dropping the obligation or re-routing to generic emission.
#[test]
fn claimed_loop_push_rejects_on_unretained_plain_lane() {
    run_on_test_thread("field-resident-plain-lane-reject", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let result = MirCompiler::with_options(false)
            .compile_normal(published_request(&holder_source(holder_seed_body())));
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("retained markers must reject on the unretained lane"),
        };
        assert!(
            error.contains("named-array/retained-source-required"),
            "unexpected rejection: {error}"
        );
        });
    });
}

/// D2 coverage split: a field-resident push outside any loop body mints no
/// contract row (issuer `resolved_loop_placement == Body` gate), so the
/// shared generic writer owns it — the plain lane compiles it with zero
/// retained obligations. The split is designed, not a residual-row leak.
#[test]
fn straight_line_field_resident_push_stays_on_generic_write() {
    run_on_test_thread("field-resident-straight-line-generic", || {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        field_resident_env(|| {
        let seed = "local a = me.free_stack a.push(1) return 0";
        let result = MirCompiler::with_options(false)
            .compile_normal(published_request(&holder_source(seed)))
            .expect("straight-line push stays on the generic coverage split");
        let writes = result.module.functions.values().flat_map(|f| f.blocks.values())
            .flat_map(|b| b.all_instructions()).filter(|i| matches!(i,
                MirInstruction::ArrayElementWrite { kind: ArrayElementWriteKind::Push, .. })).count();
        assert_eq!(writes, 1, "generic array write emits the push");
        let obligations: usize = result.module.functions.values()
            .map(|f| f.metadata.named_array_write_obligations.len()).sum();
        assert_eq!(obligations, 0, "no retained marker is minted outside a loop body");
        });
    });
}
