---
Status: Active architecture task
Date: 2026-07-19
Scope: MirBuilder function-session, fact-publication, plan, and compatibility boundaries
Related:
  - src/mir/builder/README.md
  - docs/development/current/main/design/mirbuilder-authority-based-hako-migration-ssot.md
  - docs/development/current/main/investigations/mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
---

# MirBuilder clean-architecture consolidation

## Decision

MirBuilder's local authority, fail-fast, non-Clone proof, and atomic module
publication laws are retained. The next architecture program does not rewrite
them. It reduces the mutable world underneath them into this final boundary:

```text
VerifiedModuleSemantics
  -> VerifiedFunctionLoweringPlan
  -> FunctionLoweringSession
  -> VerifiedMirFunctionDraft
  -> ModulePublicationTransaction
```

The decisive target is not a smaller Rust file. It is fewer truth owners,
fewer completion entries, and a function session that cannot mutate another
function's state.

This card is selected after
`RAW-PLAN-EMISSION-PARITY0-CARRIER-TYPE-D0` chose the representation-
precondition rebase. It does not repair the actual untyped NumericProgression
carrier: the R0 rejection remains the correct source-faithful diagnostic, and
its positive raw-primary proof remains blocked. This program instead supplies
the next BoxShape boundary before any wider callable-result representation
producer may be considered.

Its sole first code-facing row is:

```text
MIRBUILDER-CLEAN0-FSESSION0-CENSUS0
```

## Baseline finding

The existing `CanonicalFunctionLoweringSessionV1` is the correct lifecycle
facade, but its implementation temporarily turns one `MirBuilder` into another
function's builder and later restores it.

`LoweringContext` currently owns 29 `saved_*` fields plus one mode bit.
`ScopeStacksSnapshot` contains another seven state surfaces. They span:

```text
current function and block
ValueId-local type/origin facts
variables and bindings
resolved Binding SSA authority
lexical / loop / If / debug / fastmem stacks
pending PHIs and LocalSSA caches
slot, pin, record-local, and frag state
return/cleanup policy
fallback and recursion flags
source span and observer regions
function body AST and current static box
```

The snapshot is carefully restored on success, typed error, and panic. That is
a strong safety mechanism, but every new state field still requires a manual
share/snapshot/clear/restore decision. Context packaging alone has therefore
not yet produced function isolation.

## Final responsibility split

### `VerifiedModuleSemantics`

Owns immutable, branded module truth:

```text
declarations and canonical callable catalog
bindings and source-site identity
field / record / enum declarations
call targets, result dispositions, and representation facts
imports and admitted plugin ABI
lowering configuration fixed at session creation
```

It does not own function-local `ValueId`, caches, route retries, or mutable
compatibility state.

### `VerifiedFunctionLoweringPlan`

Owns one complete pre-emission function decision:

```text
structured control-flow plan
symbolic values and blocks
exact source provenance
resolved operation and call disposition
representation requirements
binding / join / exit intent
```

It contains no `MirBuilder`, real `ValueId`, `BasicBlockId`, cloned AST, raw
runtime tag, or fallback authority.

### `FunctionLoweringSession`

Owns only one function's mutable materialization state:

```text
unpublished MirFunction draft
function-local ID allocators
CFG and SSA construction state
monotone value facts
diagnostics and source observations
```

It never changes identity into a child function. Nested functions create a
new session over the same immutable module environment.

### `ModulePublicationTransaction`

Retains the existing unpublished-draft and atomic complete-set publication
law. Architecture cleanup must not weaken it.

## Program order

```text
MIRBUILDER-CLEAN0-D0
  this decision and task order

MIRBUILDER-CLEAN0-FSESSION0
  function-local state isolation

MIRBUILDER-CLEAN0-FACT0
  monotone type/kind/origin publication

MIRBUILDER-CLEAN0-PHI0
  one PHI completion state machine

MIRBUILDER-CLEAN0-FINALIZE0
  retire finalization-time correctness repair

MIRBUILDER-CLEAN0-PLAN0
  pure symbolic function-plan boundary

MIRBUILDER-CLEAN0-COMPCTX0
  module truth / cache / legacy-state split

MIRBUILDER-CLEAN0-CONFIG0
  invocation-sealed lowering configuration

MIRBUILDER-CLEAN0-RAWADAPT0
  raw compatibility adapter outside the emitter
```

Each macro row is independently selected and landed. No row may combine
BoxShape cleanup with a new accepted source shape, backend capability,
ownership operation, or runtime behavior.

## Phase 1 — FSESSION0

### `FSESSION0-CENSUS0` — closed (2026-07-19)

Add one generated or machine-readable inventory for every state surface
currently touched by function-session prepare/restore.

Each row has exactly one classification:

```text
ModuleImmutable
ModulePublication
FunctionOwned
ObservationBorrow
LegacyCompatibility
```

Required guards:

```text
unclassified snapshot fields = 0
state field in two ownership classes = 0
new MirBuilder fields absent from inventory = 0
behavior delta = 0
```

The inventory is diagnostic structure, not a new runtime state or semantic
authority.

Delivered evidence:

```text
fixture:
  tools/checks/fixtures/mirbuilder_fsession_census_v1.json

validator:
  python3 tools/checks/lib/mirbuilder_fsession_census.py

sealed current inventory:
  41 expanded prepare/restore leaf surfaces
  27 direct MirBuilder fields accounted for
  2 uncovered ValueId-keyed metadata surfaces recorded as gaps
```

The 41 rows expand the two aggregate snapshot products rather than treating
them as opaque: `saved_type_ctx` contributes all six TypeContext maps and
`saved_scope_stacks` contributes all seven scope leaves. Each row has one
ownership class, current prepare/restore anchors, and an exact BoxCompilation
Context action. In particular, `string_literals`, `map_value_types`, and
`map_literal_value_types` are FunctionOwned facts but are currently neither
saved nor cleared by the BoxCompilationContext branch. Census records that
gap; it does not repair it.

`metadata_ctx.value_origin_spans` and `value_origin_callers` are likewise
FunctionOwned ValueId-keyed state outside the current snapshot. They remain
explicit FSESSION0-S0 inputs rather than being silently classified as module
truth. Shared `core_ctx` allocation is recorded as existing legacy-compatible
storage through the direct Builder-field manifest; CENSUS0 makes no claim that
it is already session-isolated.

Next code-facing row:

```text
MIRBUILDER-CLEAN0-FSESSION0-S0
```

### `FSESSION0-S0` — one physical function-state owner

Introduce one private `FunctionLoweringStateV1` and move only the exact
`FunctionOwned` surfaces into it. Existing methods may delegate through a thin
facade during this row.

```text
duplicated field storage = 0
new accepted shape = 0
snapshot behavior delta = 0
```

This is a Refactor Series Mode task. It may use a short buildable series, but
the series has one purpose and keeps behavior unchanged until cutover.

#### Selected S0 implementation series

`CENSUS0` proves that `ScopeContext`, `CompilationContext`, and
`MetadataContext` mix FunctionOwned leaves with other lifetimes. S0 must not
move any whole context into the function state. The selected buildable series
is:

```text
S0a  expand existing function-session success/error/panic parity witnesses for
     every captured FunctionOwned surface; introduce private state component
     vocabulary only, with no production storage cutover; keep metadata gaps
     as explicit unhealed controls

S0b  one physical storage cutover: split only FunctionOwned leaves from the
     three mixed contexts, make private FunctionLoweringStateV1 own every
     FunctionOwned surface, mechanically update direct accesses, and remove
     old fields; no Deref facade and no fresh-session API

S0c  replace the FunctionOwned saved_* set with one move-only saved state
     transaction; LegacyCompatibility and ObservationBorrow snapshots remain
     separate and the BoxCompilationContext partial action is preserved

S0d  add structural old-storage-zero guards plus focused session parity and
     release-build closeout
```

`S0a -> S0b -> S0c -> S0d` is one Refactor Series Mode purpose. Every commit
must build, preserve visible MIR/session behavior, and add no accepted source
shape, type inference, backfill, fallback, or retry.

#### Selected S0a execution hand

The S0 census and three independent implementation/test audits select the
following buildable internal sequence. S0a deliberately fixes the observable
contract before moving physical storage: S0b is a broad mechanical cutover and
must not become the first place where a lost function-local surface is noticed.

```text
S0a-T0  expand existing function-session witnesses
S0a-V0  add private component vocabulary only
S0a-G0  run census/structural guards and hand S0b one frozen surface list
```

`S0a-T0` is the sole next code-facing hand. It makes no production storage
change. It extends the existing success, typed-error, and panic session tests
to observe the current behavior of every *captured* FunctionOwned surface:

```text
all six TypeContext maps
variable and exact BindingId state
resolved Binding SSA install state
function/block and lexical/loop/If/parameter/fastmem stacks
pending PHI, LocalSSA, schedule, pin, reservation, cleanup, and fallback state
record-local scratch state
FragEmitSession reset/seal observation through one test-only query
```

The same witnesses must prove the existing child-entry reset behavior and
outer-state restoration after success, typed error, and panic. They do not
claim a fresh child-state object, address separation, or repaired child
isolation; those are C0 concerns.

The two Census0 metadata gaps, `value_origin_spans` and
`value_origin_callers`, remain explicit *unhealed controls* in S0a. They are
not folded into the restoration-parity assertion, treated as module truth, or
repaired through finalization. A focused diagnostic/control may demonstrate
the current gap, but S0a must not alter it.

`S0a-V0` may introduce private, storage-free vocabulary only:

```text
FunctionLoweringStateV1
FunctionScopeStateV1
FunctionCompilationScratchV1
FunctionValueOriginFactsV1
```

Their fields and constructors stay private, `MirBuilder` gains no
`function_state` field yet, and no `Deref`/`DerefMut` compatibility facade is
permitted. This preserves Rust field-disjoint mutable borrows for the S0b
mechanical cutover.

`S0a-G0` freezes the S0b input list: the current Census0 checker remains green,
the expanded session witnesses are green, and the direct-access inventory must
still identify every FunctionOwned use. No S0a commit may move a whole
`ScopeContext`, `CompilationContext`, or `MetadataContext`.

The partial BoxCompilationContext map handling is an existing behavior that S0
must preserve, but it is not a future semantic authority. S0d models its
exact current action through one move-only FunctionLoweringState transaction:

```text
clear:
  value_types
  value_kinds
  value_origin_newbox

retain across the BoxCompilationContext branch:
  string_literals
  map_value_types
  map_literal_value_types
```

The retained maps remain fields of that one state; no compatibility sidecar,
second map, module publication, or finalization repair is allowed. Their
isolation repair needs a later explicitly selected row. The metadata origin
maps move with the same FunctionOwned state, but S0 makes no claim that their
current cross-session handling is repaired.

Forbidden in S0:

```text
whole ScopeContext / CompilationContext / MetadataContext moves
duplicate old-and-new FunctionOwned storage or Deref aliasing
fresh child-session construction or address-separation claims (C0 only)
new source acceptance, type inference, type backfill, fallback, or retry
repairing the BoxCompilationContext partial map action in the S0 series
```

### `FSESSION0-C0` — fresh child-session construction

Construct a fresh function state instead of clearing a parent state in place.
The child may borrow immutable module truth and observation sinks only.

Required proof:

```text
parent and child mutable state addresses differ
same numeric ValueId cannot consult parent facts
success restores no child-owned state into parent
typed error and panic publish no draft
fresh session remains reusable
```

### `FSESSION0-CUT0` — snapshot retirement

Atomically switch canonical and legacy function entry facades to the fresh
session owner and remove the function-owned `saved_*` fields.

The lifecycle facade may remain, but it becomes a session constructor and
draft closer rather than a whole-builder transformer.

### `FSESSION0-G0`

```text
function-owned snapshot/restore rows = 0
manual prepare/restore caller pairs = 0
one function-state constructor = 1
one function-state close owner = 1
module publication transaction delta = 0
```

## Phase 2 — FACT0

Replace direct mutable fact-map publication with one monotone API. The first
slice is type-only; kind and origin follow only after type parity is closed.

```rust
Unknown -> Exact(T)       // publish
Exact(T) -> Exact(T)      // idempotent
Exact(T) -> Exact(U)      // typed conflict
Exact(T) -> Unknown       // forbidden
```

Producer APIs identify their evidence site. Consumers are read-only and may
never repair facts.

Task order:

```text
FACT0-S0  pure decision + typed conflict vocabulary, consumers 0
FACT0-P0  parameter/Copy/Phi/Call/FieldGet publication matrix
FACT0-I0  type publication cutover
FACT0-G0  raw type-map writers zero outside the owner
```

No general type inference, union rejection, origin widening, or final-metadata
fallback is introduced.

## Phase 3 — PHI0

One semantic PHI completion operation replaces entry-specific policy.

```text
PhiDraft
  -> validate exact predecessor/input rows
  -> decide type fact
  -> prepare candidate mutation
  -> commit instruction
  -> commit type fact
  -> CompletedPhi
```

Raw emission, complete insert, provisional patch, and batch become facades
over the same completion owner. Provisional definition remains incomplete and
publishes no unanimous type. Function-level APIs without the active session do
not gain lowering-time publication authority.

```text
PHI completion decision owners = 1
entry-specific type policies = 0
failed single completion partial publication = 0
failed batch partial publication = 0
```

## Phase 4 — FINALIZE0

Finalization becomes a verifier and derived-publication boundary, not the
first producer of facts required during lowering.

Inventory every finalization pass as exactly one of:

```text
VerifyCompletedDraft
NormalizeRepresentation
PublishDerivedArtifact
RepairMissingLoweringFact
LegacySemanticInference
```

The last two classifications must reach zero. A required type, origin, call
disposition, or source identity fact is published by its lowering-time producer
before the dependent instruction is emitted, or lowering fails there.

Task order:

```text
FINALIZE0-CENSUS0
  deterministic pass inventory and first-publication sites

FINALIZE0-P0
  lowering-time versus finalized fact parity matrix

FINALIZE0-CUT0
  remove correctness repair and MIR-to-source semantic inference

FINALIZE0-G0
  repair and legacy semantic inference counts zero
```

Allowed finalization work remains explicit:

```text
whole-draft verification
representation-preserving normalization
metadata snapshot from already-sealed facts
backend-neutral derived artifact publication
```

Forbidden:

```text
making an earlier FieldGet/Call/Phi valid after the fact
inferring source target or field owner from emitted MIR spelling/order
running the lowering fact pipeline again to hide producer timing drift
```

## Phase 5 — PLAN0

Move plan construction before Builder mutation. The first pilot is the already
bounded `GenericLoopV1` family; it does not widen Loop grammar.

```text
exact verified source inputs
  -> symbolic GenericLoop plan
  -> existing plan verifier
  -> FunctionLoweringSession materialization adapter
```

Plan producers use `PlanValueId` / `SymbolicBlockId`, not real allocation
order. Route selection, source semantic decisions, and exact call dispositions
finish before the first MIR mutation.

```text
&mut MirBuilder parameters in selected plan producer = 0
Builder mutation before completed plan seal = 0
plan failure Builder delta = 0
source/evaluation order conflation = 0
```

Facts, Recipe, located representation, CorePlan, and Parts are not renamed or
merged mechanically. A representation is retired only after all of its unique
authority has one proven destination.

`RecipeBody` may remain as a compatibility input during the bounded adapter
cutover, but cloned AST is never source identity or semantic truth. PLAN0 owns
an explicit retirement subrow:

```text
PLAN0-RECIPE-RET0
  unique RecipeBody authority projected into the verified plan
  RecipeBody cloned-AST semantic reads after plan seal = 0
  source identity reconstructed from cloned AST = 0
```

## Phase 6 — COMPCTX0

Split `CompilationContext` by lifetime and trust level:

```text
VerifiedModuleEnvironment
  immutable declaration and admitted ABI truth

ModuleLoweringCaches
  correctness-independent memo only

LegacyCompatibilityState
  current_static_box, method-tail indexes, field-origin heuristics,
  and other explicitly retiring compatibility state
```

Visibility prevents canonical lowering from reading legacy compatibility
state. A cache miss may affect performance only, never acceptance or emitted
meaning.

## Phase 7 — CONFIG0

Capture process/environment configuration once, before module and function
sessions begin.

```rust
struct LoweringConfig {
    planner_mode: PlannerMode,
    diagnostics: DiagnosticMode,
    compatibility: CompatibilityMode,
}
```

Task order:

```text
CONFIG0-CENSUS0
  all environment reads reachable from semantic resolution, planning,
  function lowering, finalization, and publication

CONFIG0-S0
  immutable parsed config with typed invalid-value errors

CONFIG0-I0
  session consumers use borrowed config only

CONFIG0-G0
  process environment reads after session entry = 0
```

Diagnostic environment toggles that are explicitly outside semantic behavior
may remain only behind the repository debug contract. They cannot change route
selection, accepted syntax, representation, emitted MIR, or fallback behavior.

## Phase 8 — RAWADAPT0

Legacy AST and canonical semantic inputs both terminate at the same verified
plan boundary:

```text
legacy AST adapter ---------+
                            +-> VerifiedFunctionLoweringPlan
canonical semantic input ---+             |
                                           v
                               FunctionLoweringSession
```

The emitter accepts plans only. Raw/located mode flags, source-name recovery,
and compatibility fallback are absent from the function session.

## Retirement ledger

The program is not complete merely because the new products exist. These old
structures have explicit retirement owners and zero conditions:

| Retiring structure | Owning row | Completion condition |
| --- | --- | --- |
| giant cross-function `MirBuilder` mutable world | `FSESSION0` + `COMPCTX0` | function-owned mutable state exists only in one fresh session; module truth is immutable |
| nested-function snapshot/restore | `FSESSION0-CUT0` | function-owned `saved_*` rows and parent-state clear/restore are zero |
| planner `&mut MirBuilder` access | `PLAN0` | selected plan producers accept verified inputs and symbolic IDs only |
| `RecipeBody` cloned-AST authority | `PLAN0-RECIPE-RET0` | semantic/source-identity reads from cloned recipe AST after plan seal are zero |
| raw/located dual lowering | `RAWADAPT0` | both inputs terminate at one verified plan and one emitter |
| Builder-internal fallback | `RAWADAPT0` | selected canonical failure has no raw/alternate retry path |
| entry-specific PHI completion | `PHI0` | all complete/patch/batch/raw facades consume one completion owner |
| public mutable `TypeContext` maps | `FACT0-G0` | external raw writes are zero; producers use monotone publication APIs |
| finalization-time type/correctness repair | `FINALIZE0-CUT0` | facts needed during lowering are never first published in finalization |
| MIR scan used for source-semantic inference | `FINALIZE0-CUT0` + `RAWADAPT0` | emitted MIR spelling/order is not source target, field owner, or route authority |
| mid-process environment reads | `CONFIG0-G0` | semantic/config reads after module-session entry are zero |

Temporary compatibility code is not considered retired merely because its
production count happens to be zero in one fixture. The owning row must remove
the authority or isolate it behind a typed legacy adapter with an explicit
later removal token.

## Architecture budget

These rules apply immediately to new architecture work and become source
guards as the relevant owners land:

1. Add no unclassified field directly to `MirBuilder`.
2. Make no new source-semantic decision after the first MIR mutation.
3. Give one semantic operation exactly one completion owner.
4. If a feature needs more than one new bridge product, stop and evaluate
   whether an existing representation should be retired.
5. Move guard-only invariants into visibility, lifetime, or typestate whenever
   the owning row can do so.
6. Treat 800 lines as a hygiene limit, not the architecture metric.

Progress counters are instead:

```text
mutable surfaces owned by one function session
fact producer count per fact kind
completion owner count per semantic operation
authorities touched by one feature change
legacy compatibility reads from canonical lowering
```

## Preserved authorities

The consolidation must retain:

```text
SourcePathV1 / SourceStmtSiteV1 / SourceExprSiteV1
PATH0 child-role vocabulary
canonical callable keys and resolved refs
target/result/source-site evidence rows
activation selected/unselected law
caller ledger and single-use claim batches
callable graph inventory and SCC partition
unpublished function drafts and atomic module publication
typed fail-fast diagnostics
```

## Non-claims

```text
big-bang MirBuilder rewrite
new source grammar or accepted control-flow shape
all Facts/Recipe/CorePlan/Parts immediately merged
Hako adoption merely from Rust restructuring
general type inference or effect inference
backend/runtime/ownership widening
automatic performance improvement
removal of semantic-reference execution
```

## Stop conditions

Stop the selected architecture row if it requires:

1. moving immutable module truth into the mutable function session;
2. retaining parent and child function state in one mutable object after
   `FSESSION0-CUT0`;
3. duplicating a fact map or completion policy during cutover;
4. deriving missing types in a consumer;
5. accepting a new source shape to prove a structural refactor;
6. AST equality, name parsing, runtime tags, or final metadata as source truth;
7. raw fallback or route retry after canonical selection;
8. mixing FactStore, PHI lifecycle, and pure-plan cutovers in one commit;
9. weakening unpublished-draft or atomic-module publication;
10. touching a source/check file at or above 800 lines.

## Selection boundary

This architecture program is selected after the `CARRIER-TYPE-D0`
representation-precondition decision and before another callable/control-flow
representation widening. `CALLABLE-RESULT-NESTED-REP0` remains parked until
the census and its selected clean-architecture program establish the next
function-session boundary.

No stash is an implementation authority. Every row starts from the clean tree
and reuses only landed contracts and evidence.

## Final lock

> MirBuilder clean architecture is a staged BoxShape program, not a rewrite.
> It first converts the current whole-builder snapshot transaction into one
> fresh function-local state owner, then centralizes monotone value-fact
> publication and PHI completion, then places a pure symbolic verified plan
> before MIR mutation, retires finalization-time correctness repair, separates
> immutable module truth, caches, and retiring compatibility state, seals
> process configuration at invocation entry, and finally moves raw adaptation
> outside the emitter. Existing source identity, callable authority, claim ledgers,
> typed fail-fast, unpublished drafts, and atomic publication remain intact.
> The selected first code-facing row is
> `MIRBUILDER-CLEAN0-FSESSION0-CENSUS0`; it changes no behavior and classifies
> every function-session state surface before physical movement.
