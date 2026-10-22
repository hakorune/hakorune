# APP-json-stream-aggregator
Status: focused EXE stops in ordinary Dynamic method invocation before MIR publication. Date: 2026-10-23
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [Dynamic invocation contract](../../../../reference/language/dynamic-invocation.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence
- Last full suite: 7/11 PASS (EXE6 + marker1), 397.733s; historical, not rerun.
- `cargo quick-build-fast` succeeded for the current source.
- The filtered `cargo quick-test-fast` did not reach execution: the lib-test target fails to compile (68 errors in unrelated existing test modules); no crate-level test result is claimed. The focused Dynamic operator contract tests pass 8/8 in isolation.
- The explicit `target/quick` focused smoke now passes the source-plan stop and fails at `ordinary-new/emission/text-initializer/execution-pending`; no EXE is produced.
- The passed source-plan stop was `local n = s.length()` in `StringHelpers.json_quote` (`string_helpers.hako:53`), where `s` is an unannotated Dynamic formal.
- The remaining refusal is in `src/mir/builder/ordinary_new_admission/selected/initializer_expression.rs:76`.
- Runtime provider matching now checks exact Rust runtime type identity instead of the diagnostic `type_name()` string; the immutable selector/arity table and opaque carrier owner remain unchanged. No compiler or AOT consumer is connected yet.
- `cargo quick-build-fast` passes after that change; the focused EXE smoke still stops at `text-initializer/execution-pending`. The focused library test cannot start because the existing lib-test target has 68 unrelated compile errors.

## Current stop and contract
- The exact Dynamic source target, language-wide execution envelope, and source initializer frame now exist. The selected initializer emitter still accepts only i64/Text; the new runtime provider/carrier owner has no physical caller or Normal/Fault publication path yet.
- The existing `CheckedCallOut`/Dynamic V2 route is TextScan-specific: its site plans and C lowering admit fixed entries, and its wire cannot represent the full trivial/owner-bearing/weak carrier contract. Do not widen the initializer gate or reuse that route as generic dispatch.
- The Dynamic/String comparison envelopes are now issued for both operand orders; they still have no physical execution consumer.
- This is an ordinary Dynamic invocation even when the runtime receiver may be String. Keep the source tag and selector/arity relation; do not infer String from `length`, caller behavior, or a MIR type.
- The existing bounded Dynamic Loop CallSlot and TextScan entries are not generic invocation owners. Do not route through plugin per-call snapshots, arity retry, compatibility fallback, or a selector-specific Text operation.
- The accepted contract remains `OpaqueObservable`, synchronous/non-detached, `Normal(SelfContainedDynamicCarrier) | Fault`, borrowed-no-escape inputs, and exactly-once result forwarding or End.

## Next work
- Continue from the selected initializer-emission owner: issue a distinct source-bound Dynamic CallSlot from the exact receiver/argument evaluation frame, then connect it through the runtime provider plan to the checked C invocation and Normal/Fault lifecycle publication.
- Connect carrier values to the accepted `I64 < Dynamic`, Dynamic/String comparison, and String + Dynamic operator envelopes; preserve exactly-once forwarding/end on all exits.
- Use the current expression source plan for receiver/argument evaluation order and exact source identity. Preserve terminal Fault, input ownership, and complete cleanup; no `.hako` rewrite or app/name branch.
- Re-run the explicit focused smoke after the complete route reaches its physical/runtime consumer. Keep other app work and the full suite at the existing gate.

## Existing app obligations retained
- Continue String, Map, ArrayBox, Birth, and loop work only as the focused smoke exposes them; preserve their current canonical owners and lifecycle checks.
- App completion requires the existing smoke to produce and execute the EXE with expected output/exit. Fixed-suite status remains separate from the full MirBuilder goal.
