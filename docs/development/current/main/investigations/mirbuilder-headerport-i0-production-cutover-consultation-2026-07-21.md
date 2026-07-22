# HEADERPORT0-REENTRANT-TERM0-I0: production cutover consultation

Status: **STATE0-S0/P0/I0/G0, CONSULT0, ACCESS0-S0, and
ACCESS0-MEHEADER-S0/P0/I0/G0, ACCESS0-REWRITE-KNOWN-P0, ACCESS0-P0, and
CANDIDATE0-S0/P0, MAINROLE0-D0/S0/P0, BODYDRAIN0-S0/P0,
MAINPENDING0-S0/P0, ROOTBATCH0-S0/P0, SHELLFACT0-S0/P0, and
DRAIN0-S0/P0, MODULEFINAL0-SPLIT0/P0, MODULEFINAL0-CANDIDATE0-P0, and
WIRING-S0/P0 and WIRING-I0-HDR0-M0 are closed; M-root-prime and Candidate
A-prime are selected. The annotation and constructor/birth passive
`WIRING-I0-HDR0-P0` slices are landed; unresolved reader families remain in P0;
production capture/commit and
CUT0 remain forbidden until replacement/parity,
the compatibility-policy consultation, and the all-route cutover gates are
green**

Date: 2026-07-21

Parent:
`mirbuilder-headerport-i0-source-integration-consultation-2026-07-21.md`

Decision: **Candidate M-root-prime is selected for the production I0 task series**

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
HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-S0 (closed)
  disconnected ModuleLoweringAccessPortV1 vocabulary
  production consumers = 0

HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-P0
  thread the port through all 14 reader families
  and prove no current_module function-map read remains in the candidate

  First disconnected slice:
  ACCESS0-REWRITE-KNOWN-S0
    explicit header projection for Known/unique rewrite policy
    shared method-tail candidate policy
    production consumers = 0

  ACCESS0-MEHEADER-CONSULT0 (closed)
    Candidate A-prime typed-source header observation decision

  ACCESS0-MEHEADER-S0 (closed)
    disconnected owned observation and pure receiver/arity prepare

  ACCESS0-MEHEADER-P0
    (closed) legacy/raw/invocation/located parity and loan-before-arguments proof

  ACCESS0-MEHEADER-I0
    connect only the shared `MeCallPolicyBox` to the typed observation

  ACCESS0-REWRITE-KNOWN-P0
    (closed) parity matrix and invocation-path lookup threading

  HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-P0
    (closed) broad reader census closeout after all disconnected adapters are green

HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-S0 (closed)
  one disconnected invocation-owned shell/collector candidate
  scoped Builder loan
  typed abort/no-publication/no-retry proof

HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-P0 (closed)
  nine-row route/failure co-seal
  duplicate/missing/drift rejection
  production consumers = 0

HEADERPORT0-REENTRANT-TERM0-I0-WIRING-CONSULT0
  current design-stop frontier: shell-aware root wiring and port-aware
  finalizer must be sealed before production capture/commit

HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0 (closed)
  live shell/header bundle, port-aware Main/root order vocabulary, and
  explicit finalizer lookup; production consumers = 0
```

The current production I0 remains disconnected until the shell-aware root
wiring is implemented and green; `CUT0` remains forbidden.  This consultation
does not claim any production route has been rewired.

The `me` method reader consultation selected Candidate A-prime with a
typed-source refinement. Its disconnected S0/P0/I0/G0 vocabulary and parity
proof are closed. The Known/unique/equals rewrite P0 is also closed with its
lookup-only parity guard, and the broader access-port census is now green with
zero production access-port consumers. Candidate0 remains before any
production capture/commit or CUT0 decision.

## CANDIDATE0-S0 closeout

`module_lowering_invocation_candidate.rs` now owns one private,
non-Clone `ModuleLoweringInvocationCandidateV1` around the existing shell and
collector state.  The candidate lends `MirBuilder` only to an explicit active
lowering closure; it stores no Builder, `current_module`, collector port, fact
session, or retry authority.

The candidate's only failure transition is a typed abort product.  It records
the failure stage, compares the collector symbol prefix, shell publication
count, and root marker before/after abort, and fixes the terminal law to:

```text
external publication = unchanged
retry/fallback = forbidden
shell + collector = dropped together
```

The six root/child/preflight/verification/panic stages share the same
no-publication law.  `PendingFunctionSessionCloseV1` remains the sole owner of
parent function-state restoration; Candidate0-S0 does not duplicate that
transaction or connect a production route.  The reusable guard is
`tools/checks/lib/headerport_candidate0_guard.py`; focused Candidate0 tests,
row guard, formatting, and diff checks are green.  Production capture/commit
and CUT0 remain disconnected until the separate all-route I0 is implemented.

## CANDIDATE0-P0 closeout

`module_lowering_invocation_candidate_p0.rs` now consumes the existing
`InvocationRouteMatrixV1` as the sole route authority.  It does not recreate
raw, A+, trivial, acyclic, recursive, main, or condition-fn identity/policy.
Each of the nine matrix rows is observed exactly once and is co-sealed with
one Candidate0 abort proof.

The P0 product rejects duplicate or missing rows, route identity/publication
drift, changed shell/collector boundaries, changed external publication, and
any retry disposition.  The matrix's collector-prefix, parent-restore, root
drop, and retry laws are rechecked before the non-Clone proof is issued.
Focused route co-seal and duplicate-row fixtures are green; production
capture/commit, module drain, FACTSESSION, and CUT0 remain disconnected.

## I0 wiring audit: design stop before production capture/commit

The first production wiring audit found a concrete boundary that the
disconnected products do not yet close.  This is new source evidence, not a
reason to add a partial route cutover.

```text
ModuleLoweringPortV1
  currently owns only the collector/header loan

ModuleLoweringAccessPortV1
  names shell operations but has no live shell capability

RawInvocationChildPortV1
  can lower nested children through the collector
  rejects the root Main box

build_module/lower_root/finalize_module
  still lower through the legacy current_module path
  and finalize by taking current_module back
```

The affected production surfaces are observable in the current source:

```text
src/mir/builder/module_lifecycle.rs
  root declaration pass, Main entry, static-data writes, final module take

src/mir/builder/decls.rs
  build_static_main_box lowers the root wrapper directly

src/mir/builder/recursive_child_lowering.rs
  invocation child port is collector-aware, but its root Main operation is
  intentionally rejected

src/mir/builder/calls/lowering.rs
  the legacy finalizer still uses the current-module compatibility facade
```

Therefore an all-route I0 cannot safely be implemented by simply constructing
the existing candidate around `build_module`.  The following shortcuts remain
rejected:

```text
leave current_module as a metadata/function-map mirror
  -> duplicate header authority

cut only RawInvocationChildPortV1 first
  -> raw/Main/canonical route-family drift

make Main rejection disappear without a port-aware root builder
  -> root lowering silently returns to the legacy terminal

drain children before finalization while finalization still reads
current_module.functions
  -> collector/header freshness split
```

### Selected refinement: shell-aware root wiring

The next implementation series must first add a live, short-lived shell
capability and a port-aware root finalizer.  It may reuse the current
function-local Builder state, but it must not store a port in Builder,
CompilationContext, or TLS.

```text
I0-WIRING-S0 (closed)
  live ModuleLoweringShellPortV1 + collector/header bundle
  root Main/static-box port-aware entry
  finalizer lookup injection
  production consumers = 0

I0-WIRING-P0 (closed)
  raw child, Main, script, A+/trivial, acyclic, recursive, condition_fn
  route and failure parity

I0-WIRING-I0
  one all-route capture -> seal -> collect -> drain

I0-WIRING-G0
  current_module function-map reads during invocation = 0
  shell/header stores = 1 each
  production drains = 1
  partial route cutovers = 0
```

This is a design stop under the repository rule: no production code is
changed until the shell-aware root entry and the exact finalizer ownership are
sealed.  `FACTSESSION0`, FunctionLoweringSession separation, PHI/finalization
repair removal, JoinIR, and CUT0 remain outside this refinement.

## ACCESS0-REWRITE-KNOWN-S0 closeout

The first disconnected ACCESS0 slice now owns one
`KnownRewriteHeaderViewV1`.  It borrows only the explicit
`FunctionSignatureLookupV1` surface and projects:

```text
completed-symbol presence
function parameter count
receiver-prepend decision
unique method/arity candidate list
```

The method-tail policy is shared with the legacy method index through
`method_candidates_from_symbols` and `method_candidates_from_headers`; the
projection does not own a Builder, module map, rewrite emission, ValueId,
metadata, or fallback.  Two focused fixtures cover static-versus-instance
arity and unique-suffix ordering.  This closes only the disconnected S0
vocabulary; the invocation terminal still has zero lookup consumers and
production capture/commit remains forbidden.

The next disconnected adapter extends the same view through Known/unique
rewrite, equals/1, and the unified emitter.  Legacy `None` lookup facades
retain the existing `current_module` and method-index behavior; an explicit
lookup never falls back to those readers.  The lookup remains short-lived and
is forwarded to global presence, signature arity, annotation, candidate
trace, and rewrite selection without storing it in `MirBuilder` or TLS.

The raw terminal now has an explicit short-lived header-capability hook:
legacy raw ports return `None`, while `RawInvocationChildPortV1` lends the
collector view only during terminal emission.  The hook is not stored in the
port, Builder, or TLS.  `ACCESS0-REWRITE-KNOWN-P0` is still not closed: the
parity matrix must cover missing headers, static/instance arity, unique
0/1/>1 candidates, environment gates, primitive guards, and error/no-retry
behavior, and the production-consumer guard must remain zero until the
candidate cutover.

The reusable HeaderPort guard requires the four disconnected adapter anchors
(`KnownRewriteHeaderViewV1`, the Known/unique and equals lookup entries, and
`emit_unified_call_with_lookup`) while keeping the production access-port
consumer count at zero. The focused P0 guard is
`tools/checks/lib/rewrite_header_p0_guard.py`; it fixes the 0/1/>1 candidate,
arity, gate, primitive, and no-retry matrix without adding a production route.

## M-root-prime decision lock and task order

The external design review resolves the I0 wiring stop. `Main` is not one
pending function draft. It is a source-expansion coordinator which produces
distinct function identities:

```text
static box Main
  root entry:        key=Main, symbol=main, arity=0
  static children:   Main.foo/N, ...
  optional compat:   Main.main/N
```

The selected lifecycle is:

```text
VerifiedMainExpansion
  -> collect static children (including optional Main.main/N)
  -> lower Main.main inline as the active root function
  -> CompletedRootBodyV1
  -> complete root main with one final collector-header loan
  -> prepare/collect Main + optional condition_fn as one root batch
  -> seal module declaration facts into the function-empty shell
  -> route-owned drain -> DrainedModuleCandidateV1
  -> finalize drained module exactly once
  -> one external lifecycle commit
```

The root body completion witness is distinct from collector inventory. It
proves that recursive body descent, child restores, pending terminals, and
header loans are closed; it is not inferred from symbol count. The module
finalizer runs only on `DrainedModuleCandidateV1` and cannot receive Builder,
`current_module`, collector, HeaderPort, or function-local facts.

### Task order

```text
HEADERPORT0-I0-MAINROLE0-D0 (closed by this decision)
  Main source expansion and distinct root/child identities

HEADERPORT0-I0-MAINROLE0-S0/P0 (closed)
  VerifiedMainExpansionV1, root/child/compat dispositions, and
  app/script/feature/source-order parity fixtures

HEADERPORT0-I0-BODYDRAIN0-S0 (closed)
  CompletedRootBodyV1 activity vocabulary and one-shot witness

HEADERPORT0-I0-BODYDRAIN0-S0/P0 (closed)
  CompletedRootBodyV1 witness and nested child/pending-loan failure parity

HEADERPORT0-I0-MAINPENDING0-S0/P0 (closed)
  root completion with explicit short-lived collector/header loan and parity

HEADERPORT0-I0-ROOTBATCH0-S0/P0 (closed)
  Main + condition_fn prepared admissions and policy/failure parity

HEADERPORT0-I0-SHELLFACT0-S0/P0 (closed)
  one-way source declaration fact snapshot and lane/failure parity

HEADERPORT0-I0-DRAIN0-S0 (closed)
  route-owned inventory witness and non-Clone drained candidate

HEADERPORT0-I0-DRAIN0-P0
  exact drain/inventory/condition policy and failure matrix

HEADERPORT0-I0-MODULEFINAL0-SPLIT0 (closed)
  post-drain finalization input

HEADERPORT0-I0-MODULEFINAL0-SPLIT0-P0 (closed)
  ownership and declaration/fact failure matrix

HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0
  child/root/drain/finalizer failure matrix

HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0
  closed
  live shell/header bundle, port-aware Main/root order vocabulary,
  finalizer lookup seam, production consumers = 0

HEADERPORT0-REENTRANT-TERM0-I0-WIRING-P0
  closed
  source-derived route anchors, owner lanes, condition policy, and
  entered/changed/fallback/publication observation requirements

HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0
  parked behind I0-CONSULT0 design stop
  one all-route capture -> seal -> collect -> drain

HEADERPORT0-REENTRANT-TERM0-I0-WIRING-G0
  duplicate stores/fallback/partial cutover guard

HEADERPORT0-I0-CANDIDATE0-P0
  child/root/drain/finalizer failure matrix

FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0
  one all-route production capture/commit cutover

HEADERPORT0-I0-G0
  old terminals, direct module insertion, and current_module header reads = 0
```

The following remain forbidden throughout the series:

```text
Main.main/N replacing the root main wrapper
placeholder main header before root completion
second collector or header cache
current_module fallback during invocation
post-child-collect fallible work before parent restore
bare MirModule between drain and finalization
FACTSESSION0 or FunctionLoweringSession cutover
JoinIR/Loop widening
CUT0 before all-route parity is green
```

## MAINROLE0-P0 closeout

`HEADERPORT0-I0-MAINROLE0-P0` is closed with production consumers still zero.
The source-only expansion product now has focused parity fixtures for:

```text
app-shaped Program with unrelated top-level statements
script-shaped Program without static Main
static root and child contract rejection
duplicate Main declaration rejection
deterministic child symbol order and distinct Main.main/N compatibility identity
```

The product does not read environment or feature state; compatibility policy
remains an explicit later route decision. No Builder, collector, ValueId,
metadata, header cache, fallback, or publication authority was added. The
next row is `HEADERPORT0-I0-BODYDRAIN0-P0`; production capture/commit, root
completion, drain, module finalization, FACTSESSION0, and CUT0 remain
disconnected.

## BODYDRAIN0-S0 closeout

`HEADERPORT0-I0-BODYDRAIN0-S0` is closed with production consumers still zero.
The disconnected `root_body_completion` product provides one non-Clone
`CompletedRootBodyV1` witness over an explicit value/no-value disposition. A
short-lived token tracks child scopes, header loans, and pending terminals;
completion rejects any open activity, foreign token, or token-kind mismatch.
The product owns no Builder, collector, function map, header cache, fact store,
ValueId allocator, fallback, or publication route. Its focused fixtures cover
empty no-value completion, nested activity closure, open-activity rejection,
and foreign/mismatched-token fail-fast behavior.

The next row is `HEADERPORT0-I0-MAINPENDING0-S0`; root completion parity and
the root failure matrix remain disconnected, and production capture/commit,
FACTSESSION0, and CUT0 are still forbidden.

## BODYDRAIN0-P0 closeout

`HEADERPORT0-I0-BODYDRAIN0-P0` is closed with production consumers still
zero. The matrix proves nested child completion is inner-before-outer, header
loans and pending terminals close before the root witness, each open activity
has a distinct typed failure, and a failed completion consumes the tracker
without producing a witness. These are disconnected fixture observations;
they do not open a capture, collector, Builder, or module-finalization route.

The next row is `HEADERPORT0-I0-MAINPENDING0-P0`; root completion, Main batch
collection, drain, FACTSESSION0, and CUT0 remain forbidden.

## MAINPENDING0-S0 closeout

`HEADERPORT0-I0-MAINPENDING0-S0` is closed with production consumers still
zero. `MainCompletionRequestV1` consumes a short-lived
`MainHeaderLoanV1` and returns a non-Clone `PendingMainDraftV1` that owns the
unpublished `MirFunction`, the completed-root witness, root identity, return
disposition, and only the header-source tag. The pending draft stores no
header borrow, Builder, collector, function map, fallback, or publication
authority. Symbol/arity pairing is checked before the pending product is
issued, and the disconnected fixtures prove header-loan expiry plus foreign
draft rejection.

The next row is `HEADERPORT0-I0-ROOTBATCH0-S0`; Main/condition_fn batching,
drain, FACTSESSION0, and CUT0 remain forbidden.

## ROOTBATCH0-S0 closeout

`HEADERPORT0-I0-ROOTBATCH0-S0` is closed with production consumers still zero.
`PreparedRootDraftBatchV1` owns the already-pending root draft, an optional
validated synthetic `condition_fn` draft, and explicit Main/condition_fn
admission plans. Required, optional-missing, and forbidden-present policies
are explicit; condition symbol/arity is checked before the batch product is
issued. The batch stores no collector borrow, Builder, module map, fallback,
or publication capability, and no collection or drain route was connected.

The next row is `HEADERPORT0-I0-ROOTBATCH0-P0`; collision/failure parity,
collector collection, drain, FACTSESSION0, and CUT0 remain forbidden.

## ROOTBATCH0-P0 closeout

`HEADERPORT0-I0-ROOTBATCH0-P0` is closed with production consumers still
zero. The policy matrix proves required/optional/forbidden condition_fn
behavior, optional present/missing admission counts, and typed symbol/arity
failures before a batch product exists. Main remains the sole primary
admission; no collision mutation, collector borrow, drain, fallback, or retry
route was introduced.

The next row is `HEADERPORT0-I0-SHELLFACT0-S0`; collection, drain,
FACTSESSION0, and CUT0 remain forbidden.

## SHELLFACT0-S0 closeout

`HEADERPORT0-I0-SHELLFACT0-S0` is closed with production consumers still
zero. `SealedModuleDeclarationFactsV1` owns deterministic user-box,
typed-field, record, and enum declaration lanes only. It contains no AST
body, Builder, function map, collector, derived layout plan, fallback, or
publication capability. BTree-backed snapshots and all-four-lane fixtures
prove insertion-order stability and preserve the exact source declaration
payload without semantic refresh.

The next row is `HEADERPORT0-I0-SHELLFACT0-P0`; shell publication, drain,
FACTSESSION0, and CUT0 remain forbidden.

## SHELLFACT0-P0 closeout

`HEADERPORT0-I0-SHELLFACT0-P0` is closed with production consumers still
zero. The matrix proves all four declaration lanes move together at the
future shell boundary, while empty and populated lane shapes remain explicit.
The snapshot is non-Clone and consuming `into_parts` is the only way to move
all lanes together; no shell mutation, derived-plan refresh, collector borrow,
drain, fallback, or retry route was added.

The next row is `HEADERPORT0-I0-DRAIN0-P0`; shell publication, production
drain, FACTSESSION0, and CUT0 remain forbidden.

## DRAIN0-S0 closeout

`HEADERPORT0-I0-DRAIN0-S0` is closed with production consumers still zero.
`CompletedInvocationInventoryV1` owns the route-produced symbol inventory,
the completed-root witness, and the explicit condition-function policy. It
rejects duplicate inventory symbols before candidate issuance. The
non-Clone `DrainedModuleCandidateV1` accepts only an exact module-function
set, requires `main`, enforces the condition policy, and exposes only borrowed
module/inventory views; it has no Builder, collector, retry, fallback, or bare
module extraction API. Focused fixtures cover exact co-seal, duplicate and
condition failures, and the absence of a bare-module consumer. The product is
disconnected: production capture/commit, shell mutation, FACTSESSION0, and
CUT0 remain forbidden.

## DRAIN0-P0 closeout

`HEADERPORT0-I0-DRAIN0-P0` is closed with production consumers still zero.
The disconnected matrix exercises the old shell/collector preflight and the
new candidate boundary together: deterministic inventory order, required,
optional, and forbidden `condition_fn` policy, missing `main`, exact
inventory mismatch, duplicate inventory entries, and candidate issuance
failures are all typed before a drained candidate can be observed. The
fixtures also prove that the candidate exposes only borrowed module and
inventory views; no bare `MirModule` extraction, retry, fallback, shell
mutation, or production drain was added. The next row is
`HEADERPORT0-I0-MODULEFINAL0-SPLIT0-P0`; FACTSESSION0 and CUT0 remain forbidden.

## MODULEFINAL0-SPLIT0 closeout

`HEADERPORT0-I0-MODULEFINAL0-SPLIT0` is closed with production consumers
still zero. `DrainedModuleFinalizationInputV1` co-seals the non-Clone drained
module candidate with the sealed module declaration snapshot and exposes only
borrowed candidate/fact views plus one consuming `into_parts` handoff. It does
not expose a bare `MirModule`, Builder, collector, function-local fact store,
retry, fallback, or publication API. Focused fixtures prove both owners move
together and cannot be split through a clone or second store. The next row is
`HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0`; FACTSESSION0 and CUT0 remain forbidden.

## MODULEFINAL0-SPLIT0-P0 closeout

`HEADERPORT0-I0-MODULEFINAL0-SPLIT0-P0` is closed with production consumers
still zero. The matrix preserves all four declaration-fact lanes, keeps the
root value/no-value witness separate from module facts, and proves that the
candidate and declaration snapshot can only move together through one
consuming input. No clone, second fact store, Builder, collector, bare module,
retry, fallback, or production finalizer was introduced. The next row is
`HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0`; FACTSESSION0 and CUT0 remain
forbidden.

## MODULEFINAL0-CANDIDATE0-P0 closeout

`HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0` is closed with production
consumers still zero. The passive six-row failure matrix assigns child
primary/cleanup/admission failures to collector-prefix preservation and one
parent restore, while root completion, drain preflight, and post-drain
verification failures discard the unpublished invocation. Every row keeps
external publication unchanged and forbids retry/fallback. This is only the
failure ownership contract; it does not execute module repair, drain, or
finalization. The next row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0`; FACTSESSION0 and CUT0 remain
forbidden.

## WIRING-S0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0` is closed with production
capture/commit still zero.  The live invocation seam now owns one short-lived
shell/header bundle: shell metadata access, collector-owned header lookup, and
finalizer signature lookup are separate loans and no Builder or ambient module
function map is stored in the bundle.  The focused fixture proves that a shell
write, collector header read, and finalizer lookup can occur in sequence while
the Builder value cursor remains unchanged.

The Main/root side is represented by one disconnected
`MainRootWiringPlanV1`.  It keeps root `main` (`symbol = "main", arity = 0`)
distinct from optional `Main.main/N` compatibility children and fixes the
order to static children, optional compatibility child, then inline root body.
The plan owns no AST, Builder, collector, ValueId, metadata, or publication
route.  Main root rejection in the invocation child port remains unchanged;
this row adds vocabulary and fixtures only.

The next row is `HEADERPORT0-REENTRANT-TERM0-I0-WIRING-P0`, which must prove
raw-child, Main, script, A+/trivial, acyclic, recursive, and condition-function
parity before any capture/commit wiring.  `FACTSESSION0`, partial route
cutover, and `CUT0` remain forbidden.

## WIRING-P0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-P0` is closed as a disconnected
parity product.  `HeaderPortWiringParityV1` derives one row from the existing
nine-row `InvocationRouteMatrixV1`; it does not redeclare route identity,
publication policy, or failure law.  Each row records exact source entry
anchors, legacy owner lanes, intended shell/collector or canonical target
owners, condition-function policy, and the required `entered`, `changed`,
header-lookup, publication, and fallback-forbidden observations.

The source-anchor guard checks the referenced Rust entry symbols, while the
existing 14-row reader census remains the authority for direct publication and
lookup sites.  Fixtures cover raw script/Main expansion, static and constructor
child capture, synthetic `condition_fn`, A+/trivial single functions, and
acyclic/recursive callable batches without creating a second collector or
applying a raw `main` policy to canonical routes.

This closeout proves only route inventory and observation requirements.  It
does not claim that any production route has entered or changed, and it does
not connect capture, commit, drain, or finalization.  The next row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0`; `condition_fn` miss/stub behavior,
collector-miss no-fallback, and all-route atomicity remain explicit I0 gates.

## WIRING-I0-CONSULT0 design closeout

The route parity census exposes a policy boundary that must be resolved before
production capture/commit.  The current raw lifecycle assumes a root `main`
and may synthesize or tolerate `condition_fn`; canonical A+/trivial and
acyclic/recursive callable modules legitimately contain only resolved callable
symbols and do not have a raw `main`.  Therefore a single raw `main` drain
expectation cannot be reused as the canonical module contract.

### Source authority

```text
raw source expansion       module_lifecycle.rs / decls.rs
raw child entry            recursive_child_lowering.rs
condition policy           root_draft_batch.rs / module_invocation_drain.rs
canonical root selection   compiler/mod.rs
callable batch publication  resolved_lowering/callable_module_transaction.rs
header/read census         existing 14-row HeaderPort reader inventory
```

### Non-authority

```text
collector symbol prefix as a root-policy oracle
raw `main` required flag on canonical routes
condition_fn materializer fallback value
current_module lookup after an explicit collector miss
post-drain MIR scan to infer route identity
caller-authored symbol lists as the canonical inventory
```

### Candidate slices

```text
Candidate A: route-owned invocation inventory
  Raw inventory owns root main + script/Main condition policy.
  Canonical inventory owns callable-key/cardinality policy.
  One drain product consumes a tagged route inventory without merging policy.

Candidate B: common collector, route-specific drain expectations
  All drafts share collection, but each route seals its own required roots,
  symbol set, and condition policy before the collector is drained.

Candidate C: canonical adapter into raw Main shell
  rejected: invents a synthetic main for canonical modules and duplicates
  header/return authority.
```

### Selected refinement: Candidate A-prime

Candidate A-prime is selected.  The physical collector remains singular, while
the drain policy is owned by a tagged route inventory.  Raw/Main owns its root
`main` and script/`condition_fn` policy; A+/trivial and acyclic/recursive
callable routes own their resolved callable-key/cardinality policy.  These
policies are not merged and the raw `main` requirement is never imposed on a
canonical module.

The first code-facing slice is still passive:

```text
WIRING-I0-ROUTEINV-S0
  RouteOwnedInvocationInventoryV2 vocabulary
  route-specific root/condition policy
  exact ingress/root symbols and three-valued reachability
  production consumers = 0

WIRING-I0-ROUTEINV-P0
  all four ingress families and failure/duplicate matrices
  collector-miss no-fallback and no caller-authored inventory
  production consumers = 0

WIRING-I0-BORROW-S0
  outer orchestration helper and short ModuleLoweringPort loans
  no Builder-held invocation/collector mirror
  production consumers = 0

WIRING-I0-HDR0
  route-owned header/read authority census before function-map removal

WIRING-I0-CUT0
  one all-route capture -> seal -> collect -> drain -> finalizer -> commit
  only after the preceding passive products are green
```

The physical cutover order is fixed as:

```text
header/read authority
-> child draft capture/commit
-> canonical draft collection
-> Main/root batch
-> shell declaration facts
-> route-owned drain
-> post-drain finalizer
-> external commit
```

`WIRING-I0-CUT0` must connect Raw/Main, A+/trivial, and acyclic/recursive
routes atomically; a canary or partial route cutover is not admitted.  The
orchestration helper is required because `ModuleLoweringInvocationV1` holds a
long-lived Builder borrow: production ingress methods must not store an
invocation/collector mirror in `MirBuilder` or reborrow it directly.  FACTSESSION,
PHI repair, JoinIR conversion, and repository-wide CUT0 remain outside this
series.

### Explicit non-claims

```text
canonical modules can be drained by raw `main` policy
condition_fn fallback is semantically harmless
old and new collector inventories are interchangeable
any production route has entered or changed
FACTSESSION0 or FunctionLoweringSession separation is complete
```

## MAINPENDING0-P0 closeout

`HEADERPORT0-I0-MAINPENDING0-P0` is closed with production consumers still
zero. The disconnected matrix preserves the selected header source tag for
invocation-collector and module-compatibility loans without a fallback, and
preserves both root value and explicit no-value dispositions. Root symbol and
arity checks remain the only draft pairing gate; no collector, Builder,
module-finalizer, or retry route was introduced.

The next row is `HEADERPORT0-I0-ROOTBATCH0-S0`; Main/condition_fn collection,
drain, FACTSESSION0, and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-S0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-S0` is closed as a
disconnected route-policy product. `RouteOwnedInvocationInventoryV2` derives
only from the existing `InvocationRouteMatrixV1`; it does not recreate the
nine route identities or accept a caller-authored function-symbol list.

The tagged product keeps four drain-policy lanes distinct:

```text
Raw
  inventory authority = raw expansion receipts
  root policy          = required main
  condition policy     = selected by the sealed raw source shape

Canonical A+ / Binding-SSA trivial
  inventory authority = exact resolved owner
  root policy          = exact canonical owner
  condition policy     = forbidden

Binding-SSA acyclic
  inventory authority = exact callable catalog
  root policy          = exact callable catalog
  condition policy     = forbidden

Binding-SSA recursive
  inventory authority = exact callable catalog
  root policy          = exact callable catalog
  condition policy     = forbidden
```

Each lane seals its exact Rust ingress and lowering-root symbols. Static
reachability is represented as `Reachable | Unreachable | Unknown`; only
`Reachable + Reachable` may issue the policy, while the other states fail with
a typed seal error. Every lane fixes fallback/retry authority to absent. The
product stores no Builder, collector, module, function draft, ValueId,
TypeContext, fact map, or source AST and intentionally has no `Clone`.

Focused fixtures prove the four policy lanes, the separate raw/canonical
root and condition laws, exact ingress/root symbols, and fail-closed unknown
or unreachable topology. The reusable HeaderPort guard owns the source-shape,
line-count, negative-field, registration, and production-consumer-zero checks.

This row does not yet co-seal real raw child completion receipts, resolved
owner identity, callable catalog cardinality, duplicate/failure matrices, or
collector-miss behavior. Those are the next
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0` proof. Production
capture/commit, drain, finalization, FACTSESSION, and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-P0 worker audit and revised task order

Three read-only worker audits found that the original one-step P0 would have
been too weak. A list of Rust symbol names and a projection of the existing
route matrix can describe where an authority should live, but cannot itself
co-seal the inventory owned by that authority. The discarded local P0 draft
did exactly that and is not an implementation basis.

The existing authorities are deliberately different:

```text
Raw/Main
  successful collector admissions discovered by recursive raw expansion

Canonical A+ / Binding-SSA trivial
  one exact first-family plan + resolved owner + physical header

Binding-SSA acyclic / recursive
  canonical callable key/catalog + resolved function map + graph/SCC plan map
```

They must not be normalized into a caller-authored symbol list or a second
catalog. P0 is therefore split into the following code-facing slices.

### P0a — successful draft admission receipt

```text
WIRING-I0-ROUTEINV-P0a-RECEIPT-S0
  CollectedDraftAdmissionReceiptV1
  issued only after infallible collector commit
  exact key / symbol / arity / publication policy / replacement disposition
  non-Clone, no Builder/module/collector/header/fallback capability
  production receipt consumers = 0

WIRING-I0-ROUTEINV-P0a-RECEIPT-P0
  legacy whole-pair replacement
  canonical duplicate-key / duplicate-symbol rejection
  symbol and arity drift rejection before mutation
  failed prepare/seal/collect path emits no receipt
  collector prefix and index parity
```

The receipt is an event witness, not a second draft store. It may be returned
by the one collector commit terminal, but no production route consumes it in
P0a.

### P0b — raw expansion reservation/completion ledger

```text
WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0
  one invocation-owned RawExpansionReceiptLedgerV1
  reservation before each selected raw child/root completion
  completion consumes exactly one collector admission receipt
  legacy replacement semantics produce the final unique inventory
  event order remains separately observable for inner-before-outer proof
  raw condition disposition = RequiredCompatibility until CONDITIONFN-RET0
  production consumers = 0

WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0
  script/root Main, top-level function, static/instance/constructor child
  nested static/instance/constructor discovery
  optional callable Main.main/N selected/not-selected matrix
  inner-before-outer completion
  child failure leaves the pre-child prefix
  outer/root failure cannot seal a completed invocation
  duplicate legacy symbol replaces the whole receipt pair
  missing required condition/root receipt rejects
```

The raw inventory is sealed from successful expansion receipts, not from an
AST pre-scan, `VerifiedMainExpansionV1` alone, the callable declaration
catalog, or `collector.visit_symbols()`. The collector inventory is only the
actual side of the final equality check.

Two current compatibility behaviors are recorded as CUT0 stop conditions,
not silently normalized in P0b:

```text
duplicate Main source boxes:
  current legacy lowering and VerifiedMainExpansionV1 disagree

selected optional Main.main/N lowering failure:
  current legacy path can discard the error
```

P0b must expose either discrepancy as typed proof evidence. CUT0 may not
choose a new source behavior without a separate decision.

### P0c — canonical single-owner header seal

```text
WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0
  VerifiedResolvedOwnerHeaderV1
  co-sealed first-family brand + resolved owner + symbol + arity
  issued by CanonicalFirstFamilyPlanV1 before plan consumption
  zero-arity remains valid
  no caller constructor from owner/string/arity pieces
  production consumers = 0

WIRING-I0-ROUTEINV-P0c-SINGLEHDR-P0
  exact A+ and Binding-SSA-trivial family fixtures
  declaration/source reorder parity
  foreign owner/header pairing rejection
  canonical duplicate-key / duplicate-symbol / symbol / arity matrix
  raw Main/condition policy leakage = 0
```

The exact-i64 callable header is not reused here because its profile and
zero-arity admission differ from the general first-family root contract.

### P0d — canonical callable batch proof

```text
WIRING-I0-ROUTEINV-P0d-CALLABLE-P0
  acyclic:
    catalog keys == functions_by_key == graph nodes == plans_by_key
  recursive:
    catalog keys == functions_by_key == SCC inventory == plans_by_key
  every key owns one existing catalog header and one derived physical symbol
  declaration reorder parity
  source/catalog/plan/draft/publication failure matrix
  recursive and acyclic late-failure publication delta = 0
  new catalog / new key map / collector connection = 0
  production consumers = 0
```

This proof borrows the existing callable catalog and plan products. It must
not route canonical batches through `ModuleDraftCollectorV1`; that is a later
all-route CUT0 concern.

### P0e — route matrix and no-fallback closure

```text
WIRING-I0-ROUTEINV-P0e-MATRIX-G0
  four policy lanes / five root families / nine route rows exactly once
  every route consumes exactly one of P0b/P0c/P0d authority products
  duplicate and failure laws project from InvocationRouteMatrixV1
  explicit invocation header miss is terminal
  stale current_module cannot satisfy an empty collector lookup
  entered and changed observations remain separate
  caller-authored symbol inventory constructors = 0
  production inventory consumers = 0
```

The no-fallback claim is intentionally narrow:

```text
explicit invocation collector/header lookup miss
  -> no current_module retry
```

It does not claim that all legacy resolver families, the compatibility
`condition_fn`, or declaration-catalog recovery have already been retired.

### Fixed continuation

```text
P0a RECEIPT S0/P0
  -> P0b RAWLEDGER S0/P0
  -> P0c SINGLEHDR S0/P0
  -> P0d CALLABLE P0
  -> P0e MATRIX G0
  -> WIRING-I0-BORROW-S0
  -> WIRING-I0-HDR0
  -> WIRING-I0-CUT0
```

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0a-RECEIPT-S0`.
No further external design consultation is required for that disconnected
receipt vocabulary. Production capture/commit, drain, finalization,
FACTSESSION, and CUT0 remain forbidden.

### Stop conditions

Stop this series if any slice requires:

```text
an AST pre-scan as the complete raw inventory authority
collector.visit_symbols() as both expected and actual inventory
a caller-authored Vec<String> or symbol-list expectation
a second callable catalog or canonical key map
reusing exact-i64 callable headers for general zero-arity roots
current_module lookup after an explicit invocation-header miss
silently choosing duplicate-Main or swallowed-Main.main failure semantics
connecting canonical publication or production drain before P0e
storing Builder, module, collector, draft, ValueId, or retry authority in a receipt
```

FastMem remains an independent parked execution lane. Its selected contracted
raw-borrow V1 task order is unchanged and does not pre-empt this MirBuilder
production-cutover dependency chain.

## WIRING-I0-ROUTEINV-P0a-RECEIPT-S0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0a-RECEIPT-S0`
is closed. The one preflighted collector commit terminal now returns a
non-Clone `CollectedDraftAdmissionReceiptV1` after the infallible mutation.
The receipt seals the exact key, actual symbol and arity, publication policy,
and either `Inserted` or `ReplacedWholePair` disposition.

The product lives in a private child module of `module_draft_collector`; its
constructor is visible only to the parent collector module. No sibling
lowering route can synthesize a successful receipt. Receipt construction and
all receipt-owned allocations occur before the collector mutation, while the
product is returned only after the new draft and symbol index have both
committed.

All existing production call sites intentionally discard the new return
value. Therefore:

```text
collector commit owners = 1
receipt constructors = 1
production receipt producers = existing collector terminal only
production receipt consumers = 0
new draft/header/fact stores = 0
Builder/module/collector/ValueId/retry fields in receipt = 0
production behavior delta = 0
```

Focused fixtures cover exact insert identity, legacy whole-pair replacement,
canonical duplicate rejection before a second receipt, and symbol mismatch
with no collector effect. The reusable HeaderPort guard fixes the private
constructor, non-Clone/negative-field law, registration, line limits, and
production-consumer zero.

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0a-RECEIPT-P0`.
Raw receipt-ledger consumption, canonical header/catalog proof, production
capture/commit, drain, FACTSESSION, and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-P0a-RECEIPT-P0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0a-RECEIPT-P0`
is closed. A test-only exact snapshot observes both collector-owned draft rows
and the symbol-to-key index from the same owner. It is compiled only under
`cfg(test)` and introduces no production inventory or read API.

The matrix proves:

```text
canonical duplicate key:
  typed preflight error
  draft/index prefix unchanged
  receipt publication = 0

canonical duplicate symbol with a distinct key:
  typed preflight error
  draft/index prefix unchanged
  receipt publication = 0

symbol or arity drift:
  typed seal error
  draft/index prefix unchanged
  receipt publication = 0

sealed draft dropped before collect:
  draft/index prefix unchanged
  receipt publication = 0

legacy whole-pair replacement:
  exactly one successful receipt
  previous exact key/symbol named by the receipt
  unaffected prefix retained
  draft/index bijection retained
```

The existing injected index-drift fixture remains the preflight proof for a
malformed legacy pair. There is no fallible `collect` result or failure-retry
surface after a draft has passed prepare and seal; dropping or unwinding
before that terminal emits no receipt. The private constructor, non-Clone
law, production-consumer zero, and all source/check line limits remain guarded.

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0`.
Canonical header/catalog proof, production capture/commit, drain, FACTSESSION,
and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0b-RAWLEDGER-S0`
is closed as a disconnected invocation-local reservation/completion owner.
Raw function work is registered one discovered unit at a time; no AST pre-scan,
caller-authored inventory list, or collector symbol scan constructs the
expected inventory.

Each non-Clone reservation seals its exact ledger brand, discovery ordinal,
raw role, key, symbol, arity, and publication policy. Completion consumes both
that reservation and one successful collector receipt. Exact identity drift
poisons the ledger, after which reservation, completion, and seal cannot retry.
Foreign reservations are rejected without being adopted.

The ledger keeps two distinct observations:

```text
completion event order:
  preserves recursive inner/outer evidence including replaced events

final unique inventory:
  legacy whole-pair replacement updates key/symbol indexes together
  canonical duplicate insertion is rejected
```

Raw root policy is explicit and temporary:

```text
root main = required
condition_fn = RequiredCompatibility
retirement owner = FINALIZE0-CONDITIONFN-RET0
```

Seal rejects poisoned state, any open reservation, missing root main, or
missing compatibility condition. The open and sealed products store no
Builder, module, function draft, collector, source AST, ValueId, header loan,
fallback, retry, or publication capability. Production consumers remain zero.

Focused fixtures prove exact receipt consumption, completion-order retention,
required root/condition closure, foreign-token rejection, mismatch poisoning,
and no retry. A reusable route-inventory guard helper now owns P0b-and-later
checks so the pre-existing Candidate0 guard remains safely below 800 lines.

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0`.
Canonical header/catalog proof, production capture/commit, drain, FACTSESSION,
and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0`
is closed. The raw ledger now proves every selected raw expansion role through
one branded reservation and one successful collector receipt:

```text
root main
compatibility condition_fn
top-level function
static / instance / constructor
optional callable Main.main/N
nested static / instance / constructor
```

Reservations preserve discovery order, while successful events preserve
physical completion order. The P0 matrix reserves outer work before nested
work and completes the nested receipt first. Legacy duplicate symbols keep
both successful events but update the final key/symbol inventory as one whole
pair, so historical completion evidence and final unique inventory are not a
single truth.

Optional callable Main compatibility is no longer inferred from absence:

```text
NotSelected + no callable receipt:
  seal allowed

Selected + callable receipt:
  seal allowed

Selected + no callable receipt:
  MissingCallableMainCompatibility

NotSelected + callable receipt:
  UnexpectedCallableMainCompatibility
```

Failure consumes the complete open ledger and exact failed reservation into
`AbortedRawExpansionReceiptLedgerV1`. That product retains only the completed
prefix, failed ordinal/role/reason, and remaining open-reservation count. It
has no `seal`, reserve, completion, retry, or fallback API. A nested-child
failure therefore preserves the pre-child completion prefix, while a root
failure after successful children can never be mistaken for a completed raw
invocation.

The two known compatibility disagreements remain explicit typed CUT0 stops:

```text
DuplicateMainSourcePolicySelectionRequired
CallableMainFailurePropagationPolicySelectionRequired
```

The guard ties those stop rows to the current duplicate-Main rejection in
`VerifiedMainExpansionV1` and the legacy discarded callable-Main lowering
result. P0 does not choose either behavior. Production ledger consumers remain
zero, and no AST pre-scan, caller-authored symbol list, collector scan, second
catalog, module, Builder, draft, ValueId, header, retry, or fallback authority
was introduced.

Focused proof covers all roles, inner-before-outer completion, selected and
unselected callable Main, child/root abort typestate, whole-pair replacement,
required root/condition closure, and the typed compatibility stops. All source,
proof, and guard files remain below 800 lines.

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0`.
Canonical batch proof, production inventory consumers, capture/commit, drain,
FACTSESSION, and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0`
is closed as one disconnected first-family header authority. The selected
`CanonicalFirstFamilyPlanV1` is now the only issuer of a private, non-Clone
`VerifiedResolvedOwnerHeaderV1` before plan consumption. The product co-seals:

```text
exact first-family selection brand
resolved FunctionOwnerIdV1
physical free-function symbol
source parameter arity
```

The seal validates that the lowering input, source view, and resolved function
carry the same owner. A later attempt to pair the header with a foreign family
or foreign owner returns a typed `ForeignPlan` error. The constructor remains
`pub(super)` inside the private capability child module; callers cannot assemble
a header from owner/string/arity pieces.

The exact-i64 `VerifiedCallableHeaderV1` remains separate. Both A+ and
Binding-SSA-trivial fixtures accept zero parameters, proving that this general
first-family header has not inherited the callable catalog's scalar ABI or
nonzero-arity profile. The new seal and the existing callable catalog share one
neutral `CanonicalCallableSymbolV1::from_name_arity` projection instead of
adding another local formatter. The two live A+/Binding-SSA lowerers retain
their legacy projection until a later cutover, so repository-wide physical
naming SSOT is not yet claimed. Callable identity, source admission, scalar
ABI, and duplicate policy remain with their existing owners.

```text
VerifiedResolvedOwnerHeaderV1 production consumers = 0
plan seal production callers = 0
Builder/module/collector/draft/ValueId/fact fields = 0
retry/fallback authority = 0
production behavior delta = 0
```

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-P0`.
It must close exact A+ and Binding-SSA-trivial fixtures, nonzero arity,
declaration/source reorder parity, foreign pairing, canonical duplicate and
symbol/arity failures, and raw-policy leakage. Because the physical projection
uses `/` as its separator, P0 must also reject or otherwise prove injective any
source name containing `/`; this S0 does not silently import CAT0's spelling
admission law. Production capture/commit, drain, FACTSESSION, and CUT0 remain
forbidden.

## WIRING-I0-ROUTEINV-P0c-SINGLEHDR-P0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0c-SINGLEHDR-P0`
is closed. One test-only projection consumes the S0 header seal and the existing
canonical collector policy without creating a production consumer.

The family matrix proves:

```text
Binding-SSA-trivial:
  exact i64 parameter
  arity = 1
  symbol = binding_header/1

Current A+:
  untyped parameter
  arity = 1
  symbol = a_plus_header/1

zero arity:
  remains accepted by both S0 family fixtures
```

Reordering function-local declarations changes the resolved owner brand but
preserves family, physical symbol, and arity. A header paired with a different
owner/family plan fails with `ForeignPlan`. A source declaration name containing
`/` now fails inside the disconnected header seal with
`SourceNameContainsPhysicalSeparator`; the physical `name/arity` projection is
therefore injective for the admitted first-family source surface without
borrowing CAT0's exact-i64 profile.

The existing `CanonicalRejectDuplicate` collector law is projected directly:

```text
duplicate CanonicalResolvedOwner key:
  DuplicateKey

foreign owner with the same physical symbol:
  DuplicateSymbol

prepared header with a different physical draft symbol:
  SymbolMismatch

prepared header with a different physical draft arity:
  ArityMismatch

all four failures:
  collector draft/index prefix unchanged
  publication = 0
  retry = 0
```

The P0 product and fixtures contain no raw Main key, synthetic condition key,
raw expansion/condition disposition, Builder-held header cache, fallback, or
retry authority. `VerifiedResolvedOwnerHeaderV1` production consumers and plan
seal production callers remain zero. All source/proof/guard files remain below
800 lines.

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0d-CALLABLE-P0`.
It must borrow the existing acyclic/recursive callable catalogs and plan maps,
prove exact key/header/symbol/arity/cardinality correspondence and late-failure
publication zero, and add no catalog, key map, collector connection, or
production consumer. Production capture/commit, drain, FACTSESSION, and CUT0
remain forbidden.

## WIRING-I0-ROUTEINV-P0d-CALLABLE-P0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0d-CALLABLE-P0`
is closed without a new production product. The proof borrows the existing
callable catalog, resolved function map, acyclic graph or recursive SCC
partition, and typed plan map directly.

For the acyclic lane, every fixture now proves:

```text
catalog cardinality
  == functions_by_key cardinality
  == graph node cardinality
  == plans_by_key cardinality

functions_by_key keys
  == graph nodes
  == plans_by_key keys
```

For the recursive lane, the same proof adds both the SCC inventory nodes and
the sorted union of all SCC members. Every canonical key resolves to exactly
one existing catalog header, the header owns the same source key and arity,
and its physical symbol equals the neutral
`CanonicalCallableSymbolV1::from_name_arity` projection. No catalog key
iterator, second catalog, persistent key map, or caller-authored symbol list
was added.

Declaration reorder fixtures compare only invocation-independent key,
symbol/arity, graph/SCC, condensation, and plan observations. They do not
compare source sites, owner IDs, origins, or resolved callable identities,
which are legitimately issuance-local.

The failure matrix remains split by its existing owner:

```text
source/resolution failure:
  resolved callable module

catalog duplicate/key/symbol failure:
  callable catalog/index seals

graph/SCC/plan failure:
  acyclic and recursive plan verifiers

draft failure:
  private callable draft transaction

publication collision:
  MirModule::try_add_functions_atomic
```

The private transaction gained only a `cfg(test)` child proof. Both acyclic
and recursive fixtures successfully lower the first real draft, inject a
failure into the later draft, and observe zero candidate function publication.
A separate real atomic-publication collision preserves the preexisting module
row and publishes no fresh peer. This claim is intentionally limited to the
candidate module function map: Builder cursors, transient facts, and whole-
Builder rollback are not claimed.

```text
new production catalog/key map = 0
ModuleDraftCollectorV1 connection = 0
production inventory consumers = 0
production behavior delta = 0
source/proof/guard files >= 800 lines = 0
```

The sole next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0e-MATRIX-G0`.
It must close the four policy lanes, five root families, and nine route rows,
project each route to exactly one P0b/P0c/P0d authority, and prove explicit
invocation header miss has no stale `current_module` fallback. Production
capture/commit, drain, finalization, FACTSESSION, and CUT0 remain forbidden.

## WIRING-I0-ROUTEINV-P0e-MATRIX-G0 worker decision lock

Three read-only worker audits agree that no external design consultation is
required for P0e. The existing route matrix, route-owned policy projection,
P0b raw receipt ledger, P0c resolved-owner header, and P0d callable plans
already provide every durable authority required by this row. P0e adds only
test-only correspondence and negative-path proof; it creates no production
inventory owner.

### Exact correspondence

The sole route-row authority remains:

```text
InvocationRouteMatrixV1::rows()
```

The sole family-to-policy projection remains:

```text
RouteOwnedInvocationInventoryV2::derive(family)
```

The normalized counts are fixed as follows:

| Policy lane | Root family | Route rows | P0 authority |
| --- | --- | ---: | --- |
| `Raw` | `Raw` | 4 | P0b sealed raw expansion receipt ledger |
| `CanonicalSingle` | `CanonicalAPlus` | 2 | P0c resolved-owner header |
| `CanonicalSingle` | `BindingSsaTrivial` | 1 | P0c resolved-owner header |
| `BindingSsaAcyclic` | `BindingSsaAcyclic` | 1 | P0d acyclic callable plan/catalog proof |
| `BindingSsaRecursive` | `BindingSsaRecursive` | 1 | P0d recursive callable plan/catalog/SCC proof |

Therefore:

```text
policy lanes = 4
root families = 5
route rows = 9

P0b rows = 4
P0c rows = 3
P0d rows = 2
```

The exhaustive discriminator is the existing
`InvocationInventoryAuthorityV2` enum:

```text
RawExpansionReceipts
  -> P0b

CanonicalResolvedOwner
  -> P0c

CanonicalCallableCatalog
  -> P0d
```

No route-name string match, caller-authored symbol list, second catalog, or
persistent route map may implement this correspondence.

The raw matrix intentionally has two coarse child rows while the P0b ledger
has more detailed static, instance, constructor, callable-Main, and nested
roles. P0e proves exact correspondence for the nine route rows only. It does
not claim a new one-to-one mapping from every `RawExpansionDraftRoleV1` to a
distinct matrix row.

### Fixed implementation tasks

#### 1. Test-only matrix projection

Add one test-only sibling module:

```text
src/mir/builder/module_wiring_route_matrix_p0e.rs
```

It must:

```text
read InvocationRouteMatrixV1::rows()
derive all five families through RouteOwnedInvocationInventoryV2
observe four policy variants with row counts 4 / 3 / 1 / 1
project authority counts 4 / 3 / 2 through an exhaustive enum match
prove each matrix row appears exactly once in the flattened projection
borrow duplicate/failure/retry law from the existing matrix and candidate proof
```

It must not construct an `InvocationRouteMatrixRowV1`, repeat the nine route
names, or store Builder, module, function, collector, draft, header loan,
fallback, retry, or caller-authored function symbols.

#### 2. Entered/changed proof without false-green

The existing `WiringObservationV1::{Entered, Changed}` list is an expected
observation vocabulary, not a runtime observation. P0e must not present that
list alone as dynamic evidence.

The test-only sibling therefore owns a minimal observation fixture:

```rust
struct RouteObservationV1 {
    entered: bool,
    changed: bool,
}
```

It must include both:

```text
entered = true, changed = false
entered = true, changed = true
```

for one exact route and must never derive one bit from the other. This product
does not connect to production wiring and does not claim that all nine routes
have runtime observers.

#### 3. Explicit collector-miss negative fixture

Extend the existing raw invocation port tests with one exact fixture:

```text
raw_invocation_header_miss_does_not_retry_stale_current_module
```

The fixture must arrange:

```text
current_module:
  contains a stale matching function signature

invocation collector:
  contains no matching header
```

Expected result:

```text
MeCallParameterObservationV1::Missing
source = InvocationCollector
prepare_me_lowered_call_v1 = None
instruction delta = 0
ValueId cursor delta = 0
current_module retry = 0
```

The symbol must avoid `condition_fn`, declaration-catalog recovery, and other
compatibility heuristics. A collector miss may continue through the existing
ordinary semantic receiver/static policy, but it may not switch header source.
That distinction is part of the fixture name and assertion text.

#### 4. Reusable guard extension

Extend only:

```text
tools/checks/lib/headerport_route_inventory_guard.py
```

The guard must verify:

```text
P0e proof reads the route-matrix SSOT and does not construct route rows
four policy lanes / five families / nine rows
P0b/P0c/P0d authority counts = 4 / 3 / 2
entered and changed are distinct fields with both required fixtures
the exact collector-miss negative fixture exists
RawInvocationChildPortV1 me observation uses InvocationCollector
RawInvocationChildPortV1 me observation does not read current_module
caller-authored symbol inventory constructors = 0
production inventory consumers = 0
source/check files >= 800 lines = 0
```

Function-slice checks must be used where the same file also contains an
intentional legacy compatibility facade. A whole-file ban on
`current_module` would incorrectly reject that quarantined route.

### Verification order

```text
focused P0e matrix tests
focused raw invocation header-miss test
existing me-header observation tests
existing route-inventory tests
headerport route-inventory guard
module-draft/headerport guards
current-state pointer guard
cargo fmt --check
cargo check --release --bin hakorune
```

### P0e may claim

```text
all nine matrix rows project exactly once through one of P0b/P0c/P0d
the four policy lanes cover all five root families without a second route truth
duplicate/failure/retry laws remain projections of InvocationRouteMatrixV1
an explicit invocation collector miss never retries current_module headers
entered and changed remain separate observation dimensions
production inventory consumers remain zero
```

### P0e must not claim

```text
every detailed raw expansion role owns a distinct route row
all production routes have runtime entered/changed observers
all legacy header recovery has been retired
ordinary me-call semantic routing is fallback-free
condition_fn compatibility is retired
callable declaration-catalog recovery is retired
production capture, commit, drain, finalization, FACTSESSION, or CUT0 is active
```

### P0e stop conditions

Stop before implementation if closing the row requires:

```text
new route or policy vocabulary
route-name string matching as authority
a caller-authored function-symbol inventory
a second catalog, key map, collector, or header cache
production observation counters
current_module lookup after an explicit invocation header miss
changing ordinary me-call semantics after a branded miss
connecting any production inventory consumer
```

After the complete P0e gate is green, the sole continuation remains:

```text
WIRING-I0-BORROW-S0
  -> WIRING-I0-HDR0
  -> WIRING-I0-CUT0
```

No additional ChatGPT Pro consultation is scheduled before
`WIRING-I0-BORROW-S0`. A new consultation is required only if one of the P0e
stop conditions is reached or the BORROW-S0 inventory finds an unowned
cross-session mutable borrow.

## WIRING-I0-ROUTEINV-P0e-MATRIX-G0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-ROUTEINV-P0e-MATRIX-G0`
is closed with test-only evidence and zero production inventory consumers.
One new proof module reads the existing route matrix and derives every family
through the existing route-owned policy product. It does not repeat the nine
route names or construct a route row.

The exact closed correspondence is:

```text
four policy lanes:
  Raw = 4 rows
  CanonicalSingle = 3 rows
  BindingSsaAcyclic = 1 row
  BindingSsaRecursive = 1 row

five root families:
  Raw
  CanonicalAPlus
  BindingSsaTrivial
  BindingSsaAcyclic
  BindingSsaRecursive

existing authority products:
  P0b RawExpansionReceipts = 4 rows
  P0c CanonicalResolvedOwner = 3 rows
  P0d CanonicalCallableCatalog = 2 rows
```

Every matrix row appears exactly once in the flattened family projection.
Publication, failure stages, prefix preservation, and retry prohibition remain
direct projections of `InvocationRouteMatrixV1`; P0e adds no duplicate or
failure-law store.

The entered/changed proof is deliberately test-only. It records both
`entered=true, changed=false` and `entered=true, changed=true` for the same
route, so a no-op execution cannot be collapsed into a non-entry observation.
It does not claim that all production routes already own runtime observers.

The raw invocation integration fixture now installs a stale matching function
only in `current_module` while keeping the invocation collector empty. The
result is a branded `InvocationCollector` miss, `prepare_me_lowered_call_v1`
returns `None`, and both MIR instructions and the function ValueId cursor stay
unchanged. The invocation route never retries the stale module header. Legacy
module compatibility remains a separate explicit port and is not retired by
this claim.

The reusable route-inventory guard now fixes:

```text
matrix SSOT consumption
four / five / nine cardinality
P0b / P0c / P0d counts 4 / 3 / 2
independent entered / changed fields
exact empty-collector negative fixture
collector-only invocation me-header observer
caller-authored symbol inventory = 0
production P0e proof consumers = 0
source/check files >= 800 lines = 0
```

Focused matrix tests pass 3/3, the explicit header-miss fixture passes 1/1,
the broader raw-port suite passes 16/16, the existing me-header and route
inventory suites pass, and the reusable Candidate0/route-inventory guard is
green. Production capture/commit, drain, finalization, FACTSESSION, condition
compatibility retirement, and CUT0 remain forbidden.

The sole next code-facing row is:

```text
HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-S0
```

It must inventory every mutable borrow crossing capture, header observation,
collector admission, parent restore, and root completion before any production
cutover. `WIRING-I0-HDR0` and `WIRING-I0-CUT0` remain downstream.

## WIRING-I0-BORROW worker audit and task lock

Three independent read-only audits covered raw recursive lowering, all four
canonical families, and Main/root/drain/finalization. They found no unowned
cross-session mutable borrow. This is therefore an implementation task, not a
new external design consultation.

### Selected owner: Candidate S-prime

The production owner is an invocation-owned shell/collector candidate plus an
outer orchestration helper which lends the route Builder only for the current
phase. `MirBuilder` and `CompilationContext` store no invocation, collector,
header cache, or retry capability.

```text
outer compiler/module invocation owner
  owns:
    candidate Builder selected by the existing route session
    ModuleLoweringInvocationCandidateV1
      -> ModuleLoweringInvocationStateV1
           shell
           collector
           typed root state

  lends, one phase at a time:
    Builder exclusive loan
    collector header shared loan
    collector exclusive terminal loan
    shell exclusive publication loan

  consumes:
    route-owned drain witness
    post-drain candidate
    finalized external-commit candidate
```

The disconnected `ModuleLoweringInvocationV1<'builder>` remains useful proof
scaffolding, but its invocation-long Builder borrow is not selected as the
production owner. A long external borrow can be Rust-safe, but it makes the
canonical candidate Builder, Main/root terminal, and post-drain ownership
handoffs harder to express. The selected production seam is the existing
`ModuleLoweringInvocationCandidateV1::with_active_lowering` shape narrowed into
explicit short access methods; callers never receive the whole mutable
`ModuleLoweringInvocationStateV1`.

### Exact authority allocation

| Concern | Sole owner |
| --- | --- |
| active function-local mutable state | `MirBuilder::function_state` |
| child parent snapshot and exactly-once restore | `PendingFunctionSessionCloseV1` / `LegacyFunctionPendingSessionV1` |
| invocation shell and draft collector | `ModuleLoweringInvocationCandidateV1` |
| recursive raw reborrow | `RawInvocationChildPortV1` |
| completed-header observation | `LoweringHeaderPortV1` HRTB callback |
| prepared admission and physical collect | `ModuleLoweringPortV1` commit-only terminal |
| Main body temporal completion | `CompletedRootBodyV1` |
| Main plus compatibility root batch | future collector-wide prepared root batch |
| declaration metadata publication | owned declaration snapshot plus short shell port |
| raw/canonical drain admission | `RouteOwnedInvocationInventoryV2` projection |
| post-drain verification/finalization | owned drained-module candidate |
| external Builder/module replacement | existing `MirCompiler` route session commit |

Explicit non-authorities:

```text
MirBuilder field holding an invocation or collector
CompilationContext field holding an invocation or collector
current_module after an invocation-header miss
collector headers for canonical semantic target resolution
caller-authored symbol inventory
RefCell/Rc/Mutex/unsafe borrow escape
retry or fallback after a selected phase fails
```

### Two fixed borrow schedules

The passive schedule product records two domains instead of forcing recursive
child completion and module completion into one flat lifecycle.

```text
ChildTerminal schedule:
  BodyDescent
    BuilderMut + short recursive port reborrow
  HeaderObservation
    BuilderMut + CollectorHeaderShared; callback-scoped
  CapturePending
    BuilderMut -> owned pending; no surviving collector/header loan
  CommitPending
    CollectorMut; Builder accepted only through pending token
  ParentRestore
    pending token exactly once; collect already complete

InvocationCompletion schedule:
  RootBodyDrive
  RootBodySeal
  MainHeaderCompletion
  RootBatchPreflight
  RootBatchCommit
  ShellFactsSeal
  ShellFactsCommit
  DrainPreflight
  DrainCommit
  PostDrainFinalize
  ExternalCommit
```

Required overlap laws:

```text
CollectorHeaderShared + CollectorMut:
  forbidden

BuilderMut after PendingMainDraftV1:
  forbidden for raw root completion

any live Builder/header/collector/shell loan after DrainCommit:
  forbidden

external publication before PostDrainFinalize success:
  forbidden

restore before successful child collect:
  forbidden

retry/fallback after any selected failure:
  forbidden
```

The schedule derives route scope from `InvocationRouteMatrixV1` and
`RouteOwnedInvocationInventoryV2`. It does not repeat route-name strings,
callable keys, function symbols, or expected collector inventory.

### Raw child cutover seam

The one concrete raw hazard is already identified. Production
`RawInvocationChildPortV1::lower_static_box_method` and
`lower_instance_box_method` still call the old closure-owning
`complete_legacy_child`, and constructors share the same legacy terminal
family. Before CUT0 they must use:

```text
capture_static_box_method_pending_v1
or capture_instance_box_method_pending_v1
  -> raw child-port/header loans end
  -> ModuleLoweringPortV1::commit_legacy_pending
  -> collect
  -> parent restore exactly once
```

The semantic draft builders are not copied. The port-aware body/finalizer
entrypoints remain the only recursive implementation. The generic
`ModuleLoweringPortV1::capture_legacy_pending` fixture seam does not become the
production recursive-body authority.

### Canonical cutover seams

Canonical semantic catalogs and plans remain immutable external authorities;
collector headers do not replace them.

```text
A+:
  split restore-then-publish facade
  -> capture resolved pending
  -> common collector commit

Binding SSA trivial:
  split lower_resolved_trivial_function_draft session wrapper
  -> capture resolved pending
  -> common collector commit

Binding SSA acyclic / recursive:
  retain callable catalog and plan correspondence
  -> collect each unpublished draft through common collector
  -> retire direct VerifiedUnpublishedCallableDraftSetV1::publish_into

recursive capability/module marker:
  seal into shell before drain
```

The existing `CanonicalModuleLoweringSessionV1` remains the outer atomic
candidate-Builder owner. A failure drops its candidate and leaves the live
Builder unchanged; success replaces the live Builder exactly once only after
post-drain final verification.

### Main/root/drain seams

The raw Main path is ordered as follows:

```text
drive and seal CompletedRootBodyV1
-> one short final Main header loan
-> PendingMainDraftV1
-> whole Main + optional condition_fn batch preflight
-> infallible whole-batch collect
-> seal all declaration-fact lanes
-> one short shell commit
-> route-owned raw drain preflight
-> consume shell + collector into owned drained candidate
-> post-drain finalizer with Builder/collector reads = 0
-> final verified external commit
```

Canonical routes skip raw-only Main/root-batch phases. They join the common
chain at shell facts, route-owned drain, post-drain finalization, and external
commit. A raw `main` requirement is never imposed on a canonical route.

Known implementation gaps have named owners and do not require consultation:

```text
RootCompletionStateV1 typed transitions:
  invocation state owner

whole Main + condition_fn batch preflight:
  root-batch / collector owner

bare MirModule drain result retirement:
  drained-module candidate owner

route-tagged raw/canonical drain handoff:
  RouteOwnedInvocationInventoryV2 consumer

FinalizedModuleCandidateV1 and external handoff:
  post-drain finalizer / MirCompiler session owner
```

### Exact implementation order

```text
WIRING-I0-BORROW-S0             <- next code-facing row
  new module_lowering_borrow_schedule.rs
  passive ChildTerminal + InvocationCompletion schedules
  candidate-owned short access-method vocabulary
  production consumers = 0

WIRING-I0-BORROW-P0-RAW
  three-level static / instance / constructor recursion
  body -> short header -> pending seal -> commit -> restore ordering
  body, cleanup, admission, and panic failure matrix

WIRING-I0-BORROW-P0-CANONICAL
  A+ / trivial / acyclic / recursive phase-order proof
  immutable catalog authority remains external
  live Builder unchanged on every candidate failure

WIRING-I0-BORROW-P0-ROOT
  exact eleven-row invocation-completion schedule
  raw-only Main phases vs all-route common phases
  root batch / shell / drain / finalizer failure matrix

WIRING-I0-BORROW-G0
  extend existing Candidate0/HeaderPort guard
  no new one-off shell guard

WIRING-I0-HDR0-M0
  exact production current_module/functions reader census
  classify every reader as route header, canonical semantic catalog,
  shell/lifecycle access, diagnostic observation, or forbidden fallback

WIRING-I0-HDR0-P0
  replacement and parity proof for every reader
  explicit invocation miss remains terminal

WIRING-I0-HDR0-G0
  unclassified production function-map readers = 0
  second header cache = 0

WIRING-I0-CUT0-S0
  disconnected all-route adapters and outer orchestration helper
  production consumers = 0

WIRING-I0-CUT0-P0
  all five root families / nine matrix rows
  success, primary, cleanup, admission, drain, finalizer, and panic parity

WIRING-I0-CUT0-I0
  one atomic production cutover
  partial route cutover = 0

WIRING-I0-CUT0-G0
  old closure terminals / direct module insertion / direct callable publish = 0
  one collector / one drain / one finalizer / one external commit

then:
  FACTSESSION0-ACTIVEBIND0
  -> FACTSESSION0-I0/G0
```

### Required BORROW fixtures

```text
passive schedule:
  exact ChildTerminal phase order
  exact eleven InvocationCompletion rows
  no shared-header/exclusive-collector overlap
  no borrow survives DrainCommit

raw recursion:
  static -> instance -> constructor depth >= 3
  pending draft invisible before commit
  committed draft visible only after header callback ends
  nested body/cleanup/admission/panic failure restores exactly once
  collector prefix unchanged for the failing child

canonical:
  A+ / trivial / acyclic / recursive all reach common drain once
  callable catalog remains the semantic header owner
  failed finalizer leaves live Builder unchanged
  success replaces live Builder exactly once

root:
  raw owns Main phases; canonical skips them
  second root-batch admission failure leaves collector prefix unchanged
  declaration-fact preflight failure leaves shell unchanged
  drain/finalizer failure gives external commit count 0
  success gives external commit count 1

S0 behavior:
  Builder cursor delta = 0
  module/function inventory delta = 0
  production consumers = 0
```

### Guards

```text
borrow schedule definitions = 1
production borrow-schedule consumers through BORROW-G0 = 0

MirBuilder fields storing invocation/collector/header cache = 0
CompilationContext fields storing invocation/collector/header cache = 0
whole mutable invocation-state loans outside candidate implementation = 0

header loan crossing body/argument descent = 0
header loan crossing collector commit = 0
prepared admission crossing lowering = 0
restore before collect = 0

old raw closure-terminal consumers after CUT0 = 0
direct callable publish plus common collector coexistence after CUT0 = 0
direct pre-drain module function insertion after CUT0 = 0
bare MirModule between drain and finalizer after CUT0 = 0

retry / fallback = 0
new route-name or caller-symbol inventory = 0
new persistent ValueId/fact maps = 0
source/check files >= 800 lines = 0
```

`module_lowering_invocation.rs` is already 799 lines and is closed to further
growth. BORROW vocabulary and proof code must live in new focused files.

### Stop conditions

Stop before connection if any slice requires:

1. storing an invocation, collector, or header cache in `MirBuilder`,
   `CompilationContext`, or `MirCompiler`;
2. `RefCell`, `Rc`, `Mutex`, `unsafe`, TLS, or another borrow escape;
3. keeping a header loan across recursive descent, argument descent, commit,
   or drain;
4. keeping a prepared admission across lowering;
5. restoring a parent before its child draft is collected;
6. raw legacy or `current_module` fallback for an invocation child;
7. using collector headers for canonical semantic resolution;
8. keeping `VerifiedUnpublishedCallableDraftSetV1::publish_into` beside the
   common collector;
9. partially switching only raw or only canonical production roots;
10. mutating one root-batch member before all batch admissions are closed;
11. publishing shell facts piecemeal;
12. returning a bare `MirModule` between drain and finalization;
13. reading Builder/current_module/collector after drain;
14. committing externally before final verification;
15. mixing FACTSESSION, PHI repair, JoinIR conversion, FastMem, or source
    semantics into this series;
16. adding a source/check file at or above 800 lines; or
17. discovering a genuinely ownerless cross-session mutable borrow.

Only stop condition 17 reopens an external design consultation. The other
conditions are typed implementation failures inside the selected owner model.

### Claims after BORROW-G0

May claim:

```text
every future HEADERPORT production phase has one named mutable-borrow owner
recursive child capture, header observation, commit, and restore have a fixed
non-overlapping schedule
raw, canonical, and root/finalizer paths fit one candidate-owned short-loan
orchestration model
production behavior remains unchanged and production consumers remain zero
```

Must not claim:

```text
production capture/commit is active
current_module function-header reads are retired
all routes use the common collector
Main/root batch is production-active
drain/finalizer/external commit is production-active
FACTSESSION, PHI repair, JoinIR, or FastMem changed
```

### Decision lock

> **WIRING-I0-BORROW selects Candidate S-prime. No ownerless cross-session
> mutable borrow exists, so no new external design consultation is required.
> One invocation-owned shell/collector candidate lives outside MirBuilder and
> CompilationContext; an outer orchestration helper lends the selected route
> Builder, collector header, collector terminal, and shell only for the exact
> current phase. Recursive raw capture ends its child-port and header loans
> before the commit-only collector terminal runs, and the pending session
> restores its parent exactly once only after collection. Canonical semantic
> catalogs remain external immutable authorities while A+, trivial, acyclic,
> and recursive drafts converge on the same collector without a direct
> publish side store. Raw Main completion uses one final short header loan,
> one atomic Main/condition batch, one declaration-fact shell commit, one
> route-owned drain, one owned post-drain finalizer, and one verified external
> commit; canonical routes skip raw-only Main phases and join the common drain
> chain without inheriting a synthetic main requirement. BORROW-S0 remains a
> passive schedule with zero production consumers, followed by raw,
> canonical, and root proofs plus one reusable guard. HDR0 then closes every
> production header/read authority, and only one all-route CUT0 may activate
> capture, collect, drain, finalization, and commit. No Builder-held mirror,
> long-lived header loan, second collector/cache, caller-authored inventory,
> direct callable publish side store, partial route cutover, fallback, retry,
> or source/check file at or above 800 lines is admitted.**

FastMem remains explicitly parked in
`docs/development/current/main/investigations/fastmem-v1-execution-task-2026-07-22.md`.
Its selected contracted raw-view / FieldLoad vertical slice is already
taskified, but it does not pre-empt BORROW -> HDR0 -> CUT0 unless the active
lane is explicitly switched.

## WIRING-I0-BORROW-S0 closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-S0` is closed with one
disconnected, non-Clone `ModuleLoweringBorrowScheduleV1` and zero production
consumers. The schedule lives in the new focused
`module_lowering_borrow_schedule.rs`; the 799-line
`module_lowering_invocation.rs` remains unchanged.

The product seals two distinct domains:

```text
ChildTerminal = 5 rows
  body descent
  short final header observation
  pending-session seal
  collector commit
  parent restore

InvocationCompletion = 11 rows
  root body drive
  root body seal
  Main header completion
  root-batch preflight
  root-batch commit
  declaration-fact seal
  shell commit
  drain preflight
  drain commit
  post-drain finalization
  external commit
```

The child order deliberately reflects the physical port-aware draft builder:
the final header lookup happens inside the capture closure after body descent,
and the pending session is returned only after that header callback has ended.
No schedule row may overlap a shared completed-header loan with exclusive
collector mutation. No Builder loan appears after `PendingMainDraftV1`, and no
Builder, collector, or shell loan survives `DrainCommit`.

The first five invocation-completion rows are `RawOnly`. The remaining six
rows are `AllRoutes`; raw root-batch commit produces the neutral
`CollectedInvocationDrafts` handoff at which canonical routes join. The
schedule therefore does not impose raw `main` on canonical families and does
not duplicate route names, callable keys, or symbol inventories.

Construction validates exact cardinality, unique phases, contiguous owned
artifact handoffs, route scope, forbidden borrow overlap, the post-Main
Builder boundary, and the post-drain loan boundary. Root-batch, shell, drain,
and external commit mutations are explicitly infallible after their preceding
preflight rows.

The reusable HeaderPort guard now fixes:

```text
one passive schedule definition
5 child rows / 11 invocation rows
non-Clone product
no Builder/module/function/collector/ValueId storage
no RefCell/Mutex/unsafe escape
production schedule consumers = 0
module_lowering_invocation.rs < 800 lines
all changed source/check files < 800 lines
```

Focused schedule tests cover exact domain cardinality, contiguous artifact
handoffs, shared-header/exclusive-collector exclusion, the raw-only to
all-route boundary, no live loan after drain, and infallible commit rows.
Production capture/commit, route header replacement, root-batch wiring,
drain, finalization, external commit, FACTSESSION, and FastMem remain
unchanged.

The sole next code-facing row is:

```text
HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-P0-RAW
```

It must prove the existing port-aware raw path through at least three nested
static/instance/constructor frames, exact header-before-pending ordering,
commit-before-restore, prefix preservation, and primary/cleanup/admission/panic
failure laws before the canonical or root proof slices begin.

## WIRING-I0-BORROW-P0-RAW closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-P0-RAW` is closed. The
disconnected `RawInvocationChildPortV1` method terminal now uses only:

```text
prepare exact legacy admission
-> capture_static/instance_box_method_pending_v1
     body descent through a shorter reborrow
     short final collector-header callback
     pending-session seal
-> commit_legacy_pending
     collector mutation
     parent restore
```

The raw port's closure-owning `complete_legacy_child` facade has no remaining
definition. Static and instance method dispatch no longer return to
`build_*_method_draft_v1`; both consume the port-aware pending draft and the
same commit-only collector terminal. This is a disconnected path
normalization, not a production-root cutover: production constructions of
`RawInvocationChildPortV1` remain zero outside its implementation and
test-only proofs.

One exact AST fixture now exercises:

```text
Outer.run/0                  static child
  -> Middle.run/0            nested instance child
       -> Leaf.birth/0       nested constructor
       -> Leaf.run/0         sibling instance method
```

All four drafts reach the sole collector, and the original root parent is
restored with recursion depth zero. The existing one- and two-level static,
instance, constructor, header-before-commit, nested-Main rejection, and body
failure fixtures remain green.

One reusable invocation is also driven through the complete failure matrix
after a pre-existing `prefix/0` draft has been collected:

```text
primary body failure:
  prefix unchanged; parent restored

successful body + cleanup failure:
  prefix unchanged; parent restored

admission mismatch after pending capture:
  prefix unchanged; parent restored

panic during capture:
  prefix unchanged; unwind resumed after restore

fresh success after all failures:
  after/0 collected once; invocation remains reusable
```

The earlier legacy-terminal proof continues to retain the distinct
primary-plus-cleanup `DuringCleanup` error and unwind baseline. No failure
takes a retry or a module fallback.

The reusable HeaderPort guard now additionally fixes:

```text
raw method terminals using capture pending = 2
raw method terminals using commit pending = 2
raw closure-owning facade definitions = 0
three-level static/instance/constructor proof = 1
primary/cleanup/admission/panic prefix matrix = 1
production RawInvocationChildPort constructors = 0
all changed source/check files < 800 lines
```

Production root capture/commit, canonical draft collection, raw Main/root
batch, shell/drain/finalizer wiring, external commit, FACTSESSION, PHI repair,
JoinIR, and FastMem remain unchanged.

The sole next code-facing row is:

```text
HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-P0-CANONICAL
```

It must prove A+, trivial BindingSSA, acyclic BindingSSA, and recursive
BindingSSA phase ordering while keeping immutable callable catalogs external
to the collector and preserving the live Builder on every candidate failure.

## WIRING-I0-BORROW-P0-CANONICAL closeout

`HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-P0-CANONICAL` is closed
without a production behavior change. The four canonical families retain
their existing preflight and semantic authorities:

```text
CurrentCanonicalAPlus
  preflight = CanonicalLoweringPreflightV1
  header    = VerifiedResolvedOwnerHeaderV1

TrivialBindingSsa
  preflight = CanonicalLoweringPreflightV1
  header    = VerifiedResolvedOwnerHeaderV1

BindingSsaAcyclic
  preflight = VerifiedAcyclicCallableModulePlanV1
  headers   = VerifiedResolvedCallableModuleV1 source catalog

BindingSsaRecursive
  preflight = VerifiedRecursiveCallableModulePlanV1 + SCC partition
  headers   = VerifiedResolvedCallableModuleV1 source catalog
```

All four converge on the same physical candidate lifetime:

```text
verified preflight / immutable catalog
-> CanonicalModuleLoweringSessionV1::open
-> route-owned candidate lowering
-> finish_built_canonical_module
-> post-transform verification
-> consuming session.commit
-> live Builder replacement exactly once
```

The source-order guard checks `open < finish < commit` independently for the
first-family, acyclic, and recursive ingress bodies. A+ and trivial share the
first-family post-match finish/commit terminal but keep distinct selected
plans and finish schedules. Acyclic and recursive retain distinct verified
graph/SCC plans and never borrow collector headers for semantic call
resolution.

Per-function callable inputs continue to derive `callable_index` and
`callable_header` directly from the immutable resolved callable source
catalog. The trivial direct-call emitter consumes that header and has no
collector/header-port fallback.

A focused test-only module now proves the shared candidate owner itself:

```text
candidate drop after independent state mutation
  live repl_mode unchanged
  live recursion depth unchanged

consuming candidate commit
  candidate repl_mode adopted
  candidate recursion depth adopted
  second commit structurally unavailable
```

The existing canonical verification failure, acyclic rejection-then-success,
and recursive rejection-then-success fixtures remain the route-level reuse
baseline. Late callable-draft failures still publish zero candidate functions;
they do not authorize direct external publication.

The canonical guard was placed in the focused reusable
`headerport_borrow_canonical_guard.py` instead of growing the existing route
guard to the 800-line boundary. All changed source/check files remain below
800 lines.

The following are intentionally not retired by this P0 row:

```text
VerifiedUnpublishedCallableDraftSetV1 direct publish side store
current canonical production roots
common collector connection
route-owned drain
post-drain finalizer
external commit
```

Those remain CUT0 work. The sole next code-facing row is:

```text
HEADERPORT0-REENTRANT-TERM0-I0-WIRING-I0-BORROW-P0-ROOT
```

It must prove the exact eleven-row invocation-completion schedule, raw-only
Main phases versus the all-route common tail, and root-batch/shell/drain/
finalizer failure atomicity before BORROW-G0.

## WIRING-I0-BORROW-P0-ROOT-P0a closeout

The worker re-audit rejected an early broad ROOT closeout. The schedule was
correct, but the actual collector-wide root-batch commit, declaration-fact
shell commit, and final external commit observation do not yet exist. The
closed P0a claim is therefore deliberately narrower: one focused test-only
projection consumes the existing passive schedule and fixes the exact
invocation order:

```text
raw-only prefix = 5
  root body drive
  root body seal
  Main header completion
  root-batch preflight
  root-batch commit

all-route tail = 6
  declaration-fact seal
  shell commit
  drain preflight
  drain commit
  post-drain finalization
  external commit
```

The passive mutation boundary is explicit. Root-batch, shell, drain, and
external commit rows are marked infallible only after their matching
preflight, seal, or final-verification row. The passive failure matrix now has
eight exact, ordered owners; the previously implicit root-batch preflight and
declaration-fact seal stages are separate rows. This P0a records the required
failure disposition but does not claim that the missing physical commit APIs
already enforce it.

The proof reuses the existing owners rather than introducing an orchestration
facade:

```text
PreparedRootDraftBatchV1
SealedModuleDeclarationFactsV1
PreparedInvocationDrainV1
DrainedModuleFinalizationInputV1
```

Production consumers of the root-batch and post-drain orchestration entries
remain zero. No Builder, collector, header cache, second module store, live
commit, FACTSESSION, PHI repair, JoinIR, FastMem, or CUT0 authority is added.
All changed source/check files remain below 800 lines.

The worker audits found no ownerless cross-session mutable borrow and no new
semantic selection. They selected the following complete ROOT task queue:

```text
BORROW-P0-ROOT-P0b  <- next
  one disconnected collector-wide Main + condition_fn admission owner
  preflight every key/symbol/arity/replacement before mutation
  second admission failure preserves the exact collector prefix
  successful commit is infallible and publishes the whole batch once

BORROW-P0-ROOT-P0c
  one disconnected declaration-fact shell commit owner
  all declaration lanes move together
  failed preparation leaves the shell unchanged
  no Builder/CompilationContext read after sealing

BORROW-P0-ROOT-P0d
  co-seal all nine route rows with the eleven-phase schedule
  raw four rows own the five-phase Main prefix
  canonical five rows enter only the six-phase common tail
  drain/finalizer failure gives external commit count 0
  success gives external commit count 1

BORROW-P0-ROOT-G0
  root batch/shell/drain/finalizer owners = one each
  direct/fallback/retry owners = zero

WIRING-I0-BORROW-G0
  one reusable BORROW guard over raw, canonical, and root proofs
```

Then the fixed order is:

```text
HDR0-M0 -> HDR0-P0 -> HDR0-G0
-> CUT0-COMPAT-POLICY-CONSULT0
-> CUT0-S0 -> CUT0-P0 -> CUT0-I0 -> CUT0-G0
-> FACTSESSION0-ACTIVEBIND0-S0/P0
-> FACTSESSION0-I0/G0
-> REMATFACT0 / producer closures
-> MODULETX0-P0
```

`CUT0-COMPAT-POLICY-CONSULT0` is the one intentional future design stop. It
must decide duplicate Main source behavior and optional callable-Main failure
propagation before any all-route production cutover. BORROW, ROOT, HDR0, and
the already selected FACTSESSION Candidate A-prime need no additional design
consultation. FastMem remains parked and may resume only through an explicit
lane switch beginning at `FASTMEM-BASELINE0`.

## WIRING-I0-BORROW-P0-ROOT-P0b closeout

`BORROW-P0-ROOT-P0b` is closed as a disconnected collector transaction. The
physical owner is
`src/mir/builder/module_draft_collector/root_batch.rs`; it consumes exactly
one `ModuleDraftCollectorV1` and one already validated
`PreparedRootDraftBatchV1`.

The transaction fixes the following order:

```text
consume validated Main/condition batch
-> validate every draft symbol and arity
-> prepare every key/symbol/replacement disposition
-> if any preparation fails, return the unchanged collector
-> otherwise issue one non-Clone prepared batch
-> commit the complete batch infallibly
-> return exact per-draft collection receipts
```

The batch path reuses the same `plan_admission_v1` owner as single-draft
collection. It does not introduce a second duplicate, index-drift, or legacy
replacement policy. All fallible checks finish before the first collector
mutation.

The focused proof fixes three required cases:

```text
second_root_admission_failure_preserves_exact_collector_prefix
prepared_root_batch_commits_main_and_condition_once_after_full_preflight
legacy_main_replacement_is_prepared_with_the_whole_root_batch
```

The implementation remains construction-only. Production root-batch
consumers, external module publication, Builder mutation, fallback, retry,
FACTSESSION, PHI repair, JoinIR, FastMem, and CUT0 deltas are all zero. The
new owner is 250 lines, its focused proof is 144 lines, and the parent
collector remains 692 lines; all changed source/check files remain below 800
lines.

The remaining ROOT order is fixed without further design selection:

```text
BORROW-P0-ROOT-P0c  <- next
  one disconnected declaration-fact shell commit owner
  all declaration lanes move together
  failed preparation leaves the shell unchanged
  no Builder/CompilationContext read after sealing

BORROW-P0-ROOT-P0d
  co-seal all nine route rows with the eleven-phase schedule
  prove external commit count 0 on drain/finalizer failure
  prove external commit count 1 on success

BORROW-P0-ROOT-G0
  root batch/shell/drain/finalizer owners = one each
  direct/fallback/retry owners = zero

WIRING-I0-BORROW-G0
  one reusable guard over raw, canonical, and root borrow proofs
```

No external design consultation is required for these rows. The only
intentional later stop remains `CUT0-COMPAT-POLICY-CONSULT0`, after `HDR0-G0`
and before `CUT0-S0`, for duplicate Main source behavior and optional callable
`Main.main/N` failure propagation.

## WIRING-I0-BORROW-P0-ROOT-P0c closeout

`BORROW-P0-ROOT-P0c` is closed as one disconnected declaration-fact shell
transaction. The physical owner is
`src/mir/builder/module_lowering_shell/declaration_fact_commit.rs`.

Its exact law is:

```text
function-empty ModuleLoweringShellV1
+ sealed user-box / typed-field / record / enum facts
-> inspect all four destination lanes
-> any destination lane nonempty:
     typed rejection
     exact shell returned unchanged
     exact sealed facts returned unchanged
-> every destination lane empty:
     convert all metadata representations during preparation
     issue one non-Clone prepared commit owner
-> infallibly publish all four lanes together
-> return the same function-empty shell
```

The prepared product owns the complete metadata payload. Its commit has no
Builder, CompilationContext, collector, header, source AST, semantic
inference, derived-plan refresh, fallback, retry, or external publication
capability. After `SealedModuleDeclarationFactsV1` exists, neither preparation
nor commit can return to mutable lowering state for a missing lane.

Focused fixtures prove that all four declaration lanes move exactly once and
that a nonempty target rejects before mutation while preserving both the
existing shell metadata and the sealed incoming facts. The implementation and
proof remain below 800 lines, and production consumers remain zero.

The next row is `BORROW-P0-ROOT-P0d`. It must co-seal all nine route rows with
the eleven-phase schedule, prove that the four raw rows own the five-phase
Main prefix while the five canonical rows enter only the common six-phase
tail, and observe external commit count zero on drain/finalizer failure and
one on success. ROOT-G0 and BORROW-G0 remain forbidden until that matrix is
green.

## WIRING-I0-BORROW-P0-ROOT-P0d closeout

`BORROW-P0-ROOT-P0d` is closed as a passive co-seal over existing route,
schedule, and finalization-failure authorities. It introduces no new runtime
or lifecycle owner.

The exact projection is:

```text
route rows = 9
  raw rows = 4
    phases per row = 5 RawOnly + 6 AllRoutes
  canonical rows = 5
    phases per row = 6 AllRoutes

exact route/phase pairs = (4 * 11) + (5 * 6) = 74
```

For every route, three terminal observations are kept separate:

```text
drain-preflight failure:
  external commit count = 0
  retry = 0

post-drain finalizer failure:
  external commit count = 0
  retry = 0

success:
  ExternalCommit phase occurrences = 1
  external commit count = 1
  retry = 0
```

The observation matrix therefore contains exactly `9 * 3 = 27` rows. Failure
counts are projected from `ModuleFinalizationFailureMatrixV1`; success count
is projected from the sole `ExternalCommit` schedule phase. A zero observed
commit never claims that production wiring already exists.

Production orchestration consumers, Builder mutation, module publication,
fallback, retry, FACTSESSION, PHI repair, JoinIR, FastMem, and CUT0 deltas
remain zero. `BORROW-P0-ROOT-G0` is next, followed by the reusable whole
`WIRING-I0-BORROW-G0` guard.

## WIRING-I0-BORROW-P0-ROOT-G0 closeout

`BORROW-P0-ROOT-G0` is closed. The reusable guard proves exactly one physical
definition for each root completion owner:

```text
root-batch preparation owner = 1
declaration-fact shell preparation owner = 1
invocation drain owner = 1
post-drain finalization input owner = 1
```

It also requires every P0a/P0b/P0c/P0d proof product, all changed
source/check files below 800 lines, and zero production root-orchestration
consumers. The G0 claim is deliberately structural: it does not claim that
production capture, commit, or external publication has been enabled.

## WIRING-I0-BORROW-G0 closeout

`WIRING-I0-BORROW-G0` is closed through the single reusable
`headerport_candidate0_guard.py` entry. That entry composes the raw,
canonical, and root guards and now fixes:

```text
ownerless cross-session mutable borrow = 0
long-lived collector header loan across mutation = 0
post-restore bare publication = 0
production root orchestration consumers = 0
fallback/retry owners = 0
```

No new semantic decision was required. `WIRING-I0-HDR0-M0` is next, followed
by HDR0-P0/G0. The sole bounded design stop remains
`CUT0-COMPAT-POLICY-CONSULT0` after HDR0-G0 and before CUT0-S0.

## Reserved FastMem V1 handoff

FastMem V1 is now scheduled rather than indefinitely parked. It does not
interrupt the active HEADERPORT/FACTSESSION/finalization sequence. The exact
handoff order is:

```text
WIRING-I0-HDR0-M0/P0/G0
-> CUT0-COMPAT-POLICY-CONSULT0
-> WIRING-I0-CUT0-S0/P0/I0/G0
-> FACTSESSION0-ACTIVEBIND0-S0/P0
-> FACTSESSION0-I0/G0
-> REMATFACT0 / individual producer receipt closures
-> FINALIZE0-PHI-SPLIT0-MODULETX0-P0
-> FINALIZE0-PHI-SPLIT0-I0
-> FINALIZE0-PHI-SPLIT0-MODULE-G0
-> MODULE-FINALIZE-VERIFY-CUT0
-> FastMem lane handoff
-> FASTMEM-BASELINE0
```

`MODULETX0-P0` alone is not sufficient for the handoff. The production module
transaction, its single consumer guard, and the final module verification
cut must be green first. This prevents V1 capability, target-layout, and
access-plan owners from being built on the mutable Builder/finalization
boundaries that the preceding rows retire.

The reserved FastMem lane then follows its own board without interleaving
another MirBuilder architecture series:

```text
FASTMEM-BASELINE0
-> FASTMEM-SSOT-DRIFT0
-> FASTMEM-VOCAB-FREEZE0
-> FASTMEM-BACKEND-ID0
-> FASTMEM-BACKEND-PREFLIGHT0
-> FASTMEM-TARGET0
-> FASTMEM-CONTRACT0
-> FASTMEM-FOUNDATION0
-> FASTMEM-V1-PARSE0
-> FASTMEM-FIELDLOAD-VERTICAL0
```

The first vertical ends only after one explicit branded TableIndex plus
scalar FieldLoad executes through daily non-replay ny-llvmc and passes exact
shape, executable parity, and C comparison gates. Rust MirInterpreter MemOp
execution is not pulled forward; it must reject in backend preflight until
the separate interpreter architecture is ready. FieldStore, owner,
free-list, remote atomics, general contracts, trusted assumptions, and V0
retirement remain individually gated downstream rows.

## WIRING-I0-HDR0-M0 closeout

`WIRING-I0-HDR0-M0` is closed as a source-only inventory. The reusable
`headerport_header_reader_census.py` is invoked by the existing
`headerport_candidate0_guard.py`; it does not connect a production reader or
change route behavior.

The census covers 29 production `current_module` source occurrences in 20
semantic rows. Every row has a stable source anchor and one owner family:

```text
route_header           = 7
canonical_catalog      = 3
shell_lifecycle        = 6
forbidden_fallback     = 3
diagnostic_observation = 1
```

The route-header rows are the raw child/finalizer lookup, call annotation,
known rewrite, method-index projection/freshness, and finalizer module loan.
The three direct legacy probes in constructor/birth, tail resolution, and
materialization are explicitly classified as `forbidden_fallback`; they are
inventory only and are not accepted as collector fallback authority. The
located legacy observation is compiled but has no non-test caller, so it is
retained as a diagnostic row rather than silently counted as an active route.
Canonical callable publication remains owned by the sealed source plan/catalog;
the Builder module map is only the current publication destination.

The guard rejects a new `current_module` occurrence or source file until
HDR0-P0 assigns it a replacement/parity owner. It also asserts
`diagnostic_observation=1` and `forbidden_fallback=3`; no explicit collector
fallback or retry was found. The next code-facing row is
`WIRING-I0-HDR0-P0`, followed by HDR0-G0. Production capture/commit, CUT0,
FACTSESSION, and FastMem remain forbidden.

## WIRING-I0-HDR0-P0 annotation parity slice

The call-result annotation reader now has a focused passive parity witness.
The legacy path reads one `MirModule` signature while the invocation path
reads the same signature through `LoweringHeaderPortV1`; both publish the
same return type and `NewBox` origin, and the invocation Builder has no
ambient module. This is test-only evidence: it adds no production caller and
does not alter annotation heuristics or fallback policy.

```text
source authority: completed invocation collector header
non-authority: Builder.current_module, production capture/commit, CUT0
fail-fast boundary: explicit header loan ends before later collector mutation
landed proof: headerport_annotation_matches_legacy_module_signature_without_ambient_module
next evidence: reuse the existing Known-rewrite lookup-only proof; no new
  materializer wiring is selected while its `legacy_presence` policy remains
  unresolved. The open owner questions cover method-index freshness, static
  tail routing, materializer policy, and lifecycle activation
```

The remaining unresolved policy questions are collected in
`mirbuilder-headerport-i0-hdr0-p0-open-questions-2026-07-22.md`. Constructor/
birth presence is now mechanically owned by the explicit HeaderPort; the
open choices are method-index freshness, static tail routing, materializer
`legacy_presence`, and the lifecycle activation boundary.

## WIRING-I0-HDR0-P0 constructor/birth presence slice

The worker audit selects the constructor/birth presence read as a separate
mechanical owner. `build_new_expression_with_port_v1` now requires the
short-lived `RawFunctionHeaderLookupPortV1`; an explicit invocation header
uses `LoweringHeaderPortV1::contains_symbol`, while the legacy `None` adapter
keeps the existing `current_module` compatibility path. The constructor
fallback policy (`user_defined_boxes` and the builtin birth toggle) is
unchanged and is not inferred from the header lookup.

`headerport_birth_presence_matches_legacy_newbox_branch` compares the actual
`NewBox -> Global(<Class>.birth/N)` branch and emitted instructions against the
legacy module path. The fixture is disconnected: no production invocation
caller, capture/commit, catalog authority, or CUT0 wiring is added.

The worker decision does not extend to method-index freshness, static tail
resolver routing, materializer `legacy_presence`, or lifecycle cutover. Those
still require a separate cache/policy/route owner decision.
