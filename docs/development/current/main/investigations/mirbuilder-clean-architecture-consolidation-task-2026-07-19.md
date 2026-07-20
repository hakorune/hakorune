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

The closed FSESSION0 and FACT0 preparation rows have established the
function-state census, one receiver-only monotone publication consumer, and
the complete 47-path / 99-occurrence writer partition. PHI0-S0 is also
closed. PHI0-M0 has now closed as an evidence row and found that I0 has no
single existing predecessor-readiness authority. PRED0-D0 therefore selects a
split between generic input/type completion and route-scoped CFG readiness.
PRED0-S0, PRED0-P0, I0, and CFGREADY0-D0 are closed; the next disconnected
one-consumer integration row is now:

```text
MIRBUILDER-CLEAN0-PHI0-CFGREADY0-I0
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

FSESSION0-C0-D0
  audit the fresh-session borrow boundary only; no fresh-session cutover

MIRBUILDER-CLEAN0-FACT0
  monotone type/kind/origin publication

MIRBUILDER-CLEAN0-PHI0
  one PHI completion state machine

MIRBUILDER-CLEAN0-COMPCTX0
  module truth / cache / legacy-state split

MIRBUILDER-CLEAN0-CONFIG0
  invocation-sealed lowering configuration

MIRBUILDER-CLEAN0-FINALIZE0
  retire finalization-time correctness repair

MIRBUILDER-CLEAN0-METAPROP0
  retire the multi-fact propagation facade before plan cutover

MIRBUILDER-CLEAN0-PLAN0
  pure symbolic function-plan boundary

MIRBUILDER-CLEAN0-RAWADAPT0
  raw compatibility adapter outside the emitter

FSESSION0-C0-I0 -> FSESSION0-CUT0 -> FSESSION0-G0
  activate the fresh session and retire snapshot transformation only after
  the preceding fact, plan, context, configuration, and raw-adapter owners
  are green
```

Each macro row is independently selected and landed. No row may combine
BoxShape cleanup with a new accepted source shape, backend capability,
ownership operation, or runtime behavior.

### Execution dependency refinement

The macro list is a responsibility map, not permission to cut over a fresh
function session immediately after the S0 series. The current concrete order
is fixed as:

```text
FSESSION0-S0c-I0 -> S0c-G0 -> S0d
  -> FSESSION0-C0-D0
  -> FACT0-I0-RCV0 -> FACT0-P1-PARTITION0
  -> PHI0 -> FACT0-I1-*
  -> COMPCTX0 -> CONFIG0 -> FINALIZE0
  -> METAPROP0-D0 -> METAPROP0-CUT0
  -> PLAN0 -> PLAN0-RECIPE-RET0 -> RAWADAPT0
  -> FACT0-G0
  -> FSESSION0-C0-I0 -> FSESSION0-CUT0 -> FSESSION0-G0
```

`FSESSION0-C0-D0` may audit only the selected borrow contract:

```text
child session borrows immutable module truth and observation sinks only
child session borrows or snapshots mutable LegacyCompatibility state = 0
```

It must not activate a fresh child session. The existing mutable compatibility
state, direct fact-map writers, finalization repair, real-`ValueId` planning,
and raw/located emitter split would otherwise make the C0 law false rather
than prove it. Each prerequisite remains BoxShape-only: it changes no accepted
source shape, backend capability, ownership operation, runtime behavior,
fallback, or retry policy.

### `FSESSION0-C0-D0` — closed (2026-07-19)

Three independent read-only audits agree that a fresh child session is not yet
admissible. This row adds no session constructor, Builder consumer, test
fixture, or production behavior. It fixes the future borrow contract and parks
`C0-I0` until its already-selected prerequisite rows close.

The future session may own only its unpublished draft, function-local ID/CFG/
SSA state, monotone facts, and child-local diagnostics. It may borrow only:

| Future input | Required owner | Borrow law | Prerequisite |
| --- | --- | --- | --- |
| declaration / ABI / callable truth | frozen `VerifiedModuleEnvironment` | immutable only | `COMPCTX0` |
| route and diagnostic configuration | invocation-sealed `LoweringConfig` | fixed at session entry | `CONFIG0` |
| observations | non-semantic `ObservationSink` capability | no parent stack take/restore | `FSESSION0-C0-I0` proof |
| completed draft publication | existing `ModulePublicationTransaction` | child returns a closed draft; child publishes nothing | `FSESSION0-C0-I0` proof |

The following are explicit non-authorities for the fresh child: `&mut
MirBuilder`, mutable `CompilationContext`, shared `CoreContext` allocation,
`current_module`, `current_static_box`, Box mode, field-origin and method-tail
heuristics, direct `TypeContext` maps, raw AST/RecipeBody or raw/located mode,
environment reads after entry, finalized metadata, finalization repair, and
source inference from emitted MIR.

The evidence is concrete: GenericLoop planning still accepts `&mut MirBuilder`
and mutates bindings/body lowering; direct type maps and finalization repair
remain live; raw and located lowering still co-reside; configuration is still
read during route selection; and current lifecycle plus static-box paths still
snapshot/restore parent compatibility and observation state. `CoreContext` also
remains a shared mutable allocator, and current session close publishes through
the parent builder. None can be smuggled into C0 as a borrowed capability.

`FSESSION0-C0-I0` is forbidden until all of these are true:

```text
frozen VerifiedModuleEnvironment exists before child creation
LoweringConfig is fixed before child creation
child owns fresh state and function-local allocation; parent facts are unreachable
child observation is child-owned or a non-restoring sink
child returns a closed draft; one parent transaction alone publishes it
FACT0 -> PHI0 -> FINALIZE0 -> PLAN0 -> PLAN0-RECIPE-RET0
  -> COMPCTX0 -> CONFIG0 -> RAWADAPT0 are closed
```

Failure law for the future cutover is also fixed: typed, cleanup, publication,
and panic/Drop failures discard the child draft/state and publish nothing;
they never restore or mutate parent function-owned state. A fresh second
session remains usable. The first next code-facing row is therefore
`MIRBUILDER-CLEAN0-FACT0-S0`, not C0 implementation.

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

`S0a-T0` is closed (2026-07-19). It made no production storage change. It
extends the existing success, typed-error, and panic session tests
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

Delivered T0 evidence:

```text
legacy child entry:
  every TypeContext map, BindingId, resolved binding state, record scratch,
  and sealed Frag state is reset before the callback

legacy close:
  success, every injected typed-error checkpoint, and panic restore the outer
  captured state; child-written map facts do not leak back

BoxCompilationContext:
  value_types/value_kinds/value_origin_newbox clear at entry and close
  string_literals/map_value_types/map_literal_value_types retain
```

`S0a-V0` is closed (2026-07-19). It adds exactly one builder-private,
fieldful `function_lowering_state` vocabulary and no live storage. The module
contains `FunctionLoweringStateV1`, `FunctionScopeStateV1`,
`FunctionCompilationScratchV1`, and `FunctionValueOriginFactsV1`; it has no
constructor/accessor/session API, no `Clone`/`Copy`/`Deref` facade, and
`MirBuilder` still has no `function_state` field or consumer. The existing
census checker now verifies that exact four-component partition and rejects a
premature Builder installation or public compatibility surface. Its focused
default-state unit test, the expanded function-session witnesses, the census,
and all-target build are green. The next code-facing hand is `S0a-G0`.

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

#### Selected S0a-G0 access-route freeze

The S0a-G0 worker census selects one adjacent, read-only direct-access guard.
Census0 remains the one lifecycle-owner/destination authority: its fixture
defines the FunctionOwned selector-to-`FunctionLoweringStateV1` route map and
the current old-storage leaves. The adjacent guard consumes that route map and
owns only occurrence evidence, so it is not a second state authority. Its
pre-cutover snapshot records each old FunctionOwned *access route* with its
future private state destination:

```text
selector                 old route                         S0b destination
current block            current_block                     function_state.current_block
type / variable / binding direct context fields            function_state.*
scope leaves              scope_ctx FunctionOwned leaves   function_state.scope.*
compilation leaves        comp_ctx FunctionOwned leaves    function_state.compilation.*
metadata origins          MetadataContext origin APIs      function_state.value_origins.*
SSA / cleanup / Frag      direct Builder fields             function_state.*
```

Each fixture row is an exact pre-cutover source observation:

```text
logical selector
old access spelling or context API family
new FunctionLoweringStateV1 destination
production or test domain
sorted source-file set
observed occurrence count
```

The checker scans `src/**/*.rs`, strips comments/strings, and records only
the bounded Builder receiver forms selected by the P0 receiver-owner grammar.
It rejects Census/scanner selector drift; it does not claim to infer ownership
through arbitrary local aliases. It also inventories stateful mixed-context
APIs, not just raw fields:

```text
ScopeContext function-entry / stack helpers
CompilationContext reservation and record-local helpers
MetadataContext value-origin span/caller record/query/merge helpers
```

Line numbers are deliberately not authority: mechanical cutover changes them.
The checked sorted file set and count are the pre-cutover witness. The new
`function_lowering_state` module itself is excluded from old-route scanning.
I0's first broad observation is not S0b authority: P0 must replace it with a
lexically owner-verified snapshot before any physical cutover.

The selected task order is:

```text
S0a-G0-I0
  add the passive selector/destination grammar and old-storage assertion to
  Census0; add one adjacent checker with a generated checked-in occurrence
  snapshot; production behavior = 0

S0a-G0-P0
  prove source scan parity for production and test domains, every mixed-context
  API family, the two metadata-origin gaps, and all S0a session witnesses

S0a-G0-G0
  freeze the inventory as S0b's sole old-route input; census, focused tests,
  all-target build, pointer, format, diff, and line checks must be green

S0b-D0
  only then select the one physical cutover implementation order
```

S0b may not widen `FunctionLoweringStateV1` visibility to satisfy external
consumers. It replaces those consumers with narrow `MirBuilder` query methods.
`ScopeContext::clear_for_function_entry()` currently clears both movable scope
leaves and `debug_scope_stack`, so S0b must split that behavior deliberately.
Likewise, it moves only metadata origin maps, preserves their current
unhealed no-snapshot/no-clear behavior, and leaves span/region observation
state in `MetadataContext`. S0c, not S0b, owns replacing the legacy individual
`saved_*` lifecycle transaction.

`S0a-G0-I0` is closed (2026-07-19). Census0 now owns 32 exact old-storage to
future-state routes, including the separate mixed `scope.entry_clear` route.
It confirms every old FunctionOwned storage leaf remains physically present
through S0a. The adjacent direct-access guard consumes that map and freezes 64
selector/domain rows: 1,787 occurrences total, split into 1,188 production and
599 test occurrences. Its bounded Rust lexer ignores comments, normal/raw/byte
strings, and character literals; its receiver grammar covers `self`, `builder`,
`self.builder`, `b`, `self.0`, and wrapper `.builder` forms. The generated
snapshot is an observation only: no Builder storage, API, source acceptance, or
type/semantic behavior changed. `S0a-G0-P0` is next.

`S0a-G0-P0-D0` is now selected before its proof implementation. The I0 scanner
found syntactic receiver forms but did not prove that bare `self` belonged to
`MirBuilder`: it counted seven `LoopFormJsonOps::current_block` uses in the
JSON-v0 bridge and one unrelated parity-result use. P0 therefore replaces only
the observation mechanics, never compiler behavior. `P0-S0` is closed: its
bounded lexical owner proof removes those eight false positives, fixes the
snapshot to 96 selector/domain rows and 1,776 observations (1,175 production,
599 test, 2 shared), and reduces the checker's runtime from about 27 seconds
to about 7 seconds. Its source grammar is bounded and fail-closed:

```text
self.<route>:
  exact lexical impl MirBuilder only

builder.<route> / b.<route>:
  exact &MirBuilder or &mut MirBuilder parameter, direct MirBuilder birth,
  checked MirBuilder-returning factory, selected function-session closure, or
  finite inline array iteration whose every root is already one of those forms

self.builder.<route> / self.0.<route> / wrapper.builder.<route>:
  exact checked-in wrapper-owner contract only

unknown local alias or unknown wrapper:
  reject observation; do not count it
```

P0 also replaces the old regex-only `cfg(...test...)` split. `cfg(test)` and
`cfg(all(test, ...))` are test-only, `cfg(not(test))` is production-only, and
mixed or otherwise non-decidable predicates are recorded as a separate shared
source domain rather than silently dropped from production. This is a bounded
source classifier, not a Rust cfg evaluator or alias data-flow analysis.

The fixed P0 order is:

```text
P0-S0
  lexical receiver-owner and three-domain source observation;
  regenerate the disposable snapshot; behavior = 0

P0-R0
  mixed API owner manifest and metadata-gap/session controls;
  behavior = 0

P0-P0
  scanner self-tests plus focused existing-lifecycle witnesses;
  no metadata isolation repair

P0-G0
  one Census route map, one corrected snapshot, all session and source guards green
```

`P0-R0` is closed (2026-07-19). Census remains the sole selector-to-destination
authority. Its nine-row mixed-context API owner manifest records the exact
method-definition owners for lexical, If-merge, FastMemory, entry-clear,
reservation, legacy-body, record-local, origin-span, and origin-caller APIs;
it deliberately records no second old-storage or destination map. The checker
also proves that `scope.entry_clear` clears exactly its five FunctionOwned
scope leaves plus `debug_scope_stack`, while `function_param_names` remains a
prepare-time `take`, not an entry-clear member.

The expanded session witness confirms that child entry clears all captured
scope, reservation, slot-registry, and outer-body state before installing its
own empty legacy body capture. It also fixes the two metadata origin maps as
the explicit no-isolation control: outer origin facts are visible to the child,
and child origin facts remain visible after caller restoration. No metadata
snapshot, clear, restore, publication, or runtime behavior changed. The direct
access snapshot now has 96 rows and 1,792 observations (1,175 production, 615
test, 2 shared). `P0-P0` is next.

`P0-P0` and `P0-G0` are closed (2026-07-19). The scanner's five focused
self-tests cover literal stripping, three-domain cfg partitioning, bounded
receiver ownership, mixed-context selector attribution, and Census/scanner
selector coupling. The session witness now covers the remaining mixed leaves
and both metadata-gap controls. Census, direct-access inventory, scanner
self-test, focused function-session test, all-target check, pointer guard,
format, diff, and changed-file line limits are green. The frozen S0b input is
therefore one Census route map, one 96-row/1,792-occurrence snapshot, and the
existing session parity suite. `S0b-D0` is next and is design-only: it must
select the physical cutover order before any storage moves.

#### `S0b-D0` — selected atomic physical cutover

`S0b-D0` is closed (2026-07-19). Candidate A, a multi-commit migration of
individual FunctionOwned storage groups, is rejected: it prolongs a hybrid
owner, makes the direct-access inventory ambiguous during the series, and
invites a second lifecycle policy while metadata origins are still deliberately
unhealed. Candidate B is selected:

```text
S0b-I0:
  one buildable mechanical commit
  installs exactly one private FunctionLoweringStateV1 in MirBuilder
  removes every old FunctionOwned storage leaf at the same cutover
  updates all 32 Census routes directly to their existing destinations
```

`S0b-I0` changes storage and access spelling only. The current individual
`LoweringContext.saved_*` prepare/restore transaction, canonical session
close/drop law, publication order, and panic backstop remain intact; S0c alone
may replace them with a move-only transaction, and C0 alone may construct a
fresh child state.

Mixed-context cutover law:

```text
ScopeContext retains only debug_scope_stack.
FunctionScopeStateV1 receives current_function plus lexical/loop/If/parameter/
fastmem leaves. Its entry clear has the existing five movable clears;
ScopeContext clears debug separately at the same lifecycle point.

CompilationContext retains declarations, catalogs, current_static_box,
current_slot_registry, and compatibility state. FunctionCompilationScratchV1
receives only reserved_value_ids, fn_body_ast, and record_local_values.

MetadataContext retains current span, source-file/hint, and region observation.
FunctionValueOriginFactsV1 receives origin span/caller maps and their narrow
record/query/merge APIs. It preserves the current no-snapshot/no-clear/no-
restore behavior; METAISO, not S0b, owns any isolation repair.
```

No `Deref`, whole-state accessor, public state/context/map exposure, old/new
mirror, whole mixed-context move, `saved_function_state`, or fresh-session API
is allowed. Builder-external consumers may use only new narrow `MirBuilder`
queries/actions that replace their old field access. The implementation order
is fixed:

```text
S0b-D0  closed decision
  -> S0b-I0  all 32-route physical storage cutover
  -> S0b-P0  post-cutover session, BoxCompilationContext, metadata-gap, and
              old-route-zero proof
  -> S0b-G0  full structural/format/build/pointer closeout
  -> S0c     saved-state transaction consolidation
```

`S0b-I0` must stop rather than broaden if any route needs a second owner,
metadata isolation repair, a context-wide move, an external state borrow, a
source/route/type-policy change, fallback, retry, or a lifecycle transaction
redesign.

P0 must not add a second route/destination authority, generic ownership
inference, arbitrary alias tracking, a semantic cfg evaluator, or a metadata
snapshot/clear/restore repair. `scope.entry_clear` stays the one explicit mixed
helper until S0b splits its movable scope clear from debug observation. Metadata
origin span/caller maps remain an explicit no-isolation control: P0 proves their
current visibility/retention behavior but does not repair it; `METAISO` owns any
future isolation change.

The partial BoxCompilationContext map handling is an existing behavior that S0
must preserve, but it is not a future semantic authority. S0c models its
exact current action through one move-only FunctionOwned transaction:

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

#### `S0b-I0 -> P0 -> G0` — closed (2026-07-19)

The atomic physical cutover is green. `MirBuilder` now has exactly one private
`FunctionLoweringStateV1`; all 32 Census routes use that sole owner, and no
retired direct FunctionOwned route remains in production, tests, or shared
source. `ScopeContext` is debug-only, `CompilationContext` retains only module,
observation, and compatibility state, and `MetadataContext` retains only span,
source-hint, and region observation. The individual `LoweringContext.saved_*`
transaction remains unchanged, and the origin facts remain deliberately outside
snapshot/clear/restore until METAISO.

The historical pre-cutover inventory is retained separately at
`tools/checks/fixtures/mirbuilder_fsession_direct_access_pre_s0b_v1.json`
(1,792 observed routes). The active direct-access guard is now an old-route-zero
proof: all 96 selector/domain rows are zero. Census also verifies the exact
four-component state partition, retired mixed-context leaves, separate movable
scope/debug clears, preserved BoxCompilationContext three-clear/three-retain
behavior, absence of `saved_function_state`, and no metadata-origin lifecycle
repair.

Closeout evidence:

```text
python3 tools/checks/lib/mirbuilder_fsession_census.py
python3 tools/checks/lib/mirbuilder_fsession_direct_access_inventory.py
python3 tools/checks/lib/mirbuilder_fsession_direct_access_inventory_tests.py
cargo test -q --lib function_session
cargo check --all-targets -q
bash tools/checks/current_state_pointer_guard.sh
cargo fmt --check
git diff --check
```

#### `FSESSION0-S0c-D0` — selected captured-subset transaction

`S0c-D0` is closed (2026-07-19). Three independent read-only inventories
agree that the next cleanup is not a fresh child session. It is one private,
move-only `FunctionOwnedStateTransactionV1` that captures only the existing
FunctionOwned leaves which the legacy lifecycle already restores.

Candidate A, `mem::take` of the whole `FunctionLoweringStateV1` followed by a
default child state, is rejected: it constructs a fresh child state and would
therefore preempt C0. It also conflates the deliberately retained metadata
origin facts with caller state.

Candidate B, a thin wrapper that preserves each existing
`LoweringContext.saved_*` field and `ScopeStacksSnapshot` as independently
captured public-to-the-lifecycle details, is rejected: it repackages the same
multiple capture authorities rather than establishing one state transition.

Candidate C is selected:

```text
MirBuilder.function_state
  -> FunctionOwnedStateTransactionV1
       -> Option<CapturedFunctionOwnedStateV1>
       -> one consume-and-restore transition
```

The transaction is builder-private, non-Clone, non-Copy, and non-Deref. Its
payload is a fieldwise move of the existing captured FunctionOwned subset;
it never swaps, takes, defaults, or installs the complete
`FunctionLoweringStateV1`.

Its exact capture law is:

```text
always capture and restore:
  current function/block
  Binding/ResolvedBinding authority
  FunctionScope movable leaves and function parameter names
  compilation scratch (reserved IDs, body AST, record locals)
  pending PHI / LocalSSA / schedule / pin caches
  Frag emit session
  return/cleanup/fallback flags

Legacy mode only:
  variable map and all six TypeContext maps move and restore

BoxCompilationContext mode:
  variable map and TypeContext are not captured
  value_types, value_kinds, value_origin_newbox clear on entry and close
  string_literals, map_value_types, map_literal_value_types retain
  record-local scratch is captured, child-cleared, then caller-restored

never capture, clear, or restore:
  value_origins.spans / value_origins.callers
```

The last row is the existing METAISO no-isolation control: child facts remain
visible and child-written facts remain retained. `current_static_box` and the
mode bit remain LegacyCompatibility; slot registry, debug scope, current span,
region stack, and recursion depth remain ObservationBorrow snapshots. Module
state, `core_ctx`, and module publication remain outside the transaction.

Close law:

```text
success:
  child draft is taken
  -> session validation
  -> transaction restores caller exactly once
  -> existing module publication owner runs

primary or cleanup error:
  child state/draft is discarded
  -> transaction restores caller exactly once
  -> existing combined error vocabulary is retained

panic / unclosed drop:
  same one-shot restore backstop
  -> no second panic while unwinding
```

This does not add a persistent Builder poison flag. A failure consumes the
transaction, discards the child state, and leaves the restored outer builder
reusable; it never retries the selected lowering route.

The fixed task order is:

```text
S0c-S0
  private transaction and capture-mode vocabulary; production consumers = 0

S0c-P0
  disconnected legacy/Box matrix and one-shot proof; the existing canonical
  Drop baseline remains unchanged until I0 connects the transaction

S0c-I0
  replace only the FunctionOwned `LoweringContext.saved_*` and
  `ScopeStacksSnapshot` routes with the transaction; non-FunctionOwned
  snapshots remain separate

S0c-G0
  transaction-owner/old-saved-route/parity guards, focused release closeout

S0d
  existing final structural old-storage-zero and series closeout
```

#### `S0c-S0` — closed (2026-07-19)

S0c-S0 adds one builder-private `FunctionOwnedStateTransactionV1`, its two
explicit capture modes, and one `CapturedFunctionOwnedStateV1` payload. It
has no production lifecycle consumer: the existing prepare/restore owner
remains active. The product has no `Clone`, `Copy`, `Deref`, whole-state
take/default/install, origin-fact field, or Legacy/Observation snapshot field.
Its disconnected begin/restore smoke verifies that the move-only caller slot
can be consumed once without changing emitted MIR or session behavior.

#### `S0c-P0` — closed (2026-07-19)

The transaction now has disconnected proof for both pre-existing modes. Legacy
mode moves the variable map, all six TypeContext maps, function/block,
Binding/SSA state, scope, compilation scratch, caches, and cleanup flags out
of the child and restores the outer values while discarding child writes. Box
mode proves exactly the inherited partial action: variable state and the three
cleared type maps do not survive, while the three retained type maps preserve
both outer and child facts. The product remains disconnected from production
prepare/restore.

The existing canonical-session suite remains green for its independent close
law: success, five typed-error checkpoints, cleanup-error composition, static
and instance publication after restoration, metadata-origin no-isolation, and
panic Drop restoration. P0 does not claim that transaction Drop itself can
restore a Builder; I0 must route the existing session Drop through its one
consume-and-restore transition.

#### `S0c-I0` — closed (2026-07-19)

The canonical lifecycle now begins one `FunctionOwnedStateTransactionV1` and
consumes it exactly once during restore. `LoweringContext` has exactly three
products: that opaque transaction, one LegacyCompatibility static-box
snapshot, and one ObservationBorrow snapshot for slot/debug/span/region/
recursion state. The 34 FunctionOwned individual `saved_*` captures,
`ScopeStacksSnapshot`, and canonical lifecycle `TypeContextSnapshot` use are
physically removed. Existing close, typed-error, cleanup-error, publication,
and Drop/panic ordering remain owned by `CanonicalFunctionLoweringSessionV1`.

`S0c-G0` proves, from the existing Census0 fixture rather than a second
surface table:

```text
captured FunctionOwned Census surfaces = 34
FunctionOwnedStateTransactionV1 definitions = 1
transaction begin owner = 1
transaction consume-and-restore owner = 1
LoweringContext FunctionOwned capture products = 1
individual captured FunctionOwned saved_* fields = 0
ScopeStacksSnapshot = 0
canonical lifecycle TypeContextSnapshot use = 0
transaction Clone / Copy / Deref = 0
whole FunctionLoweringState take/default/install = 0
metadata-origin transaction capture/clear/restore = 0
Box clear set = 3; retained set = 3
old direct FunctionOwned routes = 0
```

The existing legacy/Box success, five typed-error checkpoints, cleanup-error,
panic, post-restore publication, and metadata-origin control fixtures remain
the behavioral matrix. S0c adds no new route map: Census0 remains the one
surface authority and any sibling scanner may only consume it.

#### `S0c-G0` — closed (2026-07-19)

The existing Census fixture now also guards the one transaction begin/restore
owner, the exact three-field `LoweringContext`, retired FunctionOwned snapshot
vocabulary, METAISO non-interference, and the no-whole-state rule. Census,
old-route-zero inventory, inventory tests, transaction/session tests,
all-target check, release build, pointer guard, format, diff, and line-budget
checks are green.

#### `S0d` — closed (2026-07-19)

The same Census consumer now proves the captured FunctionOwned payload's exact
field set, the legacy two-field value payload, the two capture modes, and the
inherited BoxCompilationContext three-clear/three-retain law. No second
fixture, route map, scanner, or lifecycle test authority was added. The S0
Refactor Series is closed: local helper/static-box snapshots outside canonical
function lifecycle remain outside this scope. `FSESSION0-C0-D0` is next and is
an audit-only borrow-boundary decision; it must not activate fresh sessions.

S0c must stop rather than broaden if it needs:

```text
whole FunctionLoweringState take/default/install or child address separation
origin snapshot/clear/restore or any METAISO repair
Box three-clear/three-retain change, sidecar map, or finalization repair
LegacyCompatibility or ObservationBorrow state inside the transaction
the unrelated exprs.rs static-box snapshot family
source/type policy, MIR emission, fallback, retry, or module-publication change
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
FACT0-P0-INV0  direct transient-writer census
FACT0-P0-T0  parameter/Copy/Phi/Call/FieldGet temporal witness
FACT0-I0-RCV0  exact instance-receiver parameter cutover
FACT0-P1-PARTITION0  classify the remaining direct writers and fix the terminal gate scope
PHI0  one completion owner before any PHI writer migration
FACT0-I1-*  one remaining producer family per code-facing row
FACT0-G0  final global raw type-map writer convergence after every retirement prerequisite
```

No general type inference, union rejection, origin widening, or final-metadata
fallback is introduced.

`FACT0-G0` is a terminal macro closeout, not the immediate successor of the
receiver-only cutover. `FACT0-P1-PARTITION0` consumes the existing 47-path /
99-occurrence inventory and partitions every remaining writer by evidence,
instruction-commit timing, failure behavior, and retirement prerequisite. It
also records the selected receiver's scoped closeout without claiming that the
other legacy producers have migrated. This prevents a false global-zero claim
while keeping every later code-facing migration to one producer family.
The terminal gate runs only after `COMPCTX0`, `CONFIG0`, `FINALIZE0`,
`METAPROP0`, `PLAN0`, and `RAWADAPT0`: those owners physically retire remaining
repair, compatibility, multi-fact propagation, heuristic, and
configuration-dependent writers. `FACT0-G0` is therefore the
last fact convergence gate before fresh-session activation, not a Phase 2
implementation shortcut.

### `FACT0-S0` — selected pure decision boundary

S0 adds one disconnected, map-free `TypeFactDecisionV1` in the
`hakorune-mir-builder` substrate. It receives only an existing optional
`MirType` and a proposed optional `MirType`; it owns no `TypeContext`,
`ValueId`, producer/evidence label, Builder, MIR instruction, commit, or
consumer. Producer-specific evidence vocabulary belongs to P0 after the
parameter/Copy/PHI/Call/FieldGet timing matrix is sealed.

```text
missing or Unknown + exact T  -> Publish(T)
exact T + exact T             -> Idempotent(T)
exact T + no proposal         -> PreserveExisting(T)
missing or Unknown + no proposal -> NoPublication
exact T + exact U             -> typed concrete conflict
any explicit Unknown proposal -> typed rejected proposal
```

`Void` is exact. Missing and stored `Unknown` are non-facts, but an explicit
proposal to write `Unknown` is rejected: the future monotone publisher may not
perform an `Exact -> Unknown` regression. S0 has no commit API, direct-map
writer migration, PHI input/hint change, type propagation, finalization,
origin/kind change, or Builder consumer. The existing PHI decision continues
to own logical-input unanimity; a later PHI row may delegate only its final
existing-versus-candidate comparison.

S0 must stop if it needs a map write/snapshot, `ValueId`, producer-specific
evidence, TypeContext visibility change, AST/name/runtime inference, final
metadata, or a production consumer. P0 first inventories current producer
timing and the intentionally non-monotone legacy surfaces.

#### `FACT0-S0` — closed (2026-07-19)

`hakorune-mir-builder::lowering_facts::TypeFactDecisionV1` now owns the
disconnected binary decision and its stable typed errors. It has no
`TypeContext` dependency, map/commit API, ValueId/evidence payload, Builder
consumer, PHI import, or production caller. Unit tests seal missing/Unknown to
exact publication, `Void` exactness, idempotence, preservation/no-publication,
concrete mismatch, and explicit-Unknown rejection. The existing PHI decision
and every direct writer remain unchanged. `FACT0-P0` is next: it must inventory
parameter, Copy, PHI, Call, and FieldGet timing before connecting any writer.

#### `FACT0-P0-INV0` — closed (2026-07-19)

`tools/checks/lib/mirbuilder_type_fact_producer_inventory.py` now freezes the
direct lexical census before any FACT0 writer cutover: 47 production
`type_ctx.value_types` writer paths and 99 occurrences, each classified once
by its next owner. Its five primary rows retain source anchors for Parameter,
Copy, PHI, Call, and FieldGet. The fixture is a source inventory, not a runtime
timing proof; it does not change `TypeContext`, a producer, or any consumer.
The existing PHI-specific inventory remains separate. Its stale raw-origin
literal check is recorded as guard maintenance, not as a PHI semantic drift.

#### `FACT0-P0-T0` — next code-facing row

Add one test-only in-process temporal witness for the five primary families.
For each selected value, it must observe the current transient type fact,
instruction commit point, and finalized-metadata boundary. Reuse the existing
PHI immediate-Copy witness rather than duplicating its lifecycle matrix. The
Call and FieldGet rows must classify their current failure-time behavior
explicitly, including any post-failure annotation or pre-emission type fact, as
legacy unsafe timing; T0 may not repair, normalize, or route around it.

```text
production behavior delta = 0
new production publisher = 0
new type-map API = 0
finalization repair delta = 0
PHI lifecycle policy delta = 0
```

Only after T0 is green may `FACT0-I0-RCV0` begin. Its sole prospective producer
is the existing instance-method implicit receiver (`me`, parameter zero) with
the already-built exact `MirType::Box(owner)`. Explicit/static formals that
currently publish `Unknown`, Copy origin fallback, PHI, Call, FieldGet,
origin/kind facts, and finalization remain legacy-owned. Stop for a new design
decision if receiver conflict preflight cannot precede all parameter-state
mutation, or if the receiver type must be recovered from names, origin, or
final metadata rather than existing method setup input.

#### `FACT0-P0-T0` — closed (2026-07-20)

One test-only in-process suite now owns the temporal witness under the private
generic Call emitter, without widening production visibility. Its eight cases
prove the exact receiver parameter and success-path Copy/Phi/Call/typed
FieldGet facts are transient before the metadata snapshot, then present in the
unpublished finalized draft. The existing PHI immediate-Copy test remains the
four-lifecycle-entry backstop; T0 observes one canonical completed-PHI case
only. It also records the intentional legacy split: explicit `Unknown`
parameters remain non-facts, generic unified Call annotation can survive a
failed Call emission, and typed FieldGet allocation can survive a failed
FieldGet emission. FastMem FieldLoad remains a separate producer and is not
treated as normal FieldGet timing. No producer, map API, finalization behavior,
or production consumer changed.

#### `FACT0-I0-RCV0` — closed (2026-07-20)

Connect `TypeFactDecisionV1` to exactly one existing producer shape:
`setup_method_params` publishing the verified implicit receiver `me` at formal
index zero, with the already constructed exact `MirType::Box(owner)`. Preflight
the decision before any receiver binding, variable, kind, origin, slot, or type
mutation; after success, commit through one private publisher facade. Existing
same-type receiver publication stays idempotent. A concrete conflict is a
typed fail-fast and must leave all receiver state unchanged.

```text
selected producer = instance receiver parameter zero only
explicit/static Unknown formals = legacy, unconnected
Copy/Phi/Call/FieldGet/finalization = legacy, unconnected
origin/kind publication delta = 0
new instructions or ValueIds = 0
name/runtime/final-metadata inference = 0
```

Stop rather than widen if a receiver concrete conflict is already reachable,
preflight cannot precede every state mutation, receiver type recovery needs an
authority other than existing method setup input, or any explicit/static
parameter requires the new publisher to retain current behavior.

The accepted preflight input is the existing production method skeleton: it
already owns both `function.params[0]` and
`function.signature.params[0] = Box(owner)`. A generic test helper that first
creates parameter zero inside `setup_method_params` is not an alternate
receiver admission shape. RCV0 tests must construct the same method skeleton
before setup, then prove that a conflict leaves receiver publication state
unchanged. RCV0 does not claim setup-wide rollback after a later legacy
explicit-parameter failure, nor allocator rollback, origin monotonicity, or
slot-conflict policy.

`setup_method_params` now accepts the one existing production receiver shape
through a private prepare/commit pair. Preparation reads only parameter zero,
the already-built signature `Box(owner)`, and the current transient type fact;
the pure decision runs before binding allocation, parameter-name clearing,
parameter extension, variable/binding/kind/type/origin/slot publication. Its
prepared action reaches the existing parameter identity commit through one
private enum, preserving the 47-path / 99-occurrence direct-writer inventory
instead of creating a second raw map-write site. Exact same facts are
idempotent; foreign concrete facts, non-Box signatures, and owner mismatches
fail with no receiver publication. The temporal witness now constructs the
same method skeleton as production. Explicit/static `Unknown` parameters,
Copy, PHI, Call, FieldGet, origin/kind policy, finalization, and whole-setup
rollback remain untouched.

#### `FACT0-P1-PARTITION0` — producer migration partition

After RCV0 is green, partition the remaining inventory before any broad map
cutover. This is a read-only/guarded classification step, not a new publisher:

```text
remaining writer families:
  PHI completion
  successful Copy
  typed FieldGet allocation/emission
  successful Call emission
  simple exact producers
  legacy repair/compatibility/finalization surfaces

for every row:
  evidence owner
  instruction-commit boundary
  failure-time residual behavior
  dependency / retirement row
```

`PHI0` is the first operation-owner prerequisite because raw, complete,
patch, and batch PHIs currently have distinct completion entries. Only after
that one completion law exists may its direct writer migrate. The remaining
families then move one at a time; Call and FieldGet may not move until their
T0-recorded post-failure annotation behavior has a single transaction law.
`FACT0-G0` closes only after every partitioned writer has either migrated to
the monotone owner or been physically retired by its named owner row.

The P1 implementation is one guard/fixture series, not another raw inventory:

```text
FACT0-P1-S0
  schema v2: lexical writer occurrence versus semantic producer profile

FACT0-P1-P0
  every one of the 99 lexical occurrences has gap-free ordinal coverage
  every semantic profile has evidence, commit, failure, and retirement fields

FACT0-P1-G0
  guarded profile/prerequisite matrix; production delta = 0
```

#### `FACT0-P1-S0` — closed (2026-07-20)

The existing producer-inventory checker now has a disconnected schema-v2
validator and fixture test. A v2 partition keeps the current lexical
`write_inventory` as its sole 47-path / 99-occurrence census, then requires
each source-file ordinal to be covered once without gaps or overlaps. Every
semantic profile must name its family, evidence owner, commit boundary, failure
residual, retirement prerequisite, and bounded status; one lexical occurrence
may name multiple profiles only with an explicit shared-site reason. The live
fixture remains schema v1 during S0, so no production writer or current census
classification changed. `FACT0-P1-P0` is next and must migrate that live
fixture to complete v2 coverage before any PHI or producer cutover.

One lexical write may serve more than one semantic path. For example,
`parameter_setup` retains one identity-commit write while its receiver-param0
path is `RCV0` decision-gated and explicit/static `Unknown` paths remain
legacy. P1 must represent that shared site explicitly; it must not subtract
the site from the 47/99 census or pretend that the legacy paths migrated.

The first post-P1 code-facing family is `PHI0`. After PHI completion is
unified, individual FACT0 families may proceed only in this dependency order:

```text
PHI0
  -> exact Copy-only propagation (origin fallback separate)
  -> independently sealed simple exact producers
  -> FACT0-TX0-D0
  -> normal FieldGet and Call transactions, separately
  -> COMPCTX0 -> CONFIG0 -> FINALIZE0
  -> METAPROP0-D0 / METAPROP0-CUT0
  -> PLAN0 / RAWADAPT0 retirements
  -> FACT0-G0 global lexical-zero convergence
```

`FACT0-TX0-D0` is required before normal FieldGet or Call migration because T0
observes pre-emission FieldGet and post-failure Call residual facts. FastMem
FieldLoad, origin recovery, name/current-static-box heuristics, explicit
`Unknown`, overwrite/clear, and final-MIR repair stay distinct profiles and
may never be disguised as monotone exact publication.

#### `FACT0-P1-P0` — closed (2026-07-20)

The live fixture is now schema v2 and completely partitions the unchanged
lexical census: 47 writer paths, 99 writer occurrences, and 58 gap-free
ordinal slices. It records 38 semantic profiles, including two explicitly
shared lexical sites: the receiver-param0/explicit-static identity commit and
the module-signature annotation site that also carries legacy name
normalization. No raw writer was removed or migrated.

The partition distinguishes PHI-related precompletion, provisional, rollback,
and post-PHI Bool writes; exact literal facts distinct from legacy
operator-mode paths;
signature/extern/name call annotation; FastMem, array,
record, and static-data field paths; preinstruction typed allocation; and the
multi-fact metadata facade. A profile names an actual evidence owner,
instruction boundary, failure residual, and retirement row; it is not merely
a directory label.

`metadata::propagate` is a separate multi-fact compatibility facade, not a
Copy or finalization writer: it carries type, origin, string/map, and
record-local facts and branches on the TypeRegistry environment route. Its
decision and cutover therefore occur after `COMPCTX0` and `CONFIG0`, and
before any `PLAN0` cutover can claim that planning has no hidden Builder fact
writes. The conservative execution order is now:

```text
PHI0 -> exact Copy0 -> simple exact rows -> FACT0-TX0-D0
  -> FieldGet0 -> Call0
  -> COMPCTX0 -> CONFIG0 -> FINALIZE0
  -> METAPROP0-D0 -> METAPROP0-CUT0
  -> PLAN0 -> PLAN0-RECIPE-RET0 -> RAWADAPT0
  -> FACT0-G0 -> fresh-session cutover
```

#### `FACT0-P1-G0` — closed (2026-07-20)

`mirbuilder_type_fact_partition_guard.py` now runs the existing lexical
inventory/schema guard first, then freezes the approved semantic partition.
The fixture remains the only classification surface; the G0 guard reads a
normalized projection of its profile family/status/prerequisite and sorted
writer slices, rather than creating a second raw-writer inventory. Its fixed
projection digest, 38 per-profile occurrence/slice rows, five status counts,
and two shared profile sets reject a fixture-only migration rewrite.

The guard permits JSON ordering and evidence-prose changes, but rejects a
profile/prerequisite/status change, a schema-valid lexical-slice rebinding, a
shared-profile drift, or a premature `FACT0-G0` prerequisite. Its five unit
tests prove those boundaries. Production writer, consumer, type-timing, and
raw-map deltas remain zero.

`PHI0` is now the sole next code-facing migration. It must unify raw, complete,
patch, and batch completion before any PHI profile can move; P1 authorizes no
PHI write change by itself.

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

### PHI0 task lock — 2026-07-20

The entry and proof audit selects a narrow BoxShape series. `PHI0` does not
add PHI type inference, widen origin propagation, or change a caller's
accepted source shape. It makes one semantic completion transaction explicit,
then moves the four existing Builder completion entries onto that transaction.

```text
PHI0-S0
  disconnected private completion vocabulary

PHI0-M0
  one raw/final/patch/batch timing and failure matrix

PHI0-PRED0-D0
  decide the boundary between generic PHI input replacement and
  CFG-ready completion before any facade connection

PHI0-PRED0-S0
  split the disconnected vocabulary; generic input/type completion has
  zero production consumers and CFG-ready proof remains route-scoped

PHI0-PRED0-P0
  prove generic unsealed and route-ready readiness boundaries

PHI0-I0
  connect the four Builder entries to generic input/type completion only

PHI0-CFGREADY0
  separately activate only route-owned CFG-ready consumers

PHI0-G0
  prove direct entry-specific completion decisions and partial publication are zero
```

`PHI0-S0` is the first code-facing row. It introduces a private,
production-unconnected completion vocabulary in a new small
`src/mir/builder/phi_completion/` module:

```text
PhiDraftV1
  + exact logical predecessor/input rows
  + optional explicit type hint
  -> PreparedPhiCompletionV1
  -> CompletedPhiV1
```

The sole preparation constructor delegates the existing
`PhiTransientTypeDecisionV1`; it neither reimplements the type decision nor
owns `MirBuilder`, `MirFunction`, `TypeContext`, allocation, origin, or a raw
map write. A test-only completion port may prove that type commit follows a
successful instruction commit, but S0 has zero production consumers and zero
MIR mutation.

The semantic operation is fixed as:

```text
validate exact predecessor/input rows
  -> prepare the existing type decision from logical rows
  -> prepare candidate instruction mutation
  -> commit instruction
  -> commit prepared type fact
  -> CompletedPhi
```

The later `PHI0-I0` consumer set is exactly four Builder entries:

```text
MirBuilder::emit_instruction(Phi)
define_phi_final_with_type_hint
patch_phi_inputs
define_phi_batch_prepend
```

Raw origin publication remains the separate `origin::phi` authority.
Provisional definition remains incomplete and publishes no unanimous type.
Function-level `define_phi_final_fn*` remains outside lowering-time type
publication because it has no active Builder session. Existing CFG
reachability, dominance, input rematerialization, rollback, Binding SSA
provisional `Unknown`, post-completion Bool publication, and finalization
repair remain their current owners.

S0 proof minimum:

```text
same logical request has normalized raw/final/patch/batch preparation parity
provisional definition remains incomplete
duplicate/missing/phantom predecessor rows fail before a completion product
existing concrete-type conflict is preserved from the current decision owner
single candidate failure commits no type
batch item failure commits no live instruction or type
failed patch retains an incomplete draft
```

Stop rather than broaden PHI0 if one completion transaction requires an
origin-policy merge, new CFG/dominance authority, input rematerialization
rewrite, function-level transient type publication, Binding SSA `Unknown`
retirement, finalization repair, or a new accepted PHI shape. The stale
whitespace-sensitive `phi_type_publication_inventory.py` source anchor is a
separate guard-maintenance row; it is not PHI0 semantic work.

#### `PHI-TYPE-PUBLISH0-GUARD-MAINT0` — closed (2026-07-20)

The existing publication inventory's raw-origin anchor now accepts rustfmt's
whitespace around the unchanged `value_origin_newbox.insert(dst, origin)`
operation. Its production-consumer census excludes the private disconnected
`phi_completion` owner, and its LocalSSA timing anchor follows the already
landed `function_state.type_ctx` spelling. It remains a source-order guard over
the same origin/type authorities; there is no PHI, type, origin, caller, or
runtime behavior delta. This restores the existing inventory as a usable M0
baseline without mixing maintenance into PHI0's semantic completion work.

#### `PHI0-S0` — closed (2026-07-20)

`src/mir/builder/phi_completion/` now owns the disconnected vocabulary:
`PhiDraftV1`, `PreparedPhiCompletionV1`, and `CompletedPhiV1`. Preparation
checks an exact caller-supplied predecessor-row set, normalizes logical rows,
and delegates the existing `PhiTransientTypeDecisionV1`; no Builder,
instruction, type-map, origin, allocation, rematerialization, or production
facade is touched. A test-only fake port fixes instruction-success-before-type
commit for single and candidate-batch completion.

The ten focused tests freeze raw/final/patch/batch preparation parity,
provisional incompleteness, duplicate/missing/phantom rows, inherited concrete
type conflicts, single and batch candidate failure, batch-item preparation
failure, and retained provisional draft identity. `cargo test -q --lib
phi_completion`, the existing PHI type-publication/lifecycle suites, and
`cargo check --all-targets` are green. `PHI0-M0` inventories the four real
completion entries' logical rows, physical rematerialization, and
instruction/type timing before any I0 facade connection.

#### `PHI0-M0` — closed as a predecessor-readiness design stop (2026-07-20)

The timing matrix is complete, but it disproves the premise needed to connect
the S0 product to all four entries: there is no common existing authority for
the exact expected predecessor rows. The existing transaction and residual law
is intentionally different at each entry:

| Entry | Existing completion order | Existing failure residual |
| --- | --- | --- |
| raw `emit_instruction(Phi)` | type decision, live edge rematerialization, raw-origin preparation, append, type/origin commit | prior rematerialized copies may remain; PHI/type/origin do not commit |
| complete final insert | type decision, debug metadata, live rematerialization, insert, type commit | rematerialization/debug metadata may remain; type does not commit |
| provisional patch | sort, stored hint, type decision, replace inputs, type commit | incomplete draft remains; no rematerialization; `PhiTxn` owns rollback |
| batch | preflight/type decisions, candidate rematerialization/insert, function replacement, type commits | candidate is dropped; live function/types remain unchanged |

Two existing legal cases make a blanket `compute_predecessors(current_cfg)`
connection invalid. A lifecycle test patches a provisional PHI in a block whose
predecessor set has not yet been published. A loop-header batch policy can
admit a host-entry predecessor before its terminator is present in the current
CFG. Passing the logical input rows as their own expected rows would merely
self-authorize them and make S0's missing/phantom-row law tautological.

`PHI0-I0` is therefore forbidden. The exact next decision, its candidate
boundaries, and its non-authorities live in
[`mirbuilder-phi-predecessor-readiness-consultation-2026-07-20.md`](mirbuilder-phi-predecessor-readiness-consultation-2026-07-20.md).

#### `PHI0-PRED0-D0` — closed (2026-07-20)

Three independent source audits select Candidate D′. Exact predecessor-row
validation is not a precondition of every existing PHI input/type completion.
It is a distinct semantic claim, available only to a route that already owns a
sealed predecessor witness. The common transaction is therefore:

```text
normalize generic logical input rows
  -> decide existing type fact
  -> commit successful instruction/input replacement
  -> commit prepared type fact
```

It does not claim CFG readiness. `PhiTxn::patch_phi_inputs`, raw
`emit_instruction(Phi)`, generic final insertion, generic loop patch, JoinIR
exit patch, header batch, and legacy helpers remain on this generic boundary.
They must not gain a `compute_predecessors` scan, self-certified expected rows,
or a new global CFG table.

The separate CFG-ready capability admits only a route that already proves exact
predecessor rows. The evidence inventory currently names canonical resolved-If
`VerifiedIfMergePredecessorsV1` and the CorePlan select-as-PHI proof as ready;
Binding SSA has its own `VerifiedPredecessorsV1` but its adapter currently drops
that witness before generic patch. `exprs_peek` is a later route-local read-only
candidate. Generic loop, JoinIR exit, header batch, raw emission, `if_form`,
and legacy helpers are not ready. `PHI0-PRED0-S0` may split only the private,
disconnected vocabulary; it has no production consumer, no CFG mutation, and
no accepted-source-shape delta. `PHI0-I0` later connects the four entries only
to generic input/type completion. CFG-ready activation is a separate
`PHI0-CFGREADY0` row.

#### `PHI0-PRED0-S0` — closed (2026-07-20)

The private `phi_completion` vocabulary now exposes two intentional
preparation boundaries. `prepare_input_completion` validates duplicate logical
predecessor rows and delegates the existing type decision, but makes no CFG
claim. A non-Clone private `CfgReadyPhiRowsV1` checks exact expected
predecessor-row coverage and keeps those rows inseparable from the logical
inputs before `prepare_cfg_ready` consumes it. Its constructor remains private
until a later route-specific CFGREADY0 witness is selected; neither path
derives nor persists CFG truth. Both paths retain the same post-instruction
type-commit transition.

The focused disconnected tests prove raw/final/patch/batch generic preparation
parity, an unsealed input row accepted by the generic path, duplicate-input
rejection, CFG-ready duplicate/missing/phantom rejection, existing concrete
type conflicts, and zero type commit after failed single or batch candidate
instruction commit. The module has zero production consumers, no Builder or
MIR mutation, and no new ValueId/CFG/origin map.

#### `PHI0-PRED0-P0` — closed (2026-07-20)

The proof matrix now fixes the real readiness split rather than inferring it
from the disconnected vocabulary. A provisional transaction accepts its
existing unsealed row; a loop-header host entry is deliberately an accepted
future entry predecessor before that host terminator is published. Conversely,
the canonical resolved-If matrix proves exact predecessor coverage and the
CorePlan select-as-PHI helper proves a completed two-predecessor CFG before it
inserts a final PHI. Negative resolved-If controls still reject phantom and
disconnected predecessors.

The existing PHI publication inventory guard now keeps `prepare_cfg_ready`
private with zero production consumers and rejects `compute_predecessors` from
the generic patch and batch lifecycles. It does not prohibit route-owned CFG
analysis such as the loop-header builder or CorePlan select-as-PHI helper.

#### `PHI0-I0` — closed (2026-07-20)

The four authorized Builder entries now share one `phi_completion` connection:
raw emit, complete final insertion, provisional patch, and batch prepend all
prepare from their logical input rows before their existing materialization or
instruction mutation, then transfer the prepared type fact to the existing
type-publication owner only after success. Raw unanimous origin publication
remains separate and success-committed; provisional definition and
function-level APIs remain outside the connection.

Generic completion now rejects duplicate incoming predecessor rows before any
entry-specific mutation. Focused raw/final/patch/batch fixtures prove that
rejection leaves instructions and transient type facts unchanged; the existing
unsealed provisional-patch and candidate-batch atomicity controls remain
green. The inventory fixes direct type-publication preparation consumers at
zero and the new generic completion connection at exactly four. No generic
entry reads CFG predecessors. `PHI0-CFGREADY0-D0` is next: it must select one
route-owned exact predecessor witness before activating the private CFG-ready
path.

#### `PHI0-CFGREADY0-D0` — closed (2026-07-20)

Three independent audits select Candidate A: one route-local canonical
resolved-If bridge is the sole first production consumer of CFG-ready PHI
completion. `VerifiedIfMergePredecessorsV1` already seals the header branch,
actual then/else exit reachability, distinct expected predecessors, recomputed
predecessor set, and cached merge predecessor set. `define_join_phis` also
reverifies that witness immediately before it constructs each two-row final
join PHI. This is the only audited route whose predecessor authority is both
durable and exact at the completion seam.

```text
PHI0-CFGREADY0-S0
  disconnected canonical-resolved-If bridge; production consumer = 0

PHI0-CFGREADY0-P0
  implicit/explicit actual CFG matrix plus tampered-row/no-mutation proof

PHI0-CFGREADY0-I0
  canonical resolved-If only; CFG-ready consumer count = 1

PHI0-CFGREADY0-G0
  inventory and route-exclusion guards

then PHI0-G0
```

The bridge may create `CfgReadyPhiRowsV1` only from the already-reverified
`VerifiedIfMergePredecessorsV1` and its two logical join rows. It must reuse
the same final materialize/insert/after-instruction type-commit path as the
generic final facade; it may not insert a PHI or commit a type fact directly.
Reverification/preparation failure leaves instruction, transient type, and
origin state unchanged, with no retry or fallback.

Rejected for this row:

```text
generic CFG-ready facade or raw expected-row constructor
raw emit, generic final, provisional patch, generic batch, and legacy routes
loop-header batch (its host entry may be a deliberate future edge)
CorePlan select-as-PHI (route-local observation but no durable witness)
Binding SSA and exprs_peek (separate route rows)
compute_predecessors/final verifier as generic lowering-time authority
persistent predecessor tables or Builder fields
```

The inventory must keep `prepare_cfg_ready` private, permit exactly the
canonical resolved-If route at I0, and forbid every generic lifecycle from CFG
analysis. CorePlan select-as-PHI remains parked behind a separate
`PHI0-CFGREADY0-SELECT0-D0` decision after this route is green.

#### `PHI0-CFGREADY0-S0` — closed (2026-07-20)

`VerifiedResolvedIfCfgReadyJoinRowsV1` is now the disconnected bridge owned by
canonical resolved-If lowering. Its only constructor accepts a
`VerifiedIfMergePredecessorsV1` plus resolved join rows, immediately rechecks
the completed CFG and merge cursor, rejects duplicate bindings, and seals the
two logical predecessor/value rows for each binding. The product is non-Clone
by construction and has no PHI insertion, `phi_completion` call, type/origin
publication, raw expected-row constructor, persistent CFG table, or production
consumer.

Focused tests prove the implicit two-row shape, recheck success without
mutation, CFG-drift rejection without an additional instruction or type write,
and duplicate join-row rejection without mutation. `PHI0-CFGREADY0-P0` is next
to add the explicit-else and actual route matrix before I0 connects the one
consumer.

#### `PHI0-CFGREADY0-P0` — closed (2026-07-20)

The actual matrix now covers both implicit and explicit canonical If routes.
The explicit case deliberately gives both branches an intermediate block, so
the bridge proves it uses the actual then/else merge exits rather than lexical
branch entries. In both cases bridge creation leaves merge PHI instructions and
transient types unchanged. Existing drift and duplicate-row negatives retain
the same no-additional-mutation law. `PHI0-CFGREADY0-I0` is next: it may connect
this bridge to exactly one resolved-If final-PHI path and nothing else.

#### `PHI0-CFGREADY0-I0` — closed (2026-07-20)

Canonical resolved-If lowering is now the sole CFG-ready completion consumer.
It first seals `VerifiedResolvedIfCfgReadyJoinRowsV1`, then a route-local
`phi_completion` sidecar revalidates that sealed bridge and constructs the
private `CfgReadyPhiRowsV1` immediately before preparing the final PHI. The
sidecar accepts no raw expected-row slice and performs no CFG scan, instruction
insertion, type/origin write, or generic lifecycle selection.

The existing final-PHI materialization and post-instruction type commit are
shared with the generic final facade, so a successful resolved-If join retains
the normal exact type publication timing. A failed bridge or preparation still
adds no PHI, type, or origin fact and has no retry/fallback route. Raw emit,
generic final, provisional patch, and batch remain exactly the four
input/type-only completion consumers; they do not acquire CFG readiness.

Focused resolved-If, `phi_completion`, publication, all-target compilation,
format, diff, and current-state gates are green. `PHI0-CFGREADY0-G0` is next to
freeze the one-consumer inventory and route exclusions before `PHI0-G0`.

#### `PHI0-CFGREADY0-G0` — closed (2026-07-20)

The existing PHI publication inventory now proves the selected split directly:
generic input/type preparation has exactly four Builder consumers (raw emit,
generic final, provisional patch, and batch), and CFG-ready preparation has
exactly one production consumer, the canonical resolved-If sidecar. The guard
also rejects raw expected-row construction, direct PHI/type/origin mutation,
and CFG analysis from both the sidecar and resolved-If join materialization.
The disconnected bridge remains free of PHI-completion/lifecycle and fact-write
authority. CorePlan select-as-PHI, loop-header future edges, and every generic
lifecycle remain excluded.

It also fixes the shared final physical continuation at one definition plus its
generic-final and resolved-If callers, keeps the CFG-ready constructor private,
and proves raw emit is the one Builder-session PHI origin committer. Function-
level EdgeCFG/JoinIR insertion remains a documented separate scope because it
has no Builder transient facts; it is not a hidden fifth generic completion
consumer. The guard itself is syntax-only and emits one machine-readable report;
it does not add a second semantic authority. Source/check files remain below
800 lines.
`PHI0-G0` is next to close the generic completion series without broadening
CFG-ready admission.

#### `PHI0-G0` — closed (2026-07-20)

The PHI completion series is closed as one BoxShape-only owner boundary. The
inventory proves that raw emit, generic final, provisional patch, and batch are
the exact four generic input/type completion consumers; the canonical
resolved-If sidecar is the sole CFG-ready consumer; and the shared final
physical continuation has exactly its definition plus the generic-final and
resolved-If callers. Direct entry-specific type decisions are zero, raw emit
remains the sole Builder-session PHI origin committer, and all generic routes
remain CFG-free. The guard preserves the existing single/patch/batch failure
laws rather than claiming rollback for provisional drafts or legacy raw
materialization residuals.

Function-level EdgeCFG/JoinIR PHI insertion is explicitly outside this
Builder-transient completion scope: it has no Builder type context and is not a
hidden fifth generic consumer. No new PHI route, CFG admission, origin policy,
type inference, source shape, fallback, runtime, backend, or ownership change
is introduced. `FACT0-I1-COPY0-D0` is closed as a design stop; the next row is
`COPY-UNKNOWN0-D0`.

#### `FACT0-I1-COPY0-D0` — closed as a design stop (2026-07-20)

This is a read-only decision row with code delta zero. Its scope is one
successful LocalSSA `Copy` only. `ssa/local.rs::ensure_inner` currently keeps
two distinct post-success behaviors adjacent to one another:

```text
copy_exact:
  existing source transient type -> Copy destination

copy_origin_legacy:
  receiver-origin-derived Box type fallback -> Copy destination
```

COPY0 may select only `copy_exact`; it may not infer a missing type, turn
`Unknown` into an exact fact, write or repair origin, or change the legacy
fallback. D0 found that a stored `Unknown` destination-map entry is observably
required by the current LocalSSA behavior. It does not silently treat that
legacy write as monotone publication.

```text
source:
  type = Unknown
  origin = Owner

current successful LocalSSA receiver Copy:
  dst type = Unknown
  -> receiver-origin Box(Owner) fallback is suppressed

COPY0 treating Unknown as no publication:
  dst type = absent
  -> existing receiver-origin Box(Owner) fallback activates
```

That is an origin/receiver-compatibility behavior decision, not an exact-type
transfer cleanup. The same post-success block also serves rematerialized
Const/BinOp/Compare/Select results, so it is not yet a physical-Copy-only
publisher boundary. `COPY0-S0` is forbidden until `COPY-UNKNOWN0-D0` selects
an origin-aware law and a physical-Copy boundary.

If that later decision makes exact Copy independent, the follow-up order is:

```text
FACT0-I1-COPY0-D0
  -> FACT0-I1-COPY0-S0
  -> FACT0-I1-COPY0-P0
  -> FACT0-I1-COPY0-I0
  -> FACT0-I1-COPY0-G0
```

S0 remains disconnected; P0 proves exact/idempotent/conflict/Unknown,
successful-emission, and origin-isolation cases; I0 may connect only the
successful exact source-type transfer; G0 closes the one-producer guard.
`ORIGIN0-D0`, FieldGet, Call, finalization, and every accepted source shape
remain separate rows.

#### `COPY-UNKNOWN0-D0` — closed (2026-07-20)

Candidate C′ is selected. The task order and first code-facing S0 contract are
in [`mirbuilder-copy-unknown-origin-task-2026-07-20.md`](mirbuilder-copy-unknown-origin-task-2026-07-20.md).

Source authority is the existing successful LocalSSA post-emission state:
`value_types[v]`, `value_origin_newbox[v]`, `LocalKind::Recv`, and the exact
`value_types[loc].is_none()` fallback condition. `MirFunction` final metadata,
method names, runtime tags, source syntax, and route success are non-authority.

The decision selected one coherent policy for a successful Copy whose source
has stored `MirType::Unknown` plus an origin:

```text
C′. retain stored Unknown as an explicit compatibility sentinel while splitting
    exact type, legacy Unknown, origin, and receiver fallback decisions
```

The selected task preserves a pre-emission source read, post-success commit,
and no mutation on failed emission. It may not backfill an exact type, use
finalization repair, widen source shapes, or fold string/map/record facts,
`metadata::propagate`, direct Copy emitters, or PHI origin into the row. The
minimum proof must cover missing versus stored Unknown, Unknown plus origin for
`Recv`, exact and conflicting existing facts, failed Copy, and fresh Builder
reuse. `COPY-UNKNOWN0-S0` may now add the disconnected vocabulary only; COPY0
and production fallback changes remain forbidden.

#### `FACT0-I1-CHECKSELECT0-D0` — closed (2026-07-20)

The LocalSSA compatibility and physical-Copy rows are complete. The next
independent exact-fact producer is one successful `CheckExpr` accumulator
`Select`, not generic Select inference: both selected values are already exact
Integer from closed CONST0, while the condition is non-authority. The selected
task is
[`mirbuilder-checkselect0-task-2026-07-20.md`](mirbuilder-checkselect0-task-2026-07-20.md).

`CHECKSELECT0-S0` is the sole next row. It may add only a disconnected,
map-free `TypeFactDecisionV1` preparation product. Compare, Call-backed
operators, PHI, FieldGet, origin, Unknown retirement, and finalization repair
remain separate authorities. No production consumer is authorized before I0.

`CHECKSELECT0-S0` is closed: the private `exprs_check::select_type` owner
prepares only Integer publication from an optional destination fact, with
Missing/StoredUnknown publish, Integer idempotence, and exact conflict tests.
It has no Builder, ValueId, MIR, TypeContext, or production-consumer authority.
`CHECKSELECT0-M0` is closed: CONST0 one/zero establish exact accumulator
induction; condition lowering precedes Select; and `emit_instruction` failure
returns before the legacy direct Integer write. `CHECKSELECT0-P0` is next.

`CHECKSELECT0-P0` is closed: empty and multi-item CheckExpr fixtures prove the
CONST0 Integer accumulator induction, Select failure leaves its destination
untyped, and ordinary finalization snapshots the result. `CHECKSELECT0-I0` is
next and may connect only the shared post-success Select fact commit.

`CHECKSELECT0-I0/G0` are closed: the one prepared Integer decision commits only
after the existing accumulator Select succeeds; direct `exprs_check` type
writes are zero; and the existing FACT0 guard fixes the one decision/commit
owner without claiming generic Select inference. A new EXACT0 producer requires
an explicit independent selection.

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

## Phase 7.5 — METAPROP0

Retire the generic lowering-time metadata facade before `PLAN0-CUT0`. This is
not Copy0: the current facade conditionally selects TypeRegistry or direct
TypeContext writes and transports type, origin, string-literal, map-value,
map-literal, and record-local facts across plan and non-plan callers.

```text
METAPROP0-CENSUS0
  every propagation caller and every transported fact family

METAPROP0-D0
  one fact-family ownership and transaction decision; no call-site cutover

METAPROP0-CUT0
  selected plan-path callers no longer reach the mutable facade

METAPROP0-G0
  direct facade type/origin/auxiliary-fact writes are zero or have one named
  legacy-adapter removal row
```

`COMPCTX0` must first split the mutable TypeRegistry owner, and `CONFIG0` must
first seal the propagation mode. `METAPROP0-CUT0` precedes `PLAN0-CUT0`; a
pure-plan claim is false while a plan path can still hide `&mut MirBuilder`
fact mutation through this facade. Non-plan callers are inventoried in D0 and
may remain behind an explicitly named later adapter; they are never silently
classified as Copy or finalization behavior.

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
| multi-fact metadata propagation facade | `METAPROP0-CUT0` | plan paths no longer mutate type/origin/string/map/record facts through the generic facade |
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
