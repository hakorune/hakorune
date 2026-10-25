# APP-boxtorrent-mini

Status: selected app; canonical EXE incomplete. Date: 2026-10-26
Scope: unchanged boxtorrent-mini through canonical EXE generation, execution, and expected output/exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Array owner](../../../../../src/mir/normal_callable_semantic_package/core_method_source/README.md).

## Current stop
- Binary/runtime: `NYASH_BIN=target/quick/hakorune NYASH_EMIT_EXE_NYRT=target/quick`.
- Latest smoke reaches owner73 `BoxTorrentStore.refCount(first_chunk)`, Body9 Initializer(0), and stops with `borrowed-instance/opaque-source-unsupported` (`ordinal=0`, `actual=Unknown`); EXE was not emitted.
- The owner70 Body2 `data.length()` and Body4 `manifest.seal()` now pass. `OriginalMethodResultProofV1::CallResult` preserves the exact same-module static result representation and required i64 arguments; it moved the smoke past the previous Body4 stop without granting execution authority.
- `cargo quick-build-fast -j 4` passed in36.65s (1,878 existing warnings). The explicit `target/quick/hakorune` smoke reran and stopped at the same Body9 boundary; no EXE was emitted.
- The Body9 source is `firstChunkId()` feeding `refCount(first_chunk)`; the Array element result has no proved caller payload class at this boundary. Resume by tracing its source/result and ownership/End path, without inferring Text or bypassing carrier cleanup.
- Shared handoff reports fixed suite6/11 with JSON, boxtorrent-mini, binary-trees, mimalloc-lite, and allocator-stress failing. This worktree did not rerun the suite. The handoff separately records mimalloc-lite at slot37 Body5 `borrowed-object/forward-source-identity`, and binary-trees with two final `object-field-read-definition-invalid` failures.

## Implementation boundary
- The current vertical slice connects source-selected Array Get, DynamicCarrier publication/End, loop transport, and append-source closure across MIR and the C runtime. `expression_value/cleanup.rs` is the retained cleanup owner, not a temporary file.
- The append collector now rejects duplicate source keys before accepting its closed field-append domain.
- `.hako`, tests, fixtures, and checkers are unchanged. Temporary diagnostics and the speculative direct-representation/fallback experiment are absent.
- Pointer guard and `git diff --check` pass. The build and app smoke have completed; the smoke still has no emitted EXE.

## Next
- Trace `firstChunkId()`'s Array Get source into `refCount` and close the required carrier/result ownership path without inferring payload type.
- After a source change, rebuild and rerun the selected smoke; rerun the fixed suite at app acceptance.
- Loop/Return, borrowed receiver, physical field read, R6/R7, and finite-product closeout remain incomplete.
