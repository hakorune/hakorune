# HEADERPORT0-REENTRANT-TERM0-I0: production cutover consultation

Status: **STATE0-S0/P0/I0/G0, CONSULT0, ACCESS0-S0, and
ACCESS0-MEHEADER-S0/P0/I0/G0, ACCESS0-REWRITE-KNOWN-P0, ACCESS0-P0, and
CANDIDATE0-S0/P0, MAINROLE0-D0/S0/P0, BODYDRAIN0-S0/P0,
MAINPENDING0-S0/P0, ROOTBATCH0-S0/P0, SHELLFACT0-S0/P0, and
DRAIN0-S0/P0, MODULEFINAL0-SPLIT0/P0, MODULEFINAL0-CANDIDATE0-P0, and
WIRING-S0/P0 are closed; M-root-prime and Candidate A-prime are selected.
The next passive row is `WIRING-I0-ROUTEINV-S0`; production capture/commit and
CUT0 remain forbidden until the passive route inventory, borrow seam, and
header/read census are green**

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
