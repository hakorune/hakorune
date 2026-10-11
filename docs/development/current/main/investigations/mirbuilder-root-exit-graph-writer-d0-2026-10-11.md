# MIRBUILDER-ROOT-EXIT-GRAPH-WRITER-D0

Status: decision closed; execution at `MIRBUILDER-ROOT-EXIT-GRAPH-PORT-R0`
Date: 2026-10-11
Scope: behavior-preserving graph writer boundary for selected Static root exits
Related: `mirbuilder-search-loop-root-exit-consumption-d0-2026-10-11.md`

## Known source and physical boundary

`ordinary_new_admission/selected.rs::emit_root_home_exit_payload` is the
existing physical cleanup graph owner. R0 extracted its frame access while
preserving the current graph. Its raw `append_block` inserts terminated
blocks into `MirFunction` but does not use the canonical CFG session's
creation/predecessor/seal operations. The selected Static V2 session owns a
canonical CFG whose `finish` checks every block's sealed predecessor set.
Calling the raw graph writer directly would fail that contract.

The selected two-exit path must eventually build early Plain and tail Call
graphs with the same Home order, shared frame, and sole tail Invoke. It must
leave the clean blocks unterminated for DraftSeal's exact-two Return
projection, then record the projected Return bindings before root progress
validation. That semantic extension belongs to a later slice.

## Decision: extract one graph algorithm behind a writer port

Source authority is the existing Home Normal/Fault plans and root progress
ledger. `ordinary_new_admission/selected.rs` remains the sole cleanup graph
issuer. A private writer port will carry block ID reservation, terminated
block insertion, current-block terminator emission, normal projection
insertion, and binding capture. The raw implementation delegates to today's
`append_block`, `builder.emit_instruction`, and direct projection insertion;
the initial extraction admits no selected Static caller. Include terminal
Call ingress in the same port so its physical graph does not become a
parallel path.

The canonical implementation belongs to a later slice. It must use
`CanonicalCfgSessionV1` creation and edge emission so predecessor caches
remain authoritative, then seal each graph block with canonical SSA identity.
Post-hoc repair/adoption is rejected: canonical predecessor verification
explicitly rejects stale caches. Current canonical `emit_i64_invoke` requires
empty landings while raw ingress pre-populates them; `emit_invoke_fault`
requires an already-emitted incoming Invoke while raw cleanup creates the
Fault terminal first. Canonical emission may reorder writes, but it must
retain the same block IDs, edges, source order, and recorded binding order.
Home-release Invoke operations also require a narrowly scoped canonical
writer; the existing canonical method handles only I64 Calls.

The raw binding order is contractual: clean Return first, Fault terminal,
each cleanup step's Normal Jump, Fault Jump, then Invoke, and final ingress
Jump last for Plain. The new port must keep that order even if a later
canonical implementation writes instructions in another order. The existing
root graph validators and final MIR verifier remain independent checks.

Smallest next slice: `MIRBUILDER-ROOT-EXIT-GRAPH-PORT-R0`, a behavior-preserving
raw extraction. Only after R0 may canonical writer and selected two-exit
semantic cutover be chosen; neither is claimed by this D0.

Non-claims: no progress-only ledger marking, parallel cleanup emitter,
selected Static root consumption, or EXE success.
