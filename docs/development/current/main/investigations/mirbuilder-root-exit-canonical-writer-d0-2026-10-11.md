# MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-D0

Status: decision closed; execution at `MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-R1`
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

## Decision

Source authority + canonical issuer: the existing Home Normal/Fault plans,
root exit ledger, and original selected Static call packets select the same
root graph algorithm in `ordinary_new_admission/selected.rs`. The selected
Static function's `CanonicalSsaFunctionSessionV2` owns block creation, edges,
predecessors, and sealing. DraftSeal alone issues the two clean Returns.

The writer port must make block reservation fallible. The canonical adapter
allocates an ID through the current builder and immediately registers an
empty block with canonical CFG. It reserves all root targets before emitting
edges, emits every Jump through canonical CFG, and installs cleanup and
terminal Call Invokes through a narrow canonical Invoke operation that checks
the exact source, frame, distinct landings, and open targets before recording
both predecessors. The existing empty-landing I64 Invoke rule stays intact.
The root Fault terminal uses a narrow validated frame operation; the generic
direct-Incoming-Invoke Fault rule stays intact. Projected normal results are
inserted into the created open landing. Each graph block is sealed only after
all incoming edges exist, with canonical CFG and SSA identity together;
`finish` independently checks predecessor caches. The old `ids.len() == 14`
finish assumption is replaced by an exact graph receipt, never a relaxed
whole-function check. Logical binding order remains clean Return, Fault
Return, each Home's Normal Jump/Fault Jump/Invoke, then ingress. Physical
write order may differ; no caller may forge a generic terminator through the
port.

The selected early Plain and tail Call graph leave clean targets open.
DraftSeal projects both Returns onto the detached function. The selected
entry's pending `normalize_size` lexical binding group is shared by both
exits according to each source exit's `covered_calls`; the body `bin_size`
packet remains a separate retained body packet, and `huge_bin` remains the
tail terminal Call ingress. The group is not committed during detached
corroboration. A prepared two-exit root batch validates the projected
Returns, source sites, Home order, group selection, physical bindings, and
both exits against a read-only view that includes the pending group. Its
physical child check must use the same pending view. All fallible validation
finishes before collector admission. Only after successful `.collect()` are
the entry group, body retention, and both root exits committed without a new
fallible lookup. Duplicate-collector failure must leave none of these rows
published.

S0 implementation audit refined this pending-view requirement: the view must
also exclude the pending `huge_bin` terminal route and include both pending
root rows in every physical child validation reader. Its physical validation
receipt remains pending until collector success. The active S0 card's
"atomic pending view" Decision is the exact execution contract; this D0
paragraph is historical context for the R1 writer decision.

Non-authority: all-caller inference, a raw graph inserted into a canonical
session, a second cleanup emitter, a temporary ledger insert/rollback, and
`validate_selected_static_loop_child_emissions` before pending exits are
visible. No `.hako` rewrite or fallback is permitted.

Fail-fast boundary: reject duplicate/missing blocks, sealed or occupied
sources, distinct-landing/frame mismatch, stale predecessors, unexpected
binding order or source site, unprojected Return, missing/extra lexical group,
and collector rejection without publishing ledger progress.

Smallest next slice: `MIRBUILDER-ROOT-EXIT-CANONICAL-WRITER-R1` is a
behavior-preserving canonical writer preparation. It tightens the single
port and adds canonical CFG root operations with focused synthetic positive
and negative checks; existing raw Plain/Call physical MIR remains identical.
The named production caller to switch is selected Static V2
`size_to_bin`, currently stopped at `root-exit-unconsumed`; its old
responsibility is direct early/tail termination without Home root progress.
The following `MIRBUILDER-ROOT-EXIT-STATIC-TWO-EXIT-S0` consumes both graphs
and the prepared root batch in that caller, with original-source first-stop,
duplicate-collector rollback, physical JSON/EXE, and existing root validation
as acceptance. R1 alone makes no selected or EXE success claim.
