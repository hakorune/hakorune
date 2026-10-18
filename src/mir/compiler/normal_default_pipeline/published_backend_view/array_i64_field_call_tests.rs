//! Field-resident `ArrayBox` get result admission through the ordinary
//! lane.
//!
//! The whole-Box integer-store census seals a `me.<ArrayBox field>` whose
//! `get/1` result may claim i64; the existing local-result lane then
//! installs the binding as `Trivial`, so `return v` and downstream i64
//! consumers complete. The emitted `ArrayBox.get/1` site lowers to
//! `MirInstruction::ArrayRead` — the sole physical read owner —
//! which the published view collects as an `array_element_reads` row and
//! transports as `PublishedCallKindV1::ArrayGet`.
use super::*;
use crate::mir::compiler::published_backend_view::physical_program_json;

fn array_page_source(body: &str) -> String {
    format!(
        "box Handle {{ birth() {{ }} }} \
         box Page {{ free: ArrayBox = new ArrayBox() top: i64 = 0 \
         birth() {{ }} \
         probe(handle): i64 {{ {body} }} }} \
         static box Main {{ main() {{ local p = new Page() local h = new Handle() local r = p.probe(h) return r }} }}"
    )
}

fn compile_normal(body: &str) -> Result<crate::mir::MirCompileResult, String> {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false).compile_normal(request(&array_page_source(body)))
    })
}

/// A proven `me.<numeric field>` read indexes the get — `me.top` is a
/// declared i64 field.
#[test]
fn array_i64_get_admits_numeric_field_index() {
    let result = compile_normal(
        "me.free.set(0, 7) local v = me.free.get(me.top) return v",
    );
    assert!(result.is_ok(), "numeric field index must compile: {result:?}");
}

/// A vetoed field keeps the manifest `Dynamic` result — `return v` keeps
/// its named unavailability instead of minting i64.
#[test]
fn array_i64_vetoed_field_keeps_dynamic_result() {
    for body in [
        // A non-integer `set` value vetoes the census.
        "me.free.set(0, true) local v = me.free.get(0) return v",
        // A non-integer `set` value through a local vetoes the census.
        "local flag = true me.free.set(0, flag) local v = me.free.get(0) return v",
    ] {
        let result = compile_normal(body);
        assert!(result.is_err(), "{body}: vetoed field must not mint i64");
    }
}

/// Positive publication evidence: the `ArrayBox.get/1` site lowers to
/// `ArrayRead`, the published view collects exactly one read row
/// for `probe`, and the lifecycle physical ABI input issues cleanly.
/// Each fixture dumps its physical-program JSON for the v4 execution
/// driver (`published_lifecycle_v4_get_view_execution_test.py`).
#[test]
fn array_i64_get_publishes_through_array_element_read() {
    for (body, suffix) in [
        // index 0 hits the stored slot: `probe` returns the read 7.
        ("me.free.set(0, 7) local v = me.free.get(0) return v", "ok"),
        // index 5 misses the single-slot array: `slot_load_hi` reads the
        // lane's null sentinel 0, matching `ArrayBox.get` Null-on-miss.
        ("me.free.set(0, 7) local v = me.free.get(5) return v", "oob"),
    ] {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let result = MirCompiler::with_options(false).compile_normal_with_published(
                request(&array_page_source(body)),
                |view, _verification| -> Result<(), String> {
                    let reads = view.array_element_reads();
                    assert_eq!(
                        reads.len(),
                        1,
                        "expected exactly one published array read, got {reads:?}"
                    );
                    assert_eq!(reads[0].function_name(), "Page.probe/1");
                    assert!(reads[0].dst().is_some(), "get read must carry a dst");
                    let input = view.issue_lifecycle_physical_abi_input()?;
                    let wire = physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                    let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
                    let callee = json["functions"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|f| f["name"] == "Page.probe/1")
                        .unwrap();
                    let gets: Vec<_> = callee["blocks"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .flat_map(|b| b["instructions"].as_array().unwrap())
                        .filter(|row| row["instruction"]["op"] == "array_get")
                        .collect();
                    assert_eq!(gets.len(), 1, "exactly one routed element read");
                    let get = &gets[0]["instruction"];
                    assert!(get["array"].is_u64(), "array operand must be a slot value");
                    assert!(get["index"].is_u64(), "index operand must be a slot value");
                    assert!(get["dst"].is_u64(), "get read must carry a dst");
                    assert!(get.get("site").is_none(), "pure read carries no site");
                    std::fs::write(
                        std::env::temp_dir().join(format!("hako-issued-get-view-{suffix}.json")),
                        wire,
                    )
                    .unwrap();
                    Ok(())
                },
            );
            let result = result.unwrap_or_else(|error|
                panic!("{suffix}: array read publication must complete: {error}"));
            assert!(matches!(result,
                crate::mir::compiler::normal_default_pipeline::NormalPublishedCompileOutcome::Consumed(())),
                "{suffix}: publication callback must consume the selected module");
        });
    }
}

/// Exercise the source scalar loan, rather than local-result Trivial promotion.
/// The original forwarding call selects the same participation as the app.
#[test]
fn array_i64_checked_scalar_read_publishes_and_rejects_physical_drift() {
    for body in [
        "return me.values.get(handle.block_id)",
        "local v = me.values.get(handle.block_id) if handle == null { return v } return v",
    ] {
        let text = format!("box Handle {{ block_id: i64 birth() {{ me.block_id = 0 }} }} box Page {{ values: ArrayBox = new ArrayBox() birth() {{ }} probe(handle): i64 {{ if handle == null {{ return 0 }} me.values.set(0, 7) {body} }} }} box Heap {{ page: Page birth() {{ me.page = new Page() }} relay(handle: Handle): i64 {{ if handle == null {{ return 0 }} return me.page.probe(handle) }} qualified(): i64 {{ local h = new Handle() return me.relay(h) }} }} static box Main {{ main() {{ return 0 }} }}");
        crate::runtime::ring0::ensure_global_ring0_initialized();
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            MirCompiler::with_options(false).compile_normal_with_published(
                request(&text),
                |view, _| -> Result<(), String> {
                    let handoff = view.retained_handoff.expect("original finalized source");
                    let function = &view.module().functions["Page.probe/1"];
                    assert_eq!(function.metadata.named_array_read_obligations.len(), 1);
                    let marker = function.metadata.named_array_read_obligations[0].clone();
                    assert_eq!(function.signature.return_type, crate::mir::MirType::Integer);
                    assert_eq!(function.metadata.value_types.get(&marker.dst), Some(&crate::mir::MirType::Integer));
                    assert_eq!(view.array_element_reads().len(), 1);
                    handoff.validate_named_arrays(view.module())?;
                    view.issue_lifecycle_physical_abi_input()?;
                    for mutation in 0..8 {
                        let mut changed = view.module().clone();
                        let function = changed.functions.get_mut("Page.probe/1").unwrap();
                        match mutation {
                            0..=2 => {
                                let read = function.blocks.values_mut().flat_map(|b| b.instructions.iter_mut()).find(|instruction|
                                    matches!(instruction, crate::mir::MirInstruction::ArrayRead { site_id, .. } if *site_id == marker.read)).unwrap();
                                let crate::mir::MirInstruction::ArrayRead { dst, receiver, operation: crate::mir::instruction::ArrayReadOperationV1::Element { index }, .. } = read else { unreachable!() };
                                match mutation {
                                    0 => *dst = Some(marker.receiver),
                                    1 => *receiver = marker.index,
                                    _ => *index = marker.receiver,
                                }
                            }
                            3 => { function.metadata.value_types.insert(marker.dst, crate::mir::MirType::Unknown); }
                            4 => { function.metadata.value_types.insert(marker.index, crate::mir::MirType::Unknown); }
                            5 => function.metadata.named_array_read_obligations.clear(),
                            6 => function.metadata.named_array_read_obligations.push(marker.clone()),
                            _ => {
                                let producer = function.blocks.values_mut().flat_map(|b| b.instructions.iter_mut()).find(|instruction|
                                    matches!(instruction, crate::mir::MirInstruction::FieldGet { dst, .. } | crate::mir::MirInstruction::ObjectFieldGet { dst, .. } if *dst == marker.receiver)).unwrap();
                                match producer {
                                    crate::mir::MirInstruction::FieldGet { base, .. } | crate::mir::MirInstruction::ObjectFieldGet { base, .. } => *base = marker.index,
                                    _ => unreachable!(),
                                }
                            }
                        }
                        assert!(handoff.validate_named_arrays(&changed).is_err(), "mutation {mutation}: {body}");
                        if let Ok(changed_view) = PublishedMirBackendView::try_new(&changed) {
                            assert!(changed_view.bind_finalized_root_handoff(Some(handoff)).is_err(), "publication final mutation {mutation}: {body}");
                        }
                    }
                    Ok(())
                },
            ).unwrap_or_else(|error| panic!("{body}: {error}"));
        });
    }
}
