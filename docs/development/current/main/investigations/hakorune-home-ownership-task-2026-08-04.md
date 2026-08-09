# Hakorune Home ownership — parked task order

Status: Parked design/execution board; no current execution authority

Date: 2026-08-05

Decision state:

- Home model direction: accepted;
- C′ last-Home finalization target: accepted; production activation 0;
- explicit early Home release: contextual statement `release root` accepted
  for one verified whole-root Home; ordinary/generic `release(value)` has no
  Home authority, `drop` alias rejected, production activation 0;
- exact HomeV1 grammar and physical Shared representation: provisional/D0;
- generic/composite whole-root support under the same `release root` statement:
  provisional/D0; no generic wrapper callable;
- production activation: 0;
- resume checkpoint: `MIRBUILDER-INPLACE-REPLACEMENT0` final-pipeline
  completion;
- current row remains the row named by `CURRENT_STATE.toml`.

Parser cleanliness forward-order correction (2026-08-10):

- reconciliation authority:
  `own-home-parser-cleanliness-reconciliation-2026-08-10.md`;
- before Take/Share activation, close contextual HTRIVIA parity, recut the
  direct-method parser observation transaction, then replace the Hako raw
  parameter transfer tag/builder token with the accepted typed seal;
- nested Release exact paths and Dynamic-local slot indices remain P2 polish;
- this parked correction never overrides the active row in
  `CURRENT_STATE.toml`.

Authorities:

- source semantics: `docs/reference/language/ownership.md`
- cross-layer boundary:
  `docs/development/current/main/design/ownership-home-model-ssot.md`
- terminal finalization:
  `docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md`
- failure/exit transaction:
  `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`
- language workstream:
  `docs/development/current/main/workstreams/language-v1-convergence-current.md`

Supersedes as execution order:

- `hakorune-sparse-ownership-surface-task-2026-07-15.md`
- `hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md`
- `ownership-view-missing-grammar-inventory-2026-07-28.md`
- `ownership-view-performance-compatibility-design-2026-08-04.md`

Those files remain historical evidence. They do not select work or restore
the former `move/share/view` target.

## Goal

Deliver a small source model:

```text
ordinary use        -> non-owning handle
Home-demand edge    -> one Home is transferred
share expression    -> one independent owner is added
release root         -> one verified whole-root Home ends now
terminal Home end   -> one C′ fini/field/native DropPlan
```

while retaining precise compiler products, fail-fast boundaries, and a
measured C-speed physical path.

## Non-goals for the first program

- Rust-style general lifetime syntax;
- a new immutable ownership IR;
- hidden retain/release or profile fallback;
- source-level arena/region allocation;
- general field move-out, consuming receiver, or alias PHI;
- general generic/composite/field/projection explicit release;
- all generics, dynamic dispatch, FFI, or concurrency in the first production
  slice;
- a nominal `shared box` decision before its representation D0.

## Dependency graph

```text
RESUME
  -> CENSUS
  -> TAXONOMY
     -> COMPOSITE/TRIVIAL
     -> REPRESENTATION
     -> STORAGE DESTINATIONS
     -> CALLABLE ABI
     -> TRANSFER/FAILURE TIMING
     -> BIRTH/CONSTRUCTION
     -> RESULT/EXIT D0
     -> LAST-HOME FINALIZATION C′ D0
     -> EXPLICIT WHOLE-ROOT HOME RELEASE D0
  -> SURFACE DECISION
  -> PASSIVE RELATION + ABI + BOUNDARY
  -> STRAIGHT HOME FLOW
  -> CFG HOME FLOW + DIAGNOSTICS
  -> GRAMMAR CARRIERS
  -> PASSIVE EXPLICIT-RELEASE PLAN
  -> UNIQUE CLOSED-CALL + LOCAL TERMINAL-FINALIZATION PROTOTYPE
  -> STORAGE DESTINATION ADOPTION
  -> CONTRACT BOUNDARIES
  -> SHARE MATERIALIZATION + SHARED TERMINAL WINNER
  -> C-SPEED PHYSICAL GATE
  -> PROFILE CUTOVER / RETIREMENT
```

No grammar or production row may jump over the D0 fan-out.

## Milestone 0 — safe resume

### `OWNERSHIP-HOME-RESUME-D0`

Revalidate after the MirBuilder checkpoint:

- live parser/AST/registry ownership surface;
- old inactive-syntax reject guards;
- current SharedV1 producers and fallback edges;
- Binding SSA, Ownership SSA, callable catalog, lifecycle, ObjectStoragePlan,
  and backend owners;
- current `.hako` corpus candidates.

Done:

- current authority versus historical evidence is named;
- no stale row is resumed from the old sparse/View boards;
- worktree is clean and the current-state pointer guard is green;
- the next row is a design/census row, not parser implementation.

## Milestone 1 — bounded evidence

### `OWN-HOME-CENSUS0`

Read-only bounded census of:

- return origins;
- owning stores and container insertions;
- call arguments and captures;
- field/array/map/global/registry storage;
- generic/`Any`/record/enum/Option/Result occurrences;
- function values, interface/dynamic calls, plugin/FFI boundaries;
- places where current code implicitly adds or drops a strong owner;
- existing `fini` declarations classified as ordinary method, scope-cleanup
  alias, runtime/plugin hook, or generated/native adapter;
- direct receiver `obj.fini()` calls and callable-catalog/delegate/interface
  exposure;
- manual parent-to-child `fini` cascades and their intended order;
- last-owner/Arc/Drop/global-finalizer/plugin/native routes that currently
  dispatch or bypass user finalization.

Use static search first, then one case and a small sample before any complete
corpus pass. This is evidence, not syntax authority.

Deliverable: one source-kind × destination-kind × representation matrix.

The matrix is not complete until it emits the decision inputs consumed by the
next D0 rows. These are semantic counts, not raw lexical hit totals. Each
count must include its classification rule, a small sample of source paths,
and an explicit `unknown/unresolved` bucket:

| Next D0 | Required census input |
| --- | --- |
| `OWN-HOME-REPRESENTATION-D0` | distinct nominal types that need an independent Shared owner; distinct type symbols observed in both Unique-only and potential-Shared contexts |
| `OWN-COMPOSITE-TRIVIAL-D0` | owner-bearing record/enum declarations or instantiations; `Option`/`Result` wrapping an owner-bearing payload; unresolved generic `T`/`Any`/recursive sites |
| `OWN-HOME-TAKE-EXPR0-D0` | local Home rename, explicit rebinding, or lifetime-narrowing sites that cannot use an existing destination; ordinary `local x = y` aliases are excluded |
| `OWN-HOME-FIELD-TAKE0-D0` | field/container reads that remove or replace an owner-bearing Home; ordinary field reads and stores are excluded |
| `OWN-HOME-BIRTH-D0` | `new` sites by target, birth hook declarations/parameters, declaration initializers and explicit override stores, and fallible construction paths after a prior Home store |
| `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0` | `fini` declarations/calls/catalog exposure, manual child cascades, last-owner drop and plugin/native routes, each with one migration disposition: ordinary fallible `close`/domain method, non-callable automatic hook, structural-only drop, or delete/reject |

The census must state `0` when a row has no corpus consumer. A raw `rg`
count such as every `new` token is an inventory hint only; it cannot by itself
close a representation, composite, or transfer decision.

## Milestone 2 — semantic decisions before grammar

Close `OWN-HOME-TAXONOMY-D0` first. After its vocabulary is fixed, evidence
work for the remaining D0 rows may run in parallel. Every D0 must close and
then converge through `OWN-HOME-SURFACE-D0` before passive products or grammar
activation.

### `OWN-HOME-TAXONOMY-D0`

Fix the exact meanings of:

- Home/place;
- Handle;
- transferable HomeValue;
- Unique, Shared, Weak, Trivial, Unknown;
- destination Home demand;
- terminal Home forward;
- candidate HomeV1 syntax versus parked syntax.

Acceptance includes a no-second-authority rule: the old `move/view/shared`
surface cannot remain accepted beside HomeV1.

Keep five notions distinct:

```text
object identity
Home token (one independent lifetime)
Home slot/place (storage for a token)
non-owning handle
runtime ObjectHandle/carrier
```

One Shared object identity may be supported by multiple Home tokens. Do not
describe this as one physical Home object per Box.

### `OWN-COMPOSITE-TRIVIAL-D0`

Define the recursive capability algebra for:

- primitives and identity-free leaves;
- records;
- enums and variant-sensitive payloads;
- `Option<T>` and `Result<T, E>`;
- containers;
- recursive types and cycles;
- generic `T`, constraints, monomorphization, and `Any`.

Unknown never defaults to Trivial or Shared. Decide exactly which facts are
available before Builder effects.

### `OWN-HOME-REPRESENTATION-D0`

Compare without preselection:

- per-instance Unique-to-Shared promotion;
- nominal `shared box` or another type-level Shared capability;
- direct Unique physical ownership;
- Shared control-cell/registry requirements;
- weak identity/generation coupling.
- whether a Shared HomeValue satisfies a general `take`/Home demand or needs a
  distinct declared Shared demand/type.

Done only with source law, runtime layout owner, backend capability, and
failure boundary. This row must not choose from current `Arc` convenience.

### `OWN-FIELD-CONTAINER-DEST-D0`

Seal the destination matrix for:

- local Home;
- object field;
- array/map/packed storage;
- global/static/registry storage;
- parameter and return;
- weak storage;
- replacement, empty-slot, and destruction behavior.

Also decide ordinary local initialization and reassignment:

```hako
local b = a
b = c
```

It must be exactly handle rebinding, Home replacement, or rejected for the
first profile; assignment context may not guess per runtime value. Cover
uninitialized/`null` locals separately.

Field move-out remains parked unless this row names a separate exact witness.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-D0`

Decision: accepted by
`box-lifecycle-cprime-terminal-home-finalization-ssot.md`; implementation stays
parked. This row replaces the former `OWN-SHARE-FINI-CLEANUP-D0` question and
must not retain B′ as a parallel authority.

Fix one responsibility split:

```text
share = explicit independent Home acquisition
cleanup = standalone lexical exit registration
fini { } = non-callable terminal Home hook
close()/shutdown() = ordinary domain methods
```

Seal the local/field/return/take/share/weak/birth matrix, parent-hook-before-
field-release ordering, reverse declaration-order owning-field release,
terminal-child-only hook dispatch, field replacement transaction, exactly-once
Shared winner, cycle non-guarantee, finalizer non-escape/no-suspension rules,
and pre-effect rejection for cross-thread/plugin/FFI families without an exact
contract.

Dependencies:

```text
LANGUAGE-RESULT-EXIT-C-PRIME0-D0
OWN-HOME-TAXONOMY-D0
OWN-COMPOSITE-TRIVIAL-D0
OWN-HOME-REPRESENTATION-D0
OWN-FIELD-CONTAINER-DEST-D0
OWN-HOME-TRANSFER-FAILURE-D0
OWN-HOME-BIRTH-D0
```

Done only when terminal Home release has one proposed DropPlan authority,
direct `obj.fini()` and B′ Dead-with-live-Home are rejected targets, and no
ordinary handle or runtime refcount observation can dispatch the hook.

### `OWN-EXPLICIT-HOME-RELEASE-STMT0-D0`

Decision: accepted semantic target; implementation remains parked.

**Change**: supersede the earlier ordinary-call spelling and select contextual
statement `release root` as the sole explicit early Home-end spelling.
Ordinary/generic `release(value)`, `drop root`, `drop(value)`, direct
`obj.fini()`, and identifier-based compiler magic have no Home authority.

**Contract**: the first profile accepts only a verified whole-root owning local
or owning parameter with exactly one available Home. It consumes that root at
the source point, invalidates dependent handles without hidden re-rooting, and
enters the existing C′ DropPlan only when the release is terminal. It has no
Result channel; `close()`/`shutdown()` remain ordinary domain methods. Trivial
roots reject rather than silently no-op. Generic/composite capability and
field/projection/container release are not claimed.

**Done**: the exact Unique/Shared non-terminal/terminal matrix, cleanup-capture
conflict, Fault chronology, diagnostics, contextual grammar carrier, resolved
root, sealed Home Flow plan, and post-implementation reference receipts are
named without adding production parser, Builder, runtime, or backend callers.

**Stop**: return to design if the implementation needs generic capability
guessing, more than one Home, field move-out, consuming receiver, cleanup/hook
Home consumption, cross-thread affinity, a `drop` alias, or fallback.

### `OWN-HOME-CALLABLE-ABI-D0`

Fix before schema/grammar:

- exact ClosedCallable classifier, including direct body-known recursion/SCC;
- exact ContractBoundary classifier;
- declaration-side parameter/receiver demand authority: body inference may
  verify explicit demand and infer result relation, but never invent a
  consuming plain parameter;
- public/export declaration stability;
- bodyless Shared result contract;
- whether Home ABI participates in callable identity, overload selection, and
  interface compatibility (the first profile should not overload only by Home
  demand unless explicitly decided);
- generic instantiation and unknown behavior;
- result-origin substitution at call sites;
- named stable anchors versus temporary receivers.

The first profile rejects a borrowed result rooted in a temporary such as
`makeTree().getRoot()` unless a separate lifetime-extension rule is sealed.

### `OWN-HOME-TRANSFER-FAILURE-D0`

Fix the exact transfer point relative to:

- left-to-right argument evaluation;
- failure while evaluating a later argument;
- callee entry;
- cleanup on caller/callee failure;
- terminal return and publication.

The source must retain a coherent Home when argument preparation fails before
the call boundary. Select one effect-free preflight/commit rule; do not let
individual lowerers choose when consumption becomes visible.

### `OWN-HOME-BIRTH-D0`

Dependency: `OWN-FIELD-CONTAINER-DEST-D0` plus
`OWN-HOME-TRANSFER-FAILURE-D0`. The source-level construction lifecycle is
owned by [`constructor-birth-new-lifecycle-ssot.md`](../design/constructor-birth-new-lifecycle-ssot.md);
this row supplies the missing Home boundary and must not redefine constructor
syntax or direct-`birth` policy.

Close the Home behavior of the complete construction transaction:

- `new` allocates a fresh object identity and is the source-level primary
  producer of the first candidate `HomeValue`/Home token; `share` is a later
  owner acquisition, not a second birth operation;
- declaration-site field initializers, the `birth(args...)` hook, and the
  optional `new Box { field: expr }` override block are classified through the
  destination matrix, with no hidden `share` or implicit partial transfer;
- `birth` parameters follow normal Handle/Trivial rules by default. A
  consuming/Home-demand parameter is allowed only when the resolved
  declaration/ABI explicitly seals that demand; the name `birth` alone never
  consumes an argument;
- the fresh `me` remains an unpublished/constructing object until the existing
  lifecycle reaches its publish point. A partially constructed object must not
  escape through a field, return, global, container, callback, or `share`;
- construction failures are classified by phase: argument preparation, field
  initializer, `birth` body, explicit override, and publication. For each
  phase, decide which already-installed Home tokens are cleaned, how the
  partially constructed `me` is retired, and how no-leak/no-double-cleanup is
  proven;
- direct receiver `birth(...)` remains forbidden; failed unpublished outer
  construction never invokes the outer C′ `fini` hook. Already-complete child
  Homes are released in reverse installation order and may invoke their own
  hook only when that release is terminal.

The existing constructor SSOT fixes the successful order as
`new -> declaration initializers -> birth -> optional explicit overrides ->
publish`. It does not currently define partial-construction rollback; that is
the deliberate D0 gap, not an invitation to infer cleanup from runtime tags.

Done only when the row has a phase-by-phase Home state/recovery matrix,
bounded corpus fixtures for each admitted failure boundary, and a named
single cleanup owner. No parser, Home Flow, or runtime implementation begins
from this row alone.

### `OWN-HOME-SURFACE-D0`

Convergence row after every Milestone 2 D0 above. Select the exact HomeV1
source grammar and hard rejects, including:

- declaration-side Home demand spelling;
- plain parameters remain Handle and cannot become consuming from body shape;
- opaque result relation and Shared result spelling;
- expression-side share;
- caller-side transfer omission versus optional lint;
- local reassignment and terminal return rules;
- parked `take` expression/field/receiver forms;
- contextual statement `release root` with one identifier root;
- contextual-keyword disambiguation.

This row converges the remaining candidate spellings. It carries the already
accepted `release root` spelling unchanged and cannot reopen or replace it.

## Milestone 3 — passive compiler products

These rows begin only after `OWN-HOME-SURFACE-D0` and add types/verifiers with
production callers zero.

### `OWN-HOME-RELATION0-S0`

Introduce branded, non-forgeable relation vocabulary for Home roots,
destinations, result origins, and typed rejection reasons.

Implementation task:
`docs/development/current/main/investigations/own-home-relation0-s0-implementation-task-2026-08-09.md`

The bounded S0 module is passive and caller-zero. It issues only a fresh
relation brand, opaque source ordinals, exhaustive demand/result vocabulary,
and typed foreign/duplicate rejection reasons. It does not classify types or
issue `VerifiedHomeAbi`; those remain the next `OWN-HOME-ABI0-S0` boundary.

### `OWN-HOME-ABI0-S0`

Design/implementation boundary:
[`own-home-abi0-s0-design-task-2026-08-09.md`](own-home-abi0-s0-design-task-2026-08-09.md).
The design stop is closed; the bounded implementation task is
[`own-home-abi0-s0-implementation-task-2026-08-09.md`](own-home-abi0-s0-implementation-task-2026-08-09.md).

The design stop fixes one `CallableHomeAbiIssuerV1`, one same-resolver-brand
capability environment, and one non-`Clone` `VerifiedHomeAbiV1` catalog. The
passive relation brand is batch provenance only, never nominal type identity.
The later implementation introduces receiver/parameter demands and result
relation for the explicit I64/Unit cohort. Parameter/receiver demands come
from the resolved declaration; ClosedCallable body analysis may infer result
relation and verify local flow, but cannot invent a consuming demand. Only the
canonical issuer seals the product. The bounded ABI0 implementation is
caller-zero and does not open Query behavior, target, Recipe, or Home Flow.

The next design stop is
[`own-home-query-behavior-d0-design-task-2026-08-09.md`](own-home-query-behavior-d0-design-task-2026-08-09.md),
which must co-seal typed Query behavior without reissuing Home relations.

### `OWN-HOME-BOUNDARY0-S0`

Classify ClosedCallable and ContractBoundary. ContractBoundary includes
export, separate compilation, interface/dynamic call, callback/function value,
plugin/FFI, and unresolved generic cases. Require exact declaration/manifest
ABI and compiled-artifact schema/profile/dependency fingerprints.

No user-maintained lock file becomes semantic authority.

## Milestone 4 — Home Flow and diagnostics

### `OWN-HOME-FLOW0-S0`

Straight-line, caller-zero availability verifier:

```text
Available -> Consumed
Available -> Shared acquisition + Available
Handle -> Home demand = reject
Unknown -> ownership-changing edge = reject
```

Binding SSA supplies identities; Home Flow does not remap values.

Consumption becomes visible only at the transfer point selected by
`OWN-HOME-TRANSFER-FAILURE-D0`; argument evaluation cannot partially consume a
caller Home and then retry another route.

### `OWN-HOME-FLOW-CFG0-S0`

Add joins, branches, and loop backedges:

- reject use after conditional consume;
- report the branch that consumed the Home;
- reject a consumed Home reaching a backedge without replacement;
- permit only separately proven loop-local fresh, consume+break, and
  consume+replenish shapes;
- no hidden PHI owner synthesis.

### `OWN-HOME-ARG-MATRIX0-S0`

Freeze argument/destination behavior:

- handle alias to Home-demand parameter: reject with root fix;
- `share` to handle-only parameter: reject as redundant paid owner;
- fresh Home temporary to handle-only parameter: exact scoped lifetime rule;
- Home to ordinary handle input: no consume;
- Home to Home-demand input: consume exactly once.

### `OWN-HOME-DIAG0-S0`

Golden-test typed diagnostics for branch/backedge availability, boundary ABI,
destination mismatch, redundant share, unknown capability, and result-origin
conflict. Every hint is filtered by an actual capability witness.

## Milestone 5 — grammar carriers, still production-zero

Grammar begins only after Milestones 2–4 are closed.

Within the release/finalization subfamily, the exact order is:

```text
OWN-GRAM-RELEASE0
-> OWN-GRAM-FINI-HOOK0
-> OWN-FINI-HOOK-PLAN0-S0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0
-> OWN-EXPLICIT-HOME-RELEASE0-S0
```

### `OWN-GRAM-HOME-PARAM0`

Candidate contextual form:

```hako
adopt(take node: Node)
```

Land registry, Rust parser, Hako parser, AST/schema, formatter, reject tests,
and one shared grammar guard together. It declares a destination Home demand;
call-site `take` is not part of this row.

### `OWN-GRAM-HOME-RESULT0`

Candidate ContractBoundary form:

```hako
getRoot(): Node from me
```

Resolve exact anchor grammar, multi-return consistency, generic restrictions,
whether ClosedCallable source may omit it, Shared result spelling, and
temporary-root rejection. No multi-anchor result PHI.

### `OWN-GRAM-SHARE0`

Candidate contextual expression:

```hako
adopt(share service)
```

`share(...)` remains an ordinary call. Parser transport is not permission to
materialize a Shared owner.

### `OWN-GRAM-RELEASE0`

Accepted contextual statement target:

```hako
release file
```

Land the registry, Rust parser, Hako parser, dedicated AST/schema carrier,
formatter, exact-source receipt, and positive/negative grammar guard together.
The guard proves `release root` is contextual, not a globally reserved token:
`release(value)`, `obj.release()`, a callable/binding named `release`, and
`Build.release` remain ordinary, while `unbox root` has zero Home production.
The same implementation commit updates exact `docs/reference/**` support and
examples; later FIRST/FINAL closeout rows are audits, not deferred updates.
V1 accepts one identifier root only. `release(value)`, `obj.release()`, and a
binding named `release` remain ordinary source forms; parser acceptance grants
no Home consume authority before `VerifiedExplicitHomeReleasePlanV1`. The same
implementation commit updates EBNF, ownership, lifecycle, and quick-reference
pages, plus the language status index and stage-profile matrix, to the exact
parser-live surface.

### `OWN-GRAM-FINI-HOOK0`

After `LANGUAGE-RESULT-EXIT-C-PRIME0-R0` has retired scope-position `fini`,
land one unambiguous Box-member carrier:

```hako
box Resource {
    fini {
        me.closeBestEffort()
    }
}
```

The same bounded row updates the grammar registry/corpus, Rust parser, Hako
parser, AST/schema, formatter, resolver lifecycle-declaration catalog, and
shared parse witness. The lifecycle declaration is deliberately absent from
the ordinary callable catalog. Resolution produces a typed
`LifecycleHookDeclId`, not a method-name exception; member-call resolution
rejects that declaration kind before dynamic fallback and before Builder
effects. Unknown calls may not retry a runtime method table. Negative
witnesses also reject parameters/result annotations, ordinary `fini() {}` declarations,
alias/delegate/interface exposure, and scope-position `fini`.

### `OWN-FINI-HOOK-PLAN0-S0`

Seal one passive `VerifiedFinalizerHookPlanV1` before lifecycle effects. It
proves the exact hook body, FinalizerLease non-escape, no resurrection/re-entry,
no `return`/`break`/`continue`/`?`/suspension, no `share me`, exact field/native
capability, and one exact hook descriptor for later DropPlan composition. It
does not require or dispatch a TerminalHomeDropPlan. Unknown plugin/FFI/thread
affinity rejects before Builder effects. Builder caller count stays zero in
this row.

### `OWN-TERMINAL-HOME-DROP-PLAN0-S0`

Seal one passive `VerifiedTerminalHomeDropPlanV1` as the sole lifecycle
product. It contains:

```text
verified terminal-transition strategy (StaticUnique or admitted Shared)
optional VerifiedFinalizerHookPlanV1
verified owning/weak field release descriptor and reverse order
native structural-drop capability
weak tombstone/reclaim disposition
first-Fault and suppressed-teardown receipt
```

The plan is not a hook-only product. `I0/U`, `I0/F`, and `I0/S` consume this
same schema and never re-infer hook presence, field order, native capability,
or terminal strategy. Unknown representation, plugin/FFI, or thread affinity
freezes before Builder effects. The schema row is followed by exact
`OWN-TERMINAL-HOME-DROP-PLAN0-S0/U`, `/F`, and `/S` plan receipts after each
profile's facts exist; these are sealed instances of the same product, not
three policy owners. Physical consumer count remains zero in the schema row.

### `OWN-EXPLICIT-HOME-RELEASE0-S0`

After ABI, straight/CFG Flow, diagnostics, terminal DropPlan schema, and
`OWN-GRAM-RELEASE0` are sealed, produce one caller-zero
`VerifiedExplicitHomeReleasePlanV1`. It binds the parsed release carrier, exact
resolved whole-root place, available Home, path-sensitive consume,
dependent-handle invalidation, cleanup-capture exclusion, and C′ terminal
disposition. It never derives authority from the identifier spelling and
publishes no Builder/MIR, runtime, backend, generic, field, or Shared physical
capability.

Each grammar acceptance is one BoxCount row with one fixture, shared grammar
gate, and one commit. Do not mix grammar activation with lowering.

The shared negative fixture set keeps `move`, source `view/owned/shared`,
call-site `take`, `take place_expr`, consuming receiver, and field take
rejected until their own accepted rows exist.

## Milestone 6 — first Unique production slice

### `OWN-HOME-UNIQUE0-P0`

Select one exact Unique Box representation and one closed source shape. Prove:

- no RC/control-cell/registry owner work on the selected route;
- one Home creation, use, transfer, and terminal destruction;
- compile failure leaves the unpublished candidate discarded;
- fresh compiler reuse succeeds;
- unsupported routes fail before Builder effects.

### `OWN-HOME-CLOSED-CALL0-I0`

Activate one direct ClosedCallable call with one Home-demand parameter and/or
one terminal result. The caller consumes only `VerifiedHomeAbi`; body
re-inference and fallback are zero.

Add one destination family per later BoxCount row. Do not widen to fields,
containers, dynamic calls, or generic boundaries in the same commit.

## Milestone 7 — storage destination adoption

### `OWN-HOME-STORAGE0-I0`

Activate destinations one family at a time, each with its own fixture and
physical witness:

1. local Home initialization/terminal destruction;
2. local reassignment or its selected hard reject;
3. one object field store/replacement shape;
4. one array/map/container insertion shape;
5. global/registry storage;
6. weak storage/upgrade in its separate lifecycle row.

The first child cell is named `OWN-HOME-STORAGE0-I0/L`. It activates only
local Home initialization, forwarding, and terminal destruction; completion
of `/L` does not claim field/container/global storage. The first owning-field
cell is `OWN-HOME-STORAGE0-I0/F` and owns only one exact field store/replace
shape.

Do not generalize one field proof into every container. Field move-out and
consuming receiver remain parked unless their own D0 and storage receipts
land.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/U`

This cell starts only after all of the following are green:

```text
OWN-HOME-FLOW-CFG0-S0
OWN-HOME-CLOSED-CALL0-I0
OWN-HOME-STORAGE0-I0/L
OWN-TERMINAL-HOME-DROP-PLAN0-S0/U
```

It activates one owning-local terminal release by consuming the sealed
`VerifiedTerminalHomeDropPlanV1` unchanged. The physical witness must prove:

- ordinary handles, `take`, terminal return, and non-terminal Home release
  dispatch no hook;
- declared hook plus terminal Home release dispatches the hook exactly once;
- no declared hook dispatches zero user hooks while structural teardown still
  completes;
- local cleanup runs before the terminal release;
- the FinalizerLease cannot escape or resurrect the object;
- a body/cleanup/hook Fault chronology witness preserves the first Fault,
  suppresses later teardown Faults, and still releases remaining local/native
  payload best effort;
- the Unique route performs no RC, control-cell, or global-finalizer work;
- unsupported storage, Shared, backend, plugin, and FFI routes reject before
  effects without fallback.

### `OWN-EXPLICIT-HOME-RELEASE0-I0/U`

After the Unique local, closed-call, DropPlan `/U`, and passive release plan
are green, activate one exact owning-local `release root` route. The same
implementation commit updates the exact reference pages and examples. Prove
source-point synchronous release, dependent-handle invalidation, no cleanup
capture, terminal hook exactly once, `drop` alias zero, RC/control-cell zero,
and no retry/fallback. Owning parameter, generic, composite, field, projection,
container, Shared, plugin, and FFI cases remain rejected.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/F`

After `OWN-HOME-STORAGE0-I0/L`, `OWN-HOME-STORAGE0-I0/F`, and the `/F` terminal
DropPlan product are green, activate one field replacement and parent teardown
profile. It consumes `VerifiedTerminalHomeDropPlanV1` without re-inferring
hook presence, release order, or native capability:

```text
RHS evaluate/preflight
-> commit new field Home
-> release old field Home
-> old hook only if terminal

parent hook
-> verified owning fields in reverse declaration order
-> native payload drop
```

Include partial `birth` rollback: the unpublished outer hook is zero while
already-complete child Home releases may finalize terminal children. Manual
parent-to-child `fini` calls and field-order re-inference are zero.

## Milestone 8 — contract boundaries

### `OWN-HOME-CONTRACT-BOUNDARY0`

Land exact declaration/metadata consumption for, in order:

1. exported separately compiled direct call;
2. interface/dynamic dispatch parity;
3. callback/function value;
4. resolved generic instantiation;
5. plugin/FFI manifest.

Each boundary requires exact ABI match and fail-fast unknown behavior. No
whole-program body inference crosses the boundary.

## Milestone 9 — Shared materialization

### `OWN-HOME-SHARE0-I0`

Only after `OWN-HOME-REPRESENTATION-D0`:

- map one explicit `share` site to one verified physical acquisition;
- preserve source identity and source availability;
- reject weak/trivial/handle/unknown operands;
- prove no implicit owner producer;
- prove cleanup and terminal lifecycle remain separate owners.

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S`

After Shared materialization and `OWN-TERMINAL-HOME-DROP-PLAN0-S0/S` is
sealed, activate one exact same-thread Shared terminal winner and weak fence.
It consumes `VerifiedTerminalHomeDropPlanV1` without re-inferring field/native
or hook policy:

- non-last Home release dispatches no hook;
- exactly one zero-owner winner enters Finalizing;
- weak upgrade/new lease fails once Finalizing starts;
- hook, reverse field release, native drop, and weak tombstone publish once;
- cross-thread affinity, cycles, plugin, and FFI remain rejected or explicitly
  outside the admitted profile.

### `OWN-EXPLICIT-HOME-RELEASE0-I0/S`

After same-thread Shared acquisition and the terminal winner are green, extend
the same release plan to one exact Shared Home. Prove non-last release runs no
hook, the last release enters the same DropPlan exactly once, aliases are never
silently re-rooted, and the implementation/reference commit claims no
cross-thread, cycle, generic, composite, field, plugin, or FFI support.

## Milestone 10 — C-speed physical proof

### `OWN-HOME-C-SPEED0-G0`

For a selected exact front:

- collect perf top report before code changes;
- inspect assembly at the hot symbol;
- prove Unique alias/call/return adds no RC, control-cell, handle-registry, or
  Box birth work;
- compare exact-front instructions and whole-program behavior;
- include the admitted Unique `release root` front and prove it adds no
  generic dispatch, runtime lookup, RC, or global finalizer registry;
- include the ordinary scope-end terminal path and prove automatic `fini` plus
  reverse owning-field teardown through a focused assembly/performance witness;
- keep Shared owner accounting as a separate measured profile and never credit
  it as a Unique zero-cost result;
- keep only evidence-backed representation changes.

Grammar completion is not a performance gate.

## Milestone 11 — readiness and retirement

### `OWN-LAST-HOME-FINALIZATION-C-PRIME0-R0`

After the admitted Unique/field/Shared profiles and C-speed gate are green,
retire the selected competing authorities:

```text
direct obj.fini() source/callable owner = 0
ordinary fini(...) declaration = 0
B′ Dead-with-live-Home state = 0
manual parent -> child fini cascade = 0
global Box finalizer authority = 0
terminal structural drop bypassing a declared hook = 0
canonical B′ fallback/retry = 0
```

Plugin/FFI routes either migrate in a separate bounded series or reject before
effects; a host Drop route may not silently stand in for verified Home.

### `OWNERSHIP-HOME-PRODUCT-READINESS-D0`

Required before default/profile cutover:

- admitted source units have one Home authority;
- production callable Home ABI consumer count is exact;
- Home Flow covers every admitted CFG shape;
- Unknown/opaque boundaries fail before effects;
- implicit-share producers and profile retries are zero or named blockers;
- old sparse/View target docs are historical only;
- SharedV1 corpus migration and rollback-free cutover are planned.

### `OWNERSHIP-HOME-CUTOVER0-I0-R0`

One whole-unit profile cutover. In the same series, retire selected old
production authority and forbid HomeV1 failure -> SharedV1 retry. Keep legacy
profile support only when explicitly selected by project/source-unit policy.

## Milestone 12 — normative reference closeout

### `OWN-HOME-REFERENCE-CLOSEOUT0-DOC0`

This parent row has two mandatory execution cells. It is a documentation
contract, not a grammar or lowering shortcut:

```text
OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/U
-> OWN-EXPLICIT-HOME-RELEASE0-I0/U
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST
-> OWN-HOME-STORAGE0-I0/F
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/F
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/F
-> OWN-HOME-SHARE0-I0
-> OWN-TERMINAL-HOME-DROP-PLAN0-S0/S
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-I0/S
-> OWN-EXPLICIT-HOME-RELEASE0-I0/S
-> OWN-HOME-C-SPEED0-G0
-> OWN-LAST-HOME-FINALIZATION-C-PRIME0-R0
-> OWNERSHIP-HOME-PRODUCT-READINESS-D0
-> OWNERSHIP-HOME-CUTOVER0-I0-R0
-> OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FINAL
```

`/FIRST` reports only the exact first production slice and must not claim
field/Shared/default-profile support. `/FINAL` reconciles the final cutover and
is the only cell that marks the parent DOC0 complete.

Both cells include the named receipt
`LIFECYCLE-LAST-HOME-FINI-REFERENCE-CLOSEOUT0-DOC0` with
`slice = first | final`; the Home closeout cannot be marked complete without
both lifecycle/reference proofs. They also include
`OWN-EXPLICIT-HOME-RELEASE-REFERENCE-CLOSEOUT0-DOC0` with the same slice.
Every grammar, passive-plan, implementation, and retirement cell updates its
exact live reference/support status and examples in the same commit;
FIRST/FINAL are audits, not permission to leave references stale between
implementation and closeout.

Update the normative and derived reference surfaces from provisional/parked
language to the exact implementation that actually landed:

* `docs/reference/language/ownership.md` — Home/Handle rules, accepted
  `take`/`share`/`release` surface, destination-side Home demand, rejected
  forms, diagnostics, and the exact profile/fallback policy;
* `docs/reference/language/EBNF.md` and its grammar registry — the exact
  parser-live contextual `release root` statement, with contrasting examples
  showing that `release(value)` and `obj.release()` remain ordinary calls;
* `docs/reference/language/README.md`, variables/scope, lifecycle, cleanup,
  and constructor/birth references — ownership, Box-member `fini {}` as a
  terminal hook, direct-`fini` rejection, ordinary `close()` methods,
  canonical `release root`, zero `drop` alias, standalone cleanup, `new`,
  field initializer, `birth`, and
  partial-construction boundaries must point to their separate owners;
* `docs/reference/boxes-system/memory-finalization.md`,
  `docs/reference/boxes-system/README.md`,
  `docs/reference/architecture/rust-to-hako-lifecycle-projection.md`, both
  plugin lifecycle references, and VM plugin integration — replace B′/direct
  fini/manual child cascades with the exact implemented C′ and plugin/FFI
  capability boundary;
* deprecated `docs/reference/plugin-system/plugin-system.md` and any other
  historical Box/plugin page carrying callable `fini()` or Arc-only lifecycle
  wording — retain historical status where appropriate, but remove stale live
  claims from indexes and closeout views;
* `docs/reference/ir/json_v0.md`, ownership/exit MIR references,
  callable/interface/FFI ABI references, and generated support views — exact
  Home ABI/profile metadata, one resolved release plan, no body re-inference
  at a boundary, and no hidden strong-owner producer;
* active language workstream dashboards, examples, migration notes, and
  environment-variable documentation — no stale `move/view/owned/shared`
  target or SharedV1 retry claim remains presented as the live Home surface;
* historical proposals and parked design cards — label them as evidence and
  link the accepted reference page instead of silently rewriting history.

C′ closeout evidence is mandatory after the first C′ production slice and
again after final cutover. The two cells are time-bounded and must not borrow
future-slice evidence.

`OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FIRST` requires exactly:

```text
fini hook grammar == Rust parser == Hako parser == lifecycle descriptor = 1
direct obj.fini() accepted examples                                 = 0
Unique local terminal hook dispatch exactly once                     = 1
close/shutdown reserved language syntax                              = 0
release contextual grammar/parser/AST parity                         = 1
release identifier/MIR-name authority                                = 0
resolved whole-root release plan for admitted slice                  = 1
ordinary/generic release wrapper Call as Home authority              = 0
drop root / drop(value) accepted alias                               = 0
generic/composite release support claimed by FIRST                    = 0
first-slice B′ live reference claim                                  = 0
field/Shared/default-profile support claimed by FIRST                 = 0
```

`OWN-HOME-REFERENCE-CLOSEOUT0-DOC0/FINAL` requires every `/FIRST` item plus:

```text
parent hook before reverse verified-owning-field release             = 1
Shared non-last hook dispatch                                        = 0
Shared terminal hook dispatch exactly once                           = 1
Shared release non-last/terminal parity                              = 1
final cutover/reference parity                                       = 1
B′ live reference claim across all live reference pages              = 0
```

Each closeout must compare the reference grammar with both parser registries,
the resolver/Home Flow verifier, callable ABI metadata, and only the physical
route selected at that slice. A mismatch that reflects real implementation
behavior reopens the owning code row; documentation must not hide it as
historical prose.

Required evidence:

```text
reference grammar == parser-live grammar for this slice             = 1
reference Home ABI == sealed compiler product for this slice        = 1
accepted examples compile on this slice's selected profile          = 1
rejected examples fail before Builder effects                       = 1
old claims within this slice's live reference surface               = 0
implementation-backed reference closeout before final completion    = 1
docs-only premature or future-slice reference claim                 = 0
```

This row does not add `region`, field-take, consuming receivers, multi-anchor
views, generic/dynamic/FFI ownership, or any other parked capability. It also
does not make the `fini` hook a direct-call, physical-free, or transfer
operation.

## Parked follow-ups

- `OWN-HOME-TAKE-EXPR0-D0`: `take place_expr` for lifetime narrowing/local
  renaming, with a real corpus consumer;
- `OWN-HOME-FIELD-TAKE0-D0`: field extraction, empty slot, replacement;
- `OWN-HOME-CONSUMING-RECEIVER0-D0`;
- `OWN-EXPLICIT-HOME-RELEASE-COMPOSITE-ROOT0-D0`: generic/composite whole-root
  support under the same statement after exact Home-bundle classification and
  a real consumer; no `release<T>` wrapper callable;
- `OWN-HOME-MULTI-ANCHOR0-D0` and result PHIs;
- capture, `await`/`yield`, task/channel, and cross-thread flow;
- explicit `region` after a real arena allocation/free substrate exists;
- closed-graph promotion and bulk reclaim;
- unsafe raw ownership lane.

## Guard policy

Do not create one shell script per row. Extend one reusable Home contract guard
only when code/schema lands. It should eventually check:

- one `VerifiedHomeAbi` consumer path;
- no old `move/view` production grammar authority;
- no HomeV1 -> SharedV1 fallback;
- no hidden strong-owner producer outside explicit `share`/boundary witnesses;
- exact production caller counts for selected physicalizers.
Until then, use the current-state pointer guard and document-only checks.
