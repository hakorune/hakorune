# HEADERPORT0-REENTRANT-TERM0-I0: production cutover consultation

Status: **design stop; I0-SHELL-I0-P0 is closed, production cutover is not
admitted**

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

The parent P0 matrix remains green and the working tree may only advance to
the disconnected `STATE0-S0` row after this consultation is accepted.
