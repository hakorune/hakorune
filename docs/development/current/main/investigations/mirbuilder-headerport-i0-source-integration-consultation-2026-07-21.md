# HEADERPORT0-REENTRANT-TERM0-I0: source integration consultation

Status: **Candidate S-prime selected; I0-SHELL-S0/P0, I0-SHELL-I0-S0/P0 closed; production I0 remains disconnected**
Date: 2026-07-21
Parent: `mirbuilder-headerport-reentrant-terminal-task-2026-07-21.md`
Decision: one invocation-owned shell plus one collector; shell vocabulary is
the next disconnected code-facing slice

## Evidence

`HEADERPORT0-REENTRANT-TERM0-S0/P0` is closed. The disconnected terminal
proof is green: capture-before-commit, short header loans, nested static and
instance children, constructors, TaskScope/FastMem body descent, Main
pre-effect rejection, primary failure, admission failure, parent restore, and
existing panic parity.

The production I0 boundary is not yet a mechanical call-site replacement:

```text
MirBuilder::build_module
  -> prepare_module
  -> lower_root
  -> finalize_module
```

`prepare_module` installs an empty `current_module` and a live `main` function.
`lower_root` directly lowers root instance/static functions and directly calls
`build_static_main_box` for the App entry. Those paths publish child functions
through `current_module` while lowering continues.

At the same time, `ModuleLoweringInvocationV1` owns a separate
`ModuleDraftCollectorV1`. The collector currently owns unpublished drafts and
header views, but it has no production module-shell aggregation operation. A
partial I0 would therefore create one of these forbidden states:

```text
collector owns child header A
current_module owns child header A'

collector owns a child draft
current_module publishes the same child before parent restore

collector header view is used for one child
current_module.functions remains authority for another child
```

The I0 task explicitly forbids all three. The eight audited lowering-time
header readers also cannot be switched one by one if their source authority
alternates between the two stores.

## Why a partial cutover is rejected

The following changes are not admissible:

```text
replace only RawInvocationChildPortV1::complete_legacy_child
wrap only nested BoxDeclaration lowering
leave root main in current_module and children in collector
add collector -> current_module fallback lookup
clone collector headers into a temporary module cache
```

Each one either leaves a direct post-restore publication path, introduces a
second header truth, or makes source/evaluation order depend on which route
happens to be active.

## Candidates for the missing owner boundary

### Candidate A — invocation owns the complete module shell

```text
ModuleLoweringInvocationV1
  owns module shell + collector
  main and every child become unpublished drafts
  collector drains into one shell only at terminal commit
```

Requirements:

```text
main draft capture/commit is port-aware
lower_root receives one explicit raw port
all root child terminals use capture -> prepare -> seal -> collect
collector provides the only lowering-time header view
final module assembly consumes the collector exactly once
```

This preserves one authority, but requires an explicit decision about which
module metadata remains live during lowering and how the existing `main`
FunctionState is represented before collection.

### Candidate B — current_module becomes the collector backing store

```text
ModuleDraftCollectorV1
  borrows/owns the same module function map
```

Rejected for now: it exposes mutable `MirModule` through the collector, makes
collect-before-restore impossible to prove at the type boundary, and turns the
temporary header port into a view over a mutable publication object.

### Candidate C — collector prefix plus current_module read view

```text
header lookup = collector prefix, then current_module fallback
```

Rejected: this is a dual authority and violates the selected HeaderPort law.
It also makes duplicate and replacement semantics dependent on lookup order.

## Selected Candidate A-prime

Candidate A is narrowed to a shell/collector split that preserves one function
authority without making the collector a mutable `MirModule` view.

```text
ModuleLoweringInvocationV1
  owns ModuleLoweringShellV1
  owns ModuleDraftCollectorV1
  lends LoweringHeaderPortV1 from the collector only

ModuleLoweringShellV1
  owns module name, globals, and module metadata accumulation
  owns no published function map during lowering

ModuleDraftCollectorV1
  owns every completed function draft and header identity
  owns the only completed-function header view during lowering
```

The shell may use the existing `MirModule` metadata/global vocabulary, but its
`functions` map is structurally empty until the final drain. A shell function
map is never used as a header fallback. The collector is drained exactly once
after all drafts are sealed and all module-level preflight has passed.

The root `main` is represented as the live, unpublished
`FunctionLoweringState` until its terminal capture. It is not inserted into a
shell function map while children are still lowering. The terminal sequence is:

```text
prepare shell metadata
-> lower root with one explicit raw port
-> capture main pending session
-> prepare/seal/collect Main
-> synthesize and collect condition_fn when absent
-> preflight collector-to-shell drain
-> drain collector exactly once into shell
-> final module verification and external return
```

Lowering-time metadata reads/writes are split from function-header reads:

```text
module name/globals/static plans/declared metadata
  -> ModuleLoweringShellPortV1

completed function signature/presence/inventory
  -> LoweringHeaderPortV1 (collector only)
```

No consumer may read `current_module.functions` during an active invocation.
The existing eight-reader census must therefore be rechecked after the shell
port is introduced; a reader that needs a completed body or metadata is a
typed stop, not a shell fallback.

### Why this is the smallest coherent owner

```text
one invocation = one shell + one collector
one header authority = collector
one function publication point = collector drain
one main admission = Main key / symbol main / arity 0
one synthetic admission = SyntheticConditionFn / symbol condition_fn / arity 1
```

This keeps legacy replacement and canonical duplicate policy inside prepared
collector admission. It does not redesign TypePipeline, PHI repair, JoinIR,
FACTSESSION, or finalization; those remain after the shell/collector bridge.

## I0-SHELL-S0 closeout

The disconnected shell vocabulary is now present in
`src/mir/builder/module_lowering_shell.rs`:

```text
ModuleLoweringShellV1
  accepts only a function-empty MirModule

PreparedModuleLoweringShellDrainV1
  is non-Clone and single-use
  preflights duplicate function symbols before shell mutation
```

Its six focused tests prove rejection of an already-published function map,
one successful batch drain, duplicate-symbol rejection, inventory mismatch
rejection, duplicate-inventory rejection, and the narrow metadata port. The module is
registered behind the reusable HeaderPort guard, and no production lowering
consumer calls `prepare_drain` yet. The shell is therefore a vocabulary and
transaction boundary, not a second live module store.

The next row was the P0 source census and shell preflight; that row is now
closed below. Production lowering remains disconnected.

## I0-SHELL-P0 source census

The production `current_module` references fall into four distinct families.
They must not be solved by a single `module.functions` fallback.

| Source family | Current use | Future authority | I0 consequence |
| --- | --- | --- | --- |
| `calls/annotation.rs` | Call result signature hint | `LoweringHeaderPortV1` | header loan required |
| `calls/lowering.rs` | finalizer call/await lookup | `LoweringHeaderPortV1` | port-aware path only |
| `method_call_handlers.rs` | receiver method signature/arity | `LoweringHeaderPortV1` | header loan required |
| `calls/static_resolution.rs` | method-tail candidate scan | collector header inventory or a sealed catalog | no shell fallback |
| `calls/materializer.rs` | direct global-function presence | collector header presence | no retry route |
| `rewrite/known.rs` | known method signature/presence | collector header view | identity is not spelling authority |
| `builder_method_index.rs` | method-tail index rebuild | collector inventory projection | lifecycle/cache split |
| `builder_build.rs` | lowered constructor birth presence | collector header presence | no current-module read |
| `builder_metadata.rs` | closure-body metadata intern | `ModuleLoweringShellPortV1` | shell metadata write |
| `indexing.rs` | static-data-plan lookup | `ModuleLoweringShellPortV1` | shell metadata read |
| `module_lifecycle.rs` | shell setup, metadata, final aggregation | shell + one collector drain | terminal owner |
| `calls/function_session.rs` | legacy draft publication | `ModuleLoweringPortV1` commit | direct publication retires |
| `resolved_lowering/mod.rs` | A+ draft publication | prepared shell/collector admission | route adapter required |
| `resolved_lowering/callable_module_transaction.rs` | canonical batch publication | explicit common collector adapter | separate identity policy |

Test-only `current_module` assignments remain fixtures and are not production
authority. The key P0 invariant is stronger than a count:

```text
production current_module.functions header reads during active invocation = 0
production direct function publication during active invocation = 0
shell metadata/global writes use one shell port
collector drain occurs exactly once after all function drafts are sealed
```

The census shows that `I0-SHELL-P0` is a real ownership slice, not a rename of
`ModuleLoweringInvocationV1`. It must add the shell metadata port and the
collector drain preflight before any production child or root is connected.

## I0-SHELL-P0 closeout

The P0 boundary is now mechanically guarded without production cutover:

```text
source-derived reader rows = 14
  header = 8
  shell metadata = 2
  lifecycle = 2
  canonical transaction = 2

ModuleLoweringShellPortV1
  exposes only narrow metadata/global operations

ModuleLoweringShellDrainInventoryV1
  sorts and rejects duplicate symbols before drain

PreparedModuleLoweringShellDrainV1
  rejects inventory/function mismatch before shell mutation
```

The reusable HeaderPort guard checks each source anchor and its future owner
phrase in this card, so the census cannot remain green after a source move or
an undocumented authority change. Six focused shell fixtures, the source
census guard, cargo check, and diff checks are green. No production reader,
root, child, collector, or shell drain consumer has been connected.

The next row is `HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-SELECT`: decide the
single production invocation cutover owner and exact shell/collector terminal
sequence. Partial capture/commit remains forbidden.

## I0-SHELL-I0-SELECT decision

The source audit and an independent canonical-root audit select **Candidate
S-prime**:

```text
one ModuleLoweringInvocationV2
  owns one function-empty ModuleLoweringShellV1
  owns one ModuleDraftCollectorV1
  borrows the Builder only for the active lowering session
```

The cutover is invocation-wide. A raw-child-only or canonical-only switch is
rejected because `main`, another root family, or a re-entrant child would keep
`current_module.functions` as a second function authority.

### Authority lock

```text
completed function/header truth:
  ModuleDraftCollectorV1 only

module globals and accumulated metadata:
  ModuleLoweringShellPortV1 only

shell function map during lowering:
  structurally empty

raw child identity:
  LegacySymbol + LegacyReplaceWholePair

A+ / trivial identity:
  CanonicalResolvedOwner(FunctionOwnerIdV1)

acyclic / recursive identity:
  existing sealed callable catalog remains sibling/header authority;
  physical drafts enter the collector as CanonicalCallable(key)
```

The collector is not a replacement callable catalog and does not decide
target identity, graph cardinality, or recursive capability. It receives only
already-sealed physical drafts.

### Terminal law

```text
raw / A+ / trivial child:
  capture -> scoped header reads -> validate -> prepared admission
  -> seal -> collector commit -> parent restore

main:
  remains only in FunctionLoweringState while root lowers
  -> capture -> Main admission -> collector commit

condition_fn:
  synthesize and collect only when the collector header is absent

invocation completion:
  validate complete collector inventory and empty shell
  -> prepare one drain product owning shell + collector state
  -> infallible drain exactly once
  -> final verify / external return
```

The existing shell drain API is a disconnected vocabulary only. Production
I0 must add a `PreparedInvocationDrainV1` that performs every symbol,
identity, main/condition policy, and shell-emptiness check before consuming
either owner. No fallible lookup, assertion, or retry may remain after drain.

### Failure law

```text
child primary/cleanup/admission/panic:
  restore parent exactly once
  collector prefix unchanged
  shell unchanged

root/main/preflight failure:
  drop invocation-owned shell and collector
  publish no module

canonical candidate failure:
  drop candidate Builder
  no legacy/A+/BindingSSA retry
```

This does not claim whole-legacy-Builder rollback. Only the invocation-owned
shell/collector and the unpublished candidate are transactional.

### Rejected candidates

```text
raw-only child cutover
canonical-only cutover
current_module as collector backing store
collector prefix + current_module fallback
one root at a time
```

### Fixed task order

```text
HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-SELECT  closed here
  -> HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-S0
     PreparedInvocationDrainV1 vocabulary, consumers = 0
  -> HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-P0
     raw/main/condition, A+/trivial, acyclic/recursive,
     reader and failure matrix
  -> HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-I0
     one atomic production cutover across all root families
  -> HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-G0
     active current_module header reads = 0
     direct function insertion = 0
     collector drain = 1
```

The disconnected S0 vocabulary and P0 matrix are now closed. Production
capture/commit and `CUT0` remain forbidden until the following I0/G0 series.

## I0-SHELL-I0-S0 closeout

The disconnected invocation drain vocabulary now lives in
`src/mir/builder/module_invocation_drain.rs`:

```text
ModuleLoweringInvocationDrainOwnerV1
  owns one shell and one collector

InvocationDrainExpectationV1
  owns sorted complete symbols plus main/condition_fn policy

PreparedInvocationDrainV1
  is non-Clone and single-use
  drains only after all preflight checks pass
```

The owner compares the collector inventory with the expected complete batch,
checks shell emptiness, and enforces the `main`/`condition_fn` policy while
both owners are still intact. Only the resulting prepared product may call
the shell's no-fallible-check `commit_preflighted` path. No production root,
child, canonical transaction, or `MirBuilder` consumer exists; the reusable
HeaderPort guard rejects any such consumer.

Focused S0 fixtures are green:

```text
complete inventory + required roots -> one assembled module
missing main                     -> typed preflight failure
inventory mismatch               -> typed preflight failure
```

This closeout precedes the route matrix below; no production capture/commit
was performed.

## I0-SHELL-I0-P0 closeout

The disconnected route/failure matrix now lives in
`src/mir/builder/module_invocation_route_matrix.rs`.  It is a passive product
with no Builder, module, draft, fact, or retry authority.  One row owns each
route identity and publication policy, so the collector tests no longer
recreate canonical routes as `LegacySymbol` rows.

The matrix covers:

```text
raw main root
raw static child
raw instance/constructor child
synthetic condition_fn
canonical A+ root and child
BindingSSA trivial root
BindingSSA acyclic module
BindingSSA recursive module
```

Each row also seals its failure stages and the common laws:

```text
collector prefix unchanged before a failed admission
parent restored exactly once for raw child failure
invocation/candidate dropped without external publication at root failure
retry/fallback = 0
```

The two focused matrix fixtures and the collector route-policy fixture are
green, and the reusable HeaderPort guard rejects any production matrix or
drain consumer.  Production capture/commit remains disconnected.

The next code-facing row is
`HEADERPORT0-REENTRANT-TERM0-I0-SHELL-I0-I0`: one atomic production cutover
across the complete route matrix.  `CUT0` remains forbidden until its G0.

## Resolved design questions (consultation record)

1. Does `ModuleLoweringInvocationV1` own the module shell, or does a separate
   `ModuleDraftCollector` terminal consume an explicit shell product?
2. Which module metadata is immutable lowering input, and which fields are
   accumulated only after all function drafts are collected?
3. How is the root `main` represented while `lower_root` recursively opens
   children without exposing a second live module function map?
4. What exact operation drains collected drafts into the module shell, and is
   it infallible after preflight?
5. Which current-module readers are allowed to observe only immutable
   declaration/catalog authority rather than completed function headers?
6. Does the canonical A+/BindingSSA module transaction reuse this collector
   physically, or does it adapt into the same prepared admission product?

## Minimum implementation slice after P0

The next approved code-facing slice is:

```text
I0-SHELL-I0-I0
  one atomic production cutover across the complete route matrix
```

The slice must preserve the route-specific identity and failure laws above;
it must not add a fallback, retry, or one-root-at-a-time publication route.

## Non-claims

```text
production invocation cutover
current_module/functions readers retired
collector -> module aggregation exists
FACTSESSION0 activation
PHI or finalization repair changes
JoinIR or Loop bridge widening
```
