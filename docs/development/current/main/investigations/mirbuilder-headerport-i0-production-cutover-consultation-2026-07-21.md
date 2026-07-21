# HEADERPORT0-REENTRANT-TERM0-I0: production cutover consultation

Status: **STATE0-S0/P0/I0/G0 and CONSULT0 are closed; ACCESS0-S0 is next,
production cutover remains disconnected**

Date: 2026-07-21

Parent:
`mirbuilder-headerport-i0-source-integration-consultation-2026-07-21.md`

Decision: **state-seam-first Candidate D-prime is selected for consultation**

## Why the direct I0 is not mechanical

The disconnected shell and route matrix are complete, but the current
production ownership is still one mutable `MirBuilder` world:

```text
build_module
  -> prepare_module
  -> lower_root
  -> finalize_module
```

During that sequence:

```text
current_module
  owns the live function map
  supplies completed-function header reads
  stores module metadata/global state
  receives main and condition_fn

function_state.current_function
  owns the function currently being lowered

ModuleLoweringInvocationV1
  owns a separate collector only in disconnected tests
```

The current production path therefore cannot simply construct a shell and
collector beside `MirBuilder`:

```text
collector owns child header A
current_module owns child header A'
```

would create two authorities.  Moving only child publication, only `main`,
or only canonical routes would create the same split with a different route
order.  A fallback from the collector to `current_module` is also rejected.

The source census identifies the affected lowering-time families:

```text
header readers       8
shell metadata       2
lifecycle/publicity  2
canonical adapters   2
```

The route/failure matrix is already sealed in the parent card; this
consultation concerns the missing physical state boundary only.

## Candidates

### Candidate A — mirror `current_module` into the collector

Rejected.  A cloned or mirrored function map would preserve the old API but
make header freshness and duplicate replacement order implicit.  It also
violates the one-collector law and makes nested child re-entry depend on which
copy was refreshed last.

### Candidate B — store the invocation port in `MirBuilder`

Rejected.  This makes the collector an ambient mutable field, recreates the
snapshot/restore problem for nested functions, and allows a child body to
retain an invocation capability beyond the current lowering scope.

### Candidate C — cut one root family at a time

Rejected.  Raw, A+/trivial, and acyclic/recursive routes use different
identity and duplicate policies.  A partial cutover would make `current_module`
the header authority for one route and the collector for another, which is
precisely the authority drift the P0 matrix forbids.

### Candidate D-prime — state seam before production I0

Selected for the next design/code series.

Introduce one invocation-owned state surface before changing any production
publication callsite:

```rust
struct ModuleLoweringInvocationStateV1 {
    shell: ModuleLoweringShellV1,
    collector: ModuleDraftCollectorV1,
    root: RootCompletionStateV1,
}
```

The state surface is not a second Builder.  It owns only module-invocation
storage and completion state.  Function-local lowering remains in the
existing function state until the later Function Session series.

The required ports are explicit:

```text
LoweringHeaderPortV1
  -> collector-owned completed headers only

ModuleLoweringShellPortV1
  -> globals and accumulated metadata only

FunctionLoweringPortV1 (future seam)
  -> current function/block/facts for one active function
```

No port may expose a mutable function map or provide a `current_module`
fallback.  The first code series should define the state and reader routing
contracts without wiring a production root.

## Required state-seam laws

```text
one invocation owns one shell and one collector
completed function/header truth = collector only
module metadata/global truth = shell port only
current_module function map is not a second lowering store
main and condition_fn are unpublished until final collector drain
all route families use the same final drain owner
header loans end before admission/collection begins
```

The state seam must not yet claim:

```text
whole-Builder rollback
fact-session generation isolation
PHI/finalization repair removal
JoinIR or Loop widening
canonical callable catalog replacement
```

## Next task order

```text
HEADERPORT0-REENTRANT-TERM0-I0-STATE0-S0
  disconnected invocation-state vocabulary and ownership contract
  production consumers = 0

HEADERPORT0-REENTRANT-TERM0-I0-STATE0-P0
  map all 14 source-reader rows to state/header/shell/lifecycle owners
  and prove no reader needs a completed body during lowering

HEADERPORT0-REENTRANT-TERM0-I0-STATE0-I0
  connect the state surface to one complete invocation candidate
  without changing route behavior

HEADERPORT0-REENTRANT-TERM0-I0-STATE0-G0
  duplicate function/header stores = 0
  current_module fallback readers = 0
  production collector drains = 1

then:
HEADERPORT0-REENTRANT-TERM0-I0
  atomic all-route production capture/commit
```

## Stop conditions

Stop this series and reopen architecture consultation if any of the following
becomes necessary:

```text
collector headers are copied into current_module
current_module is used as a fallback header view
one root family is cut over independently
the invocation port is stored in Builder/CompilationContext/TLS
a state seam requires a second TypeContext or fact ledger
post-collect validation or retry is added
```

## Current non-claims

```text
production invocation capture/commit
current_module reader retirement
FunctionLoweringSession physical separation
FACTSESSION0 activation
module finalization repair removal
CUT0
```

The parent P0 matrix remains green; this consultation admitted only the
disconnected `STATE0-S0` row before any production cutover.

## STATE0-S0 closeout

The disconnected ownership seam now lives in
`src/mir/builder/module_lowering_invocation_state.rs`:

```text
ModuleLoweringInvocationStateV1
  owns one ModuleLoweringShellV1
  owns one ModuleDraftCollectorV1
  owns one RootCompletionStateV1 marker
```

The existing `ModuleLoweringInvocationDrainOwnerV1` now owns this state
product rather than carrying a parallel shell/collector pair.  It still
performs the same preflight and single-use drain, so no production root or
Builder consumer was connected.  The state exposes only borrowed shell and
collector capabilities plus an consuming `into_parts` transition at the
drain boundary; it exposes no function map, Builder, `TypeContext`, or
fallback lookup.

Focused state fixtures prove:

```text
empty shell + empty collector ownership
shell/collector/root marker consumed together at drain boundary
```

The next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-STATE0-P0`: map all 14 source-reader rows to
the state/header/shell/lifecycle owners and prove that no lowering-time reader
requires a completed function body.  Production capture/commit and `CUT0`
remain forbidden.

## STATE0-P0 owner classification

The existing source-derived 14-row census is now assigned exactly once to the
state-seam owner families:

```text
collector_header          = 8
shell_port                = 2
invocation_lifecycle      = 2
canonical_catalog_adapter = 2
completed body required    = 0
```

`collector_header` rows are the eight completed-header/presence readers and
must consume only `LoweringHeaderPortV1`.  `shell_port` rows are module
metadata/global observations and must consume only `ModuleLoweringShellPortV1`.
`invocation_lifecycle` rows own publication and terminal ordering through the
invocation state/drain owner.  `canonical_catalog_adapter` rows retain the
sealed callable catalog as their sibling/header authority and do not fall back
to collector prefixes.

The guard checks the same Rust source anchors used by the parent census and
rejects any uncategorized row or any claim that a completed function body is
needed during lowering.  This closes STATE0-P0 as a disconnected evidence
slice; no reader or production route has been rewired.

The next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-STATE0-I0`: connect the state surface to one
complete invocation candidate while preserving all route-specific identity and
failure laws.  Production capture/commit and `CUT0` remain forbidden.

## STATE0-I0 closeout

The existing disconnected `ModuleLoweringInvocationV1` now owns
`ModuleLoweringInvocationStateV1` instead of carrying a parallel collector.
Its recursive header/port and admission tests therefore exercise the same
state seam that the drain owner consumes.  The candidate creates an empty
function shell for its disconnected harness; it does not publish a module or
read `current_module`.

The structural consumer set is now exactly:

```text
ModuleLoweringInvocationV1      = one disconnected candidate
ModuleLoweringInvocationDrainOwnerV1 = one drain owner
production roots                = zero
```

No root, canonical transaction, Builder field, `CompilationContext`, or
fallback reader was connected.  The existing 24 invocation tests, state and
drain fixtures, cargo check, and HeaderPort guard remain green.

The next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-STATE0-G0`: freeze the consumer census and
prove that the state seam has no current-module fallback or production root
caller before any complete invocation cutover.  `CUT0` remains forbidden.

## STATE0-G0 closeout

The structural guard now closes the state seam boundary:

```text
ModuleLoweringInvocationStateV1 consumers = 2
  ModuleLoweringInvocationV1
  ModuleLoweringInvocationDrainOwnerV1

state-seam current_module fallback readers = 0
production invocation/drain callers = 0
```

The guard scans all builder production files, excludes only the two
disconnected owners and their inline/test fixtures, and rejects any new
invocation or drain caller.  It also rejects `current_module` inside the state
seam itself.  This is a boundary proof, not a claim that the legacy
`current_module` readers elsewhere have already been migrated; the 14-row
reader census remains the migration inventory.

The following consultation row,
`HEADERPORT0-REENTRANT-TERM0-I0-STATE0-CONSULT0`, is now closed.  It selects
the invocation-owned candidate plus explicit access-port handoff described
below.  The next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-S0`; production capture/commit and
`CUT0` remain forbidden.

## STATE0-CONSULT0 decision

The complete handoff is not a one-call replacement.  `current_module` is
currently both storage and an implicit capability, so leaving it in place
while adding a collector would create a second function/header truth.

### Rejected candidates

```text
mirror collector headers into current_module
  -> stale duplicate truth and ambiguous replacement order

keep current_module as an empty shell while migrating only one route
  -> root-family-dependent authority and one-root-at-a-time cutover

store ModuleLoweringPortV1 in MirBuilder/CompilationContext/TLS
  -> ambient mutable authority and nested-borrow hazard

rewrite the complete FunctionLoweringSession in the same row
  -> mixes state ownership with the independent function-session series
```

### Selected H-prime: invocation-owned candidate plus explicit access port

The production handoff will use one invocation candidate whose ownership is
structurally explicit:

```rust
struct ModuleLoweringInvocationCandidateV1<'builder> {
    builder: &'builder mut MirBuilder,
    state: ModuleLoweringInvocationStateV1,
    _seal: InvocationCandidateSealV1,
}
```

The candidate takes the module shell out of the Builder before body lowering;
the Builder does not retain a second function map.  Lowering receives a
short-lived `ModuleLoweringAccessPortV1` with exactly three surfaces:

```text
header surface:
  collector-owned completed signatures/presence/inventory

shell surface:
  module name, globals, accumulated metadata, closure-body interning,
  and static-data-plan lookup

terminal surface:
  capture/prepare/seal/collect and the one final drain
```

The port is threaded explicitly through raw expression/body descent.  A
header loan ends before terminal mutation.  Canonical A+/trivial and
acyclic/recursive lowering keeps its existing sealed callable catalog as
semantic header authority; the collector is only the physical unpublished
draft sink.  No current-module fallback is introduced.

The shell surface is intentionally explicit rather than a blanket metadata
borrow.  `intern_closure_body` and static-data-plan lookup are the two current
metadata readers that otherwise silently reach back into `current_module`.
ACCESS0 must either expose those operations through the shell port or project
their immutable inputs before the invocation starts; it may not retain a
`current_module` fallback.  Function maps and completed headers remain
collector-only.

Function-local state remains in the existing Builder for this series.  Moving
that state into a separate `FunctionLoweringSession` is the later
`MIRBUILDER-CLEAN0-FSESSION0` owner and must not be smuggled into this I0.

### Handoff and failure law

```text
prepare invocation:
  take the module shell
  install one invocation state
  expose no current_module function map

lower root/children:
  use explicit access-port loans
  collect nested drafts before parent restore

complete main + condition_fn:
  capture and collect as unpublished drafts

finalize candidate:
  all header reads through collector/catalog ports
  no source/body/name reconstruction

commit:
  preflight complete inventory and shell emptiness
  drain exactly once
  no fallible check or retry after drain
```

```text
primary/cleanup/admission failure before drain:
  collector prefix and shell unchanged
  parent restored exactly once

root/finalizer failure before drain:
  invocation candidate dropped
  external module unchanged

post-drain verification failure:
  unpublished candidate is discarded
  no legacy/A+/BindingSSA fallback
```

### Next task order

```text
HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-S0
  disconnected ModuleLoweringAccessPortV1 vocabulary
  production consumers = 0

HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-P0
  thread the port through all 14 reader families
  and prove no current_module function-map read remains in the candidate

HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-S0/P0
  invocation-owned shell take/restore and candidate failure transaction

HEADERPORT0-REENTRANT-TERM0-I0
  one all-route production capture/commit cutover
```

The current production I0 and `CUT0` remain forbidden until ACCESS0-P0 and
CANDIDATE0-P0 are green.  This decision closes the consultation boundary but
does not claim any production route has been rewired.
