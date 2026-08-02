---
Status: Active task-order SSOT
Date: 2026-08-03
Decision: accepted — `JOINIR-LOOP-SELFHOST-RECIPE-PIPELINE0-D0`
Scope: production Loop meaning, selfhost-portable recipe, terminal candidate lowering, and atomic retirement
Related:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/joinir-loop-pre-effect-product-ssot.md
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# JoinIR Loop Selfhost-Ready Recipe Pipeline

## Decision

Choose the clean final architecture rather than the shortest wrapper around the
19 legacy physical routes.

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

## Structural owners

| Owner | Owns | Must not own |
| --- | --- | --- |
| `LoopStructuralFactsV1` | source/control observations and stable provenance | route policy, MIR IDs, emission |
| `LoopRoutePolicyV1` | one ordered winner or typed rejection | Builder probes, physical execution, retry |
| `LoopRecipeArtifactV1` | owned provenance plus one recursive `LoopRecipeV1` | route/family dispatch, AST reconstruction, physical IDs |
| `LoopRecipeV1` | condition, recursive body/control items, carriers, exits | legacy family, policy selection, physical IDs |
| `LoopRecipeVerifierV1` | recipe coverage and logical JoinSig/CFG elaboration | repair, fallback, another recipe |
| `LoopJoinSigV1` | verified logical edge payload obligations | PHI allocation, CFG repair |
| `LoopCfgSkeletonLoweringV1` | physical blocks/edges/terminators from verified roles | route selection, AST reading |
| `LoopPhiMaterializerV1` | PHIs from verified JoinSig only | source inference, route-specific repair |
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
Builder immutability claim. The M1-B fixture is now green, so M1 is closed;
M2 is the next design stop.

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

### M3 — `JOINIR-LOOP-STRUCTURAL-FACTS-ROUTE-POLICY0-P0-S1`

Change:
: Map all 19 routes to exact source observation, suppression, mode/contract
  inputs, prefix decline, and current pre/post-effect outcome. One pure policy
  consumes one frozen schedule and selects one recipe candidate.

Contract:
: AST rematch/rewrite, composer execution, Builder probe, physical execution,
  fallback, and second scheduler are zero. Every earlier candidate has a typed
  pre-effect decline.

Done:
: Legacy final winner equals the pure winner by raw schedule, route ID, prefix
  reasons, and recipe kind for all existing fixtures. Partial coverage stays
  caller-zero.

Stop:
: An execution-dependent decision becomes an exact family blocker, not `Unknown`
  or runtime probing.

### M4 — `JOINIR-GENERIC-POST-EFFECT-DEBT-RECIPE0-D0-S2`

Change:
: Enumerate every Generic V0/V1 verifier/lower `None`, preceding mutation,
  V0-to-V1 continuation, and later-success fixture. Decide whether V0/V1 remain
  distinct pure policies or one is semantically subsumed.

Contract:
: Only Builder-free decline or a verified Generic recipe with terminal physical
  `Freeze` is accepted. Legacy post-effect retry is not a recipe outcome. New
  classification remains disconnected until cutover.

Done:
: Generic winner equivalence is green and selected Generic recipe contains no
  `Option`, raw suffix, or retry capability. A production-derived V0-to-V1
  later-success fixture exists, or the policy predicates prove that edge
  impossible; distinct V0/V1 policy versus typed V1 subsumption is decided.

Stop:
: If winner cannot be known before effects, all-route cutover stops here. Do not
  weaken the architecture or restart unrelated per-route proof loops.

### M5 — `JOINIR-LOOP-ACCUM-PORTABLE-RECIPE0-S3`

Change:
: Implement AccumConstLoop through the complete final boxes: StructuralFacts,
  RoutePolicy, owned recursive Recipe, verifier/elaborated JoinSig, symbolic
  CFG/edge plan, and candidate physicalizer.

Contract:
: Only the physicalizer maps logical roles to real block/value IDs. PHIs come
  exclusively from JoinSig. The existing Accum composer is a parity oracle only.

Done:
: Normalized recipe, verifier counterexamples, MIR/PHI/type/result parity, late
  failure candidate discard, and fresh compiler reuse are green. Caller remains
  zero.

Stop:
: Do not import synthetic AST or current physical composer as new authority.
  M6 must replace and promote this pilot into the shared services; it must not
  leave a second Accum-only implementation beside them.

### M6 — `JOINIR-LOOP-CFG-JOINSIG-PHI0-S4`

Change:
: Establish one CFG skeleton owner, one JoinSig edge-payload owner, one PHI
  materializer, and one structural verifier.

Contract:
: Verifier checks predecessor count, dominance obligations, edge arity/types,
  carrier closure, exits, and unreachable policy. It rejects; it never repairs
  or chooses another recipe.

Done:
: Route-specific block allocators/PHI builders and lower-side AST route decisions
  have zero callers inside the new subtree.

Stop:
: No family adapter may bypass shared CFG/JoinSig/PHI owners.

### M7 — `JOINIR-LOOP-RECURSIVE-RECIPE-CLOSURE0-S5`

Change:
: Add representative producers for the five legacy first-mutation profiles:
  LoopV0=`AccumConstLoop`, Nested=`NestedLoopMinimal`,
  LoopTrue=`LoopTrueBreakContinue`, LoopCond=`LoopCondBreakContinue`, and
  Generic=`GenericLoopV1` after M4. Every producer emits the same recursive
  semantic `LoopRecipeV1`.

Contract:
: The five profiles are migration adapters only. They share one portable
  recursive recipe envelope, verifier/elaboration terminal,
  CFG/JoinSig/PHI services, and physicalizer. A profile-specific semantic
  variant is forbidden unless a concrete source counterexample proves a
  bounded vocabulary extension; a sixth legacy profile is never a new semantic
  recipe kind.

Done:
: Normalized recipe/MIR parity and post-first-mutation candidate-abort tests are
  green for all representatives. Family adapters cannot select, retry, or
  publish.

Stop:
: Generic may not be omitted/mocked; Nested inner semantics may not be inferred
  from outer provenance.

### M8 — `JOINIR-LOOP-ALL19-PORTABLE-RECIPE0-S6`

Change:
: Migrate the remaining 14 routes as bounded adapter cohorts: LoopV0
  recurrence, LoopV0 exits/joins, LoopV0 scans, LoopCond exits, and Generic
  V0. Add only missing source observation or portable vocabulary. Each route is
  one producer data row/golden for the same recursive recipe, not a new
  verifier, CFG, PHI, or physicalizer authority.

Contract:
: 19/19 routes have typed pre-effect decline or verified recipe. Legacy winner
  equivalence includes first-None-then-later-success. Selected physicalization
  has no `Option`, suffix, retry, or fallback.

Done:
: Accepted corpus produces verified recipes and parity MIR. Unverified direct
  lower, cloned-AST recipe reconstruction, post-effect continuation, and
  duplicate CFG/PHI authorities are zero in the new subtree.

Stop:
: An unfitting route opens one bounded vocabulary extension, never a compatibility
  escape or route-local physicalizer.

### M9 — `SELFHOST-LOOP-PORTABLE-RECIPE-PARITY0-S7`

Change:
: Implement the same StructuralFacts/RoutePolicy/Recipe/JoinSig producer in
  `.hako` over the existing Program/AST JSON boundary.

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

### M10 — `JOINIR-LOOP-PORTABLE-RECIPE-CUTOVER0-I0-R0`

Change:
: Switch `route_loop` to frozen source -> StructuralFacts -> one policy winner
  -> verified recipe -> one candidate physicalizer.

Delete in the same commit:
: Ordered `RouteFn` retry scheduler, `RouteAttemptOutcomeV1::Retry`,
  `from_retry_option`, private legacy continuation, Generic error-to-None,
  selected old physical composers/edges, and new-subtree AST reconstruction
  facades.

Contract:
: Meaning once, physical allocation once, external commit once. Failure is
  terminal `Freeze` plus whole-candidate discard.

Done:
: Rust/selfhost recipe parity, winner equivalence, five-adapter fault injection,
  fresh reuse, accepted corpus/backend parity, representative phase29bq smokes,
  quick/release, shared guards, and old-symbol census are green.

Stop:
: Retry/fallback, source redecision, unverified lower, partial publish,
  diagnostics drift, or backend mismatch blocks cutover.

### M11 — `RAW-LOCATED-LOOP-PORTABLE-HANDOFF0-R1`

Change:
: Feed located Loop source/provenance into the same StructuralFacts/recipe path;
  delete the source-erasing handoff and covered normalized-shadow entries.

Contract:
: No new ingress, source reconstruction, profile widening, or second recipe
  producer.

Done:
: Old located terminal, scheduler, selected composers, normalized mutation
  entries, and Loop R4 fences have zero callers; manifest/parity green.

Stop:
: Keep an uncovered entry under a named owner rather than hiding it in fallback.

### M12 — `JOINIR-LOOP-LEGACY-FAMILY-ADAPTER-RETIRE0-R2`

Change:
: After M10/M11 have removed the old physical edges, classify every remaining
  `11/1/1/4/2` first-mutation profile, family adapter receipt, and route wrapper
  as semantic producer input, migration-only evidence, or duplicate facade.
  Delete the migration-only and duplicate rows; keep only source-policy rows
  that produce the common recursive recipe.

Contract:
: This is a retirement pass, not a new lowering design. The new verifier,
  CFG/JoinSig/PHI owners, and physicalizer already have authority count one.
  Do not introduce an intermediate three-family semantic layer or merge
  distinct source predicates merely to reduce a count.

Done:
: Production references to `ComposerMutationFamily`/equivalent legacy
  first-mutation enums and family adapter dispatch are zero. Family-specific
  recipe variants, verifier branches, CFG/PHI branches, and physicalizer
  branches are zero. Duplicate producer wrappers are zero; retained route rows
  are data-only source recognition/policy inputs. The terminal physicalizer has
  exactly one production caller and all accepted-corpus/parity/quick/release
  gates remain green.

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
: All 19 positive recipes and the shared missing/duplicate role, undefined
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
: All 19 have deterministic normalized recipe and MIR JSON parity, verifier/VM
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
  is one; retry/fallback/hostbridge/dual dispatch are zero; all 19, full identity,
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
language expansion, Ownership/View, performance, backend expansion
whole-MIR block-argument rewrite or general immutable graph IR
```

The portable recipe itself is intentionally symbolic at the control/role level;
the parked item is a general symbolic MIR fragment rewrite of CorePlan/lowerer.
