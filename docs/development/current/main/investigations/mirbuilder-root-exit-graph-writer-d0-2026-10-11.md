# MIRBUILDER-ROOT-EXIT-GRAPH-WRITER-D0

Status: selected design stop
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

## Decision required

- Identify the smallest physical graph writer port that shares the existing
  node/edge/recording algorithm while allowing raw and canonical CFG
  operations. Map block creation, terminators, predecessor updates, sealing,
  current-block selection, and binding order individually.
- Keep one cleanup graph issuer and preserve generic root graph bytes. The
  behavior-preserving extraction must not admit selected Static yet.
- Name existing positive/negative tests that compare root Plain, terminal
  Call, cleanup Fault paths, and malformed binding/edge rejection. No new
  test or guard by default.
- If exact behavior cannot be retained by a writer port alone, state the
  missing contract and choose the smallest bounded adjustment before code.

Non-claims: no progress-only ledger marking, parallel cleanup emitter,
selected Static root consumption, or EXE success.
