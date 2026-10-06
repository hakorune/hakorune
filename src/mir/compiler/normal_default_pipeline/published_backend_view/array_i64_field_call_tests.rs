//! Field-resident `ArrayBox` get result admission through the ordinary
//! lane.
//!
//! The whole-Box integer-store census seals a `me.<ArrayBox field>` whose
//! `get/1` result may claim i64; the existing local-result lane then
//! installs the binding as `Trivial`, so `return v` and downstream i64
//! consumers complete. The emitted `ArrayBox.get/1` site lowers to
//! `MirInstruction::ArrayElementRead` — the sole physical read owner —
//! which the published view collects as an `array_element_reads` row and
//! transports as `PublishedCallKindV1::ArrayGet`.
use super::*;

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

/// A proven field mints the i64 result: `local v = me.free.get(0)` then
/// `return v` reaches a compiled module — before the arm the same source
/// stopped at `source-not-i64`.
#[test]
fn array_i64_local_get_result_reaches_normal_compile() {
    let result =
        compile_normal("me.free.set(0, 7) local v = me.free.get(0) return v");
    assert!(result.is_ok(), "proven field get result must compile: {result:?}");
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
/// `ArrayElementRead`, the published view collects exactly one read row
/// for `probe`, and the lifecycle physical ABI input issues cleanly.
#[test]
fn array_i64_get_publishes_through_array_element_read() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let result = MirCompiler::with_options(false).compile_normal_with_published(
            request(&array_page_source(
                "me.free.set(0, 7) local v = me.free.get(0) return v",
            )),
            |view, _verification| -> Result<(), String> {
                let reads = view.array_element_reads();
                assert_eq!(
                    reads.len(),
                    1,
                    "expected exactly one published array read, got {reads:?}"
                );
                assert_eq!(reads[0].function_name(), "Page.probe/1");
                assert!(reads[0].dst().is_some(), "get read must carry a dst");
                view.issue_lifecycle_physical_abi_input()?;
                Ok(())
            },
        );
        if let Err(error) = &result {
            panic!("array read publication must complete: {error}");
        }
    });
}
