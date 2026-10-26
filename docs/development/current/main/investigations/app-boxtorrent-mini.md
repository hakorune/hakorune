# APP-boxtorrent-mini

Status: selected app; canonical EXE incomplete. Date: 2026-10-26
Scope: unchanged boxtorrent-mini through canonical EXE generation, execution, and expected output/exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Array owner](../../../../../src/mir/normal_callable_semantic_package/core_method_source/README.md).

## Current stop
- Binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`.
- Latest selected smoke stops in Main owner73 Body9 Initializer(0), `store.refCount(first_chunk)`, with `borrowed-instance/opaque-source-unsupported` (ordinal0, Unknown). `first_chunk` comes from `first.firstChunkId()`; no EXE is emitted.
- `store.put`'s exact all-Return Text evidence now joins the original instance-result SUM. The `addChunk(cid)` push domain and ArrayGet Core demands close through the same graph. Reprojection retains existing canonical receipts and issues only newly closed Core sites through the original source issuer; the former owner65 Loop preparation stop is no longer the first failure.
- `firstChunkId()` returns `""` on the empty-array arm and `chunk_ids.get(0)` otherwise. The current read proof retains Null for out-of-range reads and has no bounds witness. Do not infer Text from the caller's workload or change the ArrayGet Dynamic contract.
- Latest `cargo quick-build-fast -j 4` PASS41.79s (1,878 existing warnings); selected smoke FAIL as above. Existing Text/field identity, Bool composition, mixed-class refusal, Array input identity, and SUM SCC checks give5PASS. Two additional focused reds (missing-return diagnostic assertion and Static substring fixture) reproduce identically with HEAD production and remain unresolved; they were not weakened. Test compilation1m51s, then44.11s after correcting the existing SUM fixture's self-referencing operand indices.
- Fixed suite last reported6/11 before this series; no full remeasurement here. Previous mimalloc/binary-trees handoff failures remain separate rows and are not declared resolved by this work.

## Implementation boundary
- The committed vertical slice connects source-selected Array Get, DynamicCarrier publication/End, loop transport, and append-source closure across MIR and the C runtime. `expression_value/cleanup.rs` is the retained cleanup owner, not a temporary file.
- `ArrayFieldGetCandidateV1` comes from the existing filtered field census; exact push-source seeds feed the same source SUM graph. All-Return Text classification is shared between pre-ingress source facts and ingress; this grants no result execution or ownership.
- The append collector now rejects duplicate source keys before accepting its closed field-append domain.
- Runtime support copies an exact `StringBox` from a borrowed DynamicCarrier into independent owned Text; the carrier keeps its caller-owned End. MIR Materialize still requires the source-proved Text snapshot connection before this support can execute.
- `.hako`, fixtures, and checkers are unchanged. Two existing test callers gained the missing empty Get-candidate argument; one existing SUM fixture now points to its intended Text lhs and unknown rhs. No test case/file or diagnostic output was added.

## Next
- Retain a same-array bounds witness for a surviving length guard with a terminating empty arm, a nonnegative index, and no intervening mutation/escape. Reads without that witness keep their Null alternative.
- Connect the source-proved Text snapshot through `firstChunkId()`, preserving the original DynamicCarrier's End separately from the new owned Text on Normal/Fault paths.
- Rebuild and rerun the selected smoke after the source-domain change; rerun the fixed suite at app acceptance.
- Loop/Return, borrowed receiver, physical field read, R6/R7, and finite-product closeout remain incomplete.
