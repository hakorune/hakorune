# RAW-SOURCE0 LOWER ROOT0 — DRAIN0-S0 実行タスク

Status: **Closed — DRAIN-prime-r1 implemented and pushed as c470dc12d6**
Date: 2026-07-24
Question: `cut0-i0-raw-source0-lower-root-drain0-question-2026-07-24.md`

## Decision lock

`DRAIN-prime-r1` is selected.  The Raw ROOTBATCH0 complete owner is consumed
by one compiler terminal:

```text
RawRootBatchCompleteInvocationV1::prepare_drain(self)
  -> RejectedRawDrainInvocationV1 { inspect + discard }
  -> PreparedRawDrainInvocationV1
  -> infallible drain(self)
  -> RawDrainedInvocationV1::{Script, App}
```

The sealed Raw ledger is the sole expected-inventory authority.  Only its
`final_event_by_key` events are projected, ordered by ledger ordinal.  Stale
replacement history is retained in the ledger witness but never enters the
drain manifest.  The collector is a keyed physical store that must prove
exact key, symbol, arity, index, brand, and cardinality parity.

The shell is consumed into an opaque `RawUnfinalizedModuleV1`; DRAIN0 does
not return a bare `MirModule`, run finalization, or perform external commit.

## Implementation slices

### DRAIN-MANIFEST0

Add the neutral `crate::mir::raw_physical_drain` vocabulary:

```text
RawPhysicalDrainRouteV1
RawPhysicalDrainKeyV1
RawPhysicalDrainRoleV1
RawPhysicalDrainPolicyV1
RawPhysicalReceiptProvenanceV1
RawPhysicalDrainRowV1
RawPhysicalDrainManifestV1
```

`raw_root_physical/drain_manifest.rs` projects only sealed ledger final
events.  It rejects unsupported Raw roles/keys, duplicate ordinal/key/symbol,
wrong policy, missing root pair, illegal helper/callable/root/condition order,
and callable-Main disposition drift.

### DRAIN-COLLECTOR0

`module_draft_collector/raw_drain.rs` provides a Raw-specific keyed prepare:

```text
ModuleDraftCollectorV1::prepare_raw_drain(manifest, brand)
  -> PreparedRawCollectorDrainV1
  -> infallible drain in manifest order
```

Preparation is mutation-free.  It proves collector brand, exact cardinality,
key set, symbol index bijection, draft symbol, draft arity, and no surplus
draft.  The collector does not define the expectation and no
`into_draft_functions()` path is used.

### DRAIN-PHYSICAL0

`raw_root_physical/drain_terminal.rs` consumes the named
`CompletedRawRootBatchPhysicalV1` owner through a Builder sibling terminal.
Before shell mutation it validates:

```text
Raw family
session/shell/collector/ledger/root brands
function-empty shell
sealed-ledger manifest and route topology
collector keyed parity
```

The prepared product owns the session, prepared shell drain, prepared keyed
collector drain, sealed ledger, root witness, and exact manifest.  Its only
terminal is infallible `drain(self)`, which creates the opaque unfinalized
module and a non-Clone `RawDrainWitnessV1`.

### DRAIN-I0

`compiler/raw_root_drain.rs` owns the route-specific compiler handoff.  It
keeps Script and App typed, preserves callable-Main outcome evidence, runtime
snapshot, continuation, helper receipts, completion evidence, and the opaque
physical product.  Rejection retains all route evidence and the rejected
physical owner; it exposes stage/error/discard only.

## Required guards

```text
prepare_drain(self) producer                         = 1
PreparedRawDrainInvocationV1::drain(self) producer   = 1
RawPhysicalDrainManifestV1 producer                  = 1
ledger final-event projection                        = 1

collector key order as expectation                  = 0
source/AST/catalog/current_module re-observation     = 0
caller symbol vector / require_main / policy input   = 0
canonical or DrainedModuleCandidate adapter         = 0
raw collector into_draft_functions caller           = 0
bare MirModule return                                = 0
shell mutation before complete preflight             = 0
retry/resume/fallback/second drain                   = 0
finalization/postprocess/external commit consumer    = 0
public ingress/JSON/CUT0 activation                  = 0
all touched source/check files                       < 800 lines
```

## Focused acceptance matrix

```text
success:
  empty Script -> Script drained product
  App + NotSelected -> App drained product with no callable row
  App + Selected -> App drained product with callable evidence

manifest:
  final ordinal order
  stale replacement event excluded
  required Main/condition pair exactly once
  callable disposition/event parity

collector:
  missing/surplus key
  symbol index drift
  symbol mismatch
  arity mismatch
  foreign brand

atomic rejection:
  complete route owner retained
  shell function count unchanged
  collector and sealed ledger unchanged
  no second drain/retry/fallback
```

## Explicit non-claims

```text
Raw finalization                 = 0
postprocess                      = 0
external commit                  = 0
production executor              = 0
public compile ingress           = 0
AST JSON / Program(JSON v0)      = 0
legacy retirement                = 0
CUT0 activation                  = 0
typed panic retention            = 0
```

All modified/new source and check files must remain below 800 lines.  The
next design boundary after this row is Raw finalization; it must consume only
`RawDrainedInvocationV1` and must not reacquire source authority.

## Closeout

`DRAIN-MANIFEST0`, `DRAIN-COLLECTOR0`, `DRAIN-PHYSICAL0`, and `DRAIN-I0/G0`
are closed. Focused Raw Script/App fixtures, `cargo check --lib`, the current
state pointer guard, the DRAIN0 structural guard, rustfmt, and `git diff
--check` are green. The next design stop is
`RAW-SOURCE0-LOWER0-ROOT0-FINAL0-CONSULT0`.
