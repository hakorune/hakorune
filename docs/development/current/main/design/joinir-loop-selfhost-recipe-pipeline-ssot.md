---
Status: Active task-order SSOT
Date: 2026-08-02
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
-> LoopRecipeContract + JoinSig
-> RecipeVerifier
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

It owns stable source coordinates or normalized operands, route kind, logical
block/value/binding roles, carrier and exit obligations, JoinSig edge payloads,
and typed rejection reasons. Rust and `.hako` compare a deterministic normalized
recipe representation, not allocation addresses or physical MIR IDs.

The final authority flow is the existing north star:

```text
Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower -> Seal -> Collect
-> Atomic Publish
```

## Structural owners

| Owner | Owns | Must not own |
| --- | --- | --- |
| `LoopStructuralFactsV1` | source/control observations and stable provenance | route policy, MIR IDs, emission |
| `LoopRoutePolicyV1` | one ordered winner or typed rejection | Builder probes, physical execution, retry |
| `LoopRecipeContractV1` | logical blocks, operations, carriers, exits, family | AST reconstruction, physical IDs |
| `LoopJoinSigV1` | continue/break/return/after edge payloads | PHI allocation, CFG repair |
| `LoopRecipeVerifierV1` | coverage, roles, edge arity/types, dominance obligations | repair, fallback, another recipe |
| `LoopCfgSkeletonLoweringV1` | physical blocks/edges/terminators from verified roles | route selection, AST reading |
| `LoopPhiMaterializerV1` | PHIs from verified JoinSig only | source inference, route-specific repair |
| `LoopPhysicalizerV1` | one terminal candidate mutation | `Option`, retry, raw suffix, publication |
| compile candidate | whole-compile abort and success-only external commit | Loop meaning or policy |

Names may be shortened during implementation. Ownership boundaries may not.

## Current evidence

- Production registry membership is exactly 19 ordered routes.
- First mutation families are `11/1/1/4/2`: LoopV0, Nested, LoopTrue,
  LoopCond, Generic.
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

### M2 — `JOINIR-LOOP-PORTABLE-RECIPE-CONTRACT0-D0-S0`

Change:
: Define owned vocabulary for route kind, stable source/operand references,
  logical block/value/binding roles, operations, carriers, exits, JoinSig,
  mutation family, and typed reject/decline/error reasons.

Contract:
: No AST, borrowed facts, Builder, physical IDs, callback, or retry crosses the
  boundary. Provide one deterministic normalized representation usable by Rust
  and `.hako` parity.

Done:
: Disconnected vocabulary round-trips the normalized representation and rejects
  missing/duplicate/invalid roles without Builder access.

Stop:
: Do not hide legacy route functions or AST nodes inside opaque recipe payloads.

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
  `Option`, raw suffix, or retry capability.

Stop:
: If winner cannot be known before effects, all-route cutover stops here. Do not
  weaken the architecture or restart unrelated per-route proof loops.

### M5 — `JOINIR-LOOP-ACCUM-PORTABLE-RECIPE0-S3`

Change:
: Implement AccumConstLoop through the complete final boxes: StructuralFacts,
  RoutePolicy, owned Recipe/JoinSig, verifier, symbolic CFG/edge plan, and
  candidate physicalizer.

Contract:
: Only the physicalizer maps logical roles to real block/value IDs. PHIs come
  exclusively from JoinSig. The existing Accum composer is a parity oracle only.

Done:
: Normalized recipe, verifier counterexamples, MIR/PHI/type/result parity, late
  failure candidate discard, and fresh compiler reuse are green. Caller remains
  zero.

Stop:
: Do not import synthetic AST or current physical composer as new authority.

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

### M7 — `JOINIR-LOOP-FIVE-FAMILY-PORTABLE-RECIPE0-S5`

Change:
: Add representative recipes for LoopV0=`AccumConstLoop`,
  Nested=`NestedLoopMinimal`, LoopTrue=`LoopTrueBreakContinue`,
  LoopCond=`LoopCondBreakContinue`, and Generic=`GenericLoopV1` after M4.

Contract:
: Families may have distinct data variants but share one portable envelope,
  verifier terminal, CFG/JoinSig/PHI services, and physicalizer. A sixth family
  requires evidence.

Done:
: Normalized recipe/MIR parity and post-first-mutation candidate-abort tests are
  green for all representatives. Family adapters cannot select, retry, or
  publish.

Stop:
: Generic may not be omitted/mocked; Nested inner semantics may not be inferred
  from outer provenance.

### M8 — `JOINIR-LOOP-ALL19-PORTABLE-RECIPE0-S6`

Change:
: Migrate remaining routes as pure Facts/Policy-to-family-recipe adapters. Add
  only missing source observation or portable vocabulary.

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
  callbacks. Rust remains the thin allocation/emission terminal until a later
  explicit migration.

Done:
: Route ID, prefix reasons, logical roles, JoinSig, verifier result, and
  normalized recipe match for all 19 fixtures; selfhost quick, representative
  identity, and no-hostbridge gates are green.

Stop:
: Do not widen language semantics or Rust source recognition to force parity.

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
: Rust/selfhost recipe parity, winner equivalence, five-family fault injection,
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

## Gates and commit cadence

- Prelude: one physical-move commit.
- D/P rows: docs/proof commits; no implementation claim.
- S rows: one reusable vocabulary/service or one accepted family plus fixtures.
- Partial pipeline remains caller-zero until M8/M9 closure.
- M10: one atomic I0/R0 commit; M11: one retirement commit.
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
```

The portable recipe itself is intentionally symbolic at the control/role level;
the parked item is a general symbolic MIR fragment rewrite of CorePlan/lowerer.
