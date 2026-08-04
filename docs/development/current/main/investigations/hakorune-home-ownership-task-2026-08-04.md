# Hakorune Home ownership — parked task order

Status: Parked design/execution board; no current execution authority

Date: 2026-08-04

Decision state:

- Home model direction: accepted;
- exact HomeV1 grammar and physical Shared representation: provisional/D0;
- production activation: 0;
- resume checkpoint: `MIRBUILDER-INPLACE-REPLACEMENT0` final-pipeline
  completion;
- current row remains the row named by `CURRENT_STATE.toml`.

Authorities:

- source semantics: `docs/reference/language/ownership.md`
- cross-layer boundary:
  `docs/development/current/main/design/ownership-home-model-ssot.md`
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
```

while retaining precise compiler products, fail-fast boundaries, and a
measured C-speed physical path.

## Non-goals for the first program

- Rust-style general lifetime syntax;
- a new immutable ownership IR;
- hidden retain/release or profile fallback;
- source-level arena/region allocation;
- general field move-out, consuming receiver, or alias PHI;
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
     -> SHARE/FINI/CLEANUP
  -> SURFACE DECISION
  -> PASSIVE RELATION + ABI + BOUNDARY
  -> STRAIGHT HOME FLOW
  -> CFG HOME FLOW + DIAGNOSTICS
  -> GRAMMAR CARRIERS
  -> UNIQUE CLOSED-CALL PROTOTYPE
  -> STORAGE DESTINATION ADOPTION
  -> CONTRACT BOUNDARIES
  -> SHARE MATERIALIZATION
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
- places where current code implicitly adds or drops a strong owner.

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

### `OWN-SHARE-FINI-CLEANUP-D0`

Keep three responsibilities distinct:

- `share`: independent owner acquisition;
- `fini()`: logical object lifecycle termination;
- `cleanup`: lexical exit action.

Define share eligibility, live-handle invalidation, weak interaction, and
error precedence. Decide whether `fini()` requires a Home-capable receiver,
whether the invoking handle is permitted, and how Dead is observed through all
remaining Shared owners. No hidden share and no physical-free claim.

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
- direct receiver `birth(...)` remains forbidden, and `fini()` remains the
  logical usable-lifetime hook rather than a physical-free or construction
  rollback spelling.

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
- contextual-keyword disambiguation.

Only this row may promote candidate spellings into accepted target grammar.

## Milestone 3 — passive compiler products

These rows begin only after `OWN-HOME-SURFACE-D0` and add types/verifiers with
production callers zero.

### `OWN-HOME-RELATION0-S0`

Introduce branded, non-forgeable relation vocabulary for Home roots,
destinations, result origins, and typed rejection reasons.

### `OWN-HOME-ABI0-S0`

Introduce one `VerifiedHomeAbi` containing receiver/parameter demands and
result relation. Parameter/receiver demands come from the resolved declaration;
ClosedCallable body analysis may infer result relation and verify local flow,
but cannot invent a consuming demand. Only the verifier seals the product.

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

Do not generalize one field proof into every container. Field move-out and
consuming receiver remain parked unless their own D0 and storage receipts
land.

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
- prove fini/cleanup remain separate.

## Milestone 10 — C-speed physical proof

### `OWN-HOME-C-SPEED0-G0`

For a selected exact front:

- collect perf top report before code changes;
- inspect assembly at the hot symbol;
- prove Unique alias/call/return adds no RC, control-cell, handle-registry, or
  Box birth work;
- compare exact-front instructions and whole-program behavior;
- keep only evidence-backed representation changes.

Grammar completion is not a performance gate.

## Milestone 11 — readiness and retirement

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

This row is mandatory after the first production Home slice and again after
the final profile cutover. It is a documentation contract, not a grammar or
lowering shortcut.

Update the normative and derived reference surfaces from provisional/parked
language to the exact implementation that actually landed:

* `docs/reference/language/ownership.md` — Home/Handle rules, accepted
  `take`/`share` surface, destination-side Home demand, rejected forms,
  diagnostics, and the exact profile/fallback policy;
* `docs/reference/language/EBNF.md` and its grammar registry — only the
  parser-live contextual forms, with examples for ordinary handle calls and
  explicit `share`;
* `docs/reference/language/README.md`, variables/scope, lifecycle, cleanup,
  and constructor/birth references — ownership, `fini()`, cleanup, `new`,
  field initializer, `birth`, and partial-construction boundaries must point
  to their separate owners;
* `docs/reference/ir/json_v0.md`, callable/interface/FFI ABI references, and
  generated support views — exact Home ABI/profile metadata, no body
  re-inference at a boundary, and no hidden strong-owner producer;
* active language workstream dashboards, examples, migration notes, and
  environment-variable documentation — no stale `move/view/owned/shared`
  target or SharedV1 retry claim remains presented as the live Home surface;
* historical proposals and parked design cards — label them as evidence and
  link the accepted reference page instead of silently rewriting history.

The closeout must compare the reference grammar with both parser registries,
the resolver/Home Flow verifier, callable ABI metadata, and the selected
physical route. A mismatch that reflects real implementation behavior reopens
the owning code row; documentation must not hide it as historical prose.

Required evidence:

```text
reference grammar == parser-live grammar                 = 1
reference Home ABI == sealed compiler product             = 1
accepted examples compile on the selected profile        = 1
rejected examples fail before Builder effects             = 1
old target/fallback claims left in live reference pages   = 0
ownership reference closeout before product completion    = 1
```

This row does not add `region`, field-take, consuming receivers, multi-anchor
views, generic/dynamic/FFI ownership, or any other parked capability. It also
does not make `fini()` a physical-free or transfer operation.

## Parked follow-ups

- `OWN-HOME-TAKE-EXPR0-D0`: `take place_expr` for lifetime narrowing/local
  renaming, with a real corpus consumer;
- `OWN-HOME-FIELD-TAKE0-D0`: field extraction, empty slot, replacement;
- `OWN-HOME-CONSUMING-RECEIVER0-D0`;
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
