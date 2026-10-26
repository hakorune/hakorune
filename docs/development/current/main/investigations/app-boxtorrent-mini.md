# APP-boxtorrent-mini

Status: selected app; canonical EXE incomplete. Date: 2026-10-26
Scope: unchanged boxtorrent-mini through canonical EXE generation, execution, and expected output/exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Array owner](../../../../../src/mir/normal_callable_semantic_package/core_method_source/README.md).

## Current stop
- Binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`.
- Latest selected smoke and direct EXE route still stop at owner65 `BoxTorrentManifest.chunkListText()`, Body4 LoopBody1, on `out = out + ids.get(i)`; `PrefixNotCovered` remains and no EXE is emitted.
- The narrow ArrayGet demand now comes from the existing push-only field census and enters source SUM. Exact field read and signed-i64 index are recognized, but `chunk_ids` element SUM is unresolved: its only push source is opaque formal `cid` from `addChunk` (owner60, binding1). The ArrayGet node therefore has no closed Core receipt, so the Loop recipe cannot cover the assignment.
- Latest `cargo quick-build-fast -j 4` passed in44.00s (1,878 existing warnings) with temporary trace statements enabled. Those trace statements are now removed; no rebuild was run after this semantics-preserving cleanup. The selected smoke/direct route was still failing before EXE emission.
- The fixed suite's last recorded result is6/11 before the ArrayGet-demand wiring; this worktree has not rerun the full suite after that change. Do not infer the `ids.get(i)` payload type. Next trace the exact source/call/result chain feeding `addChunk`'s `cid` from `store.put`, then require a closed element SUM before consuming the ArrayGet result.
- A prior mimalloc handoff narrowed its stop to slot37 Body5 `borrowed-object/forward-source-identity`; scalar SUM now requires flow-read + origin + signed-only point SUM when a flow read exists, and guarded forwarding is limited to caller-entry roots. Binary-trees remains at two final `object-field-read-definition-invalid` failures. These are separate rows; do not switch away from the selected boxtorrent-mini row.

## Implementation boundary
- The committed vertical slice connects source-selected Array Get, DynamicCarrier publication/End, loop transport, and append-source closure across MIR and the C runtime. `expression_value/cleanup.rs` is the retained cleanup owner, not a temporary file.
- Current uncommitted WIP adds `ArrayFieldGetCandidateV1` from the existing filtered field census and resolves array element SUMs from exact push-source seeds. This is the remaining source-domain connection, not product acceptance.
- The append collector now rejects duplicate source keys before accepting its closed field-append domain.
- Runtime WIP adds a borrowed DynamicCarrier snapshot that copies only an exact `StringBox` into an independent owned Text result; the carrier keeps its caller-owned End. The MIR Materialize operand gate remains Text-only, so this capability is not selectable until source proof is connected.
- `.hako`, fixtures, and checkers are unchanged. Two existing test callers only gained the new empty candidate argument; no test case or test file was added. Temporary diagnostics are removed. Implementation/runtime WIP remains uncommitted because it is mixed across the same owner files; no push was made. Pointer guard and `git diff --check` pass after this refresh.

## Next
- Trace the exact selected `store.put(chunk_data)` result into `addChunk(cid, ...)` and close the source SUM for `chunk_ids` only from that provenance.
- Then produce the ArrayGet Core receipt and connect its checked result through `firstChunkId()` and Normal/Fault cleanup.
- Rebuild and rerun the selected smoke after the source-domain change; rerun the fixed suite at app acceptance.
- Loop/Return, borrowed receiver, physical field read, R6/R7, and finite-product closeout remain incomplete.
