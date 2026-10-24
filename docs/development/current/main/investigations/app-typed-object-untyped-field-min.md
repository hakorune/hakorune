# APP-typed-object-untyped-field-min
Status: selected app; EXE not reached. Date: 2026-10-24
Related: [RULES](../../../RULES.md), [current state](../CURRENT_STATE.toml), [accepted untyped-storage D0](mirbuilder-untyped-object-storage-d0-2026-09-25.md), [ownership law](../../../../reference/language/ownership.md#untyped-strong-stored-slots).

## Current stop
- With `target/quick/hakorune` and `target/quick` lifecycle runtime, the existing smoke stops in its first generic MIR JSON precheck. It expects `unsupported terminator Invoke` but receives `[freeze:contract][array-holder/append-source]: HolderEscape`; direct EXE assertions are not reached.
- The fixed suite's latest result is 8/11, with this app among the three source-compile failures. JSON landed in `42289574b1`; this app has not passed its unchanged smoke.

## Contract and scope
- D0 was accepted on 2026-10-19 and grants implementation permission for source-backed tagged strong slots. Preserve `EmptyField`, separate value tag from residence, and verify transactional write/read/end through both final consumers and the existing runtime ABI.
- The unchanged sentinel `Holder.items = items` must transfer the exact available Array Home once. Do not infer a storage type from MIR, rewrite the app, or change its negative precheck to accept the current failure.
- Current refusal is issued by `NamedArrayAppendOnlySourceV1::issue_for_construction_v1` through `ordinary_new_named_array_source_selection.rs`; trace the original candidate and field-write source into the existing slot owner before changing admission.

## Next work
1. Identify why the exact available Array Home is classified as `HolderEscape` for this field write.
2. Connect the source-issued transfer through layout/storage, transactional commit, End, and independent final verification without a new owner or fallback.
3. Re-run this unchanged smoke; it must reach the generic Invoke negative pin, then produce and run the expected EXE (exit 7).
