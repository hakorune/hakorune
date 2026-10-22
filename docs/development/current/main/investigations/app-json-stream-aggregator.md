# APP-json-stream-aggregator
Status: focused EXE stops in ordinary Dynamic method invocation before MIR publication. Date: 2026-10-22
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [Dynamic invocation contract](../../../../reference/language/dynamic-invocation.md), [pipeline](../design/mirbuilder-final-pipeline-ssot.md).

## Product evidence
- Last full suite: 7/11 PASS (EXE6 + marker1), 397.733s; historical, not rerun.
- Explicit `target/quick` focused smoke reaches MIR emission, then fails with `PrefixNotCovered(Body(3))`; no EXE is produced.
- `Body(3)` is `local n = s.length()` in `StringHelpers.json_quote` (`string_helpers.hako:53`). `s` is an unannotated Dynamic formal.

## Current stop and contract
- Exact Dynamic source target and the language-wide execution envelope already exist. The production path has no ordinary-expression Dynamic CallSlot, provider-admitted immutable registry, leased executable plan, physical/runtime invocation, or carrier Normal/Fault publication.
- This is an ordinary Dynamic invocation even when the runtime receiver may be String. Keep the source tag and selector/arity relation; do not infer String from `length`, caller behavior, or a MIR type.
- The existing bounded Dynamic Loop CallSlot and TextScan entries are not generic invocation owners. Do not route through plugin per-call snapshots, arity retry, compatibility fallback, or a selector-specific Text operation.
- The accepted contract remains `OpaqueObservable`, synchronous/non-detached, `Normal(SelfContainedDynamicCarrier) | Fault`, borrowed-no-escape inputs, and exactly-once result forwarding or End.

## Next work
- Extend the existing source-bound Dynamic call owner into a generic checked ordinary-expression CallSlot, then connect it through provider admission, one immutable registry, a leased plan, one physical/runtime invocation, and Normal/Fault lifecycle publication.
- Use the current expression source plan for receiver/argument evaluation order and exact source identity. Preserve terminal Fault, input ownership, and complete cleanup; no `.hako` rewrite or app/name branch.
- Re-run the explicit focused smoke after the complete route reaches its physical/runtime consumer. Keep other app work and the full suite at the existing gate.

## Existing app obligations retained
- Continue String, Map, ArrayBox, Birth, and loop work only as the focused smoke exposes them; preserve their current canonical owners and lifecycle checks.
- App completion requires the existing smoke to produce and execute the EXE with expected output/exit. Fixed-suite status remains separate from the full MirBuilder goal.
