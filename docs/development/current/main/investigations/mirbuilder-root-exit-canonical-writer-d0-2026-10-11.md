# MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-D0

Status: selected design stop
Date: 2026-10-11
Scope: canonical CFG implementation of the existing root graph writer port
Related: `mirbuilder-root-exit-graph-writer-d0-2026-10-11.md`, `mirbuilder-root-exit-graph-port-r0-2026-10-11.md`

## Fixed boundary

The raw writer port now carries one existing root cleanup graph algorithm
and terminal Call ingress. Its unchanged Plain/Call MIR and root binding
order were physically compared before/after R0. The unchanged SizeClassBox
first stop remains `root-exit-unconsumed`; this D0 grants no selected Static
root progress or EXE success.

Canonical CFG owns block creation, edge/predecessor registration, sealing,
and Invoke landing validation. Raw completed-block insertion cannot be used
inside that session. The existing canonical I64 Call method requires empty
landings, while raw root ingress fills landing Jumps first. Cleanup Invoke
operations also need a canonical writer beyond that I64 Call method.

## Decision to close

Map each `RootGraphWriterPortV1` operation to canonical session operations
without a second cleanup algorithm. Specify the required write order,
reserved block ID and SSA identity rules, predecessor sealing, generic
cleanup Invoke handling, and how terminal Call projection/landing retains
the same graph. The two selected Static exits need DraftSeal to issue their
Returns before root binding validation; decide the exact callback point for
recording projected Return bindings. Preserve the existing Normal/Fault Home
order and sole tail Call Invoke. If the port is insufficient, revise that
single port rather than adding a parallel emitter.

Smallest next slice: one bounded canonical writer plus selected two-exit
consumption only after these mappings and focused positive/negative
acceptance are fixed. No progress-only ledger marking or raw graph adoption.
