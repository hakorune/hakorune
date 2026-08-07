---
Status: Active task-order SSOT
Date: 2026-08-04
Decision: accepted — `JOINIR-LOOP-SELFHOST-RECIPE-PIPELINE0-D0`
Scope: production Loop meaning, selfhost-portable recipe, terminal candidate lowering, and atomic retirement
Related:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/joinir-loop-pre-effect-product-ssot.md
  - docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md
  - docs/development/current/main/design/joinir-loop-scoped-nongeneric-cutover-ssot.md
  - docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# JoinIR Loop Selfhost-Ready Recipe Pipeline

## Decision

Choose the clean final architecture rather than the shortest wrapper around the
19 legacy physical routes.

### Route-count invariant

The number `19` belongs only to the legacy scheduler's ingress/coverage matrix.
It is not the number of portable Recipe variants, semantic Loop kinds, or
physicalizers. The canonical target has one recursive semantic algebra:

```text
19 legacy ingress rows
  -> typed source/family observation and one policy winner
  -> one recursive LoopRecipeV1 algebra
       LoopNode(condition = Always | Predicate)
       Item = Operation | If | Loop | Exit
  -> one verifier / JoinSig elaborator / physicalizer chain
```

`LoopSimpleWhile`, `LoopTrue*`, `LoopCond*`, scan/accum labels, and
`GenericLoopV0/V1` are migration/admission identities. `NestedLoopMinimal`
recurses through the same `Loop` item, while `break`/`continue`/`return` use
the common `Exit` item. `IfPhiJoin` contributes shared If/join obligations; it
is not a Loop Recipe kind. M7 and M8 therefore close adapter coverage into
this one algebra. They must not grow a parallel enum of completed source
patterns.

## SSOT role disambiguation

The authority split is:

- `RecipeTree + Parts` is the current production in-builder recursive
  implementation and parity oracle through portable Loop pipeline M10.
- Portable `LoopRecipeArtifactV1` is the future Rust/.hako common final semantic wire contract.
- `joinir-loop-selfhost-recipe-pipeline-ssot.md` is the replacement/cutover authority.

## Production authority status

Current production is `route_loop` -> ordered 19-route scheduler -> route
composers/CorePlan/PlanLowerer/JoinIR merge/route PHI writers. Portable
`VerifiedLoopRecipeV1` has zero physical production consumers through M9.
The current JoinIR/JoinModule path remains execution authority until M10; Retry is
scheduler-internal and exhaustion freezes; it is not a portable Recipe consumer.
M5/M6 remain caller-zero evidence; the bounded M10a resolved DirectAccum pilot
is closed through one canonical resolved production caller. M10a did not switch
`route_loop` or retire the old scheduler/PHI edges. M10b is the first all-route
consumer and retires the scheduler/fallback/remaining old PHI edges.

```text
Source / projection
-> StructuralFacts
-> one RoutePolicy winner
-> one recursive LoopRecipe artifact
-> RecipeVerifier / logical JoinSig elaboration
-> verified symbolic CFG / edge plan
-> one candidate Physicalizer
-> FunctionDraftSeal
-> compile-candidate external commit
```

The existing unpublished compile candidate is the sole abort/publication
transaction. A failed physical lower may leave that candidate dirty because the
whole candidate is dropped. This removes the need for `LoopEmissionDraftV1`, a
Loop-local Builder clone, a symbolic MIR fragment, or an undo journal.

Candidate isolation is not route qualification. Meaning must be decided before
the first Builder effect; selected physicalization returns terminal success or
`Freeze` and cannot advance to another route.

The post-Recipe common demand, fresh unpublished function session, failure
discard, open-After result, typed function-finish terminal, and
Completion/DraftSeal handoff are fixed by
`loop-common-physical-demand-and-session-ssot.md`. That boundary is one
prepared execution product over existing authorities, not a universal Callable
plan or a second physicalizer. Completion moves into the fresh function session
exactly once; Loop continuation and profile Tail remain distinct. Its
implementation remains caller-zero until the M8/M9 and production-selection
gates close.

### Bounded M10a DirectAccum pilot — `JOINIR-LOOP-ACCUM-MIR-PHYSICAL-SNAPSHOT0-M5-P4-S1` (closed)

The resolved DirectAccum profile is selected by the canonical resolved ingress
and reaches exactly one production physicalizer caller in
`resolved_lowering/direct_accum_lowerer.rs`. That pilot borrows the existing
function-owned Binding SSA, canonical CFG, and `PhiTxn`; it does not create a
second writer or change `route_loop`.

The actual unpublished DirectAccum candidate now matches the immutable alpha
semantic snapshot, including sealed-After final-carrier publication, late
candidate discard, and fresh-session reuse. Raw physical IDs, printer text,
all-route parity, Retry/fallback retirement, Generic widening, default compile
activation, grammar, and IR changes remain outside this row. The task card and
the PHI/SSA design SSOTs, MIR reference pages, `src/mir/builder/README.md`,
and current pointer mirrors were synchronized as part of the implementation
closeout.

#### M10a D2-S5 prerequisite — final-carrier publication (closed)

The first candidate audit found a missing production obligation before the
P4-S1 observer could claim final-carrier parity: the resolved `After` block
had the Unit return, but final `i`/`sum` bindings were not read at sealed
`After`. The accepted P1/D1 contract is now implemented by
`JOINIR-LOOP-ACCUM-FINAL-CARRIER-PROJECTION-M10A-D2-S5`: the generic open-After
continuation receipt remains unchanged, the existing After witness is sealed
through the canonical port, verified carrier keys are read through the same
adapter into a typed caller-owned receipt, and the existing claim/finish/commit
order follows. P4-S1 is green; its observer does not infer final values from
header PHIs.

### M10b successor premise — Decision B-prime restored

The all-route membership premise is closed by the later B-prime evidence and
must not be reopened as a universal-ingress problem:

```text
raw public / raw VM-reference NarrowV1
  -> typed Loop rejection before physical Builder open

RawLegacyChildLoweringPortV1
  -> profile-blind capability, not a compilation profile

normalized-shadow Loop mutation
  -> retired; only the non-mutating observer remains

located RawInvocation Loop
  -> exact parent / condition / body receipts
  -> active R4 source-erasing migration fence until M11
```

Therefore universal raw/reference semantic ingress is rejected. This does not
authorize M10b: M7 five-family closure, M8 all-19 producers, M9 host parity,
and the Generic D2 winner boundary remain mandatory. M7-S2-A is now closed as
caller-zero logical evidence for one `loop(true)` with an explicit `Break` arm
and `Continue` arm; it changed no Recipe wire schema, physical CFG/PHI, route,
scheduler, runtime, grammar, or diagnostics. The next Lego slice is the
design-only M7-S3 bounded LoopTrue source/policy/Recipe cohort.

## Selfhost boundary

The Rust producer lands first, but the semantic product must be directly
representable by `.hako` boxes/JSON. The portable boundary must not contain:

```text
ASTNode or synthetic/rebuilt AST
borrowed CanonicalLoopFacts or self-referential frame borrows
MirBuilder / CorePlan / ValueId / BasicBlockId / Frag
callbacks / trait objects / Rust-only lifetimes
retry / raw suffix / selector capability
```

It owns stable source coordinates or normalized operands and one recursive
structured recipe: loop condition, nested body items, carrier declarations, and
typed exits. A producer receipt may retain the legacy route name for diagnostics,
but route names and first-mutation profiles are not semantic recipe inputs.
The verifier elaborates the structured recipe into logical CFG/JoinSig
obligations. Rust and `.hako` compare a deterministic normalized recipe
representation, not allocation addresses or physical MIR IDs.

The final authority flow is the existing north star:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower -> Seal -> Collect
-> Atomic Publish
```

There are two separate cutovers over that one semantic contract:

```text
M10/M11:
  Rust production Loop authority -> the portable verified-recipe pipeline

SH5:
  .hako selfhost Loop authority -> its own verified-recipe physicalizer
```

M9 proves only the portable `.hako` producer. It is not complete selfhosting.
The Rust cutover must land and retire its old scheduler before the later `.hako`
physicalization lane becomes active. The Rust pipeline may remain afterward as
an explicit Stage0/bootstrap reference; it must never become an automatic
fallback from the `.hako` path.

## Generic Recipe handoff boundary (D4-S4-D0)

The Generic family has a stricter handoff than the current legacy facts route.
The S2 selector result is a marker-only test outcome and carries no source or
`BindingRef` provenance. Current `GenericLoopV0/V1Facts`, `RecipeBody`, and the
P2 label snapshot are AST/Builder-derived; they are not portable Recipe input.
Before a Generic winner can exist, the resolver must issue an AST-free,
source-branded candidate envelope and a one-shot non-`Clone` source/window lease
containing exact mode/coverage and role-level `BindingRef` claims. Window
`V1Only`/`Both`, `Neither`, `NoStandaloneRow`, overlap, or planner-unsealed
evidence cannot manufacture `Selected(Generic)`.

The future flow is:

```text
resolver source/window lease
  -> AST-free Generic semantic shape + candidate proof
  -> sealed family observation
  -> family selector: Selected(Generic)
  -> Generic-specific Recipe demand
  -> Generic Recipe producer
  -> VerifiedLoopRecipe + JoinSig + BindingRef/key effect relation
  -> Recipe/source verification
```

The Generic demand is distinct from the legacy
`VerifiedSelectedLoopRecipeDemandV1`, which requires a 19-route policy winner.
The dedicated Generic Recipe producer is the sole issuer of contiguous
`LoopBindingKeyV1` and the internal relation from each key to its exact
resolver `BindingRef`/role/site. The portable source-path claim and that
semantic relation are separate capabilities. Binding SSA alone later maps
`BindingRef` to physical `ValueId`/`PHI`; the producer never does. Missing or
foreign owner/site/forest/frame, mode/coverage, carrier/role, or effect
relation rejects before key allocation. Recipe/JoinSig/effect failure is
terminal: no legacy-route re-selection, retry, suffix, fallback, or alias to
DirectAccum/NestedPredicate. If no real sealed `Selected(Generic)` exists, the
next witness is `NoSafeSlice`; it must not fabricate a winner.

### D4-S4-S0 audit disposition

The first Generic semantic-demand witness is `NoSafeSlice`. There is no
`Selected(Generic)` issuer or callsite, no resolver-issued Generic
candidate-envelope, no one-shot role-level `BindingRef` lease, and no
`VerifiedGenericRecipeDemandV1`. The current Generic facts contain AST/Builder
recipes, while the resolver source-window/provenance witnesses are test-only
identity transport without Generic carrier/eligibility. The legacy selected
demand and historical synthetic handoff receipts are explicitly excluded.

The next design stop is
`JOINIR-GENERIC-RESOLVED-CARRIER-GENERIC-SEMANTIC-SHAPE-DESIGN0-D4-S4-S0-D0`.
It must define the minimum AST-free candidate/shape product, exact sites,
mode/coverage, forest/frame, carrier/step/body effect roles, and
`BindingRef` provenance, with one named issuer per field. Until that closes,
no selector winner, Recipe/key, Builder/MIR/PHI, retry, fallback, or production
caller may be added.

That design stop is now closed. The selected product chain is:

```text
resolver source projector
  -> VerifiedGenericSourceLeaseV1
  -> GenericSemanticShapeIssuerV1
  -> VerifiedGenericCandidateEnvelopeV1
  -> VerifiedGenericFamilyObservationV1 (mode + coverage)
  -> SelectedGenericFamilyV1
  -> VerifiedGenericRecipeDemandV1
  -> Generic Recipe producer
```

`GENERIC-SEMANTIC-SHAPE-SCHEMA-D1` is now closed as a worker-reviewed
docs-only contract. The resolver-owned source-lease witness is also closed as
cfg(test)-only, bounded to exactly two forest members and one each of
`NestedWrite`/`PostLoopRead`. The issuer derives forest and per-member frames
from one `VerifiedResolvedFunctionV1`; external frame mixing is therefore
unrepresentable, while an internal frame/site co-seal check remains.
The D1 boundary remains one typed schema table for `CarrierProof`,
`ConditionProof`, `StepProof`, `BodyEffectProof`, and `Coverage/Exit`, with
resolver-lease fields separated from shape-issuer fields. S1 is now closed as
only `VerifiedGenericCarrierProofV1`: it consumes the move-only lease, proves
the `NestedWrite -> PostLoopRead` same-`BindingRef` relation, and retains the
lease brand in an AST/source-lifetime-free handoff. It performs no AST borrow
because the lease already sealed the resolver maps. Condition/step/body-effect/
coverage role ingress is the next design-only D0; `Selected(Generic)`, demand,
Recipe/key, retry, and fallback remain untouched and roles are never
re-resolved by name.

The D1 product boundary is:

```text
resolver lease:
  owner/origin/source-kind/root+loop sites/forest/frame
  + non-Clone role claims (site, BindingRef, scope, ancestry)

shape issuer:
  CarrierProof
  ConditionProof (comparator/operand/bound/placement)
  StepProof (operator/target/delta/placement)
  BodyEffectProof (ordered typed effects/exits)
  Coverage/Exit (complete window/forest and opaque-transfer checks)
```

The two products are AST/source-lifetime-free after issuance and contain no
string labels, route IDs, `RecipeBody`, Builder/MIR/PHI, `ValueId`, or legacy
demand. A later candidate envelope consumes them atomically; it must not split
or re-pair their coordinates.

The resolver owns the opaque source lease, exact sites/forest/frame, and
role-level `BindingRef` claims. The shape issuer may borrow AST only while
producing bounded typed carrier/condition/step/body-effect/coverage proofs;
the output contains no AST, `RecipeBody`, or source-unit lifetime. Policy owns
the co-sealed mode/coverage context, the selector only moves the opaque
capability, the Recipe producer alone issues recipe keys/effect relations, and
Binding SSA alone owns physical `ValueId`/`PHI`. All products are move-only;
`BindingRef` identity copies do not make role capabilities clonable.

The D4-S4-S2-D1 design closeout keeps `VerifiedGenericSourceLeaseV1`
immutable. Future role claims are carried by an atomic, versioned
`GenericShapeSourceLeaseV2`; its first bounded ingress is an inner-loop
Condition+Step role catalog. BodyEffect and Coverage/Exit require separate D0
contracts.

### D4-S4-S2-D0 resolver issuer boundary

Worker API/source census closes the direct V2 witness as `NoSafeSlice`.
`GenericSourceRoleSiteV1` is caller-selected test ingress, and
`VerifiedResolvedFunctionV1` currently seals bindings, assignment targets,
calls, exits, scopes, and loop forest/frame—but not an exact inventory of all
statement/expression sites or their parent child roles. Path synthesis could
therefore publish a non-existent `LoopCondition/Rhs` as if it were exact.

The required neutral prerequisite is a resolver-owned
`VerifiedResolvedSourceSiteInventoryV1`. It is recorded during the existing
shadow traversal and co-sealed into the resolved-function product; it stores
only owner/origin/source-kind brands, separate exact statement/expression
membership, and point lookup. It stores no AST, names, node kinds, roles,
operators, literals, route IDs, or MIR identities. `SourcePathSegmentV1` is
the sole topology authority: an inventoried parent plus an inventoried child at
the canonical appended segment proves placement without a second parent-role
map. Existing resolver variable/assignment maps remain the sole `BindingRef`
authority.

The task order is therefore:

```text
RESOLVED-SOURCE-SITE-INVENTORY0-D0
  -> design inventory ownership, completeness, and typed rejects
RESOLVED-SOURCE-SITE-INVENTORY0-S0
  -> closed: resolver traversal records/seals branded membership; focused
     inventory/generic tests green; no Generic consumer or public reference row
D4-S4-S2-S1
  -> closed: cfg(test)-only V2 Condition+Step role issuer consumes the sealed
     inventory/path topology; five focused tests; no public reference row
D4-S4-S3-D0
  -> closed: worker authority split; resolver/source-view publishes AST-free
     syntax facts, while policy owns operator/type/overflow/monotonicity
D4-S4-S3-S0
  -> closed: cfg(test)-only V2-consuming witness copies as-written syntax
     facts into a move-only AST-free product; six focused tests; no public row
D4-S4-S3-D1
  -> closed: worker consensus assigns numeric_substrate exact type/range,
     resolver source bridge provenance, and loop_route_policy progression policy
D4-S4-S3-S1-D0
  -> closed: NoSafeSlice design stop; typed literals and resolver parameter
     types are not co-sealed, so no source-type proof is claimed
D4-S4-S3-S1-S0
  -> closed: cfg(test)-only exact source-unit receipt/map witness; no selector/
     demand/Recipe/MIR
D4-S4-S3-S1-S1
  -> closed: cfg(test)-only non-Clone receipt co-seals typed syntax facts and
     the owner-branded map, verifies provenance/BindingRef coverage, and leaves
     numeric policy downstream. Six focused tests are green; no selector,
     demand, Recipe/Builder/MIR caller, retry/fallback, or public row.
D4-S4-S3-S1-S2-D0
  -> design accepted: keep a substrate projection (exact type/width/sign/
     range/overflow) separate from a later policy adapter (operator/progression).
     Both are cfg(test)-only and non-Clone; dispositions are Ready/Unresolved/
     Rejected. Minimum six-boundary witness precedes any selector/demand/
     Recipe/Builder/MIR caller. Implementation updates current state, workstream,
     support README, and later `docs/reference/**` at production activation.
D4-S4-S3-S1-S2-S0
  -> closed: cfg(test)-only substrate projection consumes the co-sealed receipt
     plus explicit NumericTarget. Missing/unknown annotations remain
     Unresolved; typed range failures are Rejected; exact rows are Ready. Six
     focused tests are green; no policy/selector/demand/Recipe/Builder/MIR.
D4-S4-S3-S1-S2-S1-D0
  -> design closed: policy consumes the sealed substrate projection exactly
     once, keeps Condition-Rhs/Step-Rhs roles explicit, and owns only admitted
     comparison/update progression. Output remains move-only Ready/Unresolved/
     Rejected; no selector, demand, Recipe, Builder/MIR, or public row.
D4-S4-S3-S1-S2-S1-S1
  -> closed: cfg(test)-only policy witness; seven focused tests cover Ready,
     symbolic/unsupported/non-progressing Unresolved, direction/role rejects,
     and source/provenance retention.
D4-S4-S3-S1-S2-S2-D0
  -> superseded by the single shallow `GENERIC-SELECTION-POLICY-HANDOFF-D0`
     design boundary; do not create a deeper D4 suffix or a separate adapter.
D4-EVIDENCE-EXIT0
  -> deep D4 evidence exit: all current substrate/policy receipts and negative
     matrices are closed without a production caller; no more D4 suffixes are
     authorized unless a new design stop supersedes this gate.
GENERIC-SOURCE-TO-PORTABLE-RECIPE-D0
  -> accepted and taskized. The canonical G0 has explicit `: i64`, plain
     literals typed only through exact BindingRef context, two source bindings
     but three recurrence carriers `(L0,i)`, `(L0,j)`, `(L1,j)`, and a derived
     child-entry read immediately before L1. JoinSig now proves the common
     nested-carrier shadow/visibility rule, and route-independent
     `producer_id: LoopRecipeProducerIdV1` provenance and common logical
     Header/After binding identity and the common source-bound core relation
     schema/verifier are closed. The caller-zero core co-seals only already
     verified Recipe/JoinSig/source products; only S4 issues real G0
     keys/relations and no selector or production route is opened here.
     Legacy `LoopRouteId` parity stays in an external migration receipt. The post-loop
     `return j` remains a separate completion envelope consuming a common
     `VerifiedLoopAfterBinding`; it is never encoded as an inner Loop exit.
     G0 selection has a closed five-row overlap/admission window; the rows are
     migration profiles, not five semantic Loop kinds, and whole-unit
     NoCandidate remains closed until `JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G`.
     Current `family_selection.rs` is
     only a test marker and S2 promotes that boundary rather than reusing the
     legacy 19-route evaluator. See
     `design/generic-loop-source-to-portable-recipe-ssot.md` for the sole
     mapping, finite shallow task order, module homes, checked legacy manifest,
     and deletion boundary. S0A/S0B/S0C and the caller-zero Generic candidate
     S1 are now landed; `LOOP-FAMILY-DIRECT-OBSERVATION-S1` is also landed as
     a caller-zero source/policy observation; the current code row is the
     caller-zero `GENERIC-SELECTION-POLICY-HANDOFF-I0-R0` implementation.
```

For the later V2 issuer, no caller role list is accepted. The issuer derives
and verifies only this inner-loop profile: condition=`loop/LoopCondition`,
induction=`condition/Lhs`, bound=`condition/Rhs`; step target comes from the
carrier's nested-write site, its parent assignment supplies `Value`, and the
value's `Lhs` read plus `Rhs` delta are checked against the same `BindingRef`.
Syntax facts copy as-written operators/literals and remain AST-free; accepted
operator/type/overflow/monotonicity and temporal coverage are separate policy/
coverage owners. Missing inventory, foreign identity, wrong placement,
upvar/capture, binding mismatch, or unproven order reject before publication.
No AST/name scan, selector, demand, Recipe, Builder/MIR, retry, fallback, or
production caller may be added in these cells.

The callable full physical canary is now closed caller-zero. The next G0
parity boundary is split cleanly: `LOOP-CALLER-ZERO-PARITY-G0-D0` accepts a
compiler-side composite of the exact resolver input plus the neutral S4
product. `LOOP-CALLER-ZERO-PARITY-G0-I0-R0` is now closed after splitting G0
After into neutral continuation plus distinct tail capability, issuing that
composite, and proving common fifteen-row `prepare_all` with Builder effect
zero. The S4
Recipe/effect/After owner remains neutral; no AST reconstruction, second
resolver, profile relabel, physical emission, selector, retry, fallback, or
production caller is opened.

The bounded I0 ingress is now landed in
`src/mir/compiler/generic_g0_physical_prepare.rs` and remains `cfg(test)`.
It validates the exact resolver/source/forest/entry/tail relation, preserves
G0's post-loop read and I64 ABI in a distinct tail capability, and proves the
full fifteen-item common preflight. Positive, missing-input, foreign-input,
and After/Tail separation tests are green; physical G0 and production
cutover remain later rows.
The next design-only boundary is
`LOOP-CALLER-ZERO-PARITY-G0-I1-D0` for a fresh unpublished session.

## Structural owners

| Owner | Owns | Must not own |
| --- | --- | --- |
| `LoopStructuralFactsV1` | source/control observations and stable provenance | route policy, MIR IDs, emission |
| `CanonicalLoopFamilySelectionV1` | G0 admission at S2; complete all-route selection only after `JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G` | raw route order/cursors, Builder probes, physical execution, retry |
| `LoopRecipeArtifactV1` | owned provenance plus one recursive `LoopRecipeV1` | route/family dispatch, AST reconstruction, physical IDs |
| `LoopRecipeV1` | condition, recursive body/control items, carriers, exits | legacy family, policy selection, physical IDs |
| `LoopRecipeVerifierV1` | recipe shape, definitions, carrier/exit preconditions | logical edge elaboration, repair, fallback |
| `LoopJoinSigElaboratorV1` / `LoopJoinSigV1` | bounded logical edge/dataflow and carrier-visibility obligations | PHI allocation, CFG repair, physical identities |
| `LoopCfgSkeletonLoweringV1` / `CanonicalCfgSessionV1` | physical blocks/edges/terminators and sealed predecessor witnesses from verified roles | route selection, AST reading, Binding SSA values |
| function-owned `BindingSsaBuilderV1` | `BindingRef` reaching values, Read/Write definitions, provisional PHI, seal/finish | source inference, route policy, JoinSig discovery |
| `PhiTxn` + `MirBindingSsaAdapterV1` | the single low-level PHI insert/patch/rollback lifecycle used by Binding SSA | Loop meaning, route selection |
| `LoopPhiMaterializerV1` | caller-zero mechanical M6-B PHI-map observer/parity evidence only | production PHI authority, source inference, route-specific repair |
| `LoopPhysicalizerV1` | one terminal candidate mutation | `Option`, retry, raw suffix, publication |
| compile candidate | whole-compile abort and success-only external commit | Loop meaning or policy |

Names may be shortened during implementation. Ownership boundaries may not.

## Current evidence

- Production registry membership is exactly 19 ordered routes.
- Legacy first-mutation profiles are `11/1/1/4/2`: LoopV0, Nested, LoopTrue,
  LoopCond, Generic. They describe where old physical lowerers first mutate,
  not five semantic Loop kinds and not five portable recipe variants.
- `lower_loop_v0` allocates frame/PHI/ValueIds before later header/body failure.
- The existing compile session uses a fresh candidate Builder and publishes it
  only after whole-module success.
- Current `Retry` mixes pre-effect decline with post-effect lower debt.
- Generic V0/V1 explicitly convert verifier/lower failures into post-effect
  continuation; this is the mandatory long pole.
- Current RecipeTree builders reconstruct substantial synthetic AST and are
  parity oracles, not the selfhost-ready final recipe boundary.

## Ordered task map

### Prelude — `JOINIR-LOOP-PREFLIGHT-TEST-EXTRACT0-C0`

Change:
: Move core tests out of the 797-line live preflight parent.

Contract:
: Physical move only; no policy, acceptance, retry, Builder, or runtime change.

Done:
: Production file has comfortable headroom; focused preflight/registry, pointer,
  and MirBuilder structural gates are green.

Stop:
: Do not mix semantic helper extraction into this commit.

### M1 — `JOINIR-LOOP-COMPILE-CANDIDATE-ABORT0-P0`

Change:
: Census every normal/raw/reference/REPL/canonical production path from public
  compiler ingress through the sole `route_loop` call and external commit.
  Inventory semantic writes outside `MirBuilder` too.

Contract:
: Every Loop mutation runs inside an unpublished compile candidate. Live
  `compiler.builder` direct Loop callers and unowned ambient semantic writes are
  zero. Logging-only observation is not publication.

Done:
: Injected current Loop failure drops the complete candidate, leaves the live
  compiler unchanged, and permits a fresh request on the same compiler.

Stop:
: Move any outside caller into the existing compile transaction first. Never
  create a Loop-local candidate as a shortcut.

M1 proof bundle (closed with the focused gates below):

- `M1-A` static scope manifest: pin normal/default, REPL, vm-hako reference,
  Raw public/reference, and canonical ingress. Normal/default, REPL, and
  vm-hako reference are the positive Loop-reaching family and converge on
  `ModuleBuilderInvocationSessionV1`; Raw and canonical rows are explicit
  typed-unreachable-before-Loop rows, not silent omissions.
- `M1-B` positive behavior: inject a deterministic failure after the first
  Loop physicalization effect, compare a full live-Builder fingerprint, drop
  the whole candidate, and compile a fresh request on the same compiler. The
  fingerprint covers source/config, all five core cursors, current
  module/function/block, function/scope state, and publication state.
- `M1-C` negative behavior: Raw public/reference and canonical Script/Main
  Loop-shaped inputs reject before candidate opening or direct-live mutation;
  their existing pre-effect rejection is fixed as a reachability contract.
- `M1-D` ambient inventory: invocation identity allocation is an owned
  monotonic non-publication write and is not required to roll back. Planner
  reject/parity TLS is diagnostic/control transport; nested fast-path depth is
  RAII-scoped. Any new semantic ambient write is a blocker until it has an
  owner, reset/unwind law, and fixture.

M1 acceptance gates:

```text
route_loop production caller = 1
try_cf_loop_joinir definition/caller = 1/1
lower_loop_or_freeze_v1 callers = 2 typed child ports
all positive Loop ingress -> existing unpublished candidate -> success-only commit
Raw/canonical Loop ingress -> typed pre-effect unreachable
unowned semantic ambient writes = 0
M1-B full-fingerprint late Loop failure/reuse = green
```

The existing shared MirBuilder guard owns `M1-A` topology/manifest checks.
It does not claim dynamic rollback; `M1-B` is the required behavior proof.
Identity ordinals and diagnostic TLS are explicitly excluded from the live
Builder immutability claim. The M1-B fixture is now green, so M1 is closed.
M2-S0 is also closed by the portable-contract proof bundle below; M3 is the
next design stop.

### M2 — `JOINIR-LOOP-PORTABLE-RECURSIVE-RECIPE0-S0`

Design stop: `D0-R1` is closed after independent worker review. The selected
portable contract is a new neutral owner at `src/mir/loop_recipe_contract/`; it
is not a wrapper around the existing Builder recipe tree. This revision
explicitly demotes the old `11/1/1/4/2` first-mutation profiles from semantic
families to migration receipts.

Source authority:
: The selected RoutePolicy winner supplies owned source provenance and
  normalized operands. A producer receipt may carry the existing neutral
  `LoopRouteId` for diagnostics/parity only; the verified semantic recipe must
  not dispatch on it. Existing `LoopRouteId` is behavior-neutral identity and
  must move to, or be re-exported from, the neutral owner before portable
  producers consume it.

Non-authority:
: `RecipeTree`/`RecipeBody`/`StmtRef`/`CondBlockView`, `CorePlan`/`Frag`,
  `BasicBlockId`/`ValueId`, `MirBuilder`, route composers, PHI builders, AST
  reconstruction, callbacks, retry, and opaque legacy-emission commands.

Fail-fast boundary:
: `LoopRecipeArtifactV1` contains versioned owned provenance and one
  `LoopRecipeV1`. The semantic recipe is a structured recursive product:
  condition (`Always` or `Predicate`), ordered body items (`Operation`, `If`,
  nested `Loop`, and typed `Exit`), carriers, and exit declarations. Recipe
  keys are contiguous and local; the operation subset is Accum-ready only.
  Logical JoinSig is an elaborated/verified obligation, not a second route
  language and never a physical CFG. Legacy first-mutation family is excluded
  from the wire contract; if migration diagnostics need it, keep it in a
  Rust-only receipt outside the semantic recipe. Sibling disposition and
  validation-reason types remain separate; M3 owns policy declines and M4 owns
  Generic post-effect debt.

Canonical form:
: Encode as fixed-field structs and ordered arrays with stable snake-case tags;
  never depend on map iteration or allocation order. Recursive items use
  preorder recipe-local contiguous keys. Conditions, exits, carriers, and join
  payloads are typed products rather than a boolean feature bag, so invalid
  combinations are rejected instead of represented as contradictory flags.
  Logical keys are new `u32` newtypes and never physical MIR IDs. Existing
  source-site objects are projected to portable relative paths rather than
  borrowed across the boundary.

S0 execution brief:

Change:
: Add the disconnected neutral recursive DTO (`LoopRecipeArtifactV1` /
  `LoopRecipeV1`), canonical normalizer, structural validator, typed rejection
  reasons, and one Accum golden. Move/re-export only the neutral route identity
  needed by the producer receipt; do not connect a production caller.

Contract:
: Rust and `.hako` later consume the same normalized data contract. M2 has no
  AST producer, route selection, Generic classification, CFG/PHI allocation,
  physicalization, or publication authority.

Done:
: One valid recursive golden completes decode → validate → normalize → encode →
  decode equality, including one nested Loop item and typed exits/carrier
  closure. Malformed fixtures reject duplicate/dangling local keys, invalid
  condition/exit/carrier references, noncanonical order, and unsupported
  versions. All tests are Builder-free and touched files stay below 800 lines.

Stop:
: Do not infer all 19 operation vocabularies, add `EmitLegacyRoute`/opaque
  statement commands, put physical IDs or borrowed facts in the DTO, encode a
  fixed five-block CFG/terminator table, or let route/family names drive
  verification/physicalization. Do not advance to M3/M4/M5 before this
  contract is green.

M2-S0 proof bundle (closed 2026-08-03):

```text
neutral owner = src/mir/loop_recipe_contract/
production callers = 0
typed per-Loop source path = green
explicit input/value-definition closure = green
carrier entry availability = green
recursive loop/block/item preorder = green
duplicate arena-row counterexample = green
portable contract tests = 18/18 green
route registry tests = 80/80 green
shared MirBuilder guard = green
current-state pointer guard = green
release hakorune build = green
largest touched Rust file = verify.rs (688 lines)
independent close audit = blocker 0
```

The carrier proof here ends at entry availability. Dominance plus backedge and
exit closure remain M5/M6 JoinSig/verifier work; M2 does not claim them.

### M3 — `JOINIR-LOOP-STRUCTURAL-FACTS-ROUTE-POLICY0-P0-S1`

Change:
: Replace the borrowed, execution-coupled route decision surface with one
  owned structural observation and one pure ordered policy. M3 is split below
  because source identity, row freezing, outcome typing, and winner parity are
  distinct authorities.

Contract:
: `CanonicalLoopFacts` remains a legacy adapter input, not the portable DTO:
  it owns AST-bearing family facts. The neutral policy owner imports no AST,
  Builder, composer, CorePlan, physical ID, LoopRecipe, callback, retry, or
  `all_route_preflight`. Raw execution order comes only from the already-frozen
  legacy selection. Route IDs live only in an opaque parity receipt; qualified
  structural facts and the semantic recipe never dispatch on them.

Done:
: Non-Generic decidable fixtures agree on frozen raw schedule, selected
  route-or-exhaustion, and typed prefix reasons. Generic V0/V1 stop at an exact
  M4 debt; full all-19 winner and recipe parity are not claimed before M4/M8.
  Partial coverage remains caller-zero.

Stop:
: An execution-dependent decision becomes an exact named blocker, never
  `Unknown`, suffix skipping, runtime probing, or a new scheduler. Policy may
  not read legacy recipe-contract presence as semantic authority: that bit is
  compatibility parity only because the final order is Facts → Policy → Recipe.

M3 task order:

1. `M3-A / JOINIR-LOOP-POLICY-OUTCOME-CENSUS0-P0` — **closed** by the
   independent multi-worker code-arm audit. All 19 `ENTRIES` rows have
   predicate, suppression, outer/route gate, Retry/None stage, first-effect
   owner, and retirement class. Exact gaps are: portable root source path missing;
   TrueBreak plus Cond4 source topology missing; late AST policy rereads;
   NestedMinimal composer dependence; non-Generic `Option` terminality not
   sealed; Generic V0/V1 post-effect retry debt.
2. `M3-B / JOINIR-LOOP-ROOT-SOURCE-AUTHORITY0-D0-S1` — **closed**. Issue one owned root
   `LoopSourcePathV1` before route-local observation. Map the 12 existing
   body-topology receipts beneath it; ScopeBox lineage is either represented by
   an accepted typed path extension or fails closed. Do not infer a root path
   from route-local body indices.

   M3-B close acceptance (not a new row or a new IR/route):

   - Portable artifact JSON, including a structurally verified
     `StructurallyVerifiedLoopRecipeSourceClaimV1`, is a portable source claim; it is not
     local source authority. Only the separately named, sealed exact-loop lookup
     capability `VerifiedResolvedLoopSourceV1` is local source authority.
   - The compact path grammar is closed: the first step is `BodyItem`, and every
     later step is `ScopeBodyItem` or `LoopBodyItem`. A semantic child Loop path
     is exactly its semantic parent's full prefix, one `LoopBodyItem`, then zero
     or more `ScopeBodyItem` steps; it cannot skip an intermediate Loop.
   - The current issuer accepts only `DeclaredFunction`. `Program` remains a
     typed unsupported owner until a separate sealed producer is approved.
   - Semantic Recipe, CFG, JoinSig, PHI, and physicalization consumers cannot
     read source binding, provenance, or either source-authority capability.
   - Done requires one adapter-to-artifact verification fixture: consume the
     sealed exact-loop token, project the portable claim, bind it to the recipe,
     and pass full artifact verification end to end.

   Close proof (2026-08-03): sealed exact-loop local capability is non-Clone;
   portable JSON proves only a private structural claim; Function owner and
   compact path grammar are closed and fail typed on unsupported ancestry;
   production callers are zero; artifact fields are contract-private; the E2E
   fixture is green. Contract/source/resolved tests are 32/10/11, registry is
   80, shared/pointer guards and release build are green, largest Rust file is
   703 lines, and two-stage independent close audit reports blocker zero.
3. `M3-C / JOINIR-LOOP-FROZEN-POLICY-ROWS0-S2` — **closed**. The sibling
   neutral owner `src/mir/loop_route_policy/` seals a non-Clone 19-row schedule
   with one owned observation snapshot; typed rejects cover order, counts,
   duplicate/suffix, inconsistent global/mode facts, and empty suppression.
   The fixture adapter is `cfg(test)`-only and the facade has zero production
   callers. Focused tests 10/10, shared/pointer guards, diff/line checks, and
   release build are green; selection/retry/recipe/Builder/lowering remain out.
4. `M3-D / JOINIR-LOOP-PREFIX-OUTCOME-TYPING0-S3` — **closed**. Typed
   pre-effect Declined/Blocked versus Generic debt, selected non-Generic Loop
   terminal boundaries, and `JOINIR-LOOP-GENERIC-COMPOSER-RESULT-RECEIPT0-S3a`
   closed V0/V1 composer/result evidence without entering policy.
5. `M3-E / JOINIR-LOOP-PURE-ROUTE-POLICY0-S4` — **closed**. E0 owns a closed
   policy-evidence DTO (`SourceDeclined/Candidate/PolicyBlocked/GenericDebt`)
   without route-ID dispatch; E1 evaluates frozen rows left to right. Declined
   alone advances; Qualified/Blocked stop, and Blocked has no resume/suffix API.
   Qualified owns facts plus a private seal, never recipe, callback, or receipt;
   Generic remains an opaque M4 debt key.
6. `M3-F / JOINIR-LOOP-CALLER-ZERO-POLICY-PARITY0-P1` — **closed**. A
   `cfg(test)` adapter compares the actual isolated legacy witness scheduler
   with the pure audit; `all_route_preflight` is not an oracle. It fixes a
   non-Generic typed-decline→success mechanism fixture, all-declined
   exhaustion, and a fresh row-zero Blocked stop. This is not evidence for a
   Generic V0/V1 debt-to-success edge. Generic debt stays M4-only. Production
   Recipe callers, PHI owners, fallback removal, and second schedulers remain
   zero; those belong to M5/M6/M10.
7. `M4 / JOINIR-GENERIC-POST-EFFECT-DEBT-CLASSIFICATION0-D0-S0` — **active**
   at the D2-B4-S2 BindingRef disjointness design stop after D2-B4-S1
   closeout. Resolve the parent Generic D2 winner/disjointness disposition
   described in the dedicated M4 card before any Generic Recipe production
   connection.
Docs-only role cleanup authorizes no new IR, recipe variant, route, scheduler,
retry path, or physical owner. M3 keeps selection/decline typing, M4 owns
Generic debt, M6 owns logical JoinSig obligations and caller-zero mechanical
evidence, while canonical CFG plus function-owned Binding SSA/PhiTxn own
production physical PHI/SSA. M12 retires adapters after M10/M11 evidence. New
policy files remain below 800 lines.
### M4 — `JOINIR-GENERIC-POST-EFFECT-DEBT-CLASSIFICATION0-D0-S0`

Change:
: Follow the dedicated
  `joinir-generic-post-effect-debt-classification-ssot.md` card. Enumerate
  facts, composer, strict/release verifier/lower, nested, and receipt stages;
  assign each to a closed target disposition; and settle V0/V1 overlap and
  precedence through a production-derived execution fixture or a complete
  disjointness proof.

Contract:
: M4 is design/test-only. `PreEffectDeclined`, `PreEffectBlocked`,
  `TerminalFreezeTarget`, `ImpossibleEdge`, and `UnresolvedStop` are evidence
  dispositions, not new production Recipe variants. Post-effect retry remains
  a legacy receipt boundary until a later atomic cutover. No Generic Recipe,
  JoinSig, PHI, physicalizer, candidate publish, or JoinIR deletion is claimed.

Done:
: Every Generic V0/V1 stage has an explicit disposition and first-effect
  owner; V0/V1 precedence is fixed; and the actual legacy witness winner is
  equivalent to the target policy, or a closed predicate proves the simultaneous
  V0/V1 edge impossible. Legacy behavior and production callers remain
  unchanged.

Stop:
: If the winner cannot be selected before the first effect, or any stage is
  `UnresolvedStop`, keep the old scheduler/receipt path and do not advance to
  Generic Recipe production. Do not add a Loop-local Builder, undo journal,
  symbolic MIR replacement, or another route-by-route proof loop.

### M5 — `JOINIR-LOOP-ACCUM-PORTABLE-RECIPE0-S3`

Change:
: Before M6, execute only the design/test-only Accum contract row: real
  StructuralFacts/frame/raw-selector evidence, passive owned Recipe
  verification, and caller-zero guards. The complete vertical pilot begins
  after M6 establishes the logical JoinSig obligations and caller-zero
  evidence; production CFG/PHI/SSA remains owned by the canonical CFG session
  and function-owned Binding SSA.

Contract:
: The passive row consumes no Builder, composer, physical ID, PHI, candidate,
  or `route_loop`. The later pilot consumes logical roles through the
  canonical CFG and function-owned Binding SSA owners; JoinSig supplies only
  edge/carrier-visibility obligations. The existing Accum composer is a parity
  oracle only. Control edges remain explicit recipe items; value-only
  calculations remain operations, and called bodies remain separate recipes.

Done:
: The D0 row proves the real Accum singleton schedule, no Generic suffix,
  passive Recipe verification, fresh-repeat stability, and zero production
  callers. Named post-M6 pilot `JOINIR-LOOP-ACCUM-VERIFIED-RECIPE-CONSUMER0-P1`
  must additionally prove MIR/PHI/type/result parity, late-failure discard,
  and fresh reuse.

Stop:
: Do not import synthetic AST or current physical composer into D0, and do not
  mutate a candidate before M6. M6 must promote the later pilot into shared
  services; it must not leave a second Accum-only implementation beside them.

### M6 — `JOINIR-LOOP-CFG-JOINSIG-PHI0-S4`

Change:
: Establish one caller-zero logical-to-SSA chain: verified Recipe -> JoinSig
  obligations -> CanonicalCfgSession -> one function-owned Binding SSA -> one
  shared PhiTxn, with no production caller. M6-A remains pure logical
  elaboration. M6-B remains a mechanical PHI-map observer only and is not the
  production PHI owner.

Contract:
: M6-A's first caller-zero slice has no Builder/physical IDs and elaborates the
  bounded Accum vocabulary: deterministic logical ports/edges, value
  availability, owner/ancestor carrier payloads, self exits, and unreachable items;
  nested loops are `Always`-only. Full predecessor/dominance and wider branch/exit closure remain explicit M6-A follow-ups.
  The caller-zero M6-B `LoopPhiMaterializerV1` consumes only verified JoinSig
  plus a sealed logical-to-physical witness for mechanical parity; AST,
  route/env, `variable_map`, tags, and repair inference are forbidden. It is
  not a production PHI/SSA authority and must not be extended into the P1
  operation emitter. Production physicalization uses one
  `CanonicalCfgSessionV1`, one function-owned `BindingSsaBuilderV1`, and one
  caller-owned `PhiTxn` through `MirBindingSsaAdapterV1`.

Done:
: Deterministic Accum JoinSig/counterexamples are green; the logical owner is
  caller-zero with a non-Clone product; M6-B mechanical evidence is green
  (the focused `loop_phi_materializer` suite is 33/33 and the structural
  MirBuilder guard is green); route-specific block/PHI callers, physical IDs,
  and production wiring remain zero. `LoopPhiMaterializationReceiptV1` is
  non-Clone and the materializer remains an observer, not a second PHI owner.
  The first physical pilot must prove the Binding-SSA-first session rather
  than promote M6-B into a second PHI owner. The structural P1b edge-path
  witness is now closed as caller-zero evidence; P1b-4b full physical parity
  remains deferred to the accepted P4 physical-snapshot design stop.

Stop:
: Bypass, production wiring, route-local AST/PHI inference, duplicate writer, or temporary repair stops M6.

### M7 — `JOINIR-LOOP-RECURSIVE-RECIPE-CLOSURE0-S5`

Change:
: Establish one caller-zero producer facade, then add the five legacy
  first-mutation profiles as sequential cohorts:
  LoopV0=`AccumConstLoop`, Nested=`NestedLoopMinimal`,
  LoopTrue=`LoopTrueBreakContinue`, LoopCond=`LoopCondBreakContinue`, and
  Generic=`GenericLoopV1` only after M4. Every accepted producer emits the
  same recursive semantic `LoopRecipeV1`; M7 does not require a source-bound
  artifact until a sealed root+child source witness exists.

Contract:
: The five profiles are migration adapters only and are added one cohort at a
  time behind the same sealed demand. They share one portable recursive recipe
  envelope, verifier/elaboration terminal, JoinSig obligations, canonical
  CFG/Binding-SSA/PhiTxn services, and one physicalizer. PHI/SSA remains owned
  by the existing canonical CFG/Binding-SSA/PhiTxn chain; M6-B's materializer
  is caller-zero evidence only. A profile-specific semantic
  variant is forbidden unless a concrete source counterexample proves a
  bounded vocabulary extension; a sixth legacy profile is never a new semantic
  recipe kind. Call and completed Record construction may be value operations;
  branching Match/If/Loop are control items, while lambda/function bodies are
  separately owned recipes. Future expression-valued control requires an
  explicit block-result/join vocabulary, never AST lifting or opaque payloads.
  The real `NestedLoopMinimal` cohort has a root and child `Predicate`, a
  recurrence-owned child `j`, and an ancestor `sum` update; its bounded
  caller-zero JoinSig closure is tracked by
  `joinir-loop-nested-predicate-closure-d2-b-execution-2026-08-03.md`.
  The nested-`Always` golden remains an M6 logical witness only and is not
  NestedLoopMinimal source parity.

Done:
: The facade and each accepted cohort have deterministic normalized
  Recipe/JoinSig parity and caller-zero guards. Physical MIR/candidate-abort
  evidence is reused from the existing P1/M5 harness; family adapters cannot
  select, retry, or publish. LoopTrue/LoopCond require a shared logical
  branch/merge closure, Nested source-bound parity requires a root+child source
  forest witness, and Generic requires the M4 debt/winner gate.

Stop:
: Generic may not be omitted/mocked; Nested inner semantics may not be inferred
  from outer provenance or hand-built source paths. Conditional exits may not be
  collapsed to direct exits, and a missing JoinSig branch/merge vocabulary opens
  a shared design stop rather than an adapter-local workaround.

#### M7-S2-A — `JOINIR-LOOP-TRUE-BRANCH-EXIT-CLOSURE0-M7-S2-A-S0` (closed)

Change:
: Extend only the caller-zero logical JoinSig owner for one root `Always` Loop
  whose sole body item is an explicit-else `If`: the then arm directly breaks
  the owner Loop and the else arm directly continues it. Keep the Recipe V1
  schema unchanged. Keep the `join_sig/mod.rs` facade thin and split the
  branch/dataflow responsibilities into its child modules rather than growing
  the JoinSig subtree toward the 800-line limit.

Contract:
: `LoopJoinSigElaboratorV1` is the unique issuer. One ordered logical branch
  row retains the If item, condition, then/else arm identity, exit item, target
  Loop, and logical payload. The owning Loop row receives Break and Continue
  edges and no natural Backedge. Recipe tree remains connectivity authority;
  JoinSig owns only arm/transfer/dataflow obligations. No AST, `StmtRef`,
  `RecipeBody`, Builder, CorePlan, physical ID, PHI, Retry, or fallback enters
  the portable subtree.

  The selected logical shape is:

  ```text
  LoopJoinSigV1
    loops
    branches: Vec<LoopJoinBranchV1>

  LoopJoinBranchV1
    owner_loop
    if_item
    condition
    then_exit: LoopJoinBranchExitV1
    else_exit: LoopJoinBranchExitV1

  LoopJoinBranchExitV1
    exit_item
    role: Break | Continue
    target_loop
    payload
  ```

  An internal `BlockFlowSummaryV1` may retain fallthrough versus ordered exits
  while elaborating, but S2-A publishes a branch row only when both arms are
  direct supported exits. It never mints a physical or recipe value.

Done:
: `LoopJoinSigV1.branches` now elaborates one deterministic ordered branch row
  with Break then Continue arms, no merge obligation, and no Backedge.
  Focused caller-zero tests cover the positive shape, determinism,
  implicit-else fallthrough, divergent branch writes, and Return-arm rejection.
  Producer/physical production callers and old-edge deletion remain zero;
  `cargo test loop_recipe_contract --lib`, diff checks, and touched-file line
  budgets are green.

Stop:
: Do not widen S2-A to mixed fallthrough or binding merge. Those require a
  later S2-B obligation product. Do not add the LoopTrue source projector or
  policy/Recipe producer before this logical closure is green. Implementation
  closeout must update `src/mir/loop_recipe_contract/README.md` and create or
  update `docs/reference/mir/loop-recipe-contract.md` after code, fixtures,
  and guards land. That reference must state the exact supported envelope,
  ordered branch-arm obligations, caller-zero status, and non-claims. Since
  S2-A changes no physical PHI contract, it must not claim production PHI
  support in `phi_policy.md` or `phi_invariants.md`; update those only after a
  later physical adoption changes their contract.

#### M7-S3 — `JOINIR-LOOP-TRUE-SOURCE-RECIPE-COHORT0-M7-S3-D0` (closed)
Decision: accepted after independent worker audits; source projection S0,
policy demand S1, caller-zero Recipe/JoinSig parity S2, and the required
reference closeout S3 are closed. The next ordered blocker is the active
Generic M4 debt classification.
Source authority:
: Resolver-owned `VerifiedResolvedLoopSourceV1` plus one semantic source
  traversal issues an AST-free `VerifiedLoopTrueBreakContinueProjectionV1`.
  `PreparedLocatedRawLoopChildEntryV1` is only a transport receipt, never a
  Recipe issuer or second source authority.
Policy owner:
: `loop_route_policy` consumes the sealed projection and the frozen schedule.
  A profile brand accepts only `LoopRouteId::LoopTrueBreakContinue`; raw cursor
  data remains opaque migration provenance and never dispatches lowering.
Recipe issuer:
: A dedicated `LoopTrueBreakContinueRecipeProducerV1` consumes the
  profile-specific policy demand, emits the existing `LoopRecipeV1` envelope,
  verifies it, and calls the existing logical JoinSig elaborator; it must not
  reuse DirectAccum facts or mint a generic unbranded demand.
Fail-fast envelope:
: Reject missing/mismatched source frame, implicit else/fallthrough, nested
  control, Return/call/effect, branch writes, duplicate/partial rows, wrong
  policy winner, or verifier mismatch before Builder effects. `Option`, retry,
  suffix reconstruction, fallback, AST payloads, physical IDs, CFG/PHI, and
  route switches are forbidden.
Execution tasks:
: S0 is closed with resolver-owned source projection and source-identity/frame rejects. S1 is closed with one-consume schedule admission and a profile-specific policy receipt. S2 is Recipe -> verifier -> JoinSig caller-zero parity, and S3 is closed with the mandatory reference documentation and guards. Use the
  phase143 fixture plus implicit-else, divergent-write, Return, wrong-frame,
  and wrong-winner negatives; run focused tests, pointer/in-place/R4 guards,
  diff checks, and the <800-line budget.
Reference closeout task (`JOINIR-LOOP-TRUE-REFERENCE-CLOSEOUT0-M7-S3-S3`, closed):
: After S2 implementation, updated
  `src/mir/loop_recipe_contract/README.md` and
  `docs/reference/mir/loop-recipe-contract.md` with the source projection,
  policy brand, exact envelope, caller-zero status, and non-claims. Do not
  update `phi_policy.md` or `phi_invariants.md` unless physical adoption later
  changes their contract. LoopCond remains behind this cohort.

### M8 — `JOINIR-LOOP-ALL19-PORTABLE-RECIPE0-S6`

Change:
: Close the remaining 14 legacy-ingress rows as bounded adapter cohorts: LoopV0
  recurrence, LoopV0 exits/joins, LoopV0 scans, LoopCond exits, and Generic
  V0. Add only missing source observation or portable vocabulary. Each route is
  one producer data row/golden plus one typed source-policy observation for the
  same recursive recipe, not a new selector, verifier, CFG, PHI, or
  physicalizer authority.

Ordered rows:
: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A` ->
  `JOINIR-LOOP-M8-LOOPV0-EXITS-JOINS-S6B` ->
  `JOINIR-LOOP-M8-LOOPV0-SCANS-S6C` ->
  `JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D` ->
  `JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E` ->
  `JOINIR-LOOP-M8-ALL19-CLOSEOUT-S6G`. Each producer cohort is one
  implementation-coupled commit. S6G is an implementation-coupled closeout:
  it seals `VerifiedLoopAllRouteObservationSetV1` plus the whole-unit coverage
  proof into the same `CanonicalLoopFamilySelectionV1` introduced at S2 and
  opens its `NoCandidate` result. It does not create a second selector and its
  production caller remains zero.

Contract:
: 19/19 legacy ingress rows have typed pre-effect decline or one verified
  instance of the common recipe algebra. Legacy winner
  equivalence includes first-None-then-later-success. Selected physicalization
  has no `Option`, suffix, retry, or fallback. Raw route IDs/cursors remain
  migration evidence and cannot select through the all-route input.

Done:
: Every accepted legacy-ingress fixture produces a verified common Recipe and
  parity MIR. The canonical semantic Loop kind count remains one. Unverified direct
  lower, cloned-AST recipe reconstruction, post-effect continuation, and
  duplicate CFG/PHI authorities are zero in the new subtree. The M10 deletion
  manifest covers every old route callback, composer, Retry/continuation, and
  Loop-specific PHI/repair edge selected for same-commit retirement.

Stop:
: An unfitting route opens one bounded vocabulary extension, never a compatibility
  escape or route-local physicalizer.

### M9 — `SELFHOST-LOOP-PORTABLE-RECIPE-PARITY0-S7`

Change:
: Implement the same StructuralFacts/RoutePolicy/Recipe/JoinSig producer in
  `.hako` over the existing Program/AST JSON boundary.

Ordered rows:
: `SELFHOST-LOOP-PORTABLE-WIRE-S7A` -> five stable
  `SELFHOST-LOOP-M8*-PARITY-S7B1..S7B5` producer rows ->
  `SELFHOST-LOOP-PORTABLE-ALL19-PARITY-S7G`. Each cohort remains one commit.

Contract:
: Rust and `.hako` share the data contract and verifier expectations, not host
  callbacks. The `.hako` side emits the owned recipe and Rust verifies both
  normalized products. Rust remains the thin allocation/emission terminal until
  the later SH lane.

Done:
: Route ID, prefix reasons, logical roles, JoinSig, verifier result, and
  normalized recipe match for all 19 fixtures; selfhost quick, representative
  identity, and no-hostbridge gates are green.

Stop:
: Do not widen language semantics or Rust source recognition to force parity.
  Do not claim `.hako` verifier, CFG/PHI, physical MIR, or default authority.
  The no-hostbridge claim covers the portable producer subtree, not unrelated
  compatibility providers elsewhere in the canonical builder.

### M10b — `JOINIR-LOOP-PORTABLE-RECIPE-CUTOVER0-I0-R0` (final; optional M10a bridge is separate)

Entry row:
: `GENERIC-M10B-DELETION-MANIFEST-S0` freezes the exact current symbols and
  caller counts immediately before cutover. The atomic commit uses that checked
  manifest; names copied from historical docs are never deletion authority.

Change:
: Switch `route_loop` to frozen source -> StructuralFacts -> one policy winner
  -> verified recipe -> one candidate physicalizer.

Delete in the same commit:
: The exact current symbols frozen in a checked deletion manifest immediately
  before cutover: the ordered retry scheduler, Generic post-effect retry debt,
  private continuation/error-to-None edges, Generic V0/V1 registry
  handler/predicate edges, nested Generic `.ok()`/retry edges, selected old
  JoinIR caller/physical edges, and new-subtree AST reconstruction facades.
  Generic-only dead files may be physically removed in the immediately
  following caller-zero R1, but no old mutating or re-selection authority may
  survive this atomic cutover. Historical symbol names are not deletion
  authority.

Contract:
: Meaning once, physical allocation once, external commit once. Failure is
  terminal `Freeze` plus whole-candidate discard.

Done:
: Rust/selfhost recipe parity, winner equivalence, five-adapter fault injection,
  fresh reuse, accepted corpus/backend parity, representative phase29bq smokes,
  quick/release, shared guards, and old-symbol census are green. Production
  verified-recipe consumer and canonical CFG/Binding-SSA physicalizer counts
  are exactly one; `LoopPhiMaterializerV1` remains a caller-zero mechanical
  observer or is retired. Ordered scheduler, selected old composer/PHI edges,
  Retry, and fallback counts are zero.

Stop:
: Retry/fallback, source redecision, unverified lower, partial publish,
  diagnostics drift, backend mismatch, or an unclassified legacy Generic
  fixture blocks cutover. Every currently accepted fixture must already have
  an implemented portable owner or an accepted explicit typed reject. A future
  profile may retain only evidence that current production does not accept.

### M11 — `RAW-LOCATED-LOOP-PORTABLE-HANDOFF0-R1`

Change:
: Feed located Loop source/provenance into the same StructuralFacts/recipe path;
  delete the source-erasing handoff and covered shadow entries.

Contract:
: No new ingress, source reconstruction, profile widening, or second producer.

Done:
: Old located terminal/scheduler, selected composers, mutation entries, and
  Loop R4 fences have zero callers; manifest/parity green.

Stop:
: Keep an uncovered entry under a named owner rather than hiding it in fallback.

### M12 — `JOINIR-LOOP-LEGACY-FAMILY-ADAPTER-RETIRE0-R2`

Ordered rows:
: `JOINIR-LOOP-LEGACY-DISPOSITION-R2A` ->
  `JOINIR-LOOP-DUPLICATE-FACADE-RETIRE-R2B` ->
  `JOINIR-LOOP-MUTATION-DISPATCH-RETIRE-R2C` ->
  `JOINIR-LOOP-SOLE-AUTHORITY-CLOSEOUT-R2G`. This is one 2-5 commit retirement
  series; every commit remains buildable.

Change:
: After M10/M11 remove old physical edges, classify each `11/1/1/4/2`
  first-mutation profile, family receipt, and route wrapper as semantic input,
  migration evidence, or duplicate facade; delete migration-only/duplicate
  rows and keep only source-policy rows producing the common recipe.

Contract:
: Retirement only: verifier, JoinSig obligation producer, canonical CFG,
  function-owned Binding SSA/PhiTxn, and physicalizer each have authority count
  one; no three-family layer or predicate merge. The M6-B map/receipt observer
  is not counted as a second production PHI owner.

Done:
: Production references to `ComposerMutationFamily`/equivalent legacy
  first-mutation enums and family adapter dispatch are zero. Family-specific
  recipe/verifier/CFG/PHI/physicalizer branches and duplicate producer wrappers
  are zero; retained route rows are data-only source policy. The terminal
  physicalizer has one production caller and all parity/quick/release gates green.

Stop:
: If a retained adapter still owns physical allocation, retry, AST rematch, or
  route selection, M12 is not complete. Keep the exact owner named and repair
  it; do not relabel it as a semantic feature or leave it for selfhost cleanup.

## Complete Loop selfhost lane — reserved after M12

Entry:
: M1 through M12 are closed, the portable wire contract is frozen, and the
  Rust production path has one terminal physicalizer with no retry/fallback.
  Exact task order is `SH1 -> SH2 -> SH3 -> SH4 -> SH5`; only read-only census
  and fixture preparation may run ahead.

Authority home:
: New production `.hako` owners live under canonical `lang/src/mir/builder/**`.
  Existing `lang/src/compiler/mirbuilder/**`, direct JSON Loop templates,
  LoopSSA text rewrite, synthetic-ID PHI injection, and token-only facts are
  compatibility/parity oracles until explicitly retired; they are not promoted
  as the portable authority.

Scope:
: This lane completes Loop lowering inside the selfhost MirBuilder. It does not
  by itself delete the Rust Stage0/bootstrap compiler, migrate the parser, add
  language syntax, or change backend policy. Repository-wide Rust removal needs
  a separate zero-caller decision after SH5.

### SH1 — `SELFHOST-LOOP-RECIPE-VERIFIER0-SH1`

Change:
: Implement the portable Loop recipe/JoinSig schema and independent verifier in
  the canonical `.hako` home. Reuse data vocabulary from old recipe boxes only
  when it satisfies the new owned wire contract.

Contract:
: The verifier consumes only the owned recipe, checks role/source/carrier/exit
  coverage plus JoinSig edge arity/types, and issues a verified product or typed
  rejection. It cannot read Program JSON, select a route, repair, lower, retry,
  delegate, or call a host bridge.

Done:
: All 19 positive legacy-ingress fixtures map into the same recursive Recipe
  algebra, and the shared missing/duplicate role, undefined
  carrier, missing edge, and arity/type counterexamples match Rust verifier
  codes and structured fields. Count-only PortSig is not accepted as JoinSig.

Stop:
: Do not connect CFG allocation, PHI emission, MIR JSON, or the default route.
  Rust remains the physical authority.

### SH2 — `SELFHOST-LOOP-CFG-JOINSIG-PHI0-SH2`

Change:
: Add one `.hako` CFG skeleton service, one JoinSig edge-payload service, one
  PHI materializer, and one structural verifier over an unpublished draft.

Contract:
: Services consume only the SH1 verified product. Family adapters cannot infer
  edges from AST/Program JSON, allocate their own PHIs, repair invalid graphs,
  select, retry, publish, or invoke legacy text/synthetic-ID PHI rewriting.

Done:
: Five legacy-adapter representatives and then the 19-row matrix match Rust predecessor,
  dominance, edge payload, carrier/exit, unreachable, and normalized structural
  plan results. Shared services are the sole CFG/PHI owners in the new subtree.

Stop:
: No public MIR JSON or default-route connection. A route that does not fit opens
  one bounded portable vocabulary decision, never a route-local physicalizer.

### SH3 — `SELFHOST-LOOP-PHYSICAL-MIR-JSON0-SH3`

Change:
: Add one terminal `.hako` Loop physicalizer that maps verified logical roles
  to deterministic block/value IDs in an unpublished module draft, seals the
  function/module, and serializes MIR JSON v0 only after complete success.

Contract:
: Land as one ordered implementation series: Accum vertical slice, recursive
  recipe/feature closure, then the five M8 adapter cohorts/all 19, with one
  cohort plus fixtures per commit. The physicalizer accepts no AST, raw recipe,
  `Option`, suffix,
  retry, fallback, delegate, or route-selection capability.

Done:
: All 19 legacy-ingress fixtures have deterministic normalized Recipe and MIR
  JSON parity, verifier/VM
  result parity, PHI/type/result parity, and no partial artifact on injected
  late failure. Covered direct Loop templates, recipe rematch, and route-local
  emitters have zero callers in the new subtree.

Stop:
: Do not touch the default route, Stage identity, parser, language surface, or
  backend policy. Failure is typed terminal and discards the unpublished draft.

### SH4 — `SELFHOST-LOOP-STAGE1-STAGE2-IDENTITY0-SH4`

Change:
: Connect SH3 only to the explicit selfhost identity route and align the identity
  harness with its live Stage1-only route contract before measuring parity.

Contract:
: Program JSON v0 and MIR JSON v0 must be produced without compatibility route,
  automatic Rust fallback, hostbridge, or default-route mutation. Stage0 remains
  an explicit bootstrap/reference lane.

Done:
: The 19-route matrix, selfhost subset/full gates, portable producer/verifier
  reports, and `selfhost_identity_check.sh --mode full` are green on the exact
  Stage1/Stage2 route. Stale `auto`/`--allow-compat-route` expectations are gone.

Stop:
: Identity mismatch, missing Stage artifacts, harness drift, or delegate use
  blocks SH5; do not normalize it away by weakening the comparison.

### SH5 — `SELFHOST-LOOP-DEFAULT-CUTOVER0-SH5-I0-R0`

Change:
: Atomically switch the canonical `.hako` MirBuilder Loop branch from
  registry/fallback direct lowerers to the SH1-SH3 pipeline. In the same commit,
  delete covered default compatibility dispatch, Loop direct emitters, recipe
  shape rematch, text-PHI claims, and automatic Rust/delegate fallback.

Contract:
: Meaning once, verification once, physical allocation once, sealed MIR JSON
  publication once. Default success or typed terminal failure are the only
  outcomes; Stage0 bootstrap is explicit and never selected as fallback.

Done:
: Default selfhost Loop selection/meaning/verifier/physicalizer authority count
  is one; retry/fallback/hostbridge/dual dispatch are zero; all 19 legacy-ingress
  matrix rows, full identity,
  selfhost corpus, representative backend, quick/release, and old-caller census
  are green.

Stop:
: Any uncovered Loop stays under a named old owner and blocks cutover. Do not
  hide it in delegate/fallback or claim repository-wide Rust retirement.

## Gates and commit cadence

- Prelude: one physical-move commit.
- D/P rows: docs/proof commits; no implementation claim.
- S rows: one reusable vocabulary/service or one bounded adapter cohort plus fixtures.
- Partial pipeline remains caller-zero until M8/M9 closure.
- M10: one atomic I0/R0 commit; M11: one handoff retirement commit; M12: one
  legacy-family adapter census/retirement series.
- SH1/SH2 are disconnected service rows; SH3 is an ordered implementation
  series with one adapter cohort per commit;
  SH4 connects only the explicit identity route; SH5 is the sole `.hako`
  atomic default cutover/retirement commit.
- Daily: focused module/registry, current pointer, MirBuilder structural guard.
- Recipe milestones: normalized Rust report plus `.hako` parity when enabled.
- Cutover: accepted corpus/backend, selfhost identity, representative phase29bq,
  quick/release, shared guards, old-symbol/caller census.
- No per-row shell guard. Every touched source/check file remains below 800
  lines.

## Parked

```text
LoopEmissionDraftV1 / Loop-local Builder candidate
undo journal
symbolic MIR fragment redesign
route-local terminality mini-products and physicalizers
cosmetic Direct shell unification
language expansion, Home ownership, performance, backend expansion
whole-MIR block-argument rewrite or general immutable graph IR
```

The portable recipe itself is intentionally symbolic at the control/role level;
the parked item is a general symbolic MIR fragment rewrite of CorePlan/lowerer.
