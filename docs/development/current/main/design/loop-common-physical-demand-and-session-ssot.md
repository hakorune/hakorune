---
Status: SSOT
Date: 2026-08-16
Decision: accepted after external and independent worker review — `LOOP-COMMON-PHYSICAL-DEMAND-AND-SESSION0-D0-r2`
Activation: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, callable static-prefix
P0, bounded `LOOP-PHYSICAL-PREPARE-P0`, common-boundary design stop,
caller-zero `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0`, passive operation/effect S0,
Callable/G0 adapters, and cross-profile parity are closed. Decision B now
separates full-demand preflight from leaf emission; the Builder-free
`LOOP-RECIPE-OPERATION-PHYSICAL-DEMAND-P0` and the behavior-neutral
physicalizer module split, physical block receipt, private ConstI64
leaf-emitter canary, bounded ReadBinding I0, callable full physical P0, and
G0 exact-ingress I0 are closed. Top-down review revised the next boundary:
the private Builder-free segment/resume layout and bounded G0 fresh-session
canary are closed. A later audit found that the landed layout still derives
logical transfers from Recipe instead of consuming JoinSig authority; the
post-M9 pre-cutover R0 rows below own that correction. Operation production
activation remains 0. The bounded After-closure canary is green: the real
Prelude receipt feeds the complete seven-operation Callable dispatch, fixed
CFG edges, and canonical CFG/identity sealing. The Tail handoff now reads the
exact binding through canonical identity, validates the existing trivial ABI,
and claims Completion/return coverage once. The sealed After receipt also
moves a non-Clone callable profile-close receipt proving exact
`7 = Pure4 + Read2 + Write1` coverage, the Bool condition, owner, terminal
block, and After predecessor. Finish must consume that receipt through a
non-no-op `finish_profile_close` closure. DraftSeal, production selection,
retry/fallback, and legacy retirement remain closed. The bounded
`CALLABLE-LOOP-DRAFT-SEAL-P0` canary now consumes the profile-close receipt
through the existing typed finish terminal, then uses DraftSeal
prepare/commit to produce one unpublished `CompletedFunctionDraftV1`; no
collector or module publication is performed. The production-edge census and
Admission D0 are closed as `NoSafeSlice`. The source/facts bridge D0 is
accepted without a new semantic Bridge owner: the existing resolver ledger
plus neutral SyntaxFacts/SourceMap are the target production boundary. The
source/facts issuer S0 and bounded logical issuer D0/S0 are now closed with
bounded negatives, exact parity, and caller-zero/current receipt audit. The
profile Recipe shape is production-owned while the old shape helper remains a
test-only parity wrapper. `CALLABLE-LOOP-PRODUCTION-PREPARED-INGRESS-D0` is
accepted and its S1/S2 caller-zero products are closed.
`LOOP-CALLER-ZERO-PARITY-G0-D0` is also accepted. Its exact resolver-issued
G0 source/input/entry capability is carried by a thin compiler-side composite
ingress; neutral S4 remains the sole Recipe/effect/After owner. I0 is closed
as Builder-free exact ingress plus fifteen-row `prepare_all`. R1 is closed as
a Builder-free derived layout, R2 as a Callable adapter, and R3-I0 as the
selected Callable exact-segment/neutral-After canary. Per-transfer Predicate
value receipts, the profile-neutral `DerivedCarrierEntry` operation, and the
bounded G0 I1 canary are also closed. A later top-down audit found that current
caller-zero products can still be re-paired and current Layout code still
derives logical transfers without JoinSig authority. The accepted target is
unchanged, but semantic-program and transfer-authority R0 rows must close
after M8/M9 and before production selection. No named production caller
switch is open.
The 2026-08-15 S6C audit adds no second physicalizer: it names one missing
common-V2 pre-session contract that must close exact callable ABI and the
complete V2 envelope before any TextEq leaf or canonical session is admitted.
Scope: common Loop physical demand, fresh unpublished function session, failure discard, completion/DraftSeal handoff
Related:
  - docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/reference/mir/loop-recipe-contract.md
  - docs/reference/mir/generic-loop-stage-matrix.md
  - src/mir/builder/resolved_lowering/README.md
  - docs/development/current/main/investigations/loop-physical-prepare-design-correction-r0-task-2026-08-07.md
---

# Loop Common Physical Demand and Session SSOT

## Current Capsule

- **Current decision:** every admitted Loop profile reaches one complete
  semantic program, JoinSig-bound layout, and canonical SSA session; V1 and V2
  are exact projections of that one responsibility graph.
- **Current implementation status:** S6C source/site, ExactText formal,
  result/header, installed child, package physical-signature map, and
  caller-zero residence/backend transport substrate are closed. The common
  V2 operation/control/coverage issuers and installed Port HRTB are landed
  as caller-zero source products. Existing resolver Loop membership can issue
  the outer-If residual, and the installed S6C child can lend the actual
  Completion without cloning. The resolver-owned BlockExpr expectation is
  now batch-owned and reaches the selected/package HRTB as a borrow. The
  callback-scoped common admission is landed; the detached physical skeleton,
  slot-only ExactText adoption canary, and consuming physical-entry/session
  seam are also landed. The session-stamp retention I0 now moves the existing
  mechanical cohort witness exactly once into the canonical session and lends
  only a scoped borrow. The V2-native physical-ID-free layout/placement
  BoxShape and its caller-zero transport I0 are now landed. The Length-result
  canary I0, direct Length Call/result I0, and exclusive session-scoped Length
  receipt lifetime I0 are also landed. The source-only initial-index seed
  relation transport, its one-entry Const/exact-declaration materializer I0,
  and the receipt-owned Bool/Compare materializer I0 are now landed; the
  all-family source-parent/co-seal R0, Generic G0 source-parent BoxShape, and
  same-cohort source-view BoxShape are accepted; the Generic source-parent I0
  replaces the test-only ingress with one production issuer.  The resolver
  body-shape product is now transported from the same source-unit resolution
  into the root lowering input and Generic source parent with owner/body-root
  checks.  The private Generic no-external-effect receipt and same-cohort
  result-ABI transport I0 and direct canonical Completion transport I0 are now
  landed before demand/product consumption.  The next stop is the independent
  Generic storage/lane projection; it cannot open an EffectMask or any
  physical/session effect until that source-backed projection is accepted.
  A-prime lifecycle activation remains parked until its boundary owns
  `PreparedFunctionExitSetV1`.  The selected Dynamic physical-input
  authority is landed; the post-Dynamic unification rows below remain a
  design-stop closeout until their direct negative fixtures and old-edge
  caller census are recorded.
- **Next ordered task:**
  `LOOP-UNIFICATION-AFTER-DYNAMIC-D0` is the next design stop.  The
  Callable-first semantic-program consume and Dynamic physical-input
  authority are landed; transfer/ledger/common-boundary cleanup and the
  topology census must be reconciled against their existing implementations
  before any If/Exit BoxCount, session, route, fallback, retry, or production
  work opens.
- **Production stop line:** no leaf emission or session admission may infer
  ABI, control, transfer, or source identity from Recipe/MIR, coerce V2 to V1,
  or select a second physicalizer.
- **Retirement finish line:** all admitted profiles use one common physical
  owner and old topology, route-local schedulers, direct transfer inference,
  retry, and fallback have zero callers.

### Historical pre-cutover authority coverage census (snapshot before G0 D0)

At the time of this census, `LOOP-PRECUTOVER-AUTHORITY-G0` was not accepted.
Existing source/Facts,
Core/Recipe, JoinSig, neutral layout, and canonical CFG/SSA owners are
individually present, but no single source-backed issuer currently co-seals
the complete semantic program and its transfer/layout/CFG coverage. The
census closes this inventory stop and hands off to the more precise issuer
blocker below:

```text
  NoSafeSlice::GenericG0EntrySourceCoverageParentUnsealed
```

Decision:
  the read-only `LOOP-PRECUTOVER-AUTHORITY-COVERAGE-D0` census is complete;
  it accepted the Callable-first `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` handoff,
  while generic G0 still may not mint a repository-wide semantic receipt,
  physicalizer, scheduler, or Builder effect. The active bounded row is the
  all-family source-parent/co-seal R0; later generic work must still close
  `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0`, and only then the Always/If/Exit
  coverage rows.

Source authority + canonical issuer:
  resolver/source Facts and the Core-owned semantic program issue source
  relations; that same Core's JoinSig issues transfers; neutral Layout only
  binds already-issued placement; `CanonicalSsaFunctionSessionV2` and its
  CFG/PHI owners remain the sole physical owners. The census may reference
  these owners, but it is not a new semantic authority. The eventual
  composite issuer must co-seal exact node/source/entry coverage with the
  Core JoinSig continuation before any selection or physical effect.

Non-authority:
  the 19 task labels, raw Recipe/JoinSig/AST/MIR/JSON rescans, bare counts,
  `physical_layout.rs` or topology Recipe re-interpretation, Dynamic/V1/
  legacy physicalizers, profile-local wrappers, raw `ValueId`/
  `BasicBlockId`, benchmark/environment/fallback, and separate coverage
  receipts that can be re-paired are not G0 authority.

Fail-fast boundary:
  the census records missing/duplicate/foreign owner or brand, JoinSig/layout
  drift, missing Always/If/Exit coverage, competing target-subtree owners,
  and any re-scan/recombination requirement as a blocker before selection or
  Builder/session effect. A positive census does not itself authorize G0.

Smallest next slice:
  `LOOP-PRECUTOVER-AUTHORITY-G0-D0` — design-only Generic source-parent and
  entry/source-coverage boundary. Do not relabel the Callable parent, mint a
  Generic receipt, or open physical effect.

Acceptance:
  the Callable co-seal issuer is landed, while the all-family census records
  each missing parent/adapter and keeps `NoSafeSlice` active. It must not fill
  a gap with zero, a count,
  Recipe order, or MIR observation.

#### Census result and next co-seal design stop

The read-only census found the following existing source-backed products:

| Existing product | Owns | Current gap before G0 |
| --- | --- | --- |
| `VerifiedLoopSemanticContextV1` | resolver owner/origin/site/frame/Scope-Region relation | can still be passed as a separate argument |
| `VerifiedLoopCoreProductV1` | Recipe, Core-owned JoinSig, source claim, binding/effect relations | does not own the complete operation/source parent alone |
| `VerifiedLoopOperationEffectProductV1` | complete item-keyed operation evidence | does not seal the context/frame/JoinSig relation |
| `VerifiedLoopContinuationContractV1` | one JoinSig-owned Loop After capability | can currently be independently re-paired |
| `VerifiedLoopOperationPhysicalDemandV1::issue` | mechanical owner/scope/continuation checks and private index | is a consumer/projection, not the semantic-program issuer |

`VerifiedLoopRecipeCoSealV1` is a valid Callable-specific source product, but
it is not a repository-wide all-19 issuer. The G0 census therefore closes its
own inventory task while leaving generic G0 blocked by:

```text
NoSafeSlice::GenericG0EntrySourceCoverageParentUnsealed
```

Decision:
  accept the Callable-first BoxShape for `LOOP-SEMANTIC-PROGRAM-COSEAL-R0`.
  One private, non-`Clone` `VerifiedCallableSemanticProgramV1` consumes the
  complete `VerifiedCallableSingleLoopRecipeProductV1` parent exactly once.
  It owns the already-issued operation/effect, input, context, JoinSig-owned
  continuation, callable prelude, and callable tail until the next consumer;
  it does not mint a second semantic fact. The generic all-profile issuer
  remains a later G0 decision.

Source authority + canonical issuer:
  `issue_callable_single_loop_recipe_v1` remains the source-backed issuer of
  the complete Callable parent. The new compiler-side
  `issue_callable_semantic_program_v1` consumes that whole parent and wraps
  the existing operation/effect adapter output without accepting separately
  supplied context, Core, or continuation arguments. The existing
  `VerifiedLoopOperationPhysicalDemandV1::issue` is only a mechanical
  projection used inside this issuer. G0/M8/M9 require their own same-parent
  adapter decision and cannot reuse this Callable issuer by relabeling.

Non-authority:
  separate `VerifiedLoopOperationPhysicalDemandV1::issue` arguments, its
  lookup index, Recipe-order schedule, `PreparedLoopOperationLedgerV1`, raw
  owner/count, physical Layout, `ValueId`/`BasicBlockId`, and any profile-local
  wrapper are not the semantic-program issuer.

Fail-fast boundary:
  reject a foreign/missing/partially consumed Callable parent, operation/effect
  mismatch, owner/origin/site/frame drift, continuation from a different Core,
  or any second split/re-pair ingress before physical demand or Builder/session
  effect. The aggregate has one move-only consumer; no default, count, Recipe
  order, MIR observation, retry, or fallback repairs a failure.

Smallest next slice:
  `LOOP-SEMANTIC-PROGRAM-COSEAL-CALLABLE-I0` consumes one Callable parent,
  exposes one private aggregate to the existing prepared-operation consumer,
  and proves one-shot/foreign/operation-drift rejection. It is caller-zero and
  Builder-free; G0 remains blocked by the missing repository-wide issuer.

Non-claims:
  this Callable-first R0/I0 does not accept all-19 G0 coverage, JoinSig
  transfer migration, new physicalization, CFG/SSA/PHI, Canonical session
  construction, lifecycle/Text, route admission, publication, or legacy
  retirement.

### LOOP-SEMANTIC-PROGRAM-COSEAL-CALLABLE-I0 implementation receipt (2026-08-17)

`callable_semantic_program.rs` now consumes one complete
`VerifiedCallableSingleLoopRecipeProductV1` and keeps the existing
operation/effect product, initialized-local input, semantic context,
JoinSig-owned continuation, Callable Prelude, and Callable Tail in one
non-`Clone` parent until the prepared-operation consumer. The existing
operation/effect adapter is called only inside this issuer; no caller supplies
separate context/Core/continuation arguments and the old split ingress no
longer feeds `prepare_full_demand`.

The focused co-seal test proves one parent produces the complete seven-row
operation/effect product, matching context/continuation ownership, and the
prepared-ingress test remains green through the new consumer. The slice is
caller-zero and Builder-free: no CFG/SSA/PHI, physical IDs, lifecycle, Text,
route, fallback, retry, or production caller is opened. The repository-wide
G0/all-family semantic-program issuer remains a separate `NoSafeSlice`.

### LOOP-SEMANTIC-PROGRAM-COSEAL-ALL-FAMILY-R0 accepted design boundary (2026-08-17)

```text
Decision:
  Do not promote the Callable parent to a generic issuer. The all-family
  census accepts a two-stage boundary: each admitted family supplies one
  source-backed parent, then a thin compiler co-seal consumes exactly one
  parent. Generic G0 has its own accepted D0 and now proceeds to its source
  parent I0.

Source authority + canonical issuer:
  Each family keeps its resolver/source producer and Core-owned Recipe/JoinSig.
  A future all-family compiler issuer may consume one complete family parent
  and issue one non-Clone semantic-program envelope; it must not reconstruct
  source relations or re-pair context/Core/continuation. Callable is the only
  landed adapter so far.

Family parent census:

| family | source-backed parent | current boundary |
| --- | --- | --- |
| Callable | `VerifiedCallableSingleLoopRecipeProductV1` | `VerifiedCallableSemanticProgramV1` is landed caller-zero |
| Generic G0 | `VerifiedGenericRecipeDemandG0` -> `VerifiedGenericRecipeProductG0` -> `VerifiedGenericG0SourceParentV1` | D0 BoxShape accepted; source-parent I0 replaces the test-only split |
| Dynamic V2 | `VerifiedDynamicFullLoopSourceRecipeEnvelopeV2` -> `VerifiedDynamicFullLoopSemanticProgramV2` | versioned V2 parent exists, but common site/frame/Core/coverage co-seal is not established |
| M8 Rust cohorts | each `VerifiedVariableAccum*FactsV1` -> Recipe product | each producer keeps its own source authority; no all-family parent adapter exists |
| M9 `.hako` | portable Recipe/provenance wire | only a versioned verified-wire projection may cross into Rust; raw JSON/AST/HRTB is forbidden |

The common compiler seam is therefore two-stage: each source producer issues
its own complete parent, then one profile-neutral co-seal consumes exactly one
parent and validates owner/origin/site/frame, Core-owned JoinSig continuation,
item/carrier/input coverage, and schema revision. The compiler seam does not
re-walk source, relabel a profile parent, or turn M9 wire evidence into a Rust
source authority.

Non-authority:
  `VerifiedCallableSemanticProgramV1`, Generic test-only split helpers,
  Dynamic V2 semantic programs, M8 Facts/products, M9 parity artifacts,
  `PhysicalDemand::issue`, Recipe/route counts, Layout/CFG observations, and
  separate context/Core/Continuation arguments cannot be relabeled as G0.

Fail-fast boundary:
  Missing family parent, owner/origin/site/frame drift, Core/JoinSig mismatch,
  incomplete item/carrier/input/After coverage, V1/V2 mixing, HRTB escape, or
  any split/re-pair ingress keeps `NoSafeSlice` before selection/effect.

Smallest next slice:
  `LOOP-PRECUTOVER-AUTHORITY-G0-SOURCE-COHORT-D0` — seal the private
  same-cohort source view before replacing the Generic test-only split. Do not
  open physical effect in this slice.

Non-claims:
  No Generic G0 parent I0/physical effect, all-19 migration, JoinSig transfer migration,
  Layout/CFG/SSA/PHI, session, lifecycle, Text, route, selector, production,
  fallback, retry, or legacy retirement is opened.
```

### LOOP-PRECUTOVER-AUTHORITY-G0-D0 accepted design boundary (2026-08-17)

```text
Decision:
  Keep Generic G0 separate from Callable. Its source parent must be issued
  from one `VerifiedGenericRecipeDemandG0`/resolver source forest cohort and
  retain the complete Generic Recipe/Core/JoinSig, context, After, typed input,
  and entry/item/carrier coverage before the common co-seal consumes it.

Source authority + canonical issuer:
  The Generic producer owns source validation and Recipe/JoinSig issuance;
  a future `VerifiedGenericG0SourceParentV1` issuer retains the demand-backed
  relations once, then the compiler co-seal issues the non-Clone semantic
  program. Existing `VerifiedGenericRecipeProductG0` is only a partial parent.

Non-authority:
  `into_physical_parts_for_test`, `VerifiedLoopOperationPhysicalDemandV1`,
  Recipe counts/order, MIR/CFG observations, Callable relabeling, or a
  separate context/Core/continuation argument cannot complete the parent.

Fail-fast boundary:
  Missing source parent, initialized-local/parameter input mixing, owner/origin/
  site/frame/region drift, Core/JoinSig/After mismatch, incomplete or duplicate
  coverage, V1/V2 mixing, HRTB escape, or split/re-pair ingress keeps
  `NoSafeSlice::GenericG0EntrySourceCoverageParentUnsealed`.

Smallest next slice:
  The BoxShape is accepted as a design boundary.  The next implementation
  slice is the source-parent issuer below; no physical demand, Builder/session,
  CFG/SSA/PHI, lifecycle, Text, route, fallback, retry, or production caller
  is allowed until that slice closes.

Non-claims:
  Generic G0 semantic-program I0, all-19 convergence, M8/M9 migration,
  transfer/Layout/CFG authority, session, selection, production, and legacy
  retirement remain closed.
```

#### G0 parent BoxShape to settle before I0

The missing parent is not a second Recipe authority.  It is one move-only
co-seal of already-issued source products plus the exact resolver entry view:

```text
VerifiedGenericG0SourceParentV1<'source>
  ├─ source owner/origin/root-site/frame/region stamp
  ├─ ResolvedFunctionLoweringInputV1<'source>
  ├─ Core-owned Recipe + JoinSig + complete item/effect/carrier coverage
  ├─ JoinSig-issued After binding, parent-owned continuation projection,
  │  and Generic tail/return ABI
  ├─ typed entry rows (the two source parameter bindings, in source order)
  └─ (target/profile compatibility is a separate sibling, not semantic parent)
```

The future canonical issuer is a single Generic source-parent seam.  Its
input is one same-cohort source view (conceptually
`GenericG0SourceCohortRef<'source>`) that lends both the
`VerifiedGenericRecipeDemandG0` and resolver-owned
`ResolvedFunctionLoweringInputV1`; two independently acquired arguments are
not an accepted API.  The issuer must validate their owner, origin, root site,
frame, scope/region, and source kind before moving either into the parent.  It
may call the existing Generic producer inside that transaction, but it must not accept a separately supplied
`VerifiedLoopContinuationContractV1`, `VerifiedGenericG0TailCapabilityV1`,
entry vector, or physical demand.  The continuation is projected from the
same Core/JoinSig/After parent, and entry rows are issued from the resolver
input once, not reconstructed by a `cfg(test)` ingress.

The parent is lent through one callback-scoped view to the later common
co-seal.  The common co-seal checks only the parent stamp, schema revision,
and complete coverage; it does not re-walk source or turn the parent into a
second G0 semantic issuer.  Acceptance requires the exact root-plus-one-child-
loop shape plus missing/duplicate/foreign entry, owner/origin/frame/region
  drift, Core/JoinSig/After drift, coverage gap, target-sibling mismatch at the
  later co-seal, and one-shot/loan-escape negatives.  This parent and its
  production issuer are now landed in
  `compiler::generic_g0_source_parent`; no G0 physical or session effect is
  opened by this row.

#### LOOP-PRECUTOVER-AUTHORITY-G0-SOURCE-COHORT-D0 accepted design boundary

The current selector drops `ResolvedFunctionLoweringInputV1` after it issues
the Generic candidate, so a later parent issuer cannot safely accept a bare
`(Demand, input)` tuple.  The next design-only seam is:

```text
issue_generic_g0_source_cohort_v1(
    resolver-owned input + selected Generic source row,
    for<'loan> |cohort: GenericG0SourceCohortRef<'loan>| ...
)
```

The issuer must perform the Generic demand issuance inside the same private
cohort transaction, validate owner/origin/source-kind/root-site/frame/
scope-region and entry/coverage identity, and lend an opaque combined view.
The callback may invoke the one source-parent issuer, but it cannot receive
or return a bare Demand, input, candidate, stamp tuple, or second loan.  This
is a source/authority seam only; it does not change family selection or make
the candidate enum lifetime-parameterized.

Acceptance is the exact root-plus-one-child-loop positive shape, with
selection mismatch, foreign input, owner/origin/site/frame/region drift,
missing/duplicate entry or coverage, callback escape, and double-consumption
negatives.  The seam and its implementation are accepted; the old
`cfg(test)` ingress remains only as a historical canary.

#### LOOP-PRECUTOVER-AUTHORITY-G0-I0 implementation receipt (2026-08-17)

```text
Decision:
  The test-only Generic ingress is replaced by one production source-parent
  issuer.  It consumes one exact resolver input plus the selected Generic row
  and issues one private, non-Clone `VerifiedGenericG0SourceParentV1`; no
  semantic fact is reconstructed by the compiler co-seal.

Source authority + canonical issuer:
  The resolver/source projector and selected Generic row own source identity;
  `issue_generic_g0_recipe_demand_v1` and `produce_generic_g0_recipe_v1`
  remain the Generic source/Recipe/Core/JoinSig/After issuers.  The new
  parent issuer only co-seals those products with the resolver input, the
  exact root-plus-child-loop forest, and the two source-parameter entry rows.

Non-authority:
  `generic_g0_physical_prepare.rs`, `into_physical_parts_for_test`, raw
  counts/order, MIR/CFG, separate continuation/tail/entry arguments, target
  defaults, and any Callable/Dynamic/M8/M9 adapter are not issuers.

Fail-fast boundary:
  Reject foreign or split cohort loans, missing/duplicate entry rows, wrong
  parameter order, owner/origin/site/frame/region drift, incomplete node/item/
  carrier/effect coverage, Core/JoinSig/After drift, and test-only ingress
  reachability before any physical/session effect.

Smallest next slice:
  Keep the production parent and focused positive/negative tests as the only
  source-parent seam.  The old cfg(test) ingress remains a historical canary
  and is not a production caller.  Keep physical demand, Builder/session, CFG,
  PHI, lifecycle, Text, route, fallback, retry, and production caller at 0.

Non-claims:
  This I0 does not issue Generic physical demand, create a session, allocate
  physical IDs, migrate all-family profiles, or activate production.
```

### Post-G0 design stop: split S6C physical entry from Generic G0

The installed S6C cohort already closes its own physical-entry input: one
`S6CCommonV2PreSessionLoanRefV1` lends the catalog storage header, physical
signature/result, source-backed effects, and Completion sibling, and the
caller-zero descriptor aggregate projects those facts without Builder or
ValueId effects.  That result is S6C-specific; it is not a common Generic G0
issuer.

The Generic G0 source parent currently lends resolver/Core/JoinSig/After facts,
its exact two parameter-entry rows, result ABI, canonical Completion, and a
bounded source-effect receipt, but it does not carry a same-cohort physical
storage header, receiver/lane cohort, or physical-effect projection.  The
resolver-issued
`ResolvedFunctionBodyShapeProductV1` does contain the complete body-effect
inventory. `VerifiedResolvedSourceUnitV1` now retains the per-owner sibling
from the same resolver traversal, and `root_function_input()` lends the exact
owner-matched product to `ResolvedFunctionLoweringInputV1`; the Generic parent
checks its body root before issuing demand/product.  The callable batch has a
body-shape transport seam, but it is a separate callable cohort and is not a
Generic root issuer.  S6C receipts, raw source ParamDecls, `/N`, raw
`MirFunction` parameter length, and a default `EffectMask` must not be reused
or inferred to issue function effects.

The function-effect, result-ABI, and canonical Completion siblings are now
landed as source products.  The remaining Generic physical-entry work is a
mechanical storage/lane projection; it must not borrow the S6C package seam or
open a Builder/session effect.

### `LOOP-GENERIC-G0-PHYSICAL-ENTRY-SOURCE-PROJECTION-D0` (BoxShape accepted; source projection I0 next)

```text
Decision:
  Accept this D0 as a source-only BoxShape.  The Generic source parent is the
  sole semantic owner; the first I0 may retain one same-parent storage/lane
  projection, but it must not open a physical signature, MirFunction, Builder,
  session, or EffectMask effect.  Generic is not narrowed to a static-only
  subset: receiver policy is an explicit source axis.

Source authority + canonical issuer:
  The existing `VerifiedGenericG0SourceParentV1` transaction is the sole
  issuer.  It co-seals the exact `CallableHeaderSyntaxViewV1` row (including
  attrs/uses), `root_profile().receiver_policy()`, the optional
  `declaration_binding(SourceBindingSiteV1::Receiver)` and its binding record,
  and the already-issued explicit entry rows.  The new source-only row is
  borrowed from that parent; Package/Port and common V2 are transport only.

  Receiver policy is a separate callable axis, not an explicit formal:
  `DeclaredInstance` requires a receiver prefix and `Absent` requires none.
  `source_logical_arity` counts explicit formals only.  Explicit rows remain
  ordinal ordered and dense; a mechanical `ExistingCallableI64` carrier tag
  may be recorded only after this Generic source relation is checked.  The
  tag has no S6C semantic authority, and no S6C storage/signature/role row is
  reused.  The current fixture is therefore admitted as instance-shaped
  (receiver plus two explicit i64 formals), rather than silently rewritten as
  static.

Non-authority:
  S6C physical headers/effects/signature rows, the S6C entry-input module, raw
  ParamDecl or AST rescans after the source boundary, `/N`, MIR/JSON parameter
  length, default `EffectMask`, Recipe-local SourceRead/SourceWrite, Generic
  entry rows alone, `ValueId`, `MirType`, fixture staticness, and a caller-
  assembled descriptor cannot issue Generic storage or lane meaning.

Fail-fast boundary:
  Reject foreign owner/origin/source-kind/body-root/frame, missing or mixed
  parent siblings, attrs/uses or ParamDecl drift, policy/binding mismatch,
  missing or foreign receiver, duplicate receiver/explicit BindingRef,
  receiver/static mismatch, non-prefix receiver, parameter count/name/type/
  ordinal or BindingRef drift, missing/duplicate mechanical carrier, S6C reuse,
  or any request for `ValueId`, `MirFunction`, `EffectMask`, Builder, or session
  state.  The source projection must preserve the policy equation:
  `physical_callable_lane_count = receiver_lane_count + explicit_lane_count`,
  while `physical_formal_lane_count = explicit_lane_count`; it must not infer
  either count from `/N` or JSON length.

Smallest next slice:
  `LOOP-GENERIC-G0-STORAGE-LANE-SOURCE-PROJECTION-I0` is a caller-zero,
  source-only projection.  Add one private/non-Clone parent-owned row with
  owner/origin/source-kind/body-root/frame, attrs/uses witness, receiver
  policy plus optional receiver BindingRef, dense explicit rows, and the
  checked mechanical carrier tag.  Add positive instance/absent-policy cases
  and foreign owner, receiver mismatch, duplicate/missing row, ordinal/type/
  ABI drift, and late-failure no-publication negatives.  Do not create a
  physical descriptor, skeleton, ValueId, BindingSSA, EffectMask, Builder, or
  session.  A static-only cohort, if desired later, is a separate Decision;
  it is not a shortcut for this I0.

Non-claims:
  No physical signature reclassification, `EffectMask` issuance, skeleton,
  lane adoption, ValueId/BindingSSA, CFG/PHI, Completion claim, lifecycle,
  Text, route, production caller, fallback, or retry is opened by this census.
```

Implementation receipt (2026-08-17):
  `generic_g0_storage_lane_source.rs` now issues one private, non-`Clone`
  source row from the same Generic parent transaction.  The row retains the
  declaration attrs/uses witness, receiver policy and source `BindingRef`,
  dense explicit i64 rows, and the local mechanical
  `ExistingCallableI64` carrier.  It stores checked explicit logical arity
  and checked callable lane count; receiver count remains a separate axis.
  Instance-shaped and absent-receiver source facts, foreign-parent rejection,
  and late no-publication behavior are covered by the focused Generic source
  parent suite.  This receipt is source-only: it issues no physical signature,
  `EffectMask`, `ValueId`, skeleton, BindingSSA, Builder/session, CFG/PHI,
  lifecycle, Text, route, fallback, retry, or production caller.

### `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-D0` (BoxShape accepted; Generic-only)

```text
Decision:
  Accept a Generic-only, pre-effect physical-entry input BoxShape.  The
  existing Generic source parent remains the sole issuer and co-seals its
  declaration/header, result ABI, source-effect receipt, canonical Completion,
  and source storage/lane row.  A future input row may mechanically project
  those facts into receiver-prefix and explicit-lane descriptors, but it must
  not reuse the S6C descriptor/header/signature loans or create a MirFunction,
  ValueId, Builder/session state, or EffectMask.

Source authority + canonical issuer:
  `VerifiedGenericG0SourceParentV1` is the same-cohort owner.  Its declaration
  header owns symbol/params/types/uses/attrs/static/result spelling, its
  storage/lane child owns receiver policy and explicit BindingRef/ordinal/type
  rows, its result ABI child owns the source result ABI, its no-external-effect
  child owns the bounded source effect fact, and canonical Completion remains
  issued by `verify_function_completion_v1`.  The Generic physical-entry
  issuer is one callback-scoped compiler projection over that parent; it may
  borrow the existing mechanical `ExistingCallableI64` carrier tag only after
  the source row is checked.

  `DeclaredInstance` fixes physical order as `[receiver] + explicit rows`, and
  `Absent` fixes it as `explicit rows`; receiver count is never folded into
  source logical arity.  The future Generic descriptor owns the relation but
  does not become a second semantic authority.

Non-authority:
  `PhysicalCallableParameterDescriptorV1`, `PhysicalCallableLaneRoleV1`,
  `PhysicalCallableSignatureRowRefV1`, S6C storage/effect/signature loans,
  `/N`, MIR/JSON vector length, `ValueId`, `MirFunction`, `EffectMask`, raw
  ParamDecl/AST rescans, Recipe/JoinSig ordinals, or the local carrier enum
  alone cannot issue Generic physical-entry meaning.  ExistingCallableI64 is
  only a mechanical carrier projection, not a Generic or S6C semantic fact.

Fail-fast boundary:
  Reject foreign owner/origin/source-kind/frame, header/result/effect/
  Completion/storage-lane sibling drift, missing or duplicate receiver,
  receiver-policy mismatch, non-dense explicit ordinal/binding/type/ABI rows,
  count overflow, S6C descriptor reuse, lane-order drift, callback loan escape,
  or any request for skeleton/ValueId/BindingSSA/EffectMask/Builder/session.
  ExactText and other non-Generic physical expansions remain outside this
  Generic-only row; no default or S6C fallback is allowed.

Smallest next slice:
  `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-I0` is a caller-zero mechanical
  projection from one borrowed Generic parent into one private non-Clone
  Generic entry-input row.  It may record lane role/order, source BindingRef,
  receiver prefix, declared metadata, and the checked existing i64 carrier.
  It must not allocate a skeleton, reserve/adopt ValueIds, open a session,
  consume Completion, mutate CFG/SSA/PHI, lower operations, or select a route.

Non-claims:
  No S6C/common physical descriptor reuse, physical ABI activation, skeleton,
  entry-lane adoption, EffectMask issuance, Completion consumption, lifecycle,
  Text, route, fallback, retry, production caller, or main integration.
```

Design audit receipt (2026-08-17):
  The missing issuer is now named and bounded.  The Generic parent has all
  source siblings needed for a pre-effect row; only the mechanical carrier
  enum may be reused from the S6C-side module.  The next blocker is therefore
  implementation of the Generic-only input projection, not an S6C/common
  semantic reclassification.

Implementation receipt (2026-08-17):
  `generic_g0_physical_function_entry_input.rs` now issues one private,
  non-`Clone` Generic entry-input product from the same callback-scoped
  source parent.  It checks owner/origin/source-kind/body-root/frame,
  declaration/result/effect/Completion parity, receiver policy, dense
  ordinal/BindingRef/type/ABI rows, checked physical counts, and name
  collisions before publication.  `DeclaredInstance` is a `me` prefix and
  `Absent` has no receiver row; explicit source arity remains separate from
  callable lane count.  The parent and descriptor cohort can only be consumed
  together inside the loan.  The focused positive test is green and the
  existing parent rejection/no-publication tests cover bare and foreign
  ingress.  No S6C descriptor/header/signature reuse, skeleton, `ValueId`,
  `BindingSSA`, `EffectMask`, Builder/session, Completion consumption, CFG/PHI,
  lifecycle, Text, route, fallback, retry, or production caller opened.

### `LOOP-GENERIC-G0-PHYSICAL-EFFECT-PROJECTION-D0` (accepted BoxShape; I0 landed)

```text
Decision:
  Accept the Generic G0 physical-effect BoxShape, but open only its caller-zero
  projection I0.  The admitted Generic operation contract is the finite set
  `{ReadBinding, ConstI64, BinaryI64, CompareI64, WriteBinding}`: reads and
  writes stay in local BindingSSA, while the other three map to pure MIR
  arithmetic/compare/constant instructions.  Together with the source
  no-external-effect receipt and complete operation evidence, this permits one
  mechanical `EffectMask::PURE` projection.  No skeleton is opened here.

Source authority + canonical issuer:
  A Generic-only issuer consumes the same `VerifiedGenericG0SourceParentV1`
  cohort: source function-effect, complete Recipe operation/evidence product,
  result/header/storage rows, product `NumericTarget`, and execution frame.
  `issue_generic_g0_physical_function_effects_v1` issues one private,
  non-Clone projection carrying the owner/origin/source-kind/body-root/frame,
  target stamp, finite operation-contract stamp, and `EffectMask::PURE`.
  The source receipt remains semantic evidence; S6C physical effects are not
  reusable authority.

Non-authority:
  Bare `EffectMask::PURE`, local-write/tail-return counts alone, Recipe variant
  names alone, `/N`, `MirFunction`, `FunctionSignature`, S6C effects,
  AST/MIR/JSON rescans, host target probing, and absent calls alone cannot
  issue the Generic physical effect contract.

Fail-fast boundary:
  Before projection, reject owner/origin/source-kind/body-root/frame or target
  drift, incomplete/duplicate operation evidence, any operation outside the
  five admitted variants, heap/call/alloc/load/store/control-effect lowering,
  or result/header/storage mismatch.  Unknown future variants reject without
  mutation; `MirFunction::new` remains outside this row.

Smallest next slice:
  The caller-zero I0 is now landed.  The next design stop is
  `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-D0`; it must census the
  source-backed skeleton inputs and detached rollback owner before any effect.

Non-claims:
  No `MirFunction::new`, skeleton, entry adoption, Completion consumption,
  CFG/SSA/PHI, lifecycle, Text, route, fallback/retry, production caller, or
  main integration.
```

Design audit receipt (2026-08-17; accepted BoxShape):
  The Generic parent/entry input has the source-side symbol/mode/result,
  attrs/uses, and ordered lane facts, but no canonical physical-effect row.
  `common_v2_physical_function_skeleton.rs` requires a physical effect mask
  before `MirFunction::new`; Generic `VerifiedGenericG0NoExternalEffectV1`
  is intentionally weaker than that mask.  The resolver/Recipe audit closed
  the missing relation: all admitted Generic G0 operations are the finite
  local/pure set above, so `PURE` is permitted only through the new Generic
  mapping issuer, never as a default.  The skeleton remains closed.

  The follow-up issuer census confirms that the missing relation is not just
  a constructor call: `VerifiedGenericG0SourceParentV1::function_effect`,
  result ABI, storage/header, product target, and execution frame must be
  consumed together with a Generic physical-operation mapping.  Only when
  that mapping proves every admitted lowered operation is MIR-pure may a
  private physical projection choose `EffectMask::PURE`; local-write/tail
  counts, absent calls, `NumericTarget`, or the source receipt alone cannot
  make that choice.  The projection-only I0 below is the bounded
  implementation; it does not authorize a skeleton or `MirFunction` canary.

Implementation receipt (2026-08-17):
  `generic_g0_physical_function_effect.rs` now issues one private,
  non-`Clone` Generic physical-effect projection from the same source parent.
  It checks source identity/frame/result/header/storage parity, the product
  target, and complete evidence for the finite five-variant local/pure
  operation contract before projecting `EffectMask::PURE`.  The focused
  Generic suite (58 tests) is green and the projection remains borrowed and
  effect-free; no skeleton, `MirFunction`, `ValueId`, Builder/session,
  CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller
  opened.

### `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-D0` (accepted BoxShape, 2026-08-17)

```text
Decision:
  Accept one Generic-only detached skeleton reservation.  The entry-input
  product is consumed exactly once; the issuer internally reuses the already
  landed Generic physical-effect projection and returns one non-Clone shell
  owner.  The shell owns the unpublished `MirFunction`; dropping it is the
  complete pre-Builder rollback.  This is a BoxShape only and does not adopt
  entry lanes.

Source authority + canonical issuer:
  `PreparedGenericG0PhysicalFunctionEntryInputV1::consume` is the sole
  skeleton seam.  Its retained parent supplies the source header/mode and
  explicit parameter arity, ordered physical descriptors supply the receiver
  prefix and i64 lane rows, the result-ABI row supplies i64, and
  `issue_generic_g0_physical_function_effects_v1` supplies the same-cohort
  `EffectMask::PURE` projection.  The symbol is issued only as
  `CanonicalCallableSymbolV1::from_name_arity(header.name(),
  header.parameters().len())`; receiver lanes never change `/N`.  Current
  Generic metadata is exact-empty, so attrs/uses project only as empty.

Non-authority:
  S6C skeleton/header/signature rows, `MirFunction` parameter order,
  `ValueId` numbering, descriptor length as `/N`, JSON vector length, raw
  `ParamDecl`/AST rescans, current Builder blocks, passed-in/default
  `EffectMask`, and `new_selected_dynamic` cannot issue the Generic skeleton
  contract.

Fail-fast boundary:
  Malformed symbol/name, explicit-arity overflow, foreign parent/effect
  owner/origin/frame, non-empty metadata, mode/receiver drift, lane
  role/index/carrier/type/count drift, or any attempt to install the function,
  expose/adopt a reserved `ValueId`, open BindingSSA/CFG/PHI, consume
  Completion, or use a legacy finalizer rejects before `MirFunction::new`.

Smallest next slice:
  `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-I0`: allocate only one detached
  `MirFunction` at `BasicBlockId(0)` with canonical symbol, physical i64
  parameters, i64 result, and the issued PURE effect.  Return a private
  non-Clone wrapper retaining the parent/descriptors/shell.  Add positive and
  rejection/no-publication tests; no entry adoption.

Non-claims:
  No Builder/session effect, entry-lane adoption,
  Completion consumption, CFG/SSA/PHI, lifecycle, Text, route,
  fallback/retry, production caller, or main integration.
```

Acceptance receipt (2026-08-17): the worker census confirmed that the source
parent, entry descriptors, and physical-effect projection form one complete
Generic cohort. `MirFunction::new` is permitted only as a detached mechanical
reservation; its parameter `ValueId`s have no BindingRef/adoption meaning and
are discarded with the wrapper. The canonical symbol uses explicit source
arity (the instance fixture is `/2`) while physical lane count is three.
Generic metadata is accepted only in the current exact-empty form. The next
I0 is allocation-only; entry adoption is a separate later design stop.

Implementation receipt (2026-08-17):
`generic_g0_physical_function_skeleton.rs` now consumes the entry-input cohort,
reissues the same-parent PURE effect projection, validates source symbol/mode,
explicit arity, receiver-prefix ordering, i64 result/lanes, and exact-empty
metadata, then reserves one detached `MirFunction` at `BasicBlockId(0)`.  The
private non-`Clone` wrapper retains the parent, effect, descriptors, and shell;
descriptor drift is rejected before shell creation.  Two focused tests and the
60-test Generic suite are green.  No Builder/session, lane adoption,
Completion, CFG/PHI, lifecycle, Text, route, fallback, retry, or production
caller opened.

### `LOOP-GENERIC-G0-PHYSICAL-ENTRY-LANE-ADOPTION-D0` (accepted BoxShape, 2026-08-17)

```text
Decision:
  Accept a Generic-specific callback-scoped entry admission as a BoxShape.
  It co-seals already-issued Generic source products with the detached shell
  and gives one canonical session consumer enough evidence to adopt the
  receiver/ordinary lanes.  It does not reuse the S6C/common admission or
  issue a second semantic source fact.

Source authority + canonical issuer:
  `VerifiedGenericG0SourceParentV1` owns the resolver input, body-shape
  inventory, and semantic Completion.  The existing resolver issuer
  `issue_resolved_block_expr_expectation_v1` supplies the typed BlockExpr
  expectation, and `empty_for_owned_loop_profile` supplies the outer-If
  residual for the exact loop site retained by the Generic source parent.  The compiler-side
  `GenericG0DetachedEntryCanaryV1` only co-seals these existing views with
  the detached shell/descriptors and a Generic mechanical cohort stamp.  The
  later session consumer snapshots Completion through
  `ResolvedFunctionCompletionConsumptionV1::new_borrowed`; the sole physical
  declaration issuer remains
  `CanonicalSsaFunctionSessionV2::identity.publish_declaration_exact`.

Non-authority:
  S6C loan/key/stamp or admission aggregate, Generic shell name, `/N`, raw
  `pair_count`, `ValueId::new` alone, AST/MIR/arena recounts, Completion
  clone/move, raw BindingSSA, descriptor names, or a second sidecar/SSA owner
  cannot issue Generic adoption.  The S6C session's mechanical rollback
  pattern may be reused only behind the Generic admission boundary.

Fail-fast boundary:
  Foreign parent/origin/source-kind/body-root/frame, expectation owner/root,
  outer-If partition, Completion owner/target, shell symbol/arity or
  descriptor index/role/binding/site/type drift, `params[index]` mismatch,
  duplicate adoption, pre-existing session/function, partial publication,
  borrowed Completion escape, or raw-count/session re-pair rejects before
  Builder or BindingSSA effect.  The fresh unpublished transaction is the
  only rollback owner.

Smallest next slice:
  `LOOP-GENERIC-G0-PHYSICAL-ENTRY-LANE-ADOPTION-I0` consumes one admission,
  opens one fresh unpublished transaction, installs the detached shell, and
  adopts receiver/ordinary rows atomically through the canonical session.  A
  Generic session opener may be added as a thin consumer of the admission, but
  it must not create a second CFG/SSA owner or reuse the S6C envelope.  No
  Loop CFG, operation lowering, or Completion claim is part of this row.

Non-claims:
  This D0 does not construct a session, publish BindingSSA/ValueId, mutate
  CFG/PHI, consume Completion, lower operations, or open ExactText lifecycle,
  route, fallback/retry, production caller, or main integration.
```

Implementation receipt (2026-08-17):
`generic_g0_physical_entry_admission.rs` now co-seals the detached Generic
shell with the resolver-owned typed BlockExpr expectation, the outer-If
residual for the source parent's selected loop (so nested-loop source is not
collapsed to a function-wide singleton), the borrowed canonical Completion,
and a mechanical function/lane stamp.  The new
`with_generic_g0_physical_entry_session` consumer rejects a nonempty Builder,
opens one unpublished draft transaction, installs the detached shell, and
delegates receiver/ordinary declaration publication to the existing canonical
identity issuer.  The outer draft transaction is the sole rollback owner;
duplicate adoption is rejected.  Two focused tests and the 62-test Generic
suite are green.  Loop operations/CFG/PHI, Completion claims, lifecycle, Text,
route, fallback/retry, and production caller remain closed.

The D0 is accepted as a source/cohort BoxShape.  The implementation blocker is
now closed for this slice; the next bounded work is the post-adoption
convergence checkpoint below.

### `MIRBUILDER-CANARY-CONVERGENCE-CHECKPOINT-R0` (next design stop)

```text
Decision:
  Run one read-only convergence census before opening the next physical
  effect.  Do not add a new receipt or production edge in this checkpoint.
Source authority + canonical issuer:
  Existing active-card owners and the current production call graph; this R0
  issues no semantic product and only records retirement/ownership facts.
Non-authority:
  test-only canaries, `new_selected_dynamic`, S6C-specific envelopes,
  legacy finalizers, copied counts, and local green are not production owners.
Fail-fast boundary:
  Any duplicate authority, unresolved canary retirement owner, stale current
  pointer, or hidden caller prevents the next I0 from opening.
Smallest next slice:
  Census duplicate receipts, canary constructors, retirement conditions, and
  old edges; then publish one ordered next row with its owner and negatives.
Non-claims:
  No Generic/common session expansion, Loop CFG/ops/PHI, Completion claims,
  lifecycle/Text, route, fallback/retry, production switch, or main integration.
```

#### Adoption I0 implementation census (2026-08-17)

The D0 is accepted; this census narrows the implementation seam without
adding another source authority:

```text
source -> GenericG0SourceParentV1 + entry descriptors
       -> Generic cohort/adoption stamp (I0 implementation)
       -> neutral lane adapter (I0 implementation)
       -> fresh unpublished Builder/session transaction (I0 consumer)
       -> canonical identity.publish_declaration_exact
```

The existing `PhysicalFunctionEntryCohortStampV1` and
`with_common_v2_physical_entry_session` are S6C/common owners.  They retain
S6C selected-key and loan provenance, so reclassifying them as Generic would
create a second source relation.  The I0 must implement a Generic-owned,
non-Clone stamp/adapter while retaining the existing outer transaction as the
rollback owner.

The bounded implementation checks are ordered:

1. **Cohort stamp.**  Co-seal the retained Generic parent, detached shell,
   descriptor cohort, owner/origin/source-kind/body-root/frame, and physical
   parameter count.  The stamp must also cover each descriptor's physical
   index, receiver/formal role, BindingRef, source site, and carrier type.
2. **Neutral lane adapter.**  Prove that each descriptor consumes exactly the
   installed shell parameter at `params[index]`; preserve receiver-prefix and
   explicit-formal order; create no new `ValueId`, BindingSSA row, sidecar, or
   semantic meaning.  The adapter is a mechanical view, not an S6C session.
3. **Fresh rollback seam.**  Identify one unpublished function transaction
   whose discard owns every partial shell/identity mutation.  The canonical
   session may consume the admitted stamp, but it is not the source issuer and
   no Builder/session may open before the co-seal succeeds.
4. **Adoption contract.**  Fix the one-shot rule for receiver and ordinary
   scalar declarations, including parameter type and current entry-block
   checks.  Duplicate, foreign, reordered, or partially published rows reject
   before BindingSSA effect.

The session-admission side is a Generic consumer, not a reason to reuse the
S6C admission aggregate.  The I0 admission obtains the typed BlockExpr
  expectation from `issue_resolved_block_expr_expectation_v1`, obtains the
  outer-If residual from `empty_for_owned_loop_profile` using the exact loop
  site retained by the Generic source parent, and borrows the parent's Completion for the later
`new_borrowed` physical consumer.  The current
`LoopV2CanonicalSessionAdmissionRefV1` remains S6C-only; the Generic opener
must co-seal these three views with the Generic parent/stamp in one
callback-scoped product.

The I0 closes these four products in one fresh unpublished transaction,
performs exact shell-parameter checks, and atomically adopts receiver/ordinary
declarations.  It does not open Loop blocks, operations, PHI, Completion
claims, lifecycle, Text, route selection, fallback, or production.

### Generic G0 source-projection child tasks (ordered; post-adoption checkpoint next)

The Generic parent now has all four source siblings.  The following rows keep
the source projection and physical consumers separate; they do not authorize
a Builder/session effect unless the row explicitly says so.

#### `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-D0` (accepted 2026-08-17)

```text
Decision:
  Keep function-level effect as a separate Generic source-projection
  decision.  The future `VerifiedGenericG0NoExternalEffectV1` is a bounded
  source receipt, not a physical `EffectMask`; it is issued inside the existing
  Generic source-parent transaction before demand/product consumption.

Source authority + canonical issuer:
  `input.body_shape().effects()` is the complete body-effect authority from
  the same resolver traversal.  The future issuer co-seals it with the same
  `VerifiedResolvedFunctionV1` direct/method-call and assignment/exit
  inventories, the existing typed declaration-header metadata-empty witness,
  and the selected Generic structural facts.  The structural facts are
  borrowed from the selection before `issue_generic_g0_recipe_demand_v1`
  consumes that selection; they are never reacquired or reconstructed.

Non-authority:
  Recipe-local `SourceRead`/`SourceWrite`, raw AST/MIR/JSON rescans,
  parameter count, `/N`, `EffectMask` defaults, S6C effects, body-shape count
  alone, direct/method inventories alone, and Generic structural facts alone.

Fail-fast boundary:
  Reject missing body-shape, owner/origin/source-kind/body-root or Generic
  root-frame drift, non-empty uses/attrs/contracts, any direct or method call,
  any `Allocation`/`Await`/`QMark`/`Throw`/`NonLocalControl` effect, field,
  upvar, or index writes, extra local rebinds, write-site drift, and extra
  Break/Continue/Return exits.  The admitted G0 fixture must have exactly two
  `Write` rows, both local `BindingRebind` targets matching the outer/inner
  structural update bindings, zero call-like effects, and exactly one explicit
  tail Return.  Every rejection precedes receipt and physical/session effect.

Smallest next slice:
  This D0 is accepted as a source-projection BoxShape.  The first I0 may add
  one private, non-Clone effect module and issue the bounded source receipt in
  the parent order
  `validate input/body-shape/header/structural facts -> issue effect receipt
  -> consume Generic demand/product`.  No `EffectMask` emission is part of
  that I0.

Non-claims:
  No result/Completion co-seal, physical EffectMask, physical entry/header,
  skeleton, lane adoption, ValueId/BindingSSA, CFG/PHI, lifecycle, Text,
  route, fallback, retry, or production caller.
```

Acceptance receipt (2026-08-17): the resolver body-shape sibling, resolved
function inventories, metadata-empty declaration header, and Generic
structural facts are all available from one source cohort.  The selection
candidate exposes the structural facts before the selection is consumed by
`issue_generic_g0_recipe_demand_v1`; no AST/MIR rescan, raw count, or default
effect is needed.  The next bounded slice is implementation-only and remains
caller-zero.

#### `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-I0` (landed 2026-08-17)

```text
Change:
  add one private/non-Clone `VerifiedGenericG0NoExternalEffectV1` source
  receipt and retain it in the Generic source parent.
Contract:
  issue it after same-cohort validation and before moving the selection into
  Generic demand/product; admit exactly two local BindingRebind Write rows,
  zero calls/non-Write effects, and one explicit tail Return.
Done:
  focused positive plus missing/foreign/owner-root/frame drift, metadata,
  call/non-Write, assignment/exit drift, and late-failure no-publication
  negatives; no physical EffectMask or session effect.
Stop:
  raw count/default, Recipe-local effect authority, AST/MIR rescan, fallback,
  retry, or any Builder/session/ValueId/CFG/PHI/lifecycle effect is required.
```

Implementation receipt (2026-08-17): `VerifiedGenericG0NoExternalEffectV1`
is issued before the Generic selection is consumed and retained by the
non-Clone source parent.  It co-seals the same-resolver body-shape effect
inventory, resolved call/assignment/exit inventories, the metadata-empty
declaration header, and the selected Generic structural facts.  Focused
positive, owner/root/frame, metadata, call/non-Write, assignment/exit, and
late-failure no-publication tests are green.  This is a source receipt only;
no `EffectMask`, Builder/session, or physical effect was opened.

#### `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-D0`

```text
Decision:
  Keep Generic result ABI as a same-cohort transport decision after the landed
  declaration/header and function-effect receipts.  The existing candidate
  already owns the source result ABI, and the parent now retains one private
  one-shot result row.  Do not issue a new combined result/Completion receipt
  or reopen the physical-entry/session path.

Source authority + canonical issuer:
  `selection.candidate()` → Generic observation → its already co-sealed
  `VerifiedGenericTypedSourceBundleG0::return_abi()` is the sole source ABI
  authority.  The Generic parent transaction must borrow that row before
  `issue_generic_g0_recipe_demand_v1` consumes the selection, verify owner /
  origin / source-kind and declaration-header parity, then retain one private
  non-Clone transport row.  Package/Port may only transport that row.

Non-authority:
  S6C physical headers or Completion, `product.after().return_abi()`, Generic
  Recipe/Core return values, `/N`, raw ParamDecl/MIR/JSON, `EffectMask`,
  numeric target defaults, source names, and a copied result summary cannot
  issue the result product.

Fail-fast boundary:
  Reject missing candidate result row, foreign owner/origin/source-kind/site/
  frame, return annotation/ABI mismatch, selection consumption before the
  borrow, duplicate transport, mixed cohorts, and any need to infer result
  meaning from Recipe order, MIR arity, or a default type.  Any Completion,
  physical header, lane, skeleton, Builder/session, or ValueId effect remains
  before this boundary.

Smallest next slice:
  The transport BoxShape is accepted and its caller-zero I0 is landed.  The
  next independent sibling is the Generic Completion issuer census.  No
  physical entry or session consumer is part of this slice.

Non-claims:
  No new result classifier, Completion issuer/consumption, physical
  ABI/EffectMask, skeleton, lane adoption, ValueId/BindingSSA, CFG/PHI,
  lifecycle, Text, route, fallback, retry, production caller, or main
  integration.
```

#### `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-I0`

```text
Change:
  Retain one private/non-Clone result-ABI row in the existing Generic source
  parent.  Borrow `selection.candidate().observation().bundle().return_abi()`
  before the selection is consumed by demand/product issuance.

Contract:
  Verify candidate and declaration-header owner/origin/source-kind parity and
  exact return-annotation/ABI parity; store only the existing ABI capability.
  No new classifier, AST traversal, Recipe inference, or Completion row is
  allowed.  The row is lent only through the parent callback view.

Done:
  Positive exact-G0 transport, foreign/mixed candidate, missing or mismatched
  annotation/ABI, duplicate/re-entry, and late-failure no-publication tests;
  source, function-effect, and result rows remain same-cohort and one-shot.

Stop:
  No Completion verification/consumption, physical ABI, EffectMask, skeleton,
  lane adoption, ValueId/BindingSSA, CFG/PHI, lifecycle, Text, route,
  fallback, retry, or production caller.
```

Implementation receipt (2026-08-17):
`issue_generic_g0_result_abi_transport_v1` borrows the selected Generic
observation's existing `return_abi()` before demand/product consumption,
checks owner/origin/source-kind, exact loop site/frame, and declaration-header
parity, and retains one private `VerifiedGenericG0ResultAbiV1` in the source
parent.  Focused exact/foreign transport tests are green.  No new classifier,
Completion, EffectMask, physical ABI, skeleton, ValueId, session, CFG/PHI,
lifecycle, Text, route, fallback, retry, or production caller was opened.

#### `LOOP-GENERIC-G0-COMPLETION-PROJECTION-D0` (accepted BoxShape 2026-08-17)

```text
Decision:
  Keep Generic Completion as an independent source-projection decision after
  the result-ABI row.  Accept direct transport of the canonical
  `VerifiedFunctionCompletionV1`; do not create a combined result/Completion
  authority or a duplicate Completion wrapper.

Source authority + canonical issuer:
  `verify_function_completion_v1(input)` is the sole Completion issuer for the
  exact Generic function.  The existing Generic parent may retain that
  non-Clone canonical product once, after owner, target region, value-return
  shape, terminal site, declared `i64`, and cleanup policy are checked against
  the same source cohort.

Non-authority:
  S6C Completion, copied exit summaries, Recipe Tail/After return ABI,
  `ResolvedFunctionCompletionConsumptionV1`, DraftSeal, MIR/JSON, and any
  source-order count cannot issue Generic Completion.

Fail-fast boundary:
  The canonical verifier rejects foreign/source-region/origin/transfer/
  terminal/result drift.  The Generic transport additionally rejects owner,
  target, source-kind, non-value or implicit completion, terminal-site mismatch
  with the Generic tail statement, declared-result mismatch, cleanup-bearing
  completion, duplicate transport, and any request to consume or claim
  Completion before this source product exists.

Smallest next slice:
  Open `LOOP-GENERIC-G0-COMPLETION-PROJECTION-I0`: call the canonical verifier
  once before selection demand/product consumption, retain the canonical
  product in the existing parent, and lend only a callback-scoped borrow.
  Completion consumption remains a later physical/session slice.

Non-claims:
  No Completion consumption, physical header/lane/skeleton/session, CFG/SSA/
  PHI, lifecycle, Text, route, fallback, retry, or production caller.
```

Implementation scope for the next I0 is intentionally mechanical: the
parent may validate the canonical completion against its existing result ABI
and Generic tail facts, but it may not issue a second semantic Completion
receipt or copy sites/counts into a summary row.

Implementation receipt (2026-08-17):
`issue_generic_g0_completion_transport_v1` calls the canonical verifier once
before Generic demand/product consumption, then retains the resulting
non-Clone `VerifiedFunctionCompletionV1` directly in the source parent.  It
checks owner/target, value-return, the exact Generic tail site, declared `i64`,
and empty cleanup; the parent callback lends only a borrow.  Focused source
parent tests are green.  No Completion consumer, physical/session effect,
CFG/SSA/PHI, lifecycle, Text, route, fallback, retry, or production caller
was opened.

#### `LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0` (landed transport I0)

```text
Decision:
  Do not issue a Generic function-effect receipt.  The resolver-owned
  `ResolvedFunctionBodyShapeProductV1` is transported from the same shadow
  traversal into the root lowering input and Generic source parent.  This is a
  source-product transport seam, not a new effect authority or count-only
  adapter; the caller-zero I0 below closes this row.

Source authority + canonical issuer:
  The resolver remains the sole issuer of the function/body-shape product.
  `VerifiedResolvedSourceUnitV1` retains the per-owner body-shape inventory
  from `resolve_forest_with_body_shapes`; its root input lends the exact
  owner-matched sibling, and the Generic parent validates owner/body-root
  before issuing demand/product.  The existing callable-batch body-shape
  carrier remains a separate cohort and is not a Generic issuer; Package/Port
  only transport already-co-sealed views.

Non-authority:
  `VerifiedResolvedFunctionV1` without its body-shape sibling,
  `ResolvedFunctionLoweringInputV1` without its body-shape sibling, Generic structural facts, the
  callable-batch row from another cohort, Recipe-local effects, AST/MIR/JSON
  rescans, raw `usize` counts, `EffectMask`, and a second resolver invocation
  are not body-effect authority.

Fail-fast boundary:
  Reject missing body-shape product, foreign owner/origin/body-root, source-unit
  and Generic-cohort mismatch, duplicate or separately re-resolved product,
  incomplete effect coverage, count-only reconstruction, and HRTB loan escape
  before any effect issuer, physical header, skeleton, session, or Builder
  mutation.  No default empty body-shape is permitted.

Smallest next slice:
  The transport I0 is landed: source-unit resolution stores the same-traversal
  body-shape map, root input attaches one exact sibling, and the Generic parent
  borrows/validates it before product issuance.  Focused tests cover root
  presence, bare-input absence, owner/root transport, and the existing foreign
  cohort rejection.  The function-effect issuer is now landed in its own
  source-parent row; this transport remains a mechanical sibling only.

Non-claims:
  No EffectMask, result/Completion co-seal,
  physical entry/skeleton, ValueId/BindingSSA, CFG/PHI, lifecycle, Text,
  route, fallback, retry, production caller, or main integration.
```

#### `LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0` implementation receipt (2026-08-17)

`VerifiedResolvedSourceUnitV1::resolve_function` now retains the per-owner
body-shape inventories emitted by the same resolver traversal.  The root
`ResolvedFunctionLoweringInputV1` lends the exact owner-matched sibling, while
bare mechanical inputs remain explicitly body-shape-free.  The Generic source
parent requires that sibling and checks owner/body-root equality before issuing
its existing source demand/product.  Focused tests cover source-unit presence,
bare-input absence, owner/root transport, and foreign-cohort rejection.  The
later function-effect receipt is now landed separately; this transport row
opened no `EffectMask`, skeleton, session, Builder effect, fallback, retry,
or production caller.

#### `LOOP-GENERIC-G0-TOPLEVEL-DECLARATION-HEADER-I0` (accepted bounded source projection)

```text
Decision:
  Accept one source-backed declaration/header projection for Generic G0
  TopLevel only.  This is a mechanical source projection, not a physical ABI
  or function-effect issuer.  It may run while the parent physical-entry
  cohort remains blocked on result/effect/Completion siblings.

Source authority + canonical issuer:
  `ResolvedFunctionLoweringInputV1::source()` and its
  `CallableHeaderSyntaxViewV1` are the sole source.  The Generic source-parent
  issuer issues one private, non-Clone
  `VerifiedGenericG0TopLevelDeclarationHeaderV1` inside its existing
  callback-scoped cohort transaction and lends it through the cohort view.

Non-authority:
  S6C storage headers, raw ParamDecl outside the source view, `/N`, MIR
  parameter length, JSON, ValueId, runtime metadata, Generic effect facts, and
  any caller-side reconstructed header.

Fail-fast boundary:
  Reject non-function roots, owner/origin/source-kind drift, parameter or
  ParamDecl count/name/order drift; retain the exact declared type spelling;
  reject return-annotation drift, non-empty
  uses/attrs/contracts when the TopLevel profile requires metadata-empty, and
  callback loan escape or duplicate consumption.

Smallest next slice:
  Add source-view getters, issue/store the typed header in the existing
  Generic cohort, and add focused positive/negative tests.  Keep result ABI,
  receiver/lane layout, function effect, Completion, skeleton, and session
  outside this I0.

Non-claims:
  No physical signature, receiver lane, EffectMask, Completion, CFG/SSA/PHI,
  lifecycle, Text, route, fallback/retry, production caller, or main
  integration.
```

Implementation receipt (2026-08-17):
  The source view now exposes mechanical `uses`/`attrs` accessors, and the
  existing Generic source-parent transaction stores one typed, non-Clone
  declaration/header row.  Focused source-parent tests cover the exact name,
  parameter, type-spelling, return annotation, and metadata-empty projection;
  the parent physical-entry blocker remains unchanged.

#### `MIRBUILDER-CANARY-CONVERGENCE-CHECKPOINT-R0` (parked after the parent cohort)

After the Generic source-parent cohort and its first child projections land,
run one read-only convergence audit before adding more leaf receipts.  Record
only: (1) caller-zero/test-only constructor count, (2) duplicate receipts that
carry the same authority, (3) each canary's final production owner and delete
condition, and (4) the complete legacy-edge retirement list.  This checkpoint
does not reopen `new_selected_dynamic`, the selected-normal legacy finalizer,
physical session effects, or any fallback route.

The audit must also close four readability risks without minting a new semantic
receipt: `VerifiedCallableSemanticProgramV1::into_prepared_parts` remains a
crate-local escape hatch until it is either replaced by a direct consuming
callback or explicitly retired; `PreparedLoopOperationRowV2` remains an S6C
provenance adapter and must not be relabeled as a Generic/G0 authority; every
caller-zero canary gets a named final owner plus a same-commit deletion gate;
and `DynamicProfileOwned`, `new_selected_dynamic`, and the selected-normal
legacy finalizer each get a concrete zero-caller retirement condition.  A
receipt that merely aliases an existing authority, transports a count, or
copies a source site is folded into its parent rather than promoted to a new
task/type.

The output is a read-only convergence manifest (owner, duplicate-authority
finding, final consumer, delete condition, evidence command).  It is not an
implementation permission and does not authorize physical/session effects.
The Generic storage/lane source BoxShape is now accepted independently; an
unresolved production owner still keeps the later physical-entry cutover
closed.

#### `MIRBUILDER-CANARY-CONVERGENCE-MANIFEST-R0` (read-only taskization, 2026-08-17)

```text
Decision:
  Record the current owners and retirement gates in one manifest.  This is a
  design-stop census; it does not mint a receipt, change a caller, or open a
  physical effect.
Source authority + canonical owner:
  Generic entry adoption remains owned by
  `generic_g0_physical_entry_admission.rs` ->
  `generic_g0_physical_entry_session.rs`; common CFG/SSA remains owned by
  `CanonicalSsaFunctionSessionV2`.  The manifest records existing owners only.
Non-authority:
  Tuple decomposition, S6C provenance rows, cfg(test) ingress, detached
  `MirFunction::new`, `DynamicProfileOwned`, `new_selected_dynamic`, copied
  counts, raw ValueId/AST rescans, and local green are not canonical owners.
Fail-fast boundary:
  Keep the stop while any production/test caller, duplicate authority, HRTB
  escape, or old edge lacks an owner and a zero-caller deletion condition.
Smallest next slice:
  Publish the following owner/retirement manifest; only after it is reviewed
  may a separate design card name the next physical owner.
Non-claims:
  No canary deletion, tuple-API rewrite, Dynamic-session integration, legacy
  finalizer retrofit, Canonical session expansion, CFG/SSA/PHI, Completion,
  DraftSeal, lifecycle, Text, route, fallback, retry, or production switch.
```

| seam | current finding | final owner | deletion / cutover gate | evidence |
|---|---|---|---|---|
| `VerifiedCallableSemanticProgramV1::into_prepared_parts` | Landed sealing refactor; the old six-tuple symbol has zero Rust source callers. | `PreparedCallableOperationDemandV1` plus `normal_callable_prepared_operation::prepare_full_demand` | Keep the source-free parent/one-shot consumer; do not reintroduce tuple getters or a second semantic issuer. | `rg -n --glob '*.rs' "into_prepared_parts" src/mir` |
| `PreparedLoopOperationRowV2` | S6C source provenance is retained by design; it must not be relabeled as Generic/common authority. | S6C common-V2 adapter, later consumed by the family-neutral parent. | Keep until S6C provenance is consumed by the neutral parent; no Generic conversion or duplicate row. | `rg -n "PreparedLoopOperationRowV2" src/mir` |
| `issue_generic_g0_loop_ingress_v1` | All current callers are tests/canaries; no production caller remains after the Generic source-parent I0. | Combined Generic emitter admission -> neutral canonical session -> common dispatcher. | Delete old ingress and migrate its tests in the operation-emitter production-switch slice, then prove zero callers. | `rg -n "issue_generic_g0_loop_ingress_v1" src/mir` |
| `with_generic_g0_physical_entry_session` / `new_generic` | Caller-zero detached canary; it proves rollback/adoption but is not a production route. | One family-neutral canonical unpublished-session opener; Generic and S6C remain thin admission adapters, not duplicate session implementations. | After session-preflight parity, switch the caller and delete the detached skeleton/canary/session plus their tuple exits in the same zero-caller slice. | `rg -n "with_generic_g0_physical_entry_session|new_generic|GenericG0DetachedEntryCanaryV1" src/mir` |
| `DynamicProfileOwned` / `new_selected_dynamic` | Current selected-Dynamic physical emitter still calls it; it is a live production owner, not a removable canary. | Common canonical session admission/session cutover for selected Dynamic. | First land the replacement, switch the production caller, verify zero `new_selected_dynamic` callers, then remove the enum arm and constructor. | `rg -n "DynamicProfileOwned|new_selected_dynamic" src/mir` |
| `finalize_function_draft*` selected-normal legacy edges | Multiple production callers remain in normal cataloged methods, recursive child lowering, port-aware wrappers, indexing, and calls. | `PreparedFunctionExitSetV1` -> canonical Completion/DraftSeal -> atomic unpublished publication. | Switch every listed production caller, prove old-symbol caller count is zero, then delete the legacy facade; do not retrofit it for Text/lifecycle. | `rg -n "finalize_function_draft(_with_headers)?\\(" src/mir/builder` |
| `VerifiedGenericRecipeProductG0::into_physical_boundary` | Production-visible but caller-zero split; it is a real sealing surface, not a second semantic issuer. | One-shot Generic source-parent/cohort/admission consumer. | Isolate behind `cfg(test)` in `LOOP-GENERIC-G0-SEALED-CONSUME-I0`; prove production caller count zero before session effects. | `rg -n "into_physical_boundary" src/mir` |
| `VerifiedCallableSemanticProgramV1::into_prepared_parts` | Retired by the semantic-program consume I0; remaining mentions are documentation evidence only. | `PreparedCallableOperationDemandV1` | Zero Rust source callers is landed; preserve the one-shot parent and keep this escape hatch retired. | `rg -n --glob '*.rs' "into_prepared_parts" src/mir` |
| `EndAuthorizedTextV1` public facade/getters | Runtime lease owner is valid, but the public wrapper/getters are a separate facade debt. | Runtime lease owner with a move-only/private completion surface. | Park in `RUNTIME-END-AUTHORIZED-TEXT-FACADE-I0`; preserve lease semantics and prove facade callers before narrowing. | `rg -n "EndAuthorizedTextV1|consume_end_authorized" src/runtime` |
| `generic-loop-legacy-disposition-v1.tsv` decision column | Corpus inventory is intentionally P0 and currently contains non-applicable sentinels; it is not a replacement decision. | Manifest-led owner/retirement evidence. | Fill only observed rows with owner, parity gate, and retire row; no bulk relabel or LOC-driven deletion. | `sed -n '1p' docs/development/current/main/design/fixtures/generic-loop-legacy-disposition-v1.tsv` |
| byte-identical helper/micro-seed groups | Informational duplicate census; byte identity does not prove semantic interchangeability. | Each helper's source-backed owner, or explicit archive/keep owner. | Separate inventory R0 first; merge/delete only in a focused parity slice with zero callers. | `rg --files src lang/c-abi | wc -l` |

**Manifest result.**  The Generic entry adoption I0 is complete.  The
canary/legacy graph now has an owner and a zero-caller deletion gate for every
listed seam; those gates remain cutover blockers and are not silently deleted.
The next design stop is the operation-contract census below.  No physical
operation, CFG, lifecycle, Text, or production cutover is authorized by this
manifest. `generic-loop-legacy-disposition-v1.tsv` remains a source-corpus
inventory, not this replacement-decision authority; its inventory rows are
not bulk-relabeled to simulate retirement decisions.

### `LOOP-GENERIC-G0-PHYSICAL-OPERATION-CONTRACT-D0` (next design stop)

```text
Decision:
  Census one Generic-only physical operation contract for the admitted finite
  set {ReadBinding, ConstI64, BinaryI64, CompareI64, WriteBinding}.  Do not
  reuse the S6C `PreparedLoopOperationRowV2` or open a Builder effect here.
Source authority + canonical issuer:
  The existing `VerifiedGenericG0SourceParentV1` cohort is the only source
  input: its resolver-bound operation/evidence product, entry BindingRefs,
  function-effect receipt, result/header/storage rows, target, and frame must
  be consumed together by a future Generic operation-contract issuer.  This
  D0 only names and audits that seam; it issues no new receipt.
Non-authority:
  S6C provenance rows, `issue_generic_g0_loop_ingress_v1`, Recipe variant names
  alone, MIR/JSON/AST rescans, `MirFunction`/ValueId numbering, `/N`, raw
  counts, `EffectMask::PURE`, and `new_selected_dynamic` are not issuers.
Fail-fast boundary:
  Keep the stop if operation evidence is incomplete/duplicated, owner/origin/
  root/frame/target drifts, a binding row cannot map to the retained entry
  BindingRef, or any heap/call/alloc/load/store/control-effect operation is
  admitted. Unknown future variants reject before Builder/session effect.
Smallest next slice:
  Read-only issuer census: decide whether a family-neutral operation view can
  consume Generic rows without carrying S6C provenance, then publish one
  BoxShape and its focused negatives. No I0 is authorized until that result is
  named.
Non-claims:
  No operation emission, block/edge/CFG/SSA/PHI mutation, Completion/DraftSeal,
  lifecycle, Text, route, fallback/retry, production caller, or main switch.
```

**D0 audit result (2026-08-17).**  The source census confirms that the
canonical operation/evidence owner already exists:
`VerifiedGenericG0SourceParentV1` ->
`VerifiedGenericRecipeProductG0::operation_effect()` ->
`VerifiedLoopOperationEffectProductV1`.  Its Generic producer proves complete
coverage for 15 rows (items `0,1,2,3,5..15`); item `4` is a nested Loop and the
carrier/tail are separate contracts.  No new semantic operation receipt is
needed.  The S6C `PreparedLoopOperationRowV2` remains a provenance adapter and
is not a Generic input.

The D0 is accepted as a mechanical BoxShape.  The next bounded implementation
is a caller-zero private mapping over the five admitted operation variants:

```text
LOOP-GENERIC-G0-PHYSICAL-OPERATION-MAPPING-I0
  parent callback -> operation/evidence borrow
  -> exhaustive five-variant mechanical mapping
  -> private non-Clone mapping product
  -> no Builder / ValueId / CFG / SSA / PHI effect
```

Its issuer must preserve the Generic operation/evidence item identity and
BindingRef/value-class relations, reject item `4`/carrier/tail leakage, and
never construct an S6C source row.  The mapping is a physical projection, not
a new semantic authority.  Focused positives cover all five variants and the
15-row coverage; the landed negative boundary keeps item `4`/carrier/tail out
and rejects missing/duplicate/foreign evidence, binding or value-class drift,
unknown variants, and late discard before any Builder effect.

### `LOOP-GENERIC-G0-PHYSICAL-OPERATION-EMISSION-D0` (accepted ownership BoxShape)

The following was the input census brief.  Its unresolved owner boundary is
closed by the accepted emitter-admission audit immediately below; it is not the
current execution row.

```text
Decision:
  Keep the five-variant Generic operation mapping as a mechanical input and
  adopt the existing family-neutral Loop operation dispatcher as the sole
  physical leaf-emitter candidate.  The source-owned cohort now supplies one
  complete `PreparedLoopOperationProgramV1`; this D0 must name the remaining
  common/session/layout/rollback boundary rather than create another Generic
  program issuer.  This D0 does not emit MIR or open Builder state; it closes
  only the owner and lowering-boundary census.
Source authority + canonical issuer:
  The same Generic source parent and its verified operation/evidence product
  remain semantic authority, and the landed mapping is the only mechanical row
  input.  The accepted cohort owns the family-neutral
  `PreparedLoopOperationProgramV1` and lends a transient mapping only inside
  its callback.  The remaining canonical issuer must consume that cohort view
  once, co-seal the common session/layout/target/rollback facts, and lend the
  program to the existing dispatcher without introducing a Generic duplicate
  leaf emitter.
Non-authority:
  S6C provenance rows, the old V1 `ReadyLoopEntryV1`/block receipt as Generic
  semantic input, operation names alone, item ordinals, `/N`,
  MirFunction/ValueId numbering, current block counters, EffectMask defaults,
  `new_selected_dynamic`, JSON, and late Builder scans are not operation
  meaning or placement authorities.
Fail-fast boundary:
  Keep the stop for missing mapping rows, foreign owner/origin/frame/target,
  binding or value-class drift, target-block/session-stamp/layout drift,
  incomplete cohort program, unsupported variants, or an emitter that infers
  placement/effects after Builder mutation.  Program, mapping, and common
  session checks must finish before any MIR instruction, ValueId publication,
  CFG/SSA/PHI mutation, or retry/fallback.
Smallest next slice:
  Design-only census for a production cohort-to-common dispatcher seam:
  exact session/layout/target block inputs, one-shot consumption, rollback and
  publication boundary, and the existing common leaf callback.  If any
  source-backed input is absent, retain this NoSafeSlice and do not add a
  speculative adapter.
Non-claims:
  No operation MIR emission, block/edge/CFG/SSA/PHI mutation, Completion or
  DraftSeal claim, lifecycle, Text, route, backend, fallback/retry, production
  caller, or main integration.
```

The mapping I0 and source-owned cohort I0 are complete, and the common
dispatcher is the only accepted leaf-emitter candidate.  The corrected
admission BoxShape below now closes the source/prephysical ownership boundary:
one parent consume, neutral layout/program, physical-ID-free shell plan, entry
control, Completion, target, and one full stamp.  Actual shell/session/segment
effects remain closed behind the later session-preflight D0.  The first fast
row and combined admission are now landed; no second Generic program issuer
was added.

#### D0 audit result — emitter admission BoxShape accepted 2026-08-17

```text
Decision:
  Accept one non-Clone `PreparedGenericG0PhysicalEmitterAdmissionV1` as the
  combined source/prephysical owner.  It owns no `MirFunction`, `ValueId`,
  `BasicBlockId`, Builder, or session.  A later
  `CanonicalFunctionLoweringSessionV1` consumes it, creates the shell inside
  the unpublished transaction, and remains the sole rollback owner.  The
  existing common dispatcher stays the only leaf-emitter candidate.
Source authority + canonical issuer:
  `VerifiedGenericG0SourceParentV1` remains the sole semantic authority.
  `issue_generic_g0_physical_emitter_admission_v1` consumes it exactly once,
  issues the existing `PreparedLoopOperationProgramV1`, validates the scoped
  five-variant mapping, and moves that program into the family-neutral
  `PreparedLoopPhysicalLayoutV1`.  A private
  `VerifiedGenericG0PhysicalLayoutBindingV1` binds that layout to the same
  source cohort without making the neutral layout Generic-specific.
  The admission also owns one physical-ID-free shell plan (canonical symbol,
  ordered descriptors, declared metadata, result ABI, and physical effects),
  typed BlockExpr expectation, outer-If residual, canonical Completion,
  target, and one full mechanical cohort stamp.  The stamp co-seals owner,
  origin, source kind, body root, exact Loop site, frame/scope/region,
  explicit arity/receiver/lane coverage, program/layout coverage revisions,
  and target identity; it does not reissue those semantic facts.  Mapping is
  borrowed only from `layout.program()` inside a scoped callback.
Non-authority:
  Owner equality, the weak owner/name/lane-count
  `GenericG0DetachedEntryCanaryStampV1`, S6C
  `PhysicalFunctionEntryCohortStampV1`, the current parent-borrowed detached
  skeleton/entry admission, their tuple `into_parts`, a detached
  `MirFunction`, raw IDs, `NumericTarget` or `EffectMask::PURE` alone, the S6C
  common-V2 envelope/segment allocator, old V1 receipts, MIR/JSON rescans, and
  a second Generic dispatcher are not admission or layout authorities.
Fail-fast boundary:
  Before an unpublished transaction opens, reject a second source-parent
  issue; owner/origin/source-kind/body-root/Loop/frame/scope/region drift;
  program/mapping/layout coverage drift; symbol/explicit arity/receiver/lane,
  header/effect/result/Completion/target drift; missing or duplicate full
  stamp; callback escape; or double consume.  Admission exposes no raw tuple,
  independent program/layout/Completion/stamp getter, or stored mapping.
  After a later session opens, only the outer unpublished transaction may
  discard a failed candidate; local repair, retry, and fallback are forbidden.
Smallest next slice:
  `LOOP-GENERIC-G0-PHYSICAL-EMITTER-SESSION-PREFLIGHT-D0` names shell
  creation, lane adoption, mechanical entry/segment issuance, and rollback
  timing before any leaf effect.
Non-claims:
  No `MirFunction` generation, operation MIR, ValueId/BasicBlockId, block
  allocation, CFG/SSA/PHI, Completion consumption/DraftSeal, lifecycle, Text,
  route/backend, production caller, fallback/retry, or module publication.
```

#### Admission I0 closeout and bounded retirement order (2026-08-17)

`PreparedGenericG0PhysicalEmitterAdmissionV1` is landed in the operation
cohort's private child module. One source parent is consumed into the neutral
layout/program, declaration-only shell plan, resolver control, Completion,
target, and full stamp; mapping remains HRTB-scoped. The module is 347 lines,
its separate test file is 107 lines, the five focused tests and 68-test Generic
suite are green, and the structural test rejects physical state, S6C/V1
adapters, old-canary imports, and tuple escape surfaces.

The old entry probe was atomically retired by
`GENERIC-G0-ENTRY-CANARY-RETIREMENT-R0`. Its detached shell, weak stamp, two
tuple exits, old admission/session files, and duplicated reserved-parameter
validator are gone. The source-backed control and shell validators now live in
neutral facts modules and are used only by the combined admission. The
operation-emitter production cutover still owns the separate retirement of
`issue_generic_g0_loop_ingress_v1`; no production caller is claimed here.
The legacy disposition TSV remains a corpus inventory rather than a
replacement-decision authority. Repository-size and byte-identical-helper
counts are informational census; they do not preempt this owner chain.
Dynamic/Text getter or Seal findings stay in their owning parked lanes.

The split-API census has three distinct outcomes. The lexical `recipe.clone()`
used by the forest-binding verifier never escapes its producer and is not an
authority split. The production-visible but caller-zero
`VerifiedGenericRecipeProductG0::into_physical_boundary` is already isolated
behind the sealed-consume test boundary. The detached skeleton/canary
`into_parts` methods and old session path were removed by the retirement R0;
there is no remaining detached tuple consumer to re-pair. This classification,
rather than an aggregate line count, owns the cleanup order.

#### Generic G0 detached-entry retirement R0 closeout (2026-08-17)

The retirement slice preserved the combined admission/session behavior while
moving the two reusable validators into
`generic_g0_physical_entry_facts.rs` and
`generic_g0_physical_shell_facts.rs`. Source, Builder, and session effects did
not gain a new authority: the retained session preflight remains the sole
unpublished shell/entry/segment consumer and the outer discard remains the
rollback owner. The focused Generic suite is green (68/68), `cargo check`,
formatting, diff, and the old-symbol census are green, and every retired
detached symbol has zero source callers.

The next decision is the human-gated
`LOOP-PRODUCTION-SELECTION-D0`. Operation emission is still caller-zero and
does not authorize a production switch, fallback/retry, Completion/DraftSeal,
Text, route, or module publication. A future production slice must first name
the exact selection authority and the same-commit old-edge retirement gate.

#### Production candidate census R0 — Generic remains caller-zero (2026-08-17)

```text
Decision:
  Keep `NoSafeSlice::ProductionSelectionAuthorityUnsealed`.  The Generic G0
  source parent, admission, session-preflight, and common dispatcher are
  caller-zero probes; there is no safe Generic production selection arm yet.
  Do not turn the absence into a runtime fallback or a guessed NoCandidate
  branch.
Source authority + canonical issuer:
  The live production adapter is
  `NormalCallableSemanticPackagePortAdapterV1::lower_cataloged_static_box_method`.
  Its selected-Dynamic arm alone owns the package loan -> A-prime demand ->
  physical session -> DraftSeal -> collector commit chain.  A Generic arm
  would have to be issued from that same selected package/key/signature HRTB;
  the current Generic fixture parent is test-only.
Non-authority:
  `generic_source_unit_and_selection_for_test`, Generic mapping/preflight,
  `ReadyLoopEntryV1`, operation counts, `EffectMask::PURE`, `new_generic`, raw
  JoinIR lowering, or a copied source/AST input cannot select production.
Fail-fast boundary:
  Reject before effect while a production Generic arm cannot prove the same
  package loan/key/signature, source-backed Generic coverage, canonical
  Completion/DraftSeal handoff, and the same collector commit.  Generic ->
  Dynamic/raw retry, source reclassification, and fallback are forbidden.
Smallest next slice:
  `LOOP-PRODUCTION-CANDIDATE-CENSUS-R0`: read-only enumerate the production
  semantic arms and exact collector handoff.  If no Generic arm exists, keep
  this `NoSafeSlice` and record the bounded `NoCandidate` result in the census;
  do not add selector code.
Non-claims:
  No Generic production caller, selector branch, Completion/DraftSeal retrofit,
  raw-loop retirement, fallback/retry, or main integration.
```

#### Production selection D0 closeout — retain the selected-Dynamic arm (2026-08-18)

The human selection review is now recorded as a **no-switch decision**.  The
current selected-Dynamic branch is already the only source-backed production
arm; this D0 does not pretend that the caller-zero Generic probe is a second
candidate.

```text
Decision:
  Retain `NormalCallableSemanticPackagePortAdapterV1::lower_cataloged_static_box_method`
  as the sole live production selector.  Its selected-Dynamic arm remains the
  canonical route.  Keep `NoSafeSlice::DynamicExitPhysicalSessionConsumerUnsealed`
  for the replacement/cutover until a separately admitted common physical
  consumer exists; do not add a Generic selector or fallback.
Source authority + canonical issuer:
  `InstalledNormalCallableSemanticPackageV1` / `NormalCallableSemanticPackagePortV1`
  lend the selected key, source, parameter contract, physical signature,
  header, dynamic source relations, and Completion-backed facts in one HRTB.
  The existing Dynamic arm alone owns A-prime demand, physical session,
  canonical finish/DraftSeal, and the same collector commit brand.
Non-authority:
  Generic test parent/preflight/dispatcher, `ReadyLoopEntryV1`, counts,
  `EffectMask::PURE`, `new_generic`, raw JoinIR, copied AST/input,
  name/owner equality, fallback, and retry cannot select production.
Fail-fast boundary:
  A future replacement must prove the same package/key/signature HRTB,
  Dynamic source/header/Completion relation, target and collector brand,
  canonical session/DraftSeal, and same-commit old-edge retirement before
  changing the adapter.  Dynamic -> Generic/raw reclassification is rejected.
Smallest next slice:
  Keep the existing H2/M10b design rows as the only future cutover path:
  `H2-SELECTED-DYNAMIC-LOWERING-AUTHORITY-R0` then
  `H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0`.  Until their exact source-backed
  common handoff is closed, perform only read-only census/disposition work.
Non-claims:
  No Generic production caller, new selector branch, new Completion/DraftSeal
  owner, raw-loop retirement, fallback/retry, publication, or main integration.
```

This closes the question “which arm is live?” without claiming that the
common-V2 caller-zero probes are production-ready.  The production selector is
therefore settled for the current tree, while the replacement remains a
separate H2/M10b gate with its own same-commit retirement evidence.

#### `DYNAMIC-EXIT-PHYSICAL-SESSION-P0` — next design stop

```text
Decision:
  Design one bounded in-place replacement of the selected-Dynamic Loop work
  inside the existing callable terminal.  Do not implement or switch the
  selector until the sole consumer and its exit/lifetime owner are closed.
Source authority + canonical issuer:
  The installed package's retained Dynamic semantic program, the exact
  `PreparedDynamicLocalEntryV1`, the located Loop admission, and the existing
  Dynamic physical-demand issuer must be co-sealed from the same package/key
  HRTB.  The current A-prime/Dynamic session remains the behavior oracle only.
Non-authority:
  test-only Dynamic demand callers, Generic G0 parent/preflight, raw
  `lower_loop_or_freeze_v1`, AST/name/arity/ValueId rescans, runtime tags,
  `MirType::Unknown`, and fallback/retry are not a consumer authority.
Fail-fast boundary:
  Reject before Builder effect if package/program/owner/frame/scope/Loop,
  local-materialization, result/ABI, Completion, or target/collector brand
  cannot be tied to one exact selected-Dynamic occurrence. Ordinary/Static
  must never enter this cell, and Dynamic must not route through GenericLoop.
Smallest next slice:
  Read-only census the existing exit-transaction program, local materializer,
  located Loop, multi-site Completion claims, and DraftSeal projection; name
  one unpublished session/rollback owner and the exact old-edge retirement row.
  Only after this D0 is accepted may a bounded I0 issue/consume the admission.
Non-claims:
  No new receipt, selector branch, CFG/PHI/session effect, DraftSeal/Collector
  change, GenericLoop change, publication, fallback/retry, or main integration.
```

The existing H2 task document records the same consumer census and negative
cases.  This card keeps the row visible in the Loop pipeline SSOT; it does not
create a second H2 authority.

Current consumer census (read-only):

```text
package semantic program:
  VerifiedDynamicExitTransactionCoSealV1
  -> SelectedCallableSemanticRefV1::Dynamic
  -> selected package adapter (source-seed/origin handoff only)

local materialization:
  PreparedDynamicLocalEntryV1
  -> CallableDynamicOriginLoweringStateV1::local_entries
  -> no selected Loop physical consumer

located Loop:
  PreparedLocatedRawLoopChildEntryV1
  -> lower_with_existing_route_v1
  -> legacy lower_loop_or_freeze_v1 for the non-child shape

existing physical demand:
  issue_selected_a_prime_i64_physical_demand
  -> DynamicV2PhysicalEmissionSessionV1
  -> live selected-Dynamic production arm only
```

The missing issuer is the one-shot selected-callable bridge that relates the
package program, exact local initializer (`initializer ValueId -> local
ValueId -> BindingRef`), and exact located Loop/method/frame/scope/region.  A
diagnostic `GenericLoopAdmissionObservationV1`, a copied `ASTNode`, or the
existing raw route cannot fill that gap.  The first implementation cell must
consume the bridge in the same bounded caller; an adapter that merely creates
and drops a receipt is not progress.

#### H2-S2-S1-R1 selected-initializer bridge feasibility closeout (design stop, 2026-08-18)

The source authorities are already sufficient; the missing piece is scope
wiring, not another semantic issuer.  `SelectedCallableSemanticRefV1::Dynamic`
owns the package program/source, `VerifiedDynamicLoopSourceV1::membership()`
owns the resolver Loop/frame/scope-region relation, and
`CallableDynamicOriginLoweringStateV1::local_entries` owns the exact
initializer-to-local `PreparedDynamicLocalEntryV1`.  The current raw child
context drops the program and does not carry frame/scope/region, while the
located-loop path drops its method/admission observation before entering the
legacy route.  Therefore a receipt-only bridge would be a false completion.

```text
Decision:
  Keep `NoSafeSlice::DynamicExitPhysicalSessionConsumerUnsealed`.  The bridge
  may open only as one issue+consume cell that reaches the selected-Dynamic
  physical consumer in the same bounded caller.
Source authority + canonical issuer:
  The installed package HRTB lends Dynamic program/source; resolver membership
  and the existing local-origin ledger are borrowed in that same scope.  A
  private selected-Dynamic consumer co-seals owner, site, frame, scope/region,
  method provenance, carrier BindingRef, and initializer/local ValueIds.
Non-authority:
  `RawInvocationSourceContextV1`, Generic-loop diagnostics, local-init
  observations, copied AST/name/arity/ValueId, and the legacy raw loop route
  cannot supply the missing relation or select a physical consumer.
Fail-fast boundary:
  Reject Static/Ordinary variants, foreign owner/site/frame/scope/region,
  missing or duplicate local entries, method/child-site drift, program-loan
  escape, and any request for fallback/retry before Builder effect.
Smallest next slice:
  After this P0 is accepted, implement
  `H2-S2-S1-R1-SELECTED-INITIALIZER-ADMISSION-CONSUME-I0` in a private module:
  borrow the package Dynamic program, exact resolver membership, one local
  entry, and located child; consume them once into the selected Dynamic
  physical consumer and add positive/negative/zero-caller guards.
Non-claims:
  No standalone bridge receipt, GenericLoop change, new Dynamic semantic
  issuer, result-ABI/Completion redesign, CFG/PHI, Text, fallback/retry, or
  selector/cutover change is opened here.
```

#### `PHYSICAL-INPUT-AUTHORITY-I0` — Dynamic result/input conformance design stop (2026-08-18)

The canonical callable result contract is already closed and must not be
reopened here.  The remaining question is narrower: the existing selected
Dynamic physical demand must prove that its source-side Dynamic program,
package-owned header/ABI, exact physical input, and two-site Completion all
describe the same exact `i64` callable before any session effect is admitted.

```text
Decision:
  Keep `NoSafeSlice::DynamicPhysicalInputAuthorityUnsealed`.  Open no new
  result receipt and do not infer physical I64 compatibility from the source
  annotation alone.  The next bounded design row is a private extension of
  the existing A-prime issuer, not a second Dynamic classifier.
Source authority + canonical issuer:
  `VerifiedDynamicExitTransactionCoSealV1` owns the retained Dynamic
  semantic/physical-input view; its A-prime source relation owns the exact
  pos/end and operation classes; the package/catalog physical header owns
  parameter/result ABI and storage facts; `VerifiedFunctionCompletionV1`
  owns the exact two return sites and target.  The existing
  `issue_selected_a_prime_i64_physical_demand` is the sole co-seal issuer.
Non-authority:
  `DynamicCallableFunctionExitTargetV1` alone, catalog header alone, a bare
  `i64` string, Recipe/value IDs/MirType, static Callable Tail, owner equality,
  session/DraftSeal state, or any AST/fixture rescan cannot establish the
  physical contract.
Fail-fast boundary:
  Before Builder/session/CFG effect, reject owner/target/region/Loop drift,
  missing or non-exact `i64` parameter/result ABI, Dynamic tail-binding drift,
  missing/non-value return, Completion cardinality other than two, or any
  source/physical coverage mismatch.  No fallback or retry is allowed.
Smallest next slice:
  Design and then implement one private A-prime authority product that
  consumes the selected package loan once and co-seals physical input,
  package header/ABI, Dynamic result/Tail relation, and Completion.  Add
  positive/negative conformance tests before opening a session consumer.
Non-claims:
  No session generation, CFG/SSA/PHI, Completion consumption, DraftSeal,
  lifecycle, PinnedTextOp, GEP/load, route/performance, production selector,
  cutover, fallback, or retry is opened by this row.
```

This row supersedes the broader physical-input wording in the downstream H2
ladder only for the current design stop: the earlier
`DYNAMIC-CALLABLE-RESULT-CONTRACT-I0` is landed, while the selected
initializer bridge and physical session remain later consumers.

#### `PHYSICAL-INPUT-AUTHORITY-D0` — accepted BoxShape (2026-08-18)

The design stop is closed without adding another semantic product.  The
existing A-prime demand is the pre-session admission; its issuer now needs one
private validator that borrows the package-owned `CallablePhysicalHeaderRefV1`
and checks the already-issued Dynamic/source relations against it.

```text
Decision:
  Accept the existing `VerifiedAPrimeI64PhysicalDemandV1` as the sole
  pre-session admission.  Add one private co-seal validation step to its
  existing issuer; do not add a second Dynamic result/ABI/Completion receipt.
Source authority + canonical issuer:
  `VerifiedDynamicExitTransactionCoSealV1` and its physical-input/source
  views, package `CallablePhysicalHeaderRefV1`, the catalog ABI projection,
  and the canonical `VerifiedFunctionCompletionV1` retained by that header
  are co-sealed by `issue_selected_a_prime_i64_physical_demand`.
Non-authority:
  `APrimePhysicalFunctionHeaderV1` alone, a catalog `"i64"` string alone,
  `DynamicCallableFunctionExitTargetV1` alone, Recipe/ValueId/MirType,
  owner equality, or post-session inspection do not issue this contract.
Fail-fast boundary:
  Before Builder/session effect, reject foreign owner/target/region, missing
  package header, non-I64 result/parameters, non-value or non-two-site
  Completion, Dynamic inner-return/outer-tail drift, and physical-input
  coverage mismatch.  No fallback or retry.
Smallest next slice:
  Transport the existing package header reference through the selected
  cataloged loan, add the one issuer validator, and add focused positive,
  negative, and structural no-session tests. Keep all session/CFG/PHI work
  in the later Dynamic exit-session row.
Non-claims:
  No new semantic receipt, session generation, Completion consumption,
  DraftSeal, lifecycle, Text, GEP/load, route, performance, selector,
  production cutover, fallback, or retry.
```

Implementation closeout (2026-08-18):
  `PHYSICAL-INPUT-AUTHORITY-I0` extends the sole A-prime issuer with one
  private pre-session validator.  It requires the package-owned physical
  header and checks owner, exact I64 result, Completion owner/target/value,
  exact two-site coverage, empty cleanup, and catalog return-type parity
  before returning the existing demand.  No new receipt, Builder/session
  effect, Completion consumption, DraftSeal, selector, fallback, or retry was
  added.  The dedicated physical-input authority guard, formatting,
  `cargo check`, and the quick A-prime focused suite (11/11) are green.  The
  next stop is the fixed `LOOP-UNIFICATION-AFTER-DYNAMIC-D0` BoxShape series;
  it may not absorb If/Exit BoxCount or open the Dynamic physical session.

#### Post-Dynamic unification audit receipt (2026-08-18)

The read-only worker audit reconciled the current source with the existing
BoxShape series.  The behavior-preserving surfaces are already present in
history, but this receipt does not close the design stop by inference:

```text
542b3a794d  JoinSig transfer view -> private physical transfer binding
28c4bdd5c4  complete ordered operation/source-effect ledger consumers
46fbf8d0d7  common After stops at ReadyLoopAfterContinuationV1
1544d128d2  topology caller census guard
1e93ad6be9  topology census documentation closeout
```

The current authority split is confirmed: source/Core co-seals Recipe and
JoinSig; JoinSig issues logical transfers; Recipe supplies structure only;
Layout binds placement; the canonical CFG/session remains the sole physical
owner.  The worker found no need for a new semantic receipt, V2-to-V1 adapter,
profile repair, or public traversal plan.  Remaining acceptance evidence is
explicitly bounded: direct missing/duplicate/foreign/wrong-target tests for
the transfer binder/layout, a focused allocator negative, and the old
`operation_target::issue`/fixed-topology transitive caller census.

The dedicated `loop_physical_transfer_authority_guard.sh` now passes.  Its
S6C ingress check no longer bans the legitimate borrowed `logical_items`,
`logical_loops`, `logical_blocks`, and `logical_transfer` views; it retains the
actual detached-authority checks (`anchor_count` and semantic-context
reissuance).  This is guard correctness only, not a new authority or a
production switch.  The accepted fast slice below closes the direct evidence
gap; If/Exit coverage, physical session, initializer consumption, production
selection, fallback, and retry remain closed.

### Accepted fast slice: transfer-authority negative evidence (2026-08-18)

```text
Decision:
  close only the missing direct rejection evidence for the existing private
  JoinSig transfer view and physical binder; behavior and accepted shapes stay
  unchanged.
Source authority + canonical issuer:
  VerifiedLoopJoinSigV1 -> logical_transfer_view(); the existing
  physical_transfer::{bind_predicate, bind_backedge, bind_nested_loop} is the
  sole binder from logical evidence to private physical transfer.
Non-authority:
  Recipe condition data, Layout target inference, operation_target legacy
  lookup, V2-to-V1 adaptation, raw IDs, names, counts, and fallback/retry.
Fail-fast boundary:
  reject missing/duplicate view rows and wrong role/port/loop/condition before
  any layout/Builder/CFG effect.
Smallest next slice:
  add direct unit negatives in transfer_view_v1.rs and physical_transfer.rs;
  retain the existing positive layout parity tests and caller census as-is.
Non-claims:
  no new semantic receipt, old V1 caller migration, If/Exit/Always coverage,
  session, initializer, production selector, fallback, or retry.
```

Implementation closeout (2026-08-18):
  `transfer_view_v1.rs` now has direct missing/foreign and duplicate-row
  rejection tests.  `physical_layout.rs` covers binder role, port, loop, and
  condition drift, plus wrong-role backedge and wrong-loop nested-entry
  rejection.  The existing Callable/Generic layout parity tests remain green:
  view negatives are 2/2 and physical-layout tests are 4/4.  No semantic
  receipt, physical owner, old V1 caller migration, session effect, or
  production edge was added; `physical_layout.rs` remains 699 lines.

Allocator implementation closeout (2026-08-18):
  `segment_allocator.rs` now rejects a foreign `ReadyLoopEntryV1` owner before
  calling any Builder allocation service.  The focused allocator test is 1/1
  green; the existing view negatives remain 2/2 and physical-layout tests 4/4.
  `physical_layout.rs` remains 699 lines and the allocator remains 105 lines.
  No receipt, physical owner, old V1 caller migration, session effect, or
  production edge was added.

### Accepted design stop: topology retirement census (2026-08-18)

```text
Decision:
  stop after the transfer-authority evidence closeout and perform only a
  read-only census of the old fixed-role topology versus the segment receipt;
  deletion is not authorized by this row.
Source authority + canonical issuer:
  topology.rs issues LoopPhysicalBlockReceiptV1 for the legacy role-indexed
  path; segment_allocator.rs plus segment_topology.rs issue the ordered
  LoopPhysicalSegmentBlockReceiptV1 consumed by the segment dispatcher.
Non-authority:
  operation names, Recipe order, current Builder cursor, test-only fixtures,
  the transfer guard, and a local zero-caller result cannot select retirement.
Fail-fast boundary:
  classify every issuer and caller as production, test, guard, or docs; keep
  both paths live until the segment route is proven sole production and all
  remaining old callers are migrated or explicitly allowlisted.
Smallest next slice:
  publish the exact caller census for LoopPhysicalBlockReceiptV1,
  operation_target::{issue,issue_for_segment}, and segment receipts, with the
  reversible caller-zero deletion gate and no code edits.
Non-claims:
  no old-type deletion, operation-target migration, new semantic/physical
  receipt, If/Exit/Always coverage, session, initializer, selector, fallback,
  retry, or production cutover.
```

Worker read-only audit receipt (2026-08-18):
  the legacy `physicalize_topology_*` path is currently allowed only from
  `loop_recipe_physicalizer/tests.rs`; the old
  `VerifiedLoopOperationTargetBlockV1::issue` still has compiled callers in
  `operation_dispatcher.rs` and `operation_emitter.rs`.  The segment
  `issue_for_segment` path is owned by `segment_dispatcher.rs`; allocator
  evidence is limited to the two canaries plus the new foreign-owner test.
  The existing guard checks the topology/segment route but does not yet prove
  old `issue` caller-zero or a live production segment selector.  Therefore
  the old type and role enum remain in place until those exact caller classes
  are classified and the sole-production-route gate is observable.

Exact caller census (2026-08-18):

| surface | observed issuer/callers | classification |
| --- | --- | --- |
| `physicalize_topology_*` / `LoopPhysicalBlockReceiptV1::issue` | issuer definitions in `topology.rs`; seven `physicalize_topology_v1` calls plus old-receipt fixtures in `tests.rs`, `operation_family_tests.rs`, and `read_emitter_tests.rs`; no caller of the operation-demand variant | compiled legacy owner with test-only callers; no selected production caller |
| `operation_target::issue` | four calls from `operation_dispatcher.rs` and three from `operation_emitter.rs`; their old prepare/wrapper entry points are reached only by the old operation-family/read tests | compiled compatibility path; transitive caller-zero outside tests, but no direct guard proves this yet |
| `LoopPhysicalSegmentBlockReceiptV1` / `allocate_for_layout` | allocator definition plus Generic preflight definition, Callable/Generic canaries, and allocator negative; segment receipt validation fixtures in `segment_topology.rs`, `segment_dispatcher.rs`, and `recursive_after.rs` tests | segment owner is compiled; current external reachability is caller-zero/test-only, with no selected production selector |
| `operation_target::issue_for_segment` / segment dispatch | owned by `segment_dispatcher.rs`; two direct negative fixtures there; Generic session calls preflight/emit only from its test callbacks | canonical segment mechanical route, not yet a production caller |

Retirement gate fixed by this census:
  do not delete the old type, role enum, `physicalize_topology_*`, or
  `operation_target::issue` until (a) a named selected production caller uses
  the segment route, (b) a whole-tree guard proves zero non-test callers of
  the old topology/issuer, (c) residual tests are migrated or explicitly
  allowlisted, and (d) no compatibility wrapper pairs old and new receipts.
  The current evidence satisfies none of (a)–(d) completely, so the census
  is accepted as a design receipt only and remains a NoSafeSlice for deletion.

Next bounded slice:
  select the next BoxCount design consultation for
  `LOOP-PHYSICAL-IF-COVERAGE-I0`; keep topology retirement parked behind the
  production-route gate above.

### Accepted design stop: If branch/merge coverage consultation (2026-08-18)

```text
Decision:
  open only a design consultation for the one new If branch/merge transfer
  capability; do not add code, a receipt, or a physical CFG path yet.
Source authority + canonical issuer:
  `IfRecipeVerifierV1` verifies the source-bound artifact, and
  `IfJoinSigElaboratorV1::elaborate` issues the existing logical ports,
  True/False, Then/Else transfer, and ImplicitBaseline edges.  The existing
  `VerifiedIfPhysicalInputV1::from_artifact` co-seals that pair.
Non-authority:
  `IfRecipeV1` condition/assignment fields alone, the old
  `trivial_ssa::if_recipe_physicalizer` receipt, `lower_if_recipe_selected`,
  Builder cursor/physical IDs, names, and source rescans cannot issue a new
  common transfer capability.
Fail-fast boundary:
  before Layout or CFG/SSA/PHI effect, reject source-site/binding drift,
  explicit-vs-implicit disposition drift, wrong JoinSig port/role/value class,
  missing merge/continuation transfer, and duplicate branch/merge obligations.
Smallest next slice:
  perform a read-only worker/source audit of how the existing If JoinSig can
  lend one complete branch/merge view to a common physicalizer without pairing
  old physical receipts; only then accept the smallest BoxCount implementation.
Non-claims:
  no new `Verified*`/`Prepared*` receipt, old If physicalizer migration,
  CFG/SSA/PHI, Completion/DraftSeal, session, lifecycle, route, selector,
  fallback, retry, or production cutover.
```

### Scope correction: LoopRecipe If coverage (2026-08-18)

```text
Decision:
  row 24 is the LoopRecipe `If` item in the common Loop physicalizer, not the
  resolved-trivial fixed-shell `IfRecipe` physicalizer.  Keep the row at
  NoSafeSlice until the physical merge authority and one consumer are named.
Source authority + canonical issuer:
  `LoopRecipeVerifierV1/V2` verifies the source-bound Recipe; the existing
  `LoopJoinSigElaboratorV1::elaborate`/`branch_row` issues the logical
  `LoopJoinBranchV1/V2`; `LoopJoinLogicalTransferViewV2` lends those existing
  branch arms without physical IDs.  No new issuer is authorized.
Non-authority:
  `LoopRecipeItemV1::If` placement alone, old V1 `physical_layout`, the
  resolved-trivial `IfRecipe` physicalizer, Recipe rescans, BasicBlockId/
  ValueId, and a guessed merge/PHI plan cannot issue branch meaning.
Fail-fast boundary:
  before Layout/CFG/SSA/PHI, reject foreign or duplicate branch rows, owner or
  item drift, condition/value drift, arm-target/disposition drift, missing or
  ambiguous merge identity, predecessor/value mismatch, and merge aliasing.
Smallest next slice:
  design-only census of the existing common Loop physical CFG/SSA/PHI owner
  and the one consumer that can accept one complete V2 branch-arm view plus an
  explicit merge relation; do not adapt V2 to old V1 layout or infer merge.
Non-claims:
  no code, new `Verified*`/`Prepared*` receipt, V2-to-V1 adapter, old If
  physicalizer migration, CFG/SSA/PHI, session, lifecycle, route, selector,
  fallback, retry, production cutover, or topology retirement.
```

Worker read-only premise audit (2026-08-18):
  `LoopRecipeVerifierV1/V2` is the source-bound structural verifier and
  `LoopJoinSigElaboratorV1`/`branch_row` is the canonical logical branch
  issuer.  `LoopJoinLogicalTransferViewV2` already lends `owner_loop`,
  `if_item`, `condition`, and each arm as `Exit` or `Fallthrough`, but it has
  no merge block or PHI relation.  `PreparedLoopV2PreSessionEnvelopeV1` already
  transports the If placement and view; `CommonV2CanonicalSessionRefV1` owns
  the existing `CanonicalSsaFunctionSessionV2` and is the only plausible
  mutable consumer, but exposes no branch/merge API.  The older V1
  `physical_layout` consumes only predicate/backedge/nested transfers and
  returns `UnsupportedIf(item)`.

Source/Core merge-relation census (2026-08-18):
  both `LoopRecipeItemV1::If` and `LoopRecipeItemV2::If` carry only the
  condition and the `then_block`/`else_block` child keys.  The S6C logical If
  row and the resolver-owned control placement carry the same three-way
  relation; neither names a merge block, predecessor set, or value/PHI
  relation.  Therefore no existing source/Core issuer can currently bind the
  required physical merge identity, and the common session must not infer one
  from layout order or a next segment.

Why not Fast path:
  the source/JoinSig branch mapping is closed, but the common physical merge
  authority and named consumer are not.  Adding a view, adapter, or receipt
  now would invent a second authority or pair a V2 view with the old V1
  layout, so this row remains `NoSafeSlice`.

### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RELATION-D0 (2026-08-18)

```text
Decision:
  accept one caller-zero continuation relation for the already supported
  `Exit + Fallthrough` branch shape.  The only normal arm must resume at an
  explicit next item in the same parent block; `BlockEnd`, two normal arms,
  and PHI material remain `NoSafeSlice` in this slice.
Source authority + canonical issuer:
  `LoopJoinSigElaboratorV1::branch_row` is the logical continuation issuer
  because it owns the verified Flow context.  It issues the relation from the
  same verified Recipe view and the source-bound Core co-seals the existing
  JoinSig pair; no second source or physical issuer is added.
Non-authority:
  `ResolvedIfJoinContractV1`, `VerifiedTrivialIfMergeProfileV1`, legacy
  `IfPhiJoin`/`ControlForm`, layout order, next-segment guesses, raw ValueId,
  and the old V1 physicalizer cannot supply that relation.
Fail-fast boundary:
  before Layout/CFG/SSA/PHI, reject missing/foreign/duplicate/ambiguous
  continuation, target-block/item drift, non-strict next-item order,
  condition/arm-target/payload-class drift, and any two-normal-arm merge.
Smallest next slice:
  implement one physical-ID-free `NextItem { block, item }` relation in the
  existing JoinSig arm and lend it through the existing V2 control view.  The
  source consumer is the existing `issue_control_source`/prepared control
  program; no canonical session API or physical block is opened yet.
Non-claims:
  no BlockEnd relation, two-normal-arm PHI, new `Verified*`/`Prepared*`
  receipt, V2-to-V1 adapter, Layout inference, CFG/SSA/PHI, session mutation,
  production, fallback, or retry.
```

Worker authority audit (2026-08-18):
  `ResolvedIfJoinContractV1` records binding and PostCondition/BranchExit
  meaning only; `VerifiedTrivialIfMergeProfileV1` records value-shape
  homogeneity only.  Neither is a PHI-placement authority.  The existing
  `CommonV2CanonicalSessionRefV1` exposes condition-target and After allocation
  only; it has no branch/merge/terminator/PHI API or corresponding receipt.
  Acceptance for D0 is positive explicit-If mapping plus negative evidence for
  missing, foreign, duplicate, ambiguous, drifted, or aliased merge rows, with
  no V2-to-V1 adapter or Layout inference.

Continuation authority census (2026-08-18):
  `branch_row` receives the parent `Flow` context and the two branch flows,
  then emits only `owner_loop/if_item/condition`, `Exit` or `Fallthrough`, and
  visible payload.  `Flow.bindings/available` are merged for later logical
  processing, but no nested-If continuation block, merge identity, or exact
  two-predecessor/value relation is retained.  The JoinSig ports (`Body`,
  `After`, `FunctionExit`, etc.) cannot be reused as an If merge; `After` is a
  loop boundary.  Any extension must stay inside the verified JoinSig Flow
  and be co-sealed by the source-bound Core; synthetic merge/next-segment or
  Recipe-order inference remains forbidden.

#### LOOP-PHYSICAL-IF-CONTINUATION-RELATION-I0 — execution brief

```text
Change:
  add only the JoinSig-owned physical-ID-free `NextItem { block, item }`
  continuation on a Fallthrough arm and transport it through the existing V2
  control view.
Contract:
  the target item is in the same parent block and strictly follows `if_item`;
  the accepted shape is one Exit arm plus one normal Fallthrough arm.  Core
  pairing and the pre-Layout V2 source consumer validate the relation once.
Done:
  positive S6C/implicit-else mapping, deterministic target identity, foreign /
  duplicate / missing / non-strict target negatives, arm drift negatives, and
  no physical IDs or session/CFG/PHI effects; touched Rust files stay below
  the 760-line design trigger and 800-line hard boundary.
Stop:
  BlockEnd, two normal arms, PHI, branch terminator, Layout splitting, or a
  canonical-session consumer is a later design/physicalization row.
```

Implementation receipt (2026-08-18):
`LoopJoinNextItemV1` is now issued from the verified parent block's explicit
next item and carried by each logical `Fallthrough` arm through the existing
V2 transfer view. `issue_control_source` delegates source-block validation to
one bounded caller-zero owner, which rejects foreign, duplicate, missing, and
non-strict targets before Layout. The implicit-else/S6C target positives and
the negative matrix are green under the quick profile. The broader
`loop_recipe_contract` suite has 151 passing tests and one unchanged
`source_bound_core` baseline failure with an unchanged test, reproduced before
the I0 parity guard; it is recorded as baseline debt, not a current-change
failure. The source-bound Core parity
guard also rejects Recipe/JoinSig continuation drift before co-seal. No
physical IDs, session API,
CFG/SSA/PHI mutation, production switch, fallback, or retry was opened.

The next row is a design stop for naming the sole physical/session consumer;
I0 does not authorize Layout inference or branch terminator emission.

### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-CONSUMER-D0 (2026-08-18)

```text
Decision:
  accept one placement-only physical consumer for the already sealed
  `Exit + Fallthrough(NextItem)` relation.  It may validate the exact source
  segment, allocate one unpublished continuation target block through the
  canonical session, and lend a callback-scoped mechanical view.  It may not
  emit an edge, terminator, operation, Return, BlockEnd, or PHI.
Source authority + canonical issuer:
  `LoopJoinLogicalTransferViewV2` remains the sole logical continuation
  authority.  `PreparedLoopV2PreSessionEnvelopeV1` transports that view and
  the source-backed layout; `PreparedSegmentBlockReceiptV1` is the existing
  physical segment placement.  `CommonV2CanonicalSessionRefV1` is the only
  common consumer façade and `CanonicalSsaFunctionSessionV2::create_unpublished_block`
  is the only physical block issuer.
Non-authority:
  layout order, item+1 arithmetic, old V1 physicalizer, `After`/port rows,
  `ResolvedIfJoinContractV1`, raw `ValueId`, a second session, or a caller-
  supplied branch/target cannot establish continuation placement.
Fail-fast boundary:
  before the first physical allocation, reject missing/duplicate branch or
  If placement, wrong owner/stamp, arm/condition drift, missing or foreign
  segment row, absent/same/preceding target item, target control item, and
  aliased/colliding placement.  Any callback failure is a late terminal owned
  by `with_common_v2_physical_entry_session`, which discards the whole
  unpublished function exactly once; no local retry or fallback exists.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-TARGET-PLACEMENT-I0`: add the one-shot,
  callback-scoped `IfContinuationPhysicalTargetRefV1` and its source/segment
  parity checks.  Allocate no edge or instruction.
Non-claims:
  no branch terminator, Return terminal, operation item-to-split mapping,
  Layout rewrite, BlockEnd, two-normal-arm merge, CFG/SSA/PHI mutation,
  Dynamic exit-session, initializer bridge, production switch, fallback,
  retry, or old-topology retirement.
```

Worker authority audit (2026-08-18):
  the existing canonical session is the sole mutable physical owner and
  `with_common_v2_physical_entry_session` is the sole rollback owner.  The
  smallest safe slice is target placement only: validate `NextItem` against
  the same source segment, allocate one unpublished block through
  `create_unpublished_block`, and return a non-Clone callback-scoped view.
  Branch/Return/operation emission is unsafe until a later item-to-split and
  terminal design names those authorities.  No new semantic `Verified*` or
  `Prepared*` receipt is needed.

#### LOOP-PHYSICAL-IF-CONTINUATION-TARGET-PLACEMENT-I0 — execution brief

```text
Change:
  add one one-shot `CommonV2CanonicalSessionRefV1` consumer that borrows the
  exact envelope transfer and an existing `PreparedSegmentBlockReceiptV1`,
  validates the S6C one-branch shape and same-block strict `NextItem`, then
  allocates one unpublished continuation target block and lends
  `IfContinuationPhysicalTargetRefV1` only during the callback.
Contract:
  the view carries owner, If item, explicit NextItem, source block/split
  provenance, physical target block, and the existing entry stamp.  It has no
  ValueId, edge, terminator, operation, or publication API.
Done:
  positive S6C target placement, one-shot/duplicate rejection, late callback
  discard evidence, source/segment/stamp parity checks, focused gate, and
  line/format/pointer guards; touched source files stay below the 760-line
  design trigger and 800-line hard boundary.
Stop:
  branch/Return emission, operation relocation, Layout splitting beyond this
  target reservation, BlockEnd, PHI, production, fallback, and retry remain
  later design stops.
```

#### LOOP-PHYSICAL-IF-CONTINUATION-TARGET-PLACEMENT-I0 — implementation receipt (2026-08-18)

The common V2 canonical session now consumes the existing
`Exit + Fallthrough(NextItem)` relation exactly once through
`with_if_continuation_target`. It validates the S6C one-branch shape,
same-block strict target ordering, owner/loop/split/segment/stamp parity, and
target-item non-control status, then reserves one unpublished target block
through `CanonicalSsaFunctionSessionV2::create_unpublished_block`. The
callback-scoped `IfContinuationPhysicalTargetRefV1` is non-Clone and carries
only mechanical placement evidence; it cannot emit an edge or instruction.

Focused evidence is green:

```text
continuation_target_placement_is_callback_scoped_and_one_shot  1 passed
continuation_target_late_failure_discards_unpublished_block    1 passed
rejects_foreign_non_strict_and_duplicate_targets               1 passed
cargo check; cargo fmt --all -- --check; git diff --check; pointer guard     green
```

The touched Rust files remain below the 760-line design trigger and 800-line
hard boundary. No branch/Return/operation/BlockEnd/CFG/SSA/PHI, publication,
production selector, fallback, or retry was opened. The broader loop suite's
pre-existing `source_bound_core` failure remains baseline debt and is not a
current-change failure.

#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-BRANCH-EMISSION-D0 (2026-08-18)

```text
Decision:
  reject direct branch/Return emission as NoSafeSlice.  The placement view is
  mechanical evidence only; no source-backed item-to-physical-split relation
  or FunctionExit terminal owner exists for the common V2 consumer.
Source authority + canonical issuer:
  `LoopJoinLogicalTransferViewV2` remains the logical transfer authority;
  `PreparedLoopV2PreSessionEnvelopeV1` and the segment receipt transport
  source evidence; `CanonicalSsaFunctionSessionV2` remains the only mutable
  physical owner and `CanonicalCfgSessionV1` the only eventual CFG writer.
Non-authority:
  `PreparedLoopControlPlacementV2::If`'s logical blocks, layout segment order,
  item ordinal arithmetic, `FunctionExit` enum, placement view, Builder
  cursor, owner/name equality, and local green tests cannot invent an edge,
  Return, BlockEnd, or PHI input.
Fail-fast boundary:
  reject absent/foreign/ambiguous item-to-split, physical branch targets,
  Return value/terminal, or one-sided continuation relations before any
  terminator or CFG/SSA/PHI mutation; keep the outer unpublished function
  transaction as the sole rollback owner.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-SPLIT-TERMINAL-AUTHORITY-D0` — a design-only
  audit to name one source issuer for the continuation item-to-split relation
  and the FunctionExit/one-sided terminal relation.  No code or new semantic
  receipt is authorized until accepted.
Non-claims:
  no Layout rewrite, two-normal-arm merge, Dynamic exit session, initializer
  bridge, production cutover, fallback, retry, or legacy retirement.
```

Worker/code authority audit (2026-08-18):
  `PreparedLoopControlPlacementV2::If` carries only item, logical block,
  condition, and logical then/else blocks; `PreparedLoopV2LayoutSegmentRefV1`
  carries a block, derived split ordinal, and item slice, but does not issue an
  item-specific physical target. `CanonicalCfgSessionV1::emit_branch` requires
  two distinct physical `BasicBlockId`s and a physical `ValueId`, while
  `emit_return` terminalizes an already selected source block. Neither API
  supplies the missing FunctionExit block, Return value receipt, or operation
  handoff for the post-If item. Therefore a common consumer that calls either
  API now would infer layout/terminal meaning and create a second authority.
  The only safe ordering remains `source/JoinSig validation -> split/terminal
  preflight -> first physical mutation -> outer transaction discard`; local
  rollback, retry, and fallback stay forbidden.

#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-SPLIT-TERMINAL-AUTHORITY-D0 (2026-08-18)

```text
Decision:
  keep physical split/terminal emission at NoSafeSlice.  Existing facts carry
  the required pieces in separate views, but no canonical source issuer
  co-seals the continuation item, physical split target, and FunctionExit
  Return terminal relation.
Source authority + canonical issuer:
  `S6CPrephysicalCompletionRefV2` owns resolver source sites/values,
  `S6CLogicalItemV1::Exit` owns logical item/block/exit/value, and
  `LoopJoinLogicalTransferViewV2` owns arm disposition.  The canonical
  co-sealing issuer for their source relation is still missing; existing
  `S6CPrephysicalIngressSealV2` only proves completion count/cleanup.
Non-authority:
  downstream owner/role equality, `FunctionExit` enum, `LoopExitKeyV1`,
  segment ordinal, placement target, Builder cursor, V1 physicalizer, and a
  test fixture cannot pair the separate facts or issue a terminal.
Fail-fast boundary:
  missing/foreign/duplicate source-site/item/block/value/exit pairing stops
  before any new receipt, edge, Return, BlockEnd, operation, CFG/SSA/PHI, or
  publication effect.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COSEAL-D0` — a design-only
  census to name one source issuer for the exact Return source site/value,
  logical Exit item/block/value, and FunctionExit arm; no code or new receipt
  is authorized until accepted.
Non-claims:
  no branch emission, Layout rewrite, two-normal-arm merge, Dynamic exit
  session, initializer bridge, production cutover, or legacy retirement.
```

Worker/code authority audit (2026-08-18):
  `S6CPrephysicalCompletionRefV2` (`s6c_prephysical_ingress.rs`) exposes
  `loop_return_site` and `loop_return_value`, but no LoopItemKey, block, or
  exit key. `S6CLogicalItemV1::Exit` and the common control rows carry those
  logical keys, while `s6c_scan_with_init_joinir_output_rows.rs` validates the
  fixed role/value shape without retaining a source-site relation. The common
  ingress seal retains only completion target/count/cleanup, and the existing
  V1 `LoopPhysicalTransferV1` supports Jump/Predicate/OpenNestedLoop but not
  If/Return. Reusing any of these as a physical terminal issuer would pair
  separate authorities by convention. The safe ordering remains
  `source co-seal -> split/terminal preflight -> first physical mutation ->
  outer transaction discard`; local rollback, retry, and fallback stay closed.

#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COSEAL-D0 (2026-08-18)

```text
Decision:
  keep NoSafeSlice. Existing issuers cannot safely bind the verified Return
  site/value to the logical Exit item/block/value and its FunctionExit arm.
Source authority + canonical issuer:
  `issue_s6c_exit_tail_source_coseal_v1` owns Return site/value and Completion;
  `issue_s6c_logical_output_rows` owns the Recipe Exit row; JoinSig's transfer
  view owns the FunctionExit arm. No one canonical co-seal issuer exists yet.
Non-authority:
  `S6CPrephysicalCompletionRefV2` source-only fields, Exit item/block/value,
  FunctionExit enum, owner/role/count parity, Layout ordinal, placement block,
  and V1 physicalizer cannot pair the separate authorities by convention.
Fail-fast boundary:
  absent/foreign/duplicate/drifted source-site/item/block/value/arm pairing
  stops before any Prepared receipt or physical/session effect.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-ISSUER-BOUNDARY-D0` — design
  the canonical same-cohort issuer boundary (candidate: prephysical ingress);
  no code, fixture, or new semantic receipt is authorized until accepted.
Non-claims:
  no branch emission, item-to-split allocation, Layout rewrite, production
  cutover, initializer bridge, or legacy retirement.
```

Worker/code authority audit (2026-08-18):
  `issue_s6c_exit_tail_source_coseal_v1` (`src/mir/loop_structural_facts/s6c_exit_tail.rs:125-258`)
  already co-seals resolver Return site/value and Completion. `S6CLogicalItemV1::Exit`
  is issued by `issue_s6c_logical_output_rows` (`src/mir/loop_recipe_contract/s6c_scan_with_init_joinir_output_rows.rs:209-275,549-557`),
  while `LoopJoinBranchExitRefV2`/`FunctionExit` is issued by
  `join_sig/transfer_view_v2.rs:119-215`. `S6CPrephysicalCompletionRefV2`
  exposes only source fields, and `issue_control_source` checks only item
  sets/placement. `issue_s6c_common_v2_pre_session_v1` sees one loan but does
  not issue this relation; treating it as an issuer would pair authorities by
  convention. No code, receipt, or physical effect was opened.

#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-ISSUER-BOUNDARY-D0 (2026-08-18)

```text
Decision:
  keep NoSafeSlice. A same-cohort loan is not an issuer: prephysical ingress
  and common pre-session currently aggregate checks without the missing
  source-backed Return-to-Recipe/Join binding.
Source authority + canonical issuer:
  `issue_s6c_exit_tail_source_coseal_v1` owns source Return evidence; the S6C
  Recipe producer owns keys; JoinSig owns the FunctionExit arm. A canonical
  issuer for their exact binding is absent; ingress is only a candidate seam.
Non-authority:
  completion count/cleanup, role names, fixed key arithmetic, owner equality,
  item/block equality, placement/layout, and common pre-session aggregation
  cannot become the missing issuer implicitly.
Fail-fast boundary:
  no source-site/value ↔ Exit item/block/value ↔ FunctionExit arm relation may
  cross into a Prepared physical demand while source-to-key provenance is
  absent or foreign, missing, duplicate, or drifted.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-D0` — design the
  source-layer binding issuer and its Recipe/Join handoff; no code, fixture, or
  new semantic receipt is authorized until accepted.
Non-claims:
  no branch emission, item-to-split allocation, Layout rewrite, session,
  production cutover, initializer bridge, or legacy retirement.
```

Worker follow-up authority audit (2026-08-18):
  `issue_s6c_common_v2_pre_session_v1` sees the same source/logical/JoinSig
  loan, but `issue_control_source` proves only item-set/placement parity and
  `S6CPrephysicalIngressSealV2` retains completion target/count/cleanup.
  `issue_s6c_exit_tail_source_coseal_v1` proves the source index binding and
  nested region, then discards the source-region/semantic-role mapping. The
  missing relation is exact and source-backed: Return site/value → Recipe
  return-index item/result, source nested Return/If region → Recipe then block,
  Return occurrence → Recipe loop-return item/exit key, and that Exit → JoinSig
  Return/FunctionExit arm. Fixed role/ordinal/owner equality would still be
  convention-based pairing. No code, receipt, or physical effect opened.

#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-D0 (2026-08-18)

```text
Decision:
  accept one caller-zero BoxShape: the source co-seal retains resolver
  Return/If region and index-binding evidence, and the sole S6C Recipe
  producer issues one non-Clone source-to-Recipe/Join relation. This is
  transport only; common physical demand may aggregate it later.
Source authority + canonical issuer:
  `issue_s6c_exit_tail_source_coseal_v1` remains the source authority and is
  extended only to retain its already-proven Return/If regions and index
  binding. `produce_s6c_scan_with_init_recipe_v2` is the sole key issuer and
  canonical co-sealer of the source evidence, exact Recipe keys, and JoinSig
  Return/FunctionExit arm. Common pre-session only consumes this relation.
Non-authority:
  `S6CPrephysicalCompletionRefV2` count/cleanup, role or ordinal arithmetic,
  block order, owner/name equality, common aggregation, and physical layout do
  not issue or re-infer source-to-key provenance.
Fail-fast boundary:
  absent/foreign/duplicate/drifted source site/value, source region/binding,
  Recipe item/block/exit/value, or JoinSig Return/FunctionExit arm rejects
  before the relation is published; no physical/session effect is opened.
Smallest next slice:
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-I0`: retain the
  source evidence, issue/transport the private relation, and add focused
  positive/negative/foreign-drift guards. No physical block/edge/Return/PHI,
  production, fallback, or retry.
Non-claims:
  no common physical emission, item-to-split allocation, Layout rewrite,
  CFG/SSA/PHI, session, production cutover, initializer bridge, or retirement.
```

#### Next execution: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-I0

```text
Change:
  extend the existing source co-seal with retained Return/If region and index
  binding evidence; let the S6C Recipe producer issue one move-only relation
  carrying source site/value, source region/binding, exact Recipe Exit/read
  keys, and the JoinSig Return/FunctionExit arm. Transport it through the
  existing logical/prephysical façade without touching physical consumers.
Contract:
  one source→Facts→Recipe→Join issuer; no fixed ordinal, owner equality,
  layout inference, common aggregation, or second semantic authority.
Done:
  focused positive plus missing/foreign/duplicate/drift rejection, a reusable
  source-authority guard, touched-file line counts below 760/800, and the
  existing pointer/authority/format gates green.
Stop:
  any missing source region/binding or source-to-key proof returns to
  `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-D0`; do not add
  a default relation, physical edge, Return, PHI, session, fallback, or retry.
```

#### Semantic-program consume D0 — accepted BoxShape (2026-08-17)

```text
Decision:
  Keep `NoSafeSlice::ProductionSelectionAuthorityUnsealed` as a separate
  production-selection stop for the absent Generic arm.  Accept
  `MIRBUILDER-SEMANTIC-PROGRAM-CONSUME-D0` as a behavior-preserving
  BoxShape, then open only its bounded I0; this does not activate production
  selection.
Source authority + canonical issuer:
  `VerifiedCallableSemanticProgramV1` remains the sole owner of the complete
  Callable operation/effect, input, context, continuation, prelude, and tail
  parent.  The existing `normal_callable_prepared_operation::prepare_full_demand`
  consumer is the only production handoff; the test consumer is the only test
  observation.  The accepted successor is a source-free, non-Clone prepared
  demand parent issued by consuming the complete semantic parent once.  Its
  one consumer method may lend/move the already-issued input, prepared
  operation, prelude, and tail together; it may not expose a six-element tuple.
Non-authority:
  The six-element tuple from `into_prepared_parts`, individual context/Core/
  continuation arguments, `PreparedLoopOperationRowV2`, raw counts, source
  rescans, owner/name equality, and local test green are not new authority.
Fail-fast boundary:
  The I0 must reject foreign/partial/double consumption, preserve the existing
  source owner/origin/site/frame checks, and prove production/test parity with
  zero callers of the old tuple symbol in the same slice.  Do not mix this with
  Generic session effects or the selected-Dynamic production branch.
Smallest next slice:
  `MIRBUILDER-SEMANTIC-PROGRAM-CONSUME-I0`: add the private prepared-demand
  parent and one-shot consumer, replace the production and test tuple callers,
  add parity/negative/zero-caller guards, and delete `into_prepared_parts`.
  No Builder effect, new semantic issuer, or route selector is included.
Non-claims:
  No Generic production arm, canonical session/CFG/SSA/PHI, Completion/
  DraftSeal, lifecycle/Text, route, fallback/retry, publication, or legacy
  finalizer retrofit.
```

#### Semantic-program consume I0 closeout (2026-08-17)

`VerifiedCallableSemanticProgramV1` now consumes its complete parent into the
source-free, non-`Clone` `PreparedCallableOperationDemandV1`.  The parent owns
the already-issued input, prepared Recipe-order operation program, Prelude, and
Tail, and exposes only one `consume` callback; the Builder-side
`prepare_full_demand` handoff remains the sole production consumer.  The
compiler test observation was migrated to the same parent, and the former
`into_prepared_parts` symbol has zero source callers.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test --lib callable_single_loop_operation_effect -- --nocapture  # 3 passed
RUSTFLAGS='-Awarnings' cargo test --lib normal_callable_prepared_operation -- --nocapture      # 1 passed
RUSTFLAGS='-Awarnings' cargo check -q
cargo fmt --all -- --check
git diff --check
rg -n --glob '*.rs' "into_prepared_parts" src/mir  # zero
```

This is a BoxShape-only sealing refactor.  It opens no Builder/MIR effect,
session, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback,
retry, publication, or Generic/selected-Dynamic production selection.

The full `cargo test --lib` baseline remains red independently of this slice
(6787 passed, 146 failed, 29 ignored); the failures are in existing FileBox,
parser-freeze, legacy JoinIR, and broad MIR/route fixtures, while the changed
compiler/Builder focused tests and `cargo check` are green.  This is recorded
as known baseline debt, not a current-change failure.

#### Structural convergence audit — migration thickness is classified, not an authority (2026-08-17)

The repository-wide size census is useful for planning but is not a semantic
decision authority.  The currently coexisting Generic/S6C/common lanes are
intentional migration scaffolding until a named production consumer and a
zero-caller retirement gate exist.  A large line count, a byte-identical helper
pair, or a test-only canary does not by itself justify deletion or a new
physical owner.

```text
Decision:
  Keep the current source -> cohort -> admission -> session design.  Record
  migration thickness in an owner/retirement manifest and remove old lanes in
  the same bounded slice that switches their replacement.  Do not widen the
  current Generic session-preflight design stop to perform global cleanup.
Source authority + canonical issuer:
  Each existing family parent and the already-landed Generic emitter admission
  remain the only semantic/prephysical issuers.  A cleanup row may narrow an
  API or migrate a caller, but it may not mint a replacement receipt or infer
  meaning from a line census.  The future unpublished session remains the
  sole MirFunction/rollback owner.
Non-authority:
  Repository LOC totals, helper byte identity, micro-seed counts, TSV row
  order, public visibility alone, test-green canaries, `recipe.clone()` in its
  lexical forest verifier scope, raw `ValueId`, and owner/name equality are
  not production authority or retirement proof.
Fail-fast boundary:
  Keep a lane until its final owner, production/test caller census, replacement
  parity gate, and same-slice deletion condition are named.  A sealing escape
  (`into_physical_boundary`, semantic-program tuple decomposition, detached
  canary `into_parts`, or a public lease facade) is closed only after the
  replacement consumes the complete owner and the old symbol has zero callers.
Smallest next slice:
  The former `LOOP-GENERIC-G0-SEALED-CONSUME-I0` and detached-chain parity
  retirement are landed with zero old callers.  The live next stop is the
  human `LOOP-PRODUCTION-SELECTION-D0`; the corpus TSV, common-V2
  function-level retirement census, runtime lease facade, and byte-helper/
  micro-seed inventory remain parked as separate rows.
Non-claims:
  This audit opens no session, Builder effect, operation emission, production
  switch, fallback/retry, global helper rewrite, runtime ABI change, or broad
  legacy deletion.
```

The scale findings are therefore classified as follows.

| class | current examples | policy and exit condition |
|---|---|---|
| intentional keep | raw compatibility ingress, versioned V1/V2 projections, the `llvm_py`/ny-llvmc keep lane, and test-only observation adapters | keep only under the owning SSOT; no new caller or semantic authority; retire only when the documented replacement is integrated |
| waiting scaffolding | `loop_recipe_physicalizer`, common-V2 session/skeleton candidates, and the Text-formal runtime entry substrate | not dead code; the next production owner must be named before effects; no silent reuse of an S6C/V1 receipt |
| bounded retirement | Generic detached skeleton/canary chain, `into_physical_boundary`, semantic-program tuple decomposition, and selected-normal legacy finalizer edges | replacement parity, zero caller census, then delete in the same cutover slice; do not delete by LOC |
| parked inventory | legacy disposition TSV, byte-identical helper groups, micro-seed templates, `json_v0_bridge`, and archived `llvm_py` surface | inventory only until a source-backed owner and deletion gate are written; no bulk decision-column relabeling or mechanical dedupe |

The three current sealing debts are tracked explicitly rather than folded into
the admission design:

1. `LOOP-GENERIC-G0-SEALED-CONSUME-I0` isolates the caller-zero
   `VerifiedGenericRecipeProductG0::into_physical_boundary` split.  The
   lexical forest-verifier `recipe.clone()` remains unchanged because it does
   not escape its producer.  Detached skeleton/canary `into_parts` remains a
   later parity-retirement edge, not a second admission issuer.
2. `MIRBUILDER-SEMANTIC-PROGRAM-CONSUME-I0` replaces the production
   `VerifiedCallableSemanticProgramV1::into_prepared_parts` consumer with a
   direct scoped/one-shot consumer, migrates its test seam, and proves zero
   callers before removal.  It must not be mixed with Generic session effects.
3. `RUNTIME-END-AUTHORIZED-TEXT-FACADE-I0` is a parked runtime-lane task for
   the public `EndAuthorizedTextV1` surface and its getters.  It is not part of
   the Generic session owner and cannot be repaired by hiding a MIR receipt.

`MIRBUILDER-LEGACY-DISPOSITION-R0` is also parked to fill the disposition TSV
only from observed caller/owner evidence.  The `decision` column must not be
mass-filled with `-`, `keep`, or `delete` guesses; each non-sentinel decision
requires a final owner, parity gate, and retirement row.  The existing
`MIRBUILDER-CANARY-CONVERGENCE-MANIFEST-R0` remains the decision authority for
the current Generic seams.  A separate `MIRBUILDER-BYTE-HELPER-INVENTORY-R0`
may record the 1,145 helper pairs and micro-seed families, but byte identity is
not a permission to merge them.

#### Structural-debt disposition follow-up — parked behind production selection (2026-08-17)

The external size audit is accepted as a disposition input, not as a new
physical authority.  The current tree has no production caller for the old
Generic detached skeleton/admission/session chain; that chain is already
retired by the zero-caller R0.  The remaining thickness is therefore tracked
as three bounded, design-only follow-ups rather than another parallel emitter.

```text
Decision:
  Keep the source -> facts/recipe -> family parent -> common admission -> sole
  session shape.  Do not perform repository-wide cleanup while production
  selection is unresolved.  Every later deletion must be co-located with the
  replacement switch and its zero-caller proof.
Source authority + canonical issuer:
  Existing source parents and the owner/retirement manifest remain the only
  authorities.  These follow-ups issue no semantic or physical receipt; they
  only record observed callers, final owners, parity gates, and deletion rows.
Non-authority:
  LOC totals, byte-identical helper bodies, micro-seed counts, public
  visibility, `recipe.clone()` inside its lexical forest verifier, copied
  `ValueId`/site data, and test-only green are not deletion or production
  authority.
Fail-fast boundary:
  A row stays parked until its source-backed owner, replacement parity,
  production/test caller census, and same-slice deletion condition are all
  named.  No cleanup may add a selector, fallback, retry, or second dispatcher.
Smallest next slice:
  After the human `LOOP-PRODUCTION-SELECTION-D0`, run the one disposition R0
  below.  If no production candidate is admitted, perform inventory only and
  keep every physical/legacy edge unchanged.
Non-claims:
  No production switch, Generic selection, common-session rewrite, helper
  dedupe, runtime ABI change, Text/lifecycle work, fallback/retry, or broad
  LOC-driven deletion is opened by this follow-up.
```

| parked row | exact bounded work | acceptance / deletion gate |
|---|---|---|
| `MIRBUILDER-SEALING-ESCAPE-DISPOSITION-R0` | Census the remaining production-visible split surfaces (`VerifiedGenericRecipeProductG0::into_physical_boundary`, any non-test tuple boundary, and the public `EndAuthorizedTextV1` facade). Keep the lexical forest-verifier `recipe.clone()` as an explicitly non-escaping temporary unless a source-backed replacement is found. | Each row names one final owner, exact callers, parity evidence, and a same-slice zero-caller deletion. No public getter/tuple is narrowed merely because it is visible. Runtime facade work stays in `RUNTIME-END-AUTHORIZED-TEXT-FACADE-I0`. |
| `MIRBUILDER-COMMON-V2-FUNCTION-DUPLICATE-CENSUS-R0` | Map S6C, Generic, and common-V2 function/session implementations at function granularity, including the existing common dispatcher and its legacy `ReadyLoopEntryV1`/segment inputs. Record which are intentional keep, waiting scaffold, or bounded retirement. | A future common-session replacement must have one rollback owner, one production consumer, and a zero-caller deletion row for each replaced function. This census may not retrofit the old dispatcher or create a Generic adapter. |
| `MIRBUILDER-LEGACY-DISPOSITION-R0` | Fill `generic-loop-legacy-disposition-v1.tsv` only from the two censuses above and observed owner/caller evidence. Preserve sentinel rows and do not infer decisions from size or byte identity. | Every non-sentinel row has `owner`, `parity_gate`, `retire_row`, and evidence command; unresolved rows remain parked. |
| `MIRBUILDER-BYTE-HELPER-INVENTORY-R0` | Record the byte-identical helper groups and micro-seed template families as an informational inventory with source owner and caller evidence. | Merge/archive/delete only in a later source-backed parity slice with zero callers. Inventory completion never unlocks physical emission or production selection. |

This follow-up is a closeout/disposition plan, not a new layer.  In
particular, the old detached Generic chain is not renamed into the new
admission, the common dispatcher is not duplicated for Generic, and the
production switch is still governed solely by `LOOP-PRODUCTION-SELECTION-D0`.

#### Structural debt review follow-up — accepted disposition, still parked (2026-08-18)

The external size review is directionally correct: the target architecture is
thin (`source parent -> prephysical admission -> one session -> one leaf
dispatcher`), while the current tree is temporarily thick because old
caller-zero probes, compatibility lanes, and future consumers coexist.  The
measured LOC/helper/micro-seed totals are **informational census only**; they do
not authorize a deletion, a new owner, or a production claim.  The following
rows make the three concrete sealing escapes and the missing retirement gates
explicit without changing the current `design_stop` or Dynamic blocker.

```text
Decision:
  Keep the one-authority chain and classify the remaining thickness as
  bounded disposition work.  Do not “solve” scale by broad dedupe or by
  relabeling a caller-zero probe as production.
Source authority + canonical issuer:
  Existing S4/Generic/Callable source producers, the common dispatcher, and
  the runtime lease owner remain their current authorities.  These rows issue
  no semantic receipt; they record callers, final owners, parity, and deletion.
Non-authority:
  LOC totals, byte identity, micro-seed counts, public visibility, lexical
  recipe.clone(), Clone-able seal wrappers, owner/name equality, and test-only
  green are not retirement or production authority.
Fail-fast boundary:
  Keep a row parked until exact callers, final owner, parity gate, and
  same-slice zero-caller deletion are named.  No fallback, retry, selector,
  second dispatcher, or public getter narrowing is inferred from census data.
Smallest next slice:
  Run the bounded R1 census rows below after the active Dynamic bridge stop;
  if a source-backed replacement is absent, record keep/park and stop.
Non-claims:
  No Generic/Dynamic production switch, common-session rewrite, Text/lifecycle
  activation, runtime ABI change, broad helper dedupe, or LOC-driven deletion.
```

| parked R1 row | bounded responsibility | required evidence / exit |
|---|---|---|
| `MIRBUILDER-S4-PRODUCER-ESCAPE-DISPOSITION-R1` | Census the S4 producer's four physical-boundary splits (`into_operation_effect`, demand/physical-part helpers, and `into_physical_boundary`) together with the lexical `recipe.clone()`. Separate true production escape surfaces from producer-local temporaries; do not copy the recipe or create a fifth issuer. | Exact non-test callers, one final source-parent/cohort consumer, parity with the landed semantic-program consume shape, and a same-slice zero-caller guard for each retired symbol. A lexical verifier clone remains keep-only unless an actual escaping caller is found. |
| `RUNTIME-END-AUTHORIZED-TEXT-FACADE-I0` | Audit `EndAuthorizedTextV1` visibility, token/getter callers, and any `Clone`-derivable seal wrapper. Keep lease semantics and the runtime retirement terminal unchanged; design a private/move-only surface only where the caller census proves it is safe. | Runtime owner, exact callers, and compatibility boundary are recorded before visibility changes. No MIR receipt or compiler-side alias may replace the runtime lease owner; unresolved callers remain parked. |
| `MIRBUILDER-COMMON-V2-RETIREMENT-GATE-D0` | Extend the existing function-level S6C/Generic/common-V2 census with explicit entry/exit conditions for the common V2 session and dispatcher: one session/rollback owner, one production consumer, exact replacement inputs for legacy `ReadyLoopEntryV1`/segment receipts, and the old-edge disposition. | A replacement row is not accepted until every replaced function has a final consumer, parity gate, and zero-caller deletion row. This D0 may observe the dispatcher but may not retrofit it or create a Generic adapter. |
| `MIRBUILDER-NYLLVM-C-ALLOWLIST-REVISION-R0` | Inventory the ny-llvmc allowlist synchronization and the repeated consumer/revision literals. Centralize or version them only after the C API/JSON owner and actual callers are identified; this is a backend governance row, not a Loop physicalizer row. | Focused allowlist-sync test, one revision owner, and legacy/contract-bound consumer separation. Missing evidence keeps the row parked; no Text/GEP/load or emitter-route change is implied. |

The previously recorded `MIRBUILDER-SEALING-ESCAPE-DISPOSITION-R0`,
`MIRBUILDER-COMMON-V2-FUNCTION-DUPLICATE-CENSUS-R0`,
`MIRBUILDER-LEGACY-DISPOSITION-R0`, and
`MIRBUILDER-BYTE-HELPER-INVENTORY-R0` remain the parent inventory rows.  The
new R1 rows refine their missing evidence; they do not create a parallel
cleanup registry.  In particular, the old Generic detached chain is already
retired, and the common dispatcher remains a waiting scaffold until the
selected-Dynamic replacement/cutover proves its final consumer.

```text
Decision:
  Keep `LOOP-GENERIC-G0-PHYSICAL-EMITTER-SESSION-PREFLIGHT-D0` at design stop.
Source authority + canonical issuer:
  The landed admission owns layout/program and entry facts; one canonical
  unpublished session must consume one whole admission ref, alone create the
  shell, adopt lanes, and issue the existing mechanical entry/segment inputs
  for the sole common dispatcher. It never accepts independently supplied
  layout, shell-plan, control, or Completion siblings.
Non-authority:
  Raw `ReadyLoopEntryV1::new_for_test`, S6C envelopes, copied IDs, owner-only
  pairing, old detached canaries, and a second Generic dispatcher are not
  preflight authorities.
Fail-fast boundary:
  Retain `NoSafeSlice::GenericG0EmitterSessionPreflightUnsealed` unless exact
  Recipe key/binding/value coverage can be issued at the actual preheader in
  the same rollback-owned transaction before any leaf operation.
Smallest next slice:
  Read-only issuer census for shell creation, lane adoption, canonical
  mechanical entry projection, layout segment allocation, and rollback timing.
Non-claims:
  No session implementation, operation MIR, CFG/SSA/PHI, Completion consume,
  lifecycle/Text, route, publication, fallback/retry, or production switch.
```

#### Session-preflight issuer census — design stop remains open (2026-08-17)

The combined admission is accepted as the source/prephysical owner, but this
D0 is not yet an implementation permission. The current Generic opener still
consumes `GenericG0DetachedEntryCanaryV1` and its detached `MirFunction` in
`resolved_lowering/generic_g0_physical_entry_session.rs`; it cannot be
relabelled as the canonical consumer. `CanonicalSsaFunctionSessionV2::new_generic`
is a reusable session consumer for the already-issued control/Completion
facts, not an issuer that accepts the whole combined admission.

The next census must close these exact seams in one owner graph:

```text
PreparedGenericG0PhysicalEmitterAdmissionV1
  -> one callback-scoped consume_into_session boundary
  -> outer unpublished DraftSeal transaction
  -> shell plan materialization and install
  -> CanonicalSsaFunctionSessionV2::new_generic
  -> canonical lane adoption
  -> entry receipt from identity/BindingSSA at the actual preheader
  -> layout-owned segment allocation
  -> callback-scoped session-preflight view
```

There is a concrete P0 in the landed shape: the admission currently
destructures and drops the cohort's `VerifiedGenericG0EntryBindingV1` rows
while sealing (`emitter_admission.rs`), so a later session cannot issue the
source-backed `LoopValueKeyV1 ↔ BindingRefV1 ↔ canonical ValueId` entry rows
without re-reading or guessing. The next admission/session boundary must
retain those rows as part of the same one-shot owner and co-seal them against
the program's verified input/carrier `entry_value` relations. It may not use a
hard-coded `[0, 1]`, raw operation count, or a copied entry table.

`ReadyLoopEntryV1::new_for_test` and old V1 block/segment receipts are test or
mechanical consumers, not source authority. The entry receipt must be issued
from the adopted resolver `BindingRef` rows and canonical `read_entry_receipt`
at the live preheader; arity, item ordinal, copied `ValueId`, or owner-only
pairing are insufficient. `segment_allocator::allocate_for_layout` is a
reusable mechanical leaf only after that receipt exists in the same outer
transaction. The future session API accepts one whole admission reference,
never separately supplied layout/shell/control/Completion siblings.

The old logical `LoopPhysicalBlockReceiptV1` is also not the Generic bridge:
Generic's five-segment layout can split one logical body block across multiple
physical segments, which that receipt rejects. The session census must bind the
layout's exact segment keys to the segment-aware receipt/allocator and then
issue target rows from those same keys; no logical-block adapter or S6C receipt
may be introduced.

The census also records two bounded cleanup prerequisites: move the shared
`PreparedGenericG0EntryControlFactsV1` validator out of the old canary module
before canary retirement, and isolate the caller-zero
`VerifiedGenericRecipeProductG0::into_physical_boundary` split in
`LOOP-GENERIC-G0-SEALED-CONSUME-I0`. Neither cleanup creates a semantic owner.

#### Session-preflight entry-row retention closeout (2026-08-17)

The first implementation sub-slice closes the admission-side loss of source
entry rows. `PreparedGenericG0PhysicalEmitterAdmissionV1` now retains the
same-cohort `LoopValueKeyV1`/`BindingRefV1` rows and lends them only through
its callback-scoped view. The focused Generic suite remains green; shell
materialization, canonical preheader reads, segment allocation, and operation
effects remain successor session-preflight work.

#### Session-preflight I0 closeout (2026-08-17)

The caller-zero preflight now consumes one whole admission through
`into_session_preflight`, opens the existing unpublished function transaction,
materializes the declaration-only shell, adopts the source-ordered lanes,
reads each retained binding at the live canonical preheader, and allocates the
layout-keyed segment receipt. A late callback error discards the complete
candidate; no module publication, operation leaf, retry, or fallback is
reachable. The mechanical bridge to `ReadyLoopEntryV1` is constructed only
from canonical read receipts and is not a source authority.

The next stop is the Generic-to-common dispatcher preflight. The existing
dispatcher still requires a family-neutral operation/session input that this
I0 does not invent, so operation effects remain closed.

#### Next design stop — Generic-to-common operation-emitter owner

```text
Decision:
  Keep `NoSafeSlice::GenericG0OperationEmitterOwnerUnsealed`.  The landed
  session preflight may lend only its canonical entry/segment receipts; it
  must not adapt the S6C common session or old V1 dispatcher inputs.
Source authority + canonical issuer:
  The Generic source parent/cohort owns the complete
  `PreparedLoopOperationProgramV1` and mechanical mapping.  A single
  same-session consumer must co-seal that program, the canonical session
  stamp, layout/segment receipt, target block, and rollback owner before
  borrowing the existing family-neutral dispatcher.  It may not reconstruct
  context/effect/continuation from MIR or operation counts.
Non-authority:
  `CommonV2CanonicalSessionRefV1`, S6C provenance, old V1 block/entry receipts,
  `new_selected_dynamic`, operation names/ordinals, copied IDs, owner-only
  stamps, JSON, and a second Generic leaf emitter are not the missing owner.
Fail-fast boundary:
  Reject before the first operation instruction if the program/mapping,
  session/layout/target stamp, exact segment coverage, value ledger, or
  callback-scoped dispatcher input cannot be co-sealed from the same cohort.
  No partial leaf effect, retry, fallback, or publication is allowed.
Smallest next slice:
  Read-only census of the existing dispatcher input requirements and a
  family-neutral Generic adapter shape.  If it would require S6C/V1 receipt
  reuse or source re-inference, keep this NoSafeSlice.
Non-claims:
  No operation MIR, ReadBinding/Const/Binary/Compare/Write emission, CFG/SSA/
  PHI mutation, Completion/DraftSeal, lifecycle/Text, route, backend,
  production caller, fallback, retry, or main integration.
```

#### Dispatcher input census — design-only refinement (2026-08-17)

The read-only dispatcher audit closes the mechanical input list but does not
open an emitter effect.  The existing segment-aware dispatcher consumes one
complete `PreparedLoopPhysicalLayoutV1` (and therefore its owned
`PreparedLoopOperationProgramV1`/value ledger), one session-issued canonical
entry view, and one layout-keyed segment-block receipt.  Its target rows are
derived from those three inputs before the first leaf; the eventual leaf
callback borrows only the already-open canonical Builder, identity, and PHI
services.  The current preflight callback returns only the segment receipt, so
it is not yet safe to let a later caller re-pair that receipt with an
independently borrowed layout or program.

```text
Decision:
  Accept the dispatcher-preflight BoxShape and keep operation leaf effects
  closed.  Reuse the existing family-neutral segment dispatcher as the sole
  leaf candidate, but bind its three mechanical inputs to one callback-scoped
  view before any later leaf plan can be borrowed.  This is a mechanical
  aggregate of existing receipts, not a new semantic authority.
Source authority + canonical issuer:
  The combined Generic emitter admission owns the program/layout and source
  entry rows.  The same unpublished session preflight owns the canonical
  preheader entry view and layout-keyed segment receipt.  One future
  session-owned consumer must co-seal those facts and lend a transient
  `GenericG0SegmentDispatchInput`-shaped view exactly once to
  `prepare_loop_segment_operation_dispatch_v1`; the name is a design
  placeholder, not an implementation permission or a new semantic authority.
Non-authority:
  A standalone `LoopPhysicalSegmentBlockReceiptV1`, old logical
  `ReadyLoopEntryV1`/block receipts, S6C/CommonV2 envelopes, operation names
  or ordinals, raw `ValueId`s, owner/name equality, operation counts, and
  independent layout/program getters cannot establish dispatch ownership.
Fail-fast boundary:
  Before the first operation instruction, reject any program/layout/entry/
  segment owner, preheader, target, segment-index, producer/operand, or
  coverage drift; reject a missing or duplicate callback loan; and reject any
  attempt to save the view, retry, fallback, or publish partial leaf effects.
  All target rows must validate before the first leaf mutates MIR; the outer
  unpublished function transaction remains the sole discard owner.
Smallest next slice:
  `LOOP-GENERIC-G0-PHYSICAL-OPERATION-DISPATCH-PREFLIGHT-I0`: consume one
  whole admission through the existing unpublished session, construct one
  callback-scoped dispatch-input view, and run the existing segment-dispatch
  preflight without emitting a leaf.  If this requires an S6C/V1 adapter, a
  second Generic program issuer, or source re-inference, return to the
  design stop instead of adding an adapter.
Non-claims:
  No `Pinned*`/Text work, operation MIR, ReadBinding/Const/Binary/Compare/
  Write emission, CFG/SSA/PHI, Completion/DraftSeal, route/backend,
  production caller, publication, fallback, or retry.
```

#### Dispatcher preflight I0 closeout (2026-08-17)

The caller-zero session now moves the admission-owned layout out of the
preflight handoff only after segment allocation, pairs it with the canonical
entry rows and segment receipt in one phantom-lifetime-branded input, and
invokes the existing segment dispatcher preflight.  The input owns no source
meaning and cannot escape the callback; the prepared leaf plan is immediately
dropped.  The positive Generic suite remains green, and the shell contains no
operation instructions after the preflight.  Operation emission, publication,
retry, fallback, and production callers remain closed for the next row.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test --lib generic_g0 -- --nocapture  # 72 passed
RUSTFLAGS='-Awarnings' cargo check -q
cargo fmt --all
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

#### Operation emitter I0 closeout (2026-08-17)

The same callback-scoped input can now consume the existing common segment
dispatcher once.  `emit_all` performs its complete target preflight before the
first leaf, publishes the five-variant operation rows through the canonical
Builder/identity/PHI services, and returns only an operation-count receipt to
the caller-zero test.  A late callback failure after emission still reaches
the outer unpublished discard owner, leaving no function or block published.
No Generic leaf emitter, S6C adapter, Completion/DraftSeal claim, retry,
fallback, or production caller was added.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test --lib generic_g0 -- --nocapture  # 72 passed
RUSTFLAGS='-Awarnings' cargo check -q
cargo fmt --all
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

```text
Decision:
  Keep `NoSafeSlice::GenericG0EmitterSessionPreflightUnsealed`.
Source authority + canonical issuer:
  The combined admission remains the sole prephysical authority; a future
  family-neutral unpublished-session opener is the only issuer allowed to
  create/install the shell, adopt lanes, issue entry/segment receipts, and own
  rollback.
Non-authority:
  Detached canary/session, `new_for_test`, S6C envelope, old V1 receipts,
  copied IDs, owner-only stamps, and any sibling-argument constructor.
Fail-fast boundary:
  Reject before session effect if whole-admission consume, actual-preheader
  entry coverage, segment binding, shared control facts, or one-shot rollback
  ownership cannot be proved in the same callback.
Smallest next slice:
  Read-only census of the canonical entry issuer and the single consume opener;
  then record the bounded sealed-consume prerequisite before session I0.
Non-claims:
  No new receipt, session implementation, Builder mutation, operation effect,
  CFG/SSA/PHI, Completion/DraftSeal, lifecycle/Text, route, fallback, retry,
  publication, or production switch.
```

#### Session-preflight D0 decision — accepted BoxShape (2026-08-17)

The issuer census closes as a single consumer boundary.  This accepts the
session-preflight shape, but it does not yet open operation emission or a
production caller.

```text
Decision:
  Accept one family-neutral `with_generic_g0_physical_emitter_session_preflight`
  consumer.  It takes exactly one whole
  `PreparedGenericG0PhysicalEmitterAdmissionV1` and owns the unpublished
  transaction from shell materialization through entry/segment preflight.
  The admission is consumed once; no sibling layout/shell/control/Completion
  arguments and no `into_parts` tuple are accepted.
Source authority + canonical issuer:
  The admission's one-shot consume view retains the source input, entry rows,
  program/layout, declaration-only shell plan, control facts, typed BlockExpr
  expectation, outer-If residual, canonical Completion, target, and full stamp.
  `CanonicalFunctionLoweringSessionV1` is the sole mutable transaction and
  rollback owner: it opens the unpublished draft, materializes `MirFunction`
  from the shell plan, installs it, and opens the existing Generic canonical
  session.  The semantic Completion is borrowed once to issue the session-local
  physical consumer; no Completion claim or DraftSeal placement is opened here.
  The session then adopts the source-ordered receiver/ordinary lanes and issues
  the entry projection from retained `LoopValueKeyV1`/`BindingRefV1` rows plus
  the program's verified input/carrier `entry_value` relations.  Each row is
  read through canonical identity at the live preheader; no numeric ValueId or
  `[0, 1]` reconstruction is allowed.  The existing segment-aware allocator
  is reused only after that entry projection, through a thin mechanical bridge
  that accepts the canonical entry receipt and returns the layout-keyed segment
  receipt.  It never consumes the old logical block receipt or S6C envelope.
Non-authority:
  `GenericG0DetachedEntryCanaryV1`, detached `MirFunction` shells,
  `new_generic` as a whole-admission issuer, `ReadyLoopEntryV1::new_for_test`,
  old `LoopPhysicalBlockReceiptV1`, S6C/V1 receipts, copied IDs, owner/name/
  lane-count stamps, operation counts, late Builder scans, and a second
  dispatcher are not session authorities.  The shared control facts are moved
  to a neutral validator module before canary retirement; the old module is
  not allowed to become a Generic semantic owner.
Fail-fast boundary:
  Before any Builder effect, reject missing retained entry rows, program
  input/carrier drift, owner/origin/body-root/frame/target drift, symbol or
  lane-order drift, nonempty Builder state, stale preheader, missing/duplicate
  canonical entry row, layout/segment coverage drift, foreign Completion, or
  any HRTB/tuple escape.  After opening, the outer unpublished transaction is
  the only discard owner; late failure performs one discard and never retries
  or falls back.  Segment allocation is allowed only inside that transaction
  and only after exact entry coverage; operation leaf emission remains later.
Smallest next slice:
  `LOOP-GENERIC-G0-SEALED-CONSUME-I0` first isolates the caller-zero
  `into_physical_boundary` split.  Its successor
  `LOOP-GENERIC-G0-PHYSICAL-EMITTER-SESSION-PREFLIGHT-I0` retains entry rows,
  moves control validation to its neutral owner, and implements the one-shot
  session callback with shell/adoption/entry/segment preflight only.
Non-claims:
  No operation MIR/leaf dispatch, edge/terminator, new CFG/SSA/PHI shape,
  Completion/DraftSeal publication, lifecycle/Text/route, production caller,
  module publication, fallback, or retry.
```

#### `LOOP-GENERIC-G0-SEALED-CONSUME-I0` closeout (2026-08-17)

The caller-zero `VerifiedGenericRecipeProductG0::into_physical_boundary` split
is now `cfg(test)` only.  The production Generic source-parent/cohort/admission
path is unchanged, and the lexical forest-verifier `recipe.clone()` remains
inside its producer scope.  No session, Builder, `MirFunction`, `ValueId`,
operation, CFG/SSA/PHI, Completion/DraftSeal, route, fallback, retry, or
production caller was opened.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo test --lib generic_g0 -- --nocapture  # 69 passed
RUSTFLAGS='-Awarnings' cargo check -q
cargo fmt --all -- --check
git diff --check
rg -n "into_physical_boundary" src/mir/loop_recipe_contract/generic_g0/producer.rs src/mir
```

The remaining detached skeleton/canary `into_parts` methods stay owned by the
later parity retirement.  This I0 is a sealing refactor, not permission to
open the session-preflight effect.

### `LOOP-GENERIC-G0-PHYSICAL-OPERATION-COHORT-D0` (accepted BoxShape)

```text
Decision:
  Adopt one source-owned, one-shot `GenericG0PhysicalOperationCohortV1`.
  It owns the family-neutral `PreparedLoopOperationProgramV1` and the
  independent Generic source siblings needed by later consumers.  The
  mechanical mapping is a transient callback view over the cohort's owned
  operation/evidence product; it is never stored beside a parent borrow and
  never survives program consumption.  Operation emission stays closed until
  the common emitter boundary above is accepted.
Source authority + canonical issuer:
  `VerifiedGenericG0SourceParentV1` and its verified recipe/core/effect
  product remain the only source authority.  A single
  `with_generic_g0_physical_operation_cohort_v1(input, selection, callback)`
  issuer performs the source-parent construction and the one-shot ownership
  transition.  It co-seals context, operation effect, After/continuation,
  schedule/coverage, and the independent entry/source siblings, then issues
  an owned program through `VerifiedLoopOperationPhysicalDemandV1::issue`
  followed by `prepare_all`.  A scoped `with_mapping` port may borrow the
  program's owned effect/evidence for mechanical preflight; it cannot return
  the mapping or a parent reference outside the callback.
Non-authority:
  The current borrow-only parent/entry/skeleton wrappers, the
  `cfg(test)` `into_operation_demand_parts` split, a second mapping
  reconstructed from MIR/AST/ValueId scans, S6C provenance, item ordinals,
  `/N`, or a copied context/effect/continuation tuple are not an operation
  cohort or ownership proof.  The common dispatcher remains a mechanical
  consumer, not a Generic source issuer.
Fail-fast boundary:
  Reject before Builder/MIR effect on owner/origin/frame/target drift,
  incomplete context/effect/continuation or 15-row coverage, mapping/program
  mismatch, parent borrow retained across the ownership transition,
  mapping borrow surviving program consumption, self-reference, double
  consumption, loan escape, partial program construction, or any request to
  publish a detached program without its cohort owner.  Existing entry-input
  and skeleton wrappers must be refactored to borrow the cohort, not the
  consumed source parent.
Smallest next slice:
  Caller-zero `LOOP-GENERIC-G0-PHYSICAL-OPERATION-COHORT-I0`: add the one
  source-parent consume transition, detached non-Clone cohort owner, and
  callback-scoped transient mapping port.  This I0 may prepare the complete
  family-neutral program and run mapping preflight only; it must not emit MIR,
  open Builder/session state, issue ValueIds, or call a physical leaf.
Non-claims:
  No operation emission, block/edge/Completion/DraftSeal mutation, lifecycle,
  Text, route, backend, fallback/retry, production caller, or main integration.
```

The mapping I0 and cohort I0 are complete, and the common dispatcher is the
only accepted leaf-emitter candidate.  The cohort BoxShape is accepted and
now owns the neutral program with a scoped mapping port.  No physical
operation leaf is authorized until the next emission D0 names the common
session/layout/rollback owner.

### Canonical session admission D0 (accepted three-step boundary)

```text
Decision:
  Do not open CanonicalSsaFunctionSessionV2 yet. First issue one typed
  resolver-owned block-expression expectation; then a separate caller-zero
  admission may co-seal it with the resolved input, common V2 envelope, one
  borrowed Completion, and resolver-owned outer-If residual.

Source authority + canonical issuer:
  One resolver shadow traversal already issues both
  VerifiedResolvedFunctionV1 and VerifiedResolvedBodyShapeInventoryV1.
  ResolvedBlockExpressionExpectationIssuerV1 is the sole new issuer: it
  matches typed BlockExpr source rows to exact BlockExpr scope/region pairs
  and emits one non-Clone expectation. Later, only the exact resolver
  singleton Loop site may feed empty_for_owned_loop_profile; the installed
  S6C child lends its actual Completion through the existing nested HRTB.

Non-authority:
  FirstFamily verify_body's raw usize, Other("BlockExpr") string matching,
  resolved scope count alone, S6C's fixed zero/13/15 counts, Recipe/JoinSig
  If/Exit rows, caller-supplied Loop paths, Completion parity summaries,
  Dynamic-only constructors, AST/MIR/JSON rescans, legacy finalizers, and
  DraftSeal are not admission issuers.

Fail-fast boundary:
  Reject on function/body-shape owner, function origin, or body-root drift;
  missing/duplicate/extra BlockExpr pair; count overflow; no/multiple Loop;
  Loop-external If; foreign envelope/Completion owner or target; HRTB escape;
  Completion clone; or any need to retrofit the legacy finalizer or rescan
  Returns. Every rejection precedes session effects.

Smallest next slice:
  The source-product and transport rows are landed, and
  LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0 now issues one callback-scoped
  non-Clone fan-in with exact owner/origin/root/Loop/Completion checks. The
  caller-zero session-open canary consumes that admission once; the next
  bounded design row is LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0.

Non-claims:
  No CanonicalSsaFunctionSessionV2 construction, CFG/SSA/PHI mutation,
  Completion consumption, DraftSeal, lifecycle, Text route, production
  caller, fallback, retry, or legacy-finalizer retirement.
```

`RESOLVED-BLOCK-EXPR-EXPECTATION-I0` execution brief:

```text
Change:
  add BodyExpressionShapeV1::BlockExpr { site }; issue one batch-owned
  VerifiedResolvedBlockExpressionExpectationV1 from the same resolved
  function/body-shape row; store it once in that callable batch row
Contract:
  typed source rows and exact BlockExpr scope/region pairs have equal owner,
  origin, root, sites, and count; constructor private; product non-Clone
Done:
  zero/one/nested positives plus missing/extra/duplicate/foreign/root/count
  negatives; existing body-shape consumers stay exhaustive; source files
  stay below the 760-line split trigger and 800-line hard stop
Stop:
  string matching, AST/MIR rescan, raw usize transport, fixed zero, session
  construction, Completion consumption, fallback, or retry is required
```

### RESOLVED-BLOCK-EXPR-EXPECTATION-I0 implementation receipt (2026-08-17)

The shadow resolver now emits a typed `BlockExpr { site }` body-shape row.
`ResolvedBlockExpressionExpectationIssuerV1` co-seals the same resolved
function/body-shape product, checks exact source-site, scope, and region
coverage in both directions, and the callable semantic batch row owns the
non-Clone receipt once. Focused resolver tests cover zero, one, nested, and
foreign-owner cases; selected/package transport, session, CFG/SSA/PHI,
Completion consumption, physical lowering, fallback, retry, and production
callers remain closed.

### CALLABLE-BLOCK-EXPR-EXPECTATION-TRANSPORT-I0 implementation receipt (2026-08-17)

`SelectedCallableLoweringInputRefV1` now lends the same batch-owned
`VerifiedResolvedBlockExpressionExpectationV1` inside the existing package
HRTB. The accessor is borrow-only and non-reconstructible: no clone, reissue,
raw `usize`, AST/MIR rescan, session effect, Completion consumption, or legacy
finalizer connection is added. A selected static callable containing a
BlockExpr passes the focused transport handoff test.

### LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0 implementation receipt (2026-08-17)

`common_v2_session_admission.rs` now issues one callback-scoped,
non-`Clone` fan-in from the selected resolved input, the resolver singleton
Loop site, the existing Loop-owned outer-If residual, the batch-owned typed
BlockExpr expectation, the common V2 envelope, and the actual borrowed
Completion. Owner, function-origin/body-root, Loop cardinality, Completion
owner/target, and envelope ownership drift reject before any session effect.
The focused S6C admission test proves the 15-placement envelope, exact
Completion borrow, outer-If residual, and duplicate child-consumption fence.
No `CanonicalSsaFunctionSessionV2`, CFG/SSA/PHI, Completion consumption,
DraftSeal, lifecycle, Text route, physical lowering, fallback, retry, or
production caller is opened.

### LOOP-COMMON-V2-PHYSICAL-SESSION-I0 implementation receipt (2026-08-17)

The first caller-zero physical-session seam is now landed as a deliberately
thin opener. `LoopV2CanonicalSessionAdmissionRefV1` is consumed exactly once
into one callback-scoped parts aggregate; the typed BlockExpr expectation is
projected only inside `CanonicalSsaFunctionSessionV2::new_common_v2`, and the
installed semantic Completion remains borrowed. That borrow issues one owned
`ResolvedFunctionCompletionConsumptionV1` for the session, so semantic
Completion is not cloned or moved out of its installed cohort. The same
callback retains the common V2 envelope beside the session and cannot reacquire
another Port loan.

This canary opens the sole canonical CFG/Binding-SSA/PHI/Completion owner, but
does not mutate a Builder or emit a block, operation, control transfer, PHI,
Completion claim, Return, DraftSeal, lifecycle, Text operation, route, or
production caller. It does not reuse `new_selected_dynamic`, pass a raw
BlockExpr count, or expose a second session. This receipt is landed; the
current frontier is the physical condition-result receipt design stop, not a
session reopen or a second physical-entry authority.

### LOOP-S6C-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-D0/I0 (S6C-only; landed)

```text
Decision:
  The physical-header BoxShape and its caller-zero I0 are landed. The
  physical-function-entry input BoxShape is accepted only for the installed
  S6C cohort: one same-cohort transport-only aggregate may expose source
  ParamDecl evidence and complete physical lane descriptors without making
  the existing one-value formal-adoption API guess how an ExactText pair is
  represented. This is not a Generic G0/common issuer; its implementation is
  limited to that S6C aggregate and does not open Builder effects, skeleton
  allocation, or lane adoption in this row.

  The design stop is intentionally split into:
    LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-D0
      accepted physical parameter declaration/lane projection BoxShape
    LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0
      active transport-only descriptor aggregate
    LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON-I0
      future fresh unpublished skeleton reservation after the input I0
  Only the middle row is executable in the current fast lane.

Source authority + canonical issuer:
  Existing S6C package issuers remain the authorities: the catalog declaration
  owns symbol/ParamDecls/result/attrs/uses, the physical-signature cohort owns
  lane order/role/BindingRef, the physical header owns result/Completion, and
  `VerifiedS6CPhysicalFunctionEffectsV1` owns only the source-backed physical
  effect projection. A future compiler-side
  `PreparedCanonicalFunctionEntryInputV1` issuer may co-seal these borrowed
  siblings from one installed S6C HRTB loan, but it must issue no new semantic
  fact and must not take `CanonicalFunctionLoweringSessionV1` as an authority.
  Its only output is a non-Clone, pre-effect relation consumed by the existing
  skeleton owner.

  The S6C-only design choice is the physical ParamDecl projection: receiver
  prefix, ordinary scalar lane, and each adjacent ExactText
  `[slot,generation]` lane need a deterministic declared name/type policy.
  Source parameter names/types may be read only through the same storage
  header; `/N`, `FunctionSignature` length, raw `ValueId`, or lane index alone
  cannot supply the missing fields. The wire contract says `u64`, while the
  current MIR/LLVM callable carrier is `i64` and
  `source_type_name_to_mir("u64")` would become an unrelated `Box("u64")`.
  The carrier decision is therefore fixed as a checked package-owned
  `U64BitsOnI64` physical-lane metadata: existing MIR/LLVM `i64` is only the
  bit-preserving mechanical carrier, while the lane role remains the
  authority for unsigned-wire meaning. This does not change semantic
  `MirType`, add a new unsigned source type, or permit generation arithmetic.
  The remaining input census must prove that this carrier row, every source
  `ParamDecl`, and the lane-role rows are exposed by one same-loan issuer;
  a string spelling of `u64` is never a valid substitute. `MirParamDecl` is
  existing source-annotation metadata, not the physical lane carrier: the
  future compiler input must use a separate non-semantic physical-parameter
  descriptor rather than overload `MirParamDecl` for expanded lanes.

  The source census fixes the seam: the installed
  `VerifiedSameModuleCallableDeclarationCatalogV1::declaration` row is the
  source owner for symbol, ParamDecls, result annotation, `uses`, and attrs;
  `issue_callable_physical_signature_v1` is the lane owner; and the retained
  S6C ingress is the owner for Facts/Recipe/Join/Completion relations. The
  new `VerifiedS6CPhysicalFunctionEffectsV1` is issued only after retained
  S6C Facts prove the exact two external calls and their CoreMethod rows prove
  `PureRead`; the local index write remains source-local and does not become a
  heap-write effect. The Dynamic-only
  `CatalogedBoxMethodPhysicalHeaderProjectionV1`, legacy
  `NormalCatalogedBoxMethodDraftAdmissionV1::physical_arity`, attrs/uses
  inference, and a literal `EffectMask::READ` are not substitutes.

Non-authority:
  Common V2 operation/control/coverage rows, Recipe/JoinSig ordinals,
  logical `/N`, FunctionSignature length, raw `MirParamDecl` or source-name
  fallback, existing one-value skeleton/adoption APIs, current_block, raw
  ValueId order, ReadyLoopEntry, LoopPhysicalBlockReceipt, Dynamic-only
  session, or any legacy finalizer may issue the entry input, skeleton, or
  adoption policy.

Fail-fast boundary:
  Before any mutation, one pre-effect input must verify same owner/origin/
  brand, selected identity, complete storage header, result/Completion,
  physical effects, receiver prefix, dense non-duplicate logical ordinals,
  adjacent ExactText `[slot,generation]` lanes, and a complete physical
  physical-parameter descriptor row for every lane. `source_logical_arity` counts explicit
  formals only; `receiver_lane_count` is a separate prefix axis and is never
  inferred from `/N` or a parameter-vector length. ExactText remains one
  BindingRef with two physical lanes; slot publication versus a private
  generation sidecar must be fixed before skeleton I0. Missing lane role/type,
  source-name drift, foreign loan, or incomplete header/effects means
  `NoSafeSlice` before Builder effect. A lane without the sealed
  `U64BitsOnI64` carrier metadata is also `NoSafeSlice`; `MirType::Integer`, a
  source `StringBox` type, or a guessed `Box("u64")` mapping cannot silently
  stand in for it. The generation lane is not an ordinary arithmetic value
  and must not be reconstructed from the `i64` carrier, a raw slot, or a
  `MirType`. Any later mutation failure must discard the unpublished
  transaction once, with no retry or fallback.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0` is the active
  transport-only implementation. The physical carrier choice is closed as
  `U64BitsOnI64`; the compiler-side issuer borrows the complete same-cohort
  storage header, physical signature, effects, result, and lane rows while
  projecting every source-backed `ParamDecl` into a separate non-semantic
  physical-parameter descriptor. It constructs no Builder or `ValueId`. After
  this focused gate, the next design stop is the fresh unpublished skeleton
  reservation. Any incomplete relation still rejects before Builder effect;
  it is not repaired by a default or fallback.

  The preceding header/effects I0 already proved the package-side co-seal.
  This D0 is accepted because the compiler-side aggregate can borrow that
  complete cohort and expose every physical ParamDecl row without reissuing
  source meaning. The active I0 still rejects missing/foreign header or
  effects projection, lane gap/swap, receiver drift, parameter/result drift,
  or duplicate ExactText BindingRef adoption before Builder effect; it never
  repairs them with a default or fallback.

Carrier decision receipt (2026-08-17):
  The accepted wire remains two scalar `u64` lanes for one logical ExactText
  formal. The accepted MIR/LLVM mechanical carrier is checked
  `U64BitsOnI64`; it preserves bits but does not become semantic
  `MirType::Integer`, a source `u64` type, or a new unsigned MIR type. The
  package-owned lane role and physical-signature row remain the sole meaning
  authority. The compiler-side same-loan ParamDecl/lane aggregate is now the
  active transport-only I0; no skeleton, session, ValueId adoption, or direct
  Text lowering is authorized by this receipt.

Physical descriptor policy (design-only):
  The next compiler-side view is named
  `PhysicalCallableParameterDescriptorV1` for design purposes. It is one
  row per physical lane and carries only mechanical/source relation:
  `physical_index`, `lane_role`, optional `logical_ordinal`, source
  `BindingRef`, deterministic diagnostic name, source annotation text when
  present, and a physical carrier tag. It has no `ValueId`, `MirType`, runtime
  token, pointer, or semantic ownership effect. The carrier tags are
  `ExistingCallableI64` for the receiver and ordinary scalar lanes, and
  `U64BitsOnI64` for both adjacent ExactText lanes.

  Naming is deterministic and never an authority: an instance receiver is
  `me`; an ordinary lane uses its catalog-backed source `ParamDecl.name`; an
  ExactText formal expands to `<source-name>.slot` followed by
  `<source-name>.generation`. Missing or duplicate source names reject rather
  than falling back to an ordinal, `/N`, or a generated placeholder. The
  source annotation remains attached as evidence only; physical carrier is
  selected by the package-owned lane role. Receiver is a prefix row and is
  not an explicit logical ordinal. This descriptor is projected by the future
  same-loan compiler aggregate, not stored as a second package semantic
  authority.

  Required negatives for the census are: foreign owner/brand, receiver row
  in a static method, missing receiver row in an instance method, non-prefix
  receiver, lane gap/overlap/swap, non-adjacent ExactText pair, duplicate
  logical ordinal or BindingRef, missing source name, guessed `u64` type,
  guessed `MirType::Integer`/`Box("u64")` carrier, source ParamDecl count
  used as physical lane count, and any `ValueId` or Builder access before the
  aggregate is consumed.

D0 acceptance gate (accepted BoxShape, 2026-08-17):
  Accept the input BoxShape only when one compiler-side, non-Clone
  same-loan view exposes the catalog-backed source `ParamDecl` rows and a
  separate physical-parameter descriptor for the receiver-prefix and every
  physical lane, together with physical-signature lane roles,
  `U64BitsOnI64` carrier rows, result/header, and source-backed effects under
  one owner/origin/brand.
  The view must be callback-scoped, issue no new semantic fact, and expose
  complete/disjoint physical coverage. Caller and callee ValueId projection,
  skeleton reservation, and BindingSSA adoption remain downstream consumers;
  they are not evidence for closing this D0. This gate is now closed as a
  design contract: the compiler-side issuer is a mechanical same-loan
  consumer, not a new semantic authority. Its implementation is the bounded
  input I0 below; it must still reject before any Builder effect when the
  relation is incomplete.

Current code census (2026-08-17):
  `S6CInstalledCallableLoanRefV1` already lends `selected()`, `signature()`,
  `storage_header()`, `physical_effects()`, `result()`, and scoped Completion
  from one installed Port callback. The catalog-backed storage projection
  owns source `ParamDecl`/name/type/attrs/uses rows; the physical-signature
  row owns lane role/order/BindingRef; and the S6C effects projection owns
  only the source-backed effect mask. The compiler-side issuer is now landed
  as the transport-only input I0: it joins only these borrowed siblings into
  physical parameter descriptors. Existing `MirParamDecl` and
  `create_resolved_function_skeleton` remain source-annotation and one-lane
  consumers, respectively. This census authorizes only the input aggregate,
  not a skeleton call or inference from `FunctionSignature` length.

Implementation receipt after D0 acceptance:
  `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0` adds one
  callback-scoped compiler module that consumes the same loan and emits no
  Builder effect. It exposes descriptor rows and borrowed source siblings to
  the later skeleton consumer, but reserves no blocks, creates no `ValueId`s,
  adopts no BindingSSA, consumes no Completion, and lowers no loop. Skeleton
  reservation remains a separate later row.

Implementation receipt (2026-08-17):
  The I0 is landed. `VerifiedS6CStorageHeaderProjectionV1` is a distinct
  catalog-backed storage projection, and `VerifiedS6CPhysicalFunctionEffectsV1`
  is issued from the retained S6C call contracts only after both external
  operations prove `PureRead`. The installed Port lends both siblings beside
  the existing signature/result header in one callback-scoped loan. Focused
  S6C/package tests, format, check, and pointer guards are green. The next
  design stop is the physical function-entry/skeleton issuer; no Builder
  effect was opened by this I0.

Entry-input I0 receipt boundary (2026-08-17):
  The accepted D0 now permits one callback-scoped compiler transport module
  to consume the same installed S6C loan and build the nonsemantic physical
  descriptor rows. The module may widen package accessors needed to borrow
  source `ParamDecl`s, but it must not mint a second signature/header/effects
  authority. Its focused gate must cover static and instance receiver order,
  mixed ordinary/ExactText lanes, aliasing occurrences, and every rejection
  in the descriptor policy above. This receipt does not authorize skeleton
  allocation or any `ValueId`/Builder mutation.

Non-claims:
  No skeleton or entry-lane adoption is authorized by this transport slice. No
  Loop CFG block allocation, Loop topology, Binding read/write beyond the
  future entry-lane adoption, PHI sealing,
  operation/control lowering, Completion claim, Return/DraftSeal, lifecycle,
  Text route, production caller, fallback, or retry.
```

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON`

```text
Decision:
  Accept the skeleton BoxShape. The landed same-loan descriptor aggregate is
  now sufficient to reserve one fresh unpublished physical skeleton. The
  caller-zero I0 below may create that detached shell, but it must not install
  it in a MirBuilder or adopt an ExactText BindingRef.

Source authority + canonical issuer:
  The same `PreparedCanonicalFunctionEntryInputV1` loan co-seals the selected
  catalog key/mode, S6C storage symbol/ParamDecl/return/attrs/uses, source
  effects/result, and the package-owned complete lane descriptor list. The
  new skeleton issuer is mechanical: it projects those facts to one
  unpublished `MirFunction` shell with one existing i64 carrier per physical
  lane. The future Builder/session remains the install/rollback consumer.

Non-authority:
  `MirParamDecl`, `FunctionSignature` length, logical `/N`, raw `ValueId`
  order, `ValueId(ordinal)`, Recipe/JoinSig ordinal, AST re-scan, and the
  existing one-lane `create_resolved_function_skeleton` path cannot define the
  physical carrier or choose the ExactText sidecar policy. A local entry block
  id in the unpublished shell is not a module allocation authority.

Fail-fast boundary:
  Selected/storage-key or mode/namespace drift, owner/effect mismatch, missing
  result/attrs/uses, descriptor count/index/carrier drift, and any attempt to
  install the shell or publish a BindingRef reject before Builder effect. No
  fixture-built skeleton, default effect, or fallback may repair it.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON-I0` consumes one accepted input,
  reserves the detached mechanical-i64 shell, retains descriptor carrier
  tags, and proves that no Builder/module publication occurred. The next
  design stop is a separate ExactText lane-adoption census: one logical
  BindingRef publishes once to the slot lane while generation remains a
  private move-only sidecar.

Non-claims:
  No Builder installation, ExactText lane adoption/BindingSSA publication,
  Loop CFG/PHI, Completion consumption/claims, DraftSeal, lifecycle, Text,
  route, production caller, fallback, or retry.
```

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-ENTRY-LANE-ADOPTION`

```text
Decision:
  Accept this BoxShape and open the caller-zero adoption I0. BindingSSA remains a
  one-value map `(entry block, BindingRef) -> slot ValueId`: an ExactText
  formal publishes exactly once to the slot lane. The adjacent generation
  lane is retained in a private, move-only physical sidecar and is never
  published as a second semantic BindingRef value. The detached skeleton is
  only a reservation product; it is not itself an adoption owner.

Source authority + canonical issuer:
  The compiler-side same-loan entry issuer consumes the installed S6C
  physical signature/header/effects/descriptor cohort and issues one
  `PreparedCallablePhysicalParameterListV1` together with one private
  `PhysicalTextEntryLaneSidecarV1` plan. The future
  `CanonicalFunctionLoweringSessionV1::open` transaction is the consumer and
  rollback owner, not an issuer of lane meaning. Receiver is prefix `%0`
  with `SourceBindingSite::Receiver`; ordinary formals publish one
  `Parameter{ordinal}` lane; ExactText publishes one logical BindingRef to
  the adjacent slot lane and records the generation lane in the sidecar.
  Callee parameter ValueIds are pairwise distinct by physical lane. Caller
  argument occurrences may reuse a ValueId for aliases, but caller and
  callee ValueId scopes are never compared.

  The sidecar is an entry/forward transport receipt only. It may provide a
  scoped pair view for an admitted physical Text consumer, but it cannot
  rebind a BindingRef, perform generation arithmetic, expose generation via
  ordinary `read_entry`, or become a source/lifetime authority. Rebinding an
  ExactText formal is rejected in this first slice; a later pair-aware
  rebind receipt would be a separate decision.

Non-authority:
  Existing one-lane `adopt_exact_formal_parameter`, repeated publication of
  the same BindingRef, `ValueId(ordinal)`, `MirFunction.params` order alone,
  `/N`, `MirParamDecl`, raw slot/generation integers, ordinary BindingSSA
  reads, Recipe/JoinSig ordinals, a copied detached skeleton, fixture-built
  parameters, or a second Dynamic/legacy session cannot choose the policy.

Fail-fast boundary:
  Before Builder publication, one transaction must verify the same owner,
  selected identity, physical signature revision, dense lane order, receiver
  prefix, adjacent ExactText pair, lane carrier/type, and exact skeleton
  parameter count. It must publish each logical BindingRef at most once,
  retain every generation lane in the same non-Clone sidecar bound to the
  retained skeleton, and reject
  missing/duplicate/foreign lanes, swapped or non-adjacent pairs, duplicate
  BindingRef publication, rebind, generation-only read, type drift, or
  caller/callee scope mixing. Any failure drops the unpublished skeleton,
  pending BindingSSA state, and sidecar together exactly once; no partial
  Builder/module publication, retry, or fallback is allowed.

Smallest next slice:
  `EXACT-TEXT-ENTRY-LANE-ADOPTION-I0` consumes the retained skeleton in one
  fresh unpublished function transaction. It installs the physical shell with
  a live Builder entry block, adopts ordinary lanes and one ExactText slot
  lane through the canonical identity/SSA owner, and stores the adjacent
  generation lane in the skeleton-bound sidecar. The same transaction owns
  discard/rollback; it creates no Loop block/operation/control/PHI, consumes
  no Completion claim, and issues no ReadyLoopEntry.

Acceptance:
  This D0 is accepted as a BoxShape because the same-cohort prepared parameter
  list, retained skeleton, canonical identity publisher, and existing one-shot
  function-session discard terminal describe the intended ownership spine. The
  caller-zero canary proves the positive install/adopt path and duplicate
  adoption rejection, but it does not yet prove that the skeleton, descriptors,
  common-V2 session, BindingSSA publication, sidecar, and discard path are one
  consuming owner. The next design stop therefore keeps the I0 from being
  treated as a complete atomic transaction until the `into_parts` split and
  partial-publication failure path are sealed.

Non-claims:
  No loop CFG/PHI, Completion consumption/claim, DraftSeal, lifecycle,
  PinnedTextOp, Text route, literal/StringBox origin, production caller,
  fallback, retry, or main integration is opened by this D0. The I0 remains
  caller-zero and unpublished; its positive canary is landed, but atomic
  same-cohort/session ownership remains a separate design stop.
```

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM`

```text
Decision:
  Accept the one consuming transaction BoxShape and open a caller-zero I0.
  Replace the public `PreparedPhysicalFunctionSkeleton::into_parts` seam with
  a compiler-only, non-Clone `PreparedPhysicalEntrySessionInputV1` that owns
  the retained loan, detached shell, descriptor rows, and one cohort stamp.
  A builder-side `with_common_v2_physical_entry_session` consumes that input
  together with the one-shot common-V2 admission and the fresh function
  session; no caller can retain a shell or descriptor slice for re-pairing.

Source authority + canonical issuer:
  The installed S6C HRTB loan remains the sole source/cohort owner. The entry
  input and skeleton issuers project the package-owned signature/header/effects
  rows; the compiler-only session input retains
  `PhysicalFunctionEntryCohortStampV1 { owner, selected_key,
  signature_identity, lane_count }`. The consuming seam compares that stamp
  and all storage/result/effects metadata with the same admission loan, then
  calls `CanonicalSsaFunctionSessionV2::new_common_v2`. The outer
  `CanonicalFunctionLoweringSessionV1` is the Builder transaction and its
  `discard_unpublished` terminal is the sole rollback owner.

Non-authority:
  A public `into_parts` result, detached `MirFunction`, bare descriptor slice,
  `MirParamDecl`, `FunctionSignature` length, raw `ValueId` order, logical
  `/N`, fixture names, `CanonicalSsaFunctionSessionV2::new`, and a copied
  Completion/sidecar cannot establish same-cohort physical entry meaning.

Fail-fast boundary:
  Before the first Builder effect, reject foreign/reordered skeleton or
  descriptor rows, owner/key/signature-identity drift, metadata/result/effects
  drift, lane/value/type/count drift, duplicate BindingRef publication, and
  any non-empty Builder function slot. If a later publish or adoption step
  fails, the consuming owner drops the canonical session and calls
  `discard_unpublished` exactly once. A rejected transaction leaves no current
  function, BindingSSA entry, sidecar, or module-visible state; no retry or
  fallback is permitted.

Smallest next slice:
  The caller-zero I0 is landed. Its next execution slice is
  `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0`: lend the accepted typed topology
  through the same operation/control/JoinSig cohort. Keep block/effect
  emission, Loop operation/control lowering, Completion claims, DraftSeal,
  lifecycle, Text lowering, route, and production caller at zero until that
  transport closes.

Implementation receipt (2026-08-17):
  `with_common_v2_physical_entry_session` consumes the prepared input and
  issues admission from its retained loan, installs the detached shell and
  source Binding authority in one fresh `CanonicalFunctionLoweringSessionV1`,
  adopts the slot-only BindingSSA plus generation sidecar once, and calls the
  outer `discard_unpublished` terminal exactly once on both success and late
  callback failure. No session, Builder view, descriptor slice, Completion,
  or sidecar escapes the callback; the focused positive and late-failure tests
  are green.

Non-claims:
  No common-V2 physical operation/control lowering, CFG/SSA/PHI beyond the
  entry reservation, Completion consumption/claim, DraftSeal, lifecycle,
  PinnedTextOp, route/perf, production caller, fallback, retry, or main
  integration is opened by this D0/I0.
```

### `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept retention of the existing PhysicalFunctionEntryCohortStampV1 as a
  mechanical same-cohort witness. Move it exactly once from the prepared
  physical skeleton/session input into CanonicalSsaFunctionSessionV2 before
  any callback-scoped physical consumer is exposed. Do not create a new
  semantic stamp, session nonce, or physical-result authority.

Source authority + canonical issuer:
  reserve_common_v2_physical_function_skeleton is the sole issuer of the
  stamp from the installed S6C loan and complete descriptor cohort. The
  stamp carries only owner, selected callable key, callable signature
  identity, and physical lane count. The consuming physical-entry session
  moves that stamp into the canonical session; CommonV2CanonicalSessionRefV1
  may lend only a scoped borrow to a later same-session materializer.

Non-authority:
  FunctionOwnerId alone, selected key alone, descriptor count, logical /N,
  MirFunction/FunctionSignature, raw ValueId/BasicBlockId, Builder cursor,
  copied or reconstructed stamp, a runtime/session nonce, or a second Port
  loan cannot establish cohort identity. The stamp is not an exit, result,
  Text, lifecycle, or source-semantic authority.

Fail-fast boundary:
  Reject missing or already-moved stamp, owner/key/signature/lane drift
  against the retained loan, callback exposure before attachment, foreign
  session borrow, stamp clone/re-pair, result-receipt value copying, and
  escape beyond the session callback. Late unpublished-function discard must
  drop the session-held stamp exactly once; no retry or fallback is allowed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-I0` removes the clone/
  discard seam, moves the stamp into the canonical session, adds one private
  scoped accessor through the common session wrapper, and tests positive
  retention plus missing/foreign/drift/late-discard negatives. It issues no
  physical condition result or ValueId.

Non-claims:
  No Compare lowering, physical Bool result, branch/edge effect, CFG/PHI,
  Completion/DraftSeal claim, lifecycle, Text/PinnedTextOp, route/performance,
  production caller, fallback, retry, or main integration.
```

The stamp is a cohort witness, not a unique invocation nonce. Its only valid
consumer is the same unpublished canonical session that consumed the prepared
entry input. A later condition-result receipt must borrow the session-held
stamp rather than copy it into a detached product.

### `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-I0` implementation receipt (2026-08-17)

The prepared physical-entry input is now move-only at the stamp boundary:
`take_install_parts` consumes the existing cohort stamp, and the consuming
physical-entry session attaches it before opening the outer Builder
transaction. `CanonicalSsaFunctionSessionV2` owns the stamp once, while
`CommonV2CanonicalSessionRefV1` exposes only a callback-scoped borrow. The
positive ExactText entry/session canary checks retained owner identity, and
the common-session canary checks the missing-stamp state before attachment.
`cargo test -q physical_entry --lib` (7 tests), `cargo test -q common_v2_ --lib`
(15 tests), format, diff, and pointer guards are green; the repository's
existing warning census is unchanged. No physical condition result, ValueId,
edge, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller
was opened.

### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-D0`

```text
Decision:
  Accept the V2-native physical-ID-free layout/placement BoxShape and open a
  caller-zero transport I0. The I0 only lends typed topology into the common
  envelope; it does not allocate blocks or emit effects.

Source authority + canonical issuer:
  The installed S6C loan's source-owned S6CLogicalOutputRowsV1 supplies typed
  loop/block/item topology, while the same cohort's existing JoinSig transfer
  view supplies control roles and ports. The existing S6C common-V2 issuer
  co-seals one non-Clone physical-layout input from those borrowed views and
  the operation/control/coverage siblings. The future session effect owner
  consumes this input; the outer CanonicalFunctionLoweringSessionV1 remains
  the sole rollback owner.

Non-authority:
  PreparedLoopPhysicalLayoutV1, fixed 13/15 counts, Recipe order alone,
  current_block, MirFunction.blocks, bare BasicBlockId/ValueId, V1 adapters,
  Dynamic topology, EffectMask, generic MirInstruction emission, and the
  entry block are not layout or placement authority.

Fail-fast boundary:
  Before any Builder effect, reject missing/foreign/reordered operation,
  control, JoinSig, segment, or transfer coverage; owner/target/stamp drift;
  duplicate or missing placement; and any attempt to infer layout from MIR.
  Before the transport callback returns, reject missing/foreign/reordered
  topology, item↔block drift, JoinSig role/port/loop mismatch, segment overlap,
  owner/target/stamp drift, or any physical-ID/MIR leakage. If a later
  allocation/effect is opened, every failure must use exactly one outer
  discard terminal with no retry, fallback, or publication.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0`: lend the source-owned typed
  topology through the existing installed HRTB/common-V2 envelope, co-seal it
  with operation/control/coverage, and add focused foreign/missing/duplicate
  and drift negatives. No Builder/session effect is allowed.

Census receipt (2026-08-17):
  `S6CLogicalOutputRowsV1` already retains loop/block/item rows, but the
  current common V2 envelope lends only operation rows, control rows, JoinSig
  transfer, and passive coverage. `LoopJoinLogicalTransferViewV2` intentionally
  carries ports/roles/payload, not Recipe block placement. The issuer is
  therefore a typed borrowed topology view, not a new count. The
  accepted layout shape is physical-ID-free `(loop, block, split ordinal)`
  segments with ordered item sets plus entry/After/resume and JoinSig transfer
  bindings. Allocation remains a later session effect; its I0 must decide
  whether the current CoreContext block cursor leaves an allowed ID gap on
  outer-session discard or receives an explicit transaction rollback.

Non-claims:
  No block allocation, operation/read/Const emission, CFG edge/terminator,
  PHI, Completion/DraftSeal, lifecycle, Text/PinnedTextOp, route/perf,
  production caller, fallback/retry, or main integration.
```

### Physical layout input I0 implementation receipt (2026-08-17)

`issue_s6c_common_v2_pre_session_v1` now co-seals a
`PreparedLoopV2PhysicalLayoutInputV1` from the same retained S6C ingress as
the operation/control/coverage siblings. The layout is physical-ID-free: it
contains source-owned loop/block keys, ordered item slices, checked split
ordinals, and the existing After binding. It validates owner, loop parent,
block ownership, item uniqueness, and After membership before the envelope is
returned. A second relation check proves that operation and If/Exit rows cover
the same disjoint item set, every referenced block belongs to the borrowed
topology, and every row item belongs to its specified block segment. The
focused operation/If/Exit block-drift negatives are green.

The new view is transported through the existing common-V2 envelope and is
accessible only inside the installed cohort loan. Focused common-V2 and
prephysical-ingress tests are green, including the foreign-owner rejection;
upstream source-row issuers retain the duplicate/missing topology negatives.
This I0 allocates no blocks, emits no operations/effects, opens no CFG/PHI or
Completion/DraftSeal claim, and does not select a Text route, fallback, retry,
or production caller. The post-layout effect frontier is now closed; this is a
historical receipt, not a live physicalizer or session reopen. The live
frontier is the physical condition-result receipt design stop below.

### Source-segment allocation boundary (landed 2026-08-17)

```text
Decision:
  Receipt: the source-segment/block skeleton allocation BoxShape was accepted
  and its caller-zero I0 landed. The first effect was not ReadBinding, Const,
  operation lowering, synthetic After allocation, or generic MirInstruction
  emission. After allocation was handled by its separate later D0.

Source authority + canonical issuer:
  The same common-V2 envelope's physical-ID-free layout is the topology input
  for source segments. Its retained JoinSig transfer view remains a logical
  transfer input, not an issuer of a new physical After block. A
  canonical-session block allocator is the sole physical block issuer, and the
  outer CanonicalFunctionLoweringSessionV1 is the sole unpublished-function
  rollback owner.

Non-authority:
  V1 physical layout/adapter, Recipe 13/15 counts, split ordinals alone,
  current_block, MirFunction.blocks, raw BasicBlockId/ValueId, EffectMask,
  the JoinSig After port alone, the After binding/class alone, synthetic
  After inference, generic MIR emission, ReadBinding, and a second rollback
  owner do not issue the allocation plan or its physical receipt.

Fail-fast boundary:
  Before Builder effect, require same owner/layout/session stamp, complete
  ordered segment coverage, checked block-count/cursor range, and no collision
  with the existing function entry. Allocate only source segments in layout
  order. Any late failure performs one outer discard with no retry, fallback,
  or publication. This D0 adopts the monotonic unpublished-ID-gap policy:
  discard restores function/CFG state but does not rewind CoreContext's global
  cursor; the gap is unobservable and never reused. No ambient cursor inference
  is allowed.

Smallest next slice:
  The existing source-backed V2 layout seam is sufficient for a segment-only
  plan. Open LOOP-COMMON-V2-PHYSICAL-SEGMENT-BLOCK-ALLOCATION-I0: consume one
  plan in the same callback, allocate only private segment->BasicBlock rows,
  preflight the checked cursor range, and test late discard. Then open a separate
  LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0 for a source-backed synthetic After
  row and its allocation owner. If the segment plan's owner/coverage/range
  checks fail, reject before effect.

Non-claims:
  No ReadBinding/Const/CallSlot/Text operation, synthetic After block,
  edges/terminators, CFG/PHI, Completion/DraftSeal, lifecycle, route/perf,
  production caller, fallback, retry, or main integration is opened by this
  D0.
```

### Segment block allocation I0 implementation receipt (2026-08-17)

`LOOP-COMMON-V2-PHYSICAL-SEGMENT-BLOCK-ALLOCATION-I0` is landed as a
caller-zero effect slice. The existing physical-ID-free layout now issues one
callback-scoped `PreparedLoopV2SegmentAllocationPlanV1`; the canonical session
consumes it and allocates exactly one unpublished `BasicBlockId` per ordered
source segment. The receipt retains only the source loop/block/split relation
and the newly allocated physical block; it does not issue edges, terminators,
operations, effects, or a synthetic After block.

Allocation preflights owner/function identity, checked count and cursor range,
entry collision, and segment coverage before mutation. The surrounding
`CanonicalFunctionLoweringSessionV1` remains the sole discard owner. A late
callback error discards the unpublished function while the CoreContext block
cursor remains monotonic; the resulting ID gap is unpublished, unobservable,
and never reused. Focused positive and late-discard tests are green.

This receipt does not open synthetic After allocation, edge/terminator or
operation lowering, ReadBinding/Const, CFG/PHI, Completion/DraftSeal,
lifecycle, Text, route/performance, fallback/retry, publication, or a
production caller. The next design stop is the separate
`LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0`, which must obtain a source-backed
resume relation before any After block can be allocated.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0` — accepted BoxShape

```text
Decision:
  Accept a source-backed After-boundary relation, but keep After block
  allocation closed.  The segment-only allocation I0 is complete; a
  synthetic After block is authorized only after this typed relation is
  transported.  The current S6C cohort admits RootAfter only; ParentResume
  remains a future source-backed arm until its issuer input exists.

Source authority + canonical issuer:
  One resolver/source semantic handoff retains the owner, function/frame,
  root/parent loop forest, JoinSig After, and source segment membership.  Its
  common issuer produces a callback-scoped
  `VerifiedLoopV2AfterBoundarySourceRelationV1` with a typed `RootAfter` or
  `ParentResume` relation, exact source membership, owner/frame stamp, and a
  later fresh-block allocation policy.  Existing JoinSig/layout issuers are
  inputs only; neither is extended to guess physical meaning.

Non-authority:
  JoinSig's `(loop, binding, class)`, parent index alone, segment order,
  split ordinal, Recipe/JoinSig counts, old V1 layout, nested-predicate
  topology fixture, MirFunction/Builder/BasicBlockId, current cursor,
  EffectMask, and copied source-site arrays cannot issue root/resume meaning
  or an After block.

Fail-fast boundary:
  Before any Builder effect, require one owner/cohort/frame, one root loop,
  one typed `RootAfter | ParentResume` relation, layout/transfer parity, and
  exact source-segment membership. Missing, foreign, ambiguous, nested-drift,
  duplicate-boundary, unsupported ParentResume, or HRTB-escape cases reject;
  no second After authority or fallback is permitted.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-I0` issues and transports the typed
  relation through the existing common-V2 envelope only.  After block
  allocation remains a later effect slice owned by the canonical session and
  outer discard owner.

Non-claims:
  No After BasicBlock allocation, rollback/cursor change, edge/terminator,
  operation/ReadBinding/Const/CallSlot, CFG/PHI, Completion/DraftSeal,
  lifecycle, Text, route, performance, production caller, fallback, retry, or
  publication.
```

### After-boundary transport I0 implementation receipt (2026-08-17)

`LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-I0` is landed as a transport-only
slice. The same S6C ingress that issues the physical-ID-free layout now issues
one non-Clone `VerifiedLoopV2AfterBoundarySourceRelationV1` carrying the
owner, source loop/frame evidence, and typed `RootAfter | ParentResume`
disposition. The admitted S6C path is `RootAfter`; `ParentResume` remains
parked until its source issuer input exists. The relation is retained inside
the existing common-V2 envelope, with focused RootAfter/owner parity evidence
and no second loan or JoinSig reissue.

This receipt performs no After block allocation, cursor/rollback change,
edge/terminator or operation emission, CFG/PHI, Completion/DraftSeal claim,
lifecycle, Text, route, fallback, retry, publication, or production caller.
The next bounded design stop is
`LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0`.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0` — accepted BoxShape

```text
Decision:
  Accept one synthetic After allocation effect after the typed source-backed
  boundary is transported. Keep allocation separate from edges, operations,
  CFG/PHI, Completion, and session construction.

Source authority + canonical issuer:
  Consume the existing VerifiedLoopV2AfterBoundarySourceRelationV1 and the
  source-segment allocation receipt from the same common-V2 envelope/session
  scope. A compiler-side PreparedLoopV2AfterAllocationPlanV1 is the one-shot
  plan issuer. CanonicalSsaFunctionSessionV2::create_unpublished_block is the
  sole BasicBlockId issuer; the outer unpublished-function transaction is the
  sole discard owner.

Non-authority:
  JoinSig After tuple, layout order, segment split ordinal, current Builder
  cursor, MirFunction blocks, Recipe counts, BasicBlockId, ValueId, ParentResume,
  or a profile-specific After physicalizer cannot infer allocation or a
  successor. A raw block receipt cannot be reacquired or paired with another
  relation after the plan is consumed.

Fail-fast boundary:
  Before Builder effect require one owner/function/relation stamp, RootAfter
  disposition, exact source-segment coverage, one unconsumed allocation slot,
  no entry/segment collision, and checked monotonic cursor range. The receipt
  is session-scoped and cannot escape the callback. Late failure must discard
  the unpublished function once; cursor gaps remain non-semantic and no
  retry/fallback is allowed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0`: consume one prepared plan,
  allocate exactly one unpublished After block, and return only a
  session-branded `PreparedAfterBlockViewV1` inside the callback. Add no edges,
  terminators, operations, CFG/PHI, Completion/DraftSeal, lifecycle, Text,
  route, fallback, retry, or production caller.

Non-claims:
  No ParentResume admission, physical successor choice, operation/ReadBinding,
  effect emission, session publication, route selection, or legacy retirement.
```

The accepted BoxShape is intentionally a one-shot placement effect rather than
a new topology owner. The plan must be derived from the already-issued typed
After relation plus the already-issued source-segment receipt; it may not
rescan Recipe/layout facts or use the Builder cursor as meaning. The returned
physical block view is unpublished and callback-scoped. `create_unpublished_block`
advances the monotonic cursor; a late outer discard may leave an unused numeric
gap, which is non-semantic and is never reused.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0` — landed 2026-08-17

```text
Change:
  Add one caller-zero allocator for a RootAfter-only prepared plan. The plan is
  issued and consumed inside the common session, allocates one unpublished
  block through the canonical session allocator, and exposes only a
  session-branded view.

Contract:
  same owner/function/relation stamp
  RootAfter only; ParentResume = 0
  source-segment receipt is same-scope and complete
  one allocation slot -> one BasicBlockId
  outer function transaction owns every discard
  monotonic cursor gaps are non-semantic

Done:
  positive one-block allocation
  duplicate allocation rejection through the session-local one-shot state
  RootAfter and exact segment-coverage preflight
  cursor range/collision preflight with no mutation
  late callback failure leaves no published function/module
  callback-scoped view cannot escape or be consumed twice
  focused physical-entry gate, README/reference receipt, pointer guard

Stop:
  ParentResume, successor/edge/terminator, operation or ReadBinding emission,
  CFG/PHI, Completion/DraftSeal, lifecycle/Text, route/performance,
  production caller, fallback/retry, and legacy retirement.
```

Implementation receipt: the new `common_v2_after_block_allocation` module is
216 lines and keeps the prepared plan private to the common session. The
canonical CFG allocator issues the one physical block; no generic Builder
allocator or profile-specific physicalizer is introduced. Existing focused
physical-entry tests cover positive allocation, one-shot rejection, and late
discard after the After block has already been created.

The next design stop is a separate source-backed successor/edge decision. This
I0 does not make the new block reachable or choose a resume target.

The current layout view still carries only source loop/block/item segments and
an After binding, so it remains a transport input rather than an After
authority. The accepted boundary BoxShape names the missing source-backed
relation explicitly: the next I0 must issue and transport a typed
`RootAfter | ParentResume` relation from the resolver/source handoff. Current
S6C admits only `RootAfter`; `ParentResume` stays parked until its source
issuer input exists. No After block or physical resume target is inferred from
the JoinSig tuple, `EffectMask`, or Recipe order. `ReadBinding` stays a later
sibling: its source `BindingRef`, source-site, and Core effect anchor are not
present in the physical-ID-free layout rows and must not be invented.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-EDGE-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept a complete source-backed predicate branch plan and condition-carrier
  requirement as the next transport boundary, while keeping the newly
  allocated RootAfter block unreachable.
  The logical shape is the root predicate's Header -> Body / Header -> After
  pair, with PredicateFalse targeting RootAfter. A false-only edge plan is
  invalid because the canonical CFG branch issuer requires both successors.

Source authority + canonical issuer:
  The resolver-backed S6C logical loop condition (`condition_block` and
  `condition_value`), the existing `LoopJoinBoundaryTransferRefV2` predicate
  boundary rows, the
  source physical segment receipt, and the typed RootAfter relation are the
  inputs. The common-V2 source issuer co-seals the condition logical value, its
  condition segment, PredicateTrue -> Body, PredicateFalse -> RootAfter,
  owner/frame stamps, and a future physical condition carrier requirement in
  one same-session scope. The resulting plan is physical-ID-free and
  callback-scoped; CanonicalSsaFunctionSessionV2 remains the sole physical
  branch/edge issuer after a later effect slice consumes it.

Non-authority:
  JoinSig tuple alone, Recipe order, segment ordinal, current cursor,
  BasicBlockId, MirFunction predecessor lists, EffectMask, a copied source-site
  array, an already allocated After view, or a legacy V1 physical condition
  receipt may not infer a successor or condition carrier. Tail, Completion,
  ParentResume, operations, and generic CFG repair remain outside this
  boundary.

Fail-fast boundary:
  Before any edge effect require one owner/function/frame relation, RootAfter
  disposition, exact branch row and source-segment coverage, both Body and
  After targets, a source-backed condition carrier admission, and unused edge
  slots. Missing/duplicate/foreign rows, nested or outer-loop drift, absent
  condition projection, false-only planning, HRTB escape, late failure, or a
  second edge owner reject and invoke the outer unpublished-function discard
  exactly once; no retry or fallback.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-BRANCH-PLAN-I0` must transport one typed,
  callback-scoped complete branch plan and condition-carrier requirement from
  the same S6C cohort. It must not issue a ValueId, call `emit_branch`, mutate
  CFG, or add operations, ReadBinding, Completion/DraftSeal, lifecycle, Text,
  route, or production. A later edge-effect D0/I0 may consume the plan only
  after the condition physical carrier is itself closed.

Non-claims:
  No ParentResume admission, physical condition ValueId, `emit_branch` or
  `emit_jump`, edge/terminator implementation, operation lowering, PHI,
  Completion claim, DraftSeal, lifecycle, Text, route/performance, fallback,
  retry, publication, or production caller.
```

The After allocation I0 remains the only landed effect: it creates one
unpublished block and does not make it reachable. The source condition and both
logical branch rows already exist in the same installed S6C ingress. The
missing physical condition `ValueId` is deliberately represented as a future
carrier requirement rather than guessed or issued here. `CanonicalCfgSessionV1::emit_branch`
is one atomic two-successor operation; a false-only edge is never a valid
intermediate product.

### Branch-plan transport I0 implementation receipt (2026-08-17)

`common_v2_predicate_branch_plan.rs` now issues the accepted transport product
from the same installed S6C ingress. The non-`Clone`
`PreparedLoopV2PredicateBranchPlanV1` retains the resolver Bool condition, its
condition segment, the `Header -> Body` and `Header -> RootAfter` logical
targets, and a future condition-carrier requirement. It contains no physical
IDs and performs no CFG mutation. The envelope lends it through the existing
exactly-once HRTB; the focused installed-loan test checks owner, Bool class,
Body, and RootAfter. Unit negatives reject a missing or duplicate predicate
boundary. The next design stop is the physical condition-result BoxShape; no
edge-effect slice is opened until that carrier has its own session receipt.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-CARRIER-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept a two-stage condition carrier boundary. First, the same-cohort
  common issuer co-seals the exact source-backed CompareI64 producer relation
  with the logical branch requirement. Later, an operation materializer issues
  a session-local physical result receipt; no physical carrier is issued by
  this D0 or by the next transport-only I0.

Source authority + canonical issuer:
  The resolver-sealed S6C logical loop condition and its exact CompareI64
  producer row are the source spine. The common-V2 issuer must co-seal the
  producer item/block, `Less` operation, left/right logical operands, result
  key, Bool class, root loop, and owner/frame stamp. A later operation
  materializer uses operand physical receipts and
  `CanonicalSsaFunctionSessionV2` as the sole ValueId/type issuer; the branch
  consumer only borrows that physical receipt.

Non-authority:
  `PreparedLoopOperationProgramV2` result keys alone, Recipe order, MIR
  `ValueId`, `EffectMask`, legacy V1 Compare emitters, block cursor, or the
  branch plan may not infer the physical carrier. The condition requirement is
  logical evidence, not a physical value receipt.

Fail-fast boundary:
  Missing/duplicate/non-Compare producer, operand/result/block/class drift,
  foreign session/owner, stale or duplicate physical result, and Body/RootAfter
  target drift reject before `emit_branch` or any edge mutation. A producer
  must be materialized before the branch consumer can borrow its carrier.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-PRODUCER-I0` transports one typed
  producer relation from the same envelope and tests missing/duplicate,
  non-Compare, operand/result/block/class, and owner drift. It opens no
  operation lowering, ValueId issuance, `emit_branch`, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production
  caller.

Non-claims:
  No physical condition ValueId, operation emission, edge/terminator, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text route, performance, fallback/retry, or
  production caller is accepted by this D0.
```

### Condition producer relation I0 implementation receipt (2026-08-17)

`common_v2_condition_producer.rs` now issues one private, non-`Clone`
`PreparedLoopV2ConditionProducerRelationV1` from the same installed S6C
ingress as the predicate branch plan and operation program. It checks the
resolver loop condition against exactly one source `CompareI64` row, requires
the fixed `Less` producer and `I64` operands, and then checks the matching
generic operation row, owner, block, result, and Bool class before the envelope
is returned. The relation retains only logical keys and the producer item; the
canonical session remains the sole future `ValueId`/type issuer.

The focused common-V2 suite is green with the new non-Compare operation-row
drift negative and the installed-loan positive assertions. This I0 does not
issue a physical result, lower a Compare, call `emit_branch`, mutate CFG/PHI,
claim Completion/DraftSeal, open lifecycle/Text/route/performance, or add a
production caller. The next design stop is the physical result BoxShape, not a
second condition or edge authority.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-D0` — parent design stop 2026-08-17

```text
Decision:
  Keep this D0 at NoSafeSlice. A physical condition result may be issued only
  by one common-V2 operation materializer after it has borrowed the logical
  producer relation and exact physical operand receipts from the same
  canonical session. The materializer then uses the canonical session's sole
  ValueId/type issuer and returns one session-scoped Bool result receipt to the
  later branch consumer. This D0 does not issue or lower that result.

Source authority + canonical issuer:
  The source producer is `PreparedLoopV2ConditionProducerRelationV1`. Physical
  SSA/type authority remains `CanonicalSsaFunctionSessionV2`; the future
  materializer is only the co-seal bridge. The receipt must retain owner, a
  scoped borrow of the session-held physical-entry stamp, producer
  item/block/result key, and Bool type relation. Session-stamp retention is now
  landed; the result seam may borrow it but may not copy or reconstruct it.

Operand issuer census (2026-08-17):
  A source-backed two-row inventory is now carried by the common V2 envelope:
  Left is the condition-block ReadBinding and Right is the condition-block
  Length CallSlot. This closes source provenance only. The left still lacks a
  LoopValueKey/session-stamp physical read receipt, and the right now has an
  unpublished canonical Length Call/result canary whose full session lifetime
  is not yet sealed. `PreparedLoopOperationProgramV2`
  retains source rows only; verification definition maps are transient; old V1
  and Selected-Dynamic value ledgers are foreign authorities. Therefore both
  operands still cannot be borrowed as one same-session typed physical pair.

Smallest source-backed relation candidate:
  `PreparedLoopV2LengthOperandProducerRelationV1` is the only narrow candidate
  currently justified. The canonical issuer is the existing
  `issue_s6c_common_v2_pre_session_v1` source-ingress callback, which must
  co-seal the Length source contract (`StringLen`, Condition placement, arity
  zero), its `CallSlot` row/result/class, the matching Common V2 operation row,
  and the CompareI64 right key. It is a physical-demand relation, not a second
  language-semantic value authority; raw `LoopValueKeyV1`, generic CallSlot,
  role numbers, and later MIR lookup remain non-authorities.

  The relation and its fixed two-row inventory are issued with
  owner/placement/item/block/result/class coverage and foreign/duplicate/drift
  rejection. The direct Length canary is a same-session effect witness, not a
  freely re-pairable operand receipt; the left read and the full Length receipt
  lifetime must close before Bool materialization.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0` — design stop 2026-08-17

```text
Decision:
  Keep the parent result at NoSafeSlice and fix one thin, session-local
  materialization shape before any Compare or branch effect. The future
  `PreparedLoopV2PhysicalConditionResultPlanV1` is a physical-demand plan, not
  a semantic result: it borrows the source CompareI64 relation, the fixed
  two-row operand inventory, and the session-held cohort stamp. Its eventual
  `PreparedLoopV2ConditionResultReceiptV1<'session>` contains exactly one
  canonical Bool result ValueId plus its published Bool type and is valid only
  inside the same unpublished canonical session callback.

Source authority + canonical issuer:
  `PreparedLoopV2ConditionProducerRelationV1` and
  `PreparedLoopV2ConditionOperandInventoryV1` remain the source authorities.
  `CanonicalSsaFunctionSessionV2` remains the sole ValueId/type issuer; one
  future common-V2 condition materializer is the only bridge allowed to consume
  the two source rows, resolve the Left binding through canonical identity,
  consume the full session-scoped Length result, emit the CompareI64 `Less`,
  and issue the Bool receipt. The complete stamp is borrowed from the session
  wrapper and never copied into a detachable receipt.

Non-authority:
  raw LoopValueKey, operation result key, raw ValueId, caller-supplied
  `MirType`, `EffectMask`, old V1/Selected-Dynamic value ledgers, generic
  CallSlot lookup, Recipe order, branch-plan condition key, or a standalone
  `emit_compare_i64_at` call cannot issue the result. The Length call's
  temporary physical result is private to the same materializer; it is not a
  second public operand authority.

Fail-fast boundary:
  Reject before physical mutation on missing/foreign session stamp, owner or
  function drift, producer item/block/result/op drift, missing or duplicate
  operand rows, Left binding read failure, absent/incorrect Length call result,
  non-`I64` operands, non-`Less` op, duplicate result publication, pre-existing
  result ValueId/type, materializer re-entry, branch-consumer mismatch, or
  receipt escape. Late failure uses the existing outer unpublished-function
  discard exactly once; no local rollback, fallback, or retry.

Acceptance boundary:
  This historical BoxShape is superseded by the ordered lifetime row below.
  The same-session direct Length issuer now exists as an unpublished canary;
  acceptance remains blocked until its full session stamp/lifetime is carried
  by the receipt. Do not infer that lifetime from `stamp_owner`, raw MIR, or a
  legacy CallSlot emitter.

After acceptance:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-I0` may begin as a typed
  materializer/receipt-admission canary. It still emits no ValueId, Compare,
  edge, or terminator until its own BoxCount is opened.

Non-claims:
  No ValueId issuance, Compare instruction, call lowering, branch/edge,
  `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened by this D0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept only a fixed two-row, physical-ID-free operand inventory for the
  current S6C predicate. Row Left is the condition-block ReadBinding producer;
  row Right is the condition-block Length CallSlot producer. The inventory is
  not a ValueId ledger and does not issue a physical result.

Source authority + canonical issuer:
  The existing source-ingress issuer
  `issue_s6c_common_v2_pre_session_v1` co-seals the typed Length source call,
  its CallSlot row/result/class, the matching common operation row, and the
  Compare producer relation. The same ingress supplies the ReadBinding source
  relation. This is a projection of existing authority, not a new semantic key
  space.

Non-authority:
  raw LoopValueKey, role numbers, generic CallSlot, transient verifier maps,
  CanonicalBindingReadReceiptV1 alone, raw ValueId, old V1/Selected-Dynamic
  ledgers, and later MIR lookup cannot issue or re-pair either row.

Fail-fast boundary:
  owner/cohort, producer item/block, result/class, Length role/op/placement/
  arity, receiver/argument shape, Compare-right relation, duplicate/missing
  row, detached provenance, or loan escape rejects before any physical value,
  instruction, session mutation, or edge effect.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` transports
  this non-Clone two-row inventory through the existing callback only.

Non-claims:
  no ValueId, Compare instruction, call lowering, branch, CFG/PHI,
  session-stamp retention, lifecycle, Text, route, fallback, retry, or
  production caller.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` — landed 2026-08-17

The I0 now issues/transports the accepted inventory through the existing
callback only. It adds no physical operand, ValueId, Builder, or session
effect. Focused positive, foreign-owner, and Length-operation-drift negatives
are green. The parent Result D0 remains the sole later authority for the
physical Bool result.

Non-authority:
  `LoopOperationV2` rows, branch-plan condition keys, raw `ValueId`, legacy V1
  Compare emitters, block cursors, MIR type maps, or a copied session stamp may
  not issue or re-pair the result. The branch plan is only a consumer of the
  future receipt; CFG/SSA/PHI ownership stays in the canonical session.

Fail-fast boundary:
  Missing same-session operand receipts, producer/item/block/result/op/class
  drift, owner/session/function-stamp drift, use-before-producer, duplicate
  materialization or receipt, missing Bool type publication, and receipt escape
  reject before `emit_branch` or any edge mutation. Late failure uses the outer
  unpublished-function discard exactly once; monotonic unissued ValueId gaps
  are non-semantic and no local rollback/retry or fallback is allowed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` is now the next
  design stop. Session-stamp retention is landed, but a canonical same-session
  Length result issuer is still missing; the parent Bool result must remain
  unaccepted until that seam is named.

Ordered sub-slices (design-only; no session/CFG effect):
  1. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0`
     and its `...OPERAND-INVENTORY-I0` transport are landed. They issue only
     the fixed source-backed logical rows; no physical receipt is issued.
  2. `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` and its I0 are
     landed; the prepared skeleton and installed loan remain the only stamp
     authorities.
  3. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` and its I0
     canary are landed; the direct target/receiver/Call/result I0 is now also
     landed, but its full session lifetime remains unsealed.
  4. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-D0`
     and its I0 are landed; the receipt now owns the same-session lifetime.
  5. `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-D0` is accepted
     below; it fixes the one same-session Bool materializer and later branch
     consumer.
  6. `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-D0` is the current design
     stop; the Bool Compare/result canary remains blocked until its source
     initializer relation and same-session declaration/value are sealed.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept one thin same-session boundary for the Length CallSlot result before
  the parent Bool result. The future physical result remains unimplemented;
  the first accepted consumer is only a private, non-Clone, one-shot
  `LengthCallMaterializationCanaryV1` that carries source/stamp provenance and
  issues no ValueId, type, CallSlot, CFG, or edge effect.

Source authority + canonical issuer:
  The existing S6C Length source contract, fixed Right operand row, matching
  common operation row, and Compare-right relation are the only source inputs.
  A future common-V2 materializer must consume them inside the same canonical
  session callback and use only the session's
  `issue_physical_value_id` / `publish_physical_value_type` mechanics. The
  `CommonV2CanonicalSessionRefV1` is the only bridge and the canonical session
  remains the mechanical ValueId/type issuer. The direct Call/result issuer
  and its caller-zero canary are recorded below; this earlier D0 remains a
  no-effect source/stamp admission and does not itself publish the physical
  result.

Non-authority:
  Generic `LoopOperationV2::CallSlot`, raw `LoopValueKeyV1`, raw `ValueId`,
  `MirType`, selected-Dynamic/legacy CallSlot emitters, CheckedCallOut
  projections, Recipe order, or a later MIR lookup may not issue or re-pair
  the Length result. No existing emitter may be renamed as the common owner.

Fail-fast boundary:
  Reject before physical mutation on missing/foreign Length role, non-
  `StringLen`, wrong Condition placement or zero-arity shape, receiver/args/
  result/class drift, producer or session owner/stamp drift, duplicate or
  re-entered materialization, missing canonical result publication, and any
  loan/receipt escape. Late failure uses the outer unpublished-function
  discard exactly once; local rollback, fallback, and retry remain forbidden.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` consumes the
  relation, inventory, and session stamp once inside the existing callback and
  proves duplicate/foreign/drift rejection while leaving Builder state intact.
  After this canary, the direct Call/result I0 and its full receipt-lifetime
  D0 are the next ordered boundary; no published physical Length result is
  claimed by this source/stamp canary.

Non-claims:
  No ValueId issuance, CallSlot lowering, Compare instruction, branch/edge,
  `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is admitted by this D0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` — landed caller-zero canary 2026-08-17

```text
Decision:
  Consume one source-backed Length relation, two-row operand inventory, and
  borrowed physical-entry stamp inside the existing common-V2 session callback
  exactly once. The canary is protocol state only: it emits no ValueId, type,
  CallSlot, instruction, CFG, edge, or semantic result.

Source authority + canonical issuer:
  `CommonV2CanonicalSessionRefV1` owns the callback-scoped consumer seam;
  envelope accessors lend the existing S6C relation/inventory and the session
  lends the existing cohort stamp. No source fact is reissued by the canary.

Non-authority:
  raw logical keys, raw ValueId, Builder maps, MIR type maps, generic
  CallSlot rows, selected-Dynamic/legacy emitters, and a second session are
  forbidden inputs and cannot be used to prove success.

Fail-fast boundary:
  Missing/foreign/duplicate relation, inventory or stamp, owner/function drift,
  wrong Length role/operation/placement/arity/receiver/result/class, re-entry,
  receipt escape, or late callback failure rejects with Builder unchanged.
  The outer unpublished-function transaction remains the only discard owner.

Acceptance:
  Positive same-cohort consumption, duplicate one-shot rejection, missing
  stamp, source-shape validation, and late-failure no-mutation tests are green.
  The parent Bool result BoxShape is now the next design stop for physical
  materialization.

Non-claims:
  No physical Length result, CallSlot lowering, Compare, branch/edge,
  `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is admitted by this I0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` — landed caller-zero 2026-08-17

```text
Decision:
  Land one source-backed, non-physical StringLen target plan in the existing
  canonical-session callback. The one-shot plan is a pure admission product;
  it does not create a MIR Call, ValueId, type, or physical result.

Source authority + canonical issuer:
  `CommonV2CanonicalSessionRefV1` retains the installed S6C envelope and the
  moved physical-entry stamp. Its issuer consumes the existing Length source
  relation, CallSlot row, condition operand inventory, and CoreMethod target
  facts to issue `PreparedLoopV2StringLenCallTargetPlanV1` exactly once.
  The plan records only target/receiver/zero-args/I64/PureRead/non-suspending
  facts; canonical session remains the future physical ValueId/Call issuer.

Non-authority:
  Raw logical keys, raw ValueId, `CoreMethodOp` alone, canonical spelling,
  `/N`, MIR/JSON, generic/legacy CallSlot, CheckedCallOut, Selected-Dynamic
  ledgers, host/default target data, and a second session cannot issue or repair
  the plan.

Fail-fast boundary:
  Missing stamp, owner/brand/block/placement drift, wrong receiver/args/result/
  class/effect/policy, malformed source CallSlot, duplicate plan issuance, or
  late callback failure rejects before physical mutation; the outer unpublished
  function transaction remains the sole discard owner.

Acceptance:
  Same-cohort target facts, canonical StringBox.length, plan/canary parity,
  duplicate one-shot rejection, missing-stamp rejection, and late-discard
  no-mutation tests are green. Source and builder modules remain under the
  760/800-line limits.

Non-claims:
  No canonical Length Call/result receipt, parent Bool result, Compare,
  branch/edge, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened.
```

The canary is landed without opening a physical result. The next bounded
decision is `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0`:
one same-session physical Length-result receipt issuer must be named before
the parent Bool plan/receipt can be admitted.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept one non-physical target-realization plan for the S6C StringLen row.
  This closes the source-to-target shape only; it does not emit a Call or
  issue a ValueId. The plan is the only input allowed to the later canonical
  Length-call materializer.

Source authority + canonical issuer:
  One same-cohort callback borrows `S6CLogicalCallInputRefV1`, its verified
  CoreMethod target facts, the fixed Length CallSlot row, the two-row condition
  operand inventory, and the retained physical-entry stamp. A resolver/recipe
  seam issues one non-Clone
  `PreparedLoopV2StringLenCallTargetPlanV1` containing owner/item/block,
  target/manifest brands, canonical StringBox method target, receiver relation,
  zero argument shape, I64 result relation/class, PureRead effect, and the
  non-suspending/non-control policy. It owns no physical IDs.

Non-authority:
  `CoreMethodOp::StringLen` alone, a canonical method string, `/N`, raw
  `LoopValueKeyV1`, `MirInstruction::Call`, generic/legacy CallSlot,
  Selected-Dynamic ledgers, CheckedCallOut, host/default target data, or a
  second session cannot issue or repair this plan.

Fail-fast boundary:
  Foreign owner/function/target/manifest brand, wrong placement or item/block,
  receiver/args/result/class/effect/policy drift, missing or duplicate target
  facts, missing inventory/stamp, target-plan re-entry, and loan/plan escape
  reject before Builder/session physical mutation. No selector, method name,
  receiver ValueId, or result ValueId may be reconstructed from MIR/JSON.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` issues and
  consumes this plan once in the existing callback, with positive mixed-cohort
  coverage and foreign/duplicate/drift/late-failure no-mutation tests. It
  leaves the outer Builder unpublished and emits no Call, ValueId, type,
  Compare, edge, terminator, CFG, or PHI.

Non-claims:
  No canonical Length Call/result receipt, parent Bool result, Compare,
  branch/edge, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened.
```

### `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Add one mechanical projection from the already allocated common-V2 segment
  receipt to the physical block that owns the logical condition block. This is
  required before a Length Call can be placed; it does not issue a Call or a
  ValueId.

Source authority + canonical issuer:
  `allocate_v2_segment_blocks` remains the sole BasicBlockId issuer. The same
  `CommonV2CanonicalSessionRefV1` creates the segment receipt, finds the exact
  condition-block row from the envelope layout, and lends a callback-scoped
  `ConditionBlockPhysicalTargetRefV1`. The projection carries owner, logical
  condition block, physical block, and the retained entry-session stamp. It is
  a mechanical view, not a second layout or semantic authority.

Non-authority:
  Builder current block, block cursor arithmetic, logical `LoopBlockKeyV1`
  alone, `BasicBlockId` guesses, owner equality alone, copied segment rows,
  After allocation, target-plan facts, and a receipt imported from another
  session cannot select the condition block.

Fail-fast boundary:
  Missing/duplicate condition row, foreign owner, layout/segment drift,
  missing retained stamp, After-row confusion, callback escape, or late
  failure rejects before the target is lent. The generated segment blocks stay
  unpublished and the outer function transaction remains the sole discard
  owner; Call, ValueId, Compare, edge, and PHI effects are still closed.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-I0` uses a same-session
  callback to allocate the existing source segments and lend exactly one
  condition-block target. Positive, missing/duplicate-layout, foreign/late
  discard, and target-escape negatives are required.

Non-claims:
  No Length Call/result receipt, receiver ValueId, Compare, branch/edge,
  terminator, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route,
  performance, production caller, fallback, or retry is opened.
```

The condition-block target must be closed before the Length materializer can
be admitted. It is deliberately callback-scoped so a segment receipt from a
different physical session cannot be re-paired by owner or block equality.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIVER-OPERAND-D0` — accepted BoxShape 2026-08-18

```text
Decision:
  Split the Length bridge before opening a Call effect. This first BoxShape
  issues only one callback-scoped physical receiver operand receipt from the
  canonical identity/SSA read seam. It consumes the source-backed StringLen
  receiver relation, the fixed operand inventory, the callback-scoped physical
  condition-block target, and the retained physical-entry stamp. It emits no
  Call and no result ValueId.

Source authority + canonical issuer:
  The resolver CoreMethod callable contract is the source authority for the
  exact `ResolvedLexicalRefV1::Local(BindingRefV1)` receiver relation. The
  existing condition-inventory issuer projects that relation as a private
  `LengthReceiverBindingRefV1` view after its source `verify_call` proves the
  receiver equals the subject binding. This is mechanical transport, not a
  second semantic issuer. The target plan, operand inventory, condition-block
  target, and stamp are the already-sealed projections from the same S6C
  cohort. The canonical session identity/SSA read seam is the sole physical
  issuer of the existing `CanonicalBindingReadReceiptV1` (or a typed,
  callback-scoped Length receiver view). No raw ValueId map, second session,
  or legacy CallSlot path may issue or retain the operand.

Non-authority:
  Raw LoopValueKey/ValueId, `CoreMethodOp::StringLen` alone, method spelling,
  the CallSlot row alone, current Builder block/cursor, MIR lookup,
  `EffectMask` alone, CheckedCallOut, Selected-Dynamic/legacy emitters, a
  copied condition target, or a second canonical session cannot resolve or
  retain the receiver operand.

Fail-fast boundary:
  A non-local or missing resolver receiver, missing/foreign projected binding,
  owner/target/manifest/stamp drift, condition-block or operand mismatch,
  unavailable canonical read receipt, wrong binding/type, duplicate issuance,
  receipt escape, or late failure rejects before any Call or result effect.
  The outer unpublished-function transaction remains the sole rollback owner;
  fallback/retry is forbidden.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIVER-OPERAND-I0`
  consumes the same-session `LengthReceiverBindingRefV1` projection and
  lends exactly one canonical read receipt/view. Positive, non-local/missing
  receiver, type/owner drift, duplicate/re-entry, foreign target,
  callback-escape, and late-discard tests are required. No Call is emitted by
  this slice.

Non-claims:
  No Call, I64 result receipt, parent Bool receipt, Compare instruction,
  `emit_branch`, edge/terminator, CFG/SSA/PHI beyond the existing read,
  Completion/DraftSeal, lifecycle, Text, route, performance, production
  caller, fallback, or retry is opened by this BoxShape.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Admit the Length Call only as a second bounded product. A same-session
  `CanonicalLengthCallDirectEmitterV1` consumes the prepared target plan,
  the callback-scoped receiver operand receipt, the fixed operand inventory,
  the physical condition-block target, and the retained stamp. It emits one
  direct `StringBox.length` Call and one I64 result receipt, with no parent
  Bool or edge effect.

Source authority + canonical issuer:
  The first receiver-operand product remains the only receiver projection
  authority. The canonical session is the sole direct Call/result issuer and
  owns the one-shot state, `MirInstruction::Call`, result type publication,
  and non-Clone result receipt. It mechanically consumes the source target
  plan and does not re-resolve source meaning. The typed method-target
  projection is derived from that already-sealed plan plus the callback-scoped
  receiver view; spelling, raw CallTarget, and MIR lookup are not authorities.
  The same outer unpublished function transaction is the sole rollback owner.

Non-authority:
  Raw ValueId, CallSlot rows, method spelling, current Builder cursor,
  `EffectMask` alone, MIR lookup, CheckedCallOut, legacy/Selected-Dynamic
  emitters, a copied receiver target, or a second canonical session cannot
  insert the Call or re-pair its result.

Fail-fast boundary:
  Missing/foreign receiver receipt, target/operand/condition-block/stamp
  drift, non-zero source arity, wrong `StringBox.length` callee/effect,
  non-I64 result, generic-emitter alternate route, duplicate/re-entry, receipt
  escape, or late failure rejects before the Call is published.
  Fallback/retry is forbidden.

Smallest next slice (landed below):
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-I0`
  emits exactly one canonical Call and one I64 receipt after the receiver
  operand I0. Its positive, final-shape, one-shot, and late-discard tests are
  green.

Non-claims:
  No parent Bool receipt, Compare instruction, `emit_branch`, edge/terminator,
  CFG/SSA/PHI beyond the consumed receiver read, Completion/DraftSeal,
  lifecycle, Text, route, performance, production caller, fallback, or retry
  is opened by this BoxShape.
```

Acceptance clarification:
The following I0 is caller-zero and remains inside the existing unpublished
function transaction. Its successful Call/result receipt is discarded with
that transaction; no module publication or production caller is implied. A
future commit/publication path is a separate boundary and cannot be inferred
from this D0.

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-I0` — landed caller-zero canary 2026-08-17

```text
Decision:
  Emit exactly one generic `StringBox.length` Call and one canonical I64
  result receipt from the already-sealed target/receiver/condition/stamp
  cohort, then discard the unpublished function transaction.

Source authority + canonical issuer:
  The canonical session is the sole physical result/type issuer and the
  unified Call emitter is the sole Call constructor. The session verifies the
  final emitted callee, receiver, destination, and READ effect before issuing
  the non-Clone receipt. The outer unpublished function transaction is the
  only rollback owner.

Non-authority:
  Raw ValueId, CallSlot, method spelling, MIR lookup, EffectMask alone,
  alternate/legacy emitters, or a second session cannot construct or repair
  the Call/result pair.

Fail-fast boundary:
  Target/receiver/condition/stamp drift, alternate Call shape/effect,
  non-I64 destination, duplicate/re-entry, or late callback failure rejects
  without publication; fallback/retry is forbidden.

Acceptance:
  Focused positive and late-discard tests are green. The canary is caller-zero
  only: its Call/result receipt is discarded with the outer transaction.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0` — design stop 2026-08-17

```text
Decision:
  Keep the parent Bool result at NoSafeSlice and split the remaining design
  boundary into two thin, ordered products. First, a Length-result receipt
  must be session-scoped strongly enough that it cannot be re-paired with a
  different canonical session. Only after that receipt lifetime is sealed may
  one condition-result materializer consume the Left read receipt and the
  Right Length result to issue the Bool result. The direct Length Call/result
  canary is landed, but its current receipt is not yet a closed cross-session
  capability.

Source authority + canonical issuer:
  The source Length contract, fixed Right operand row, matching operation row,
  Compare-right relation, retained physical-entry stamp, and the
  callback-scoped condition-block physical target are borrowed from the same
  common-V2 session. The target-plan, receiver-operand, and direct Call/result
  BoxShapes plus their caller-zero I0 are landed. The canonical session must
  retain the full physical-entry/session stamp (not only an owner projection)
  and issue the Length receipt and the later Bool receipt from one
  callback-scoped owner. The future condition materializer is the sole bridge:
  it consumes the canonical Left `BindingRef` read receipt and the same-session
  Length result, emits one mechanical `Less` Compare, publishes one Bool type,
  and returns one scoped non-Clone result receipt.

Non-authority:
  `LengthCallMaterializationCanaryV1`, an owner-only/stamp-only copied receipt,
  raw `LoopValueKeyV1`, raw `ValueId`, generic/legacy `CallSlot`,
  `CoreMethodOp::StringLen` alone, method spelling, Selected-Dynamic ledgers,
  CheckedCallOut, MIR lookup, `emit_compare_i64_at` alone, or a second session
  cannot issue or re-pair either physical operand. The source inventory and
  branch-plan condition key remain logical transport only.

Fail-fast boundary:
  Missing/foreign/full-session stamp, owner/function drift, Length role/
  operation/placement/arity/receiver/result/class drift, absent canonical
  result/type, target/manifest/target-brand drift, Left binding read failure,
  non-I64 operands, non-`Less` operation, duplicate publication, materializer
  re-entry, cross-session receipt pairing, callback escape, or late failure
  rejects before the next physical effect. The outer unpublished-function
  transaction remains the sole discard owner; fallback/retry is forbidden.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-D0`
  fixes the full session-scoped Length receipt and its one-shot callback
  lifetime without adding a new effect. If accepted, the next bounded row is
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-D0/I0`: the canonical
  session issues one Bool ValueId/type and one `Less` Compare receipt. Branch,
  edge, terminator, CFG/PHI, and publication remain closed until that row has
  its own acceptance.

Non-claims:
  No branch/edge/terminator, CFG/SSA/PHI, Completion/DraftSeal, lifecycle,
  Text, route, performance, production caller, publication, fallback, or
  retry is opened by this D0. A direct Call/result canary is not a production
  caller or a published physical result.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept one linear, callback-scoped Length Call/result receipt. The receipt
  owns the exclusive borrow of the exact `CommonV2CanonicalSessionRefV1` that
  issued its ValueId/type; it is non-Clone and can only be consumed by a
  receipt-owned later materializer. Do not add a second session stamp or a
  detachable owner-only token.

Source authority + canonical issuer:
  The existing retained physical-entry stamp and the live
  `CommonV2CanonicalSessionRefV1` are the only issuer inputs. The canonical
  session returns the receipt from the same callback that emitted the direct
  Length Call and published its I64 type. The receipt stores the exclusive
  `&mut CommonV2CanonicalSessionRefV1` borrow plus immutable result metadata;
  it cannot outlive the callback or be accepted by another session.

Non-authority:
  `stamp_owner`, raw ValueIds, physical block numbers, CallSlot/result keys,
  instruction scans, or a copied owner/stamp struct cannot prove session
  identity or issue/consume the receipt. A second Common/session has no
  accepting API. The outer unpublished function transaction remains the only
  discard owner.

Fail-fast boundary:
  Missing full stamp, foreign session, result/type/condition drift, duplicate
  issuance, callback escape, or any attempt to use the receipt after session
  close is rejected by the private type/API boundary before Bool materialization.
  While the receipt exists, another mutable borrow of the canonical session is
  unavailable; duplicate/re-entry must first consume or drop the receipt. No
  local rollback, fallback, or retry is permitted.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-I0`
  changes only the receipt return signature and adds focused lifetime,
  duplicate, callback-escape, and late-discard tests. After that I0, open the
  `...BOOL-RESULT-D0` materializer; keep branch and edge effects closed.

Non-claims:
  No Compare, Bool ValueId publication, branch/edge/terminator, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text, route, performance, production,
  fallback, or retry is opened here.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-I0` — landed caller-zero 2026-08-17

The direct Length Call/result API now returns a non-Clone receipt carrying an
exclusive borrow of the exact `CommonV2CanonicalSessionRefV1` that issued the
Call and I64 type. The receipt cannot be re-paired with another session or
escape the physical-entry callback; callers must drop it before attempting a
duplicate/re-entry check. The outer unpublished function transaction remains
the only late-discard owner. Focused direct-length and full physical-entry
suites are green. No Bool ValueId/Compare, branch, edge/terminator, CFG/PHI,
Completion/DraftSeal, lifecycle, Text, route, publication, fallback, retry, or
production caller is opened; the next design stop is the Bool-result
materializer D0.

The earlier canonical-session admission and physical-function-entry rows are
already landed caller-zero seams. This stop must not reopen them or use their
logical producer/descriptor rows as a second physical-result authority.

Non-claims:
  No physical ValueId, Compare lowering, `emit_branch`, edge/terminator,
  CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, performance,
  production caller, fallback, or retry is admitted by this D0.
```

### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Accept one receipt-owned same-session Bool materializer. It consumes the
  existing `CanonicalLengthCallResultReceiptV1` by value, resolves the source
  Left ReadBinding through the same canonical session, issues one fresh Bool
  ValueId/type, emits exactly one mechanical `Less` Compare in the condition
  block, and returns one non-Clone Bool result receipt carrying the exclusive
  borrow of that same session. This is the only bridge from the two operand
  receipts to the physical condition result.

Source authority + canonical issuer:
  `PreparedLoopV2ConditionProducerRelationV1` owns the Less/left/right/result
  relation and `PreparedLoopV2ConditionOperandInventoryV1` owns the exact
  Left ReadBinding + Right Length rows. The canonical session is the sole
  issuer of the Left `CanonicalBindingReadReceiptV1`, Bool ValueId/type, and
  Compare instruction. The Length receipt's owned session borrow is recovered
  only by its receipt-owned `consume_for_condition_bool` method.

Non-authority:
  raw `ValueId`, raw producer/inventory rows, `emit_compare_i64_at` alone,
  copied owner/stamp metadata, current Builder block, MIR/type-map scans,
  `CallSlot`, branch-plan Bool key, legacy/Selected-Dynamic emitter, and a
  second canonical session cannot issue or re-pair either operand or result.
  The Bool receipt is a physical result witness, not a branch or CFG authority.

Fail-fast boundary:
  Missing/foreign Length receipt, missing or duplicate Left row, owner/session/
  stamp/block drift, non-I64 operands, wrong producer/result/op/class, stale or
  conflicting Bool destination type, duplicate materialization, receipt escape,
  or late callback failure rejects before the next effect or publication. The
  outer unpublished function transaction is the sole discard owner; no local
  rollback, fallback, or retry is allowed.

Smallest next slice:
  The seed transport/materializer are landed. The caller-zero Bool materializer
  may consume the Length receipt, read the seeded Left binding at canonical
  entry, emit one `Less` Compare, publish Bool, and return its scoped receipt.
  Branch/edge/terminator remain closed.

Non-claims:
  No `emit_branch`, Header/Body or Header/After edge, terminator, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text, route, performance, production
  caller, publication, fallback, or retry is opened here.
```

### `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-D0` — accepted BoxShape 2026-08-17

```text
Decision:
  Keep the Bool-result BoxShape accepted, but do not open its I0 until the
  condition Left `ReadBinding` has a source-backed active declaration/value in
  the same canonical session. The current fresh physical-entry canary proves
  the gap by rejecting `read_entry_receipt` with declaration_not_active for
  the S6C local index binding. The missing unit is a pre-loop local seed, not
  a Bool or Compare workaround.

Source authority + canonical issuer:
  `VerifiedS6CTypedInputRelationV1::initializer()`, the resolver-owned
  `ResolvedInitializerRelationV1`, and the source ledger's
  `ResolvedLiteralSourceV1::Integer(0)` are the initializer, binding, type,
  and value authorities. A same-cohort issuer must project them into one
  `PreparedLoopV2InitialIndexSeedRelationV1`, including the S6C index carrier
  and entry relation. The canonical session then remains the sole issuer of
  the physical `ConstI64(0)`, exact declaration publication, and later Left
  read receipt. Package/Port and the Bool materializer only transport or
  consume that relation/receipt.

Non-authority:
  `index_binding` alone, a carrier entry alone, a logical `LoopValueKey`, a
  loop-local ConstI64/WriteBinding row, hardcoded zero,
  `activate_declaration_without_value`, raw `ValueId`, the physical-entry cursor, `read_entry`
  without an active initialized binding, MIR/type-map scans, or the old loop
  physicalizer cannot establish the source initializer or reaching value.

Fail-fast boundary:
  Missing/foreign initializer relation, non-Local source, declaration/site or
  binding/carrier/entry/stamp drift, absent Integer(0) witness, non-I64 type,
  duplicate/re-entry, seed-before-read violation, and callback escape reject
  before physical effect. No defaulting, local rollback, fallback, or retry is
  allowed; the outer unpublished function transaction remains the only discard
  owner.

Smallest next slice:
  `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-I0` may begin only after this
  source-only relation is issued. It will emit one unpublished `ConstI64(0)`
  plus one canonical `publish_declaration_exact` seed/read receipt, with
  positive, missing, foreign, duplicate, and late-discard tests. If the
  resolver/S6C issuer cannot be co-sealed, retain `NoSafeSlice` rather than
  inventing a count or value. Only after this row is closed may
  `...BOOL-RESULT-I0` consume the Length receipt.

Non-claims:
  No Bool ValueId/Compare, branch/edge/terminator, CFG/PHI,
  Completion/DraftSeal, lifecycle, Text, route, publication, fallback, retry,
  or production caller is opened here.
```

### `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-SOURCE-TRANSPORT-I0` — landed caller-zero slice 2026-08-17

```text
Decision:
  Land only the source-to-ingress transport for the accepted initial-index
  seed BoxShape. The transport lends one typed, callback-scoped seed relation
  from the same S6C cohort; it issues no Const, declaration, ValueId, read
  receipt, or physical effect.

Source authority + canonical issuer:
  `VerifiedS6CTypedInputRelationV1::initializer()` remains the S6C source
  relation, `ResolvedInitializerRelationV1` and
  `ResolvedLiteralSourceV1::Integer(0)` remain resolver/source-ledger facts,
  and one package/common-ingress issuer stores their mechanical projection as
  `PreparedLoopV2InitialIndexSeedRelationV1`. The existing ingress/envelope
  HRTB is the only transport path; no second package issuer is allowed.

Non-authority:
  `index_binding` plus carrier entry without initializer evidence, a logical
  `LoopValueKey`, loop-local `ConstI64`/`WriteBinding`, hardcoded zero, raw
  `ValueId`, MIR/type-map scans, or the old loop physicalizer cannot issue the
  seed relation.

Fail-fast boundary:
  Missing/foreign initializer, site/literal/type or owner/binding/carrier
  mismatch, non-Local source, duplicate/re-entry, loan escape, or relation
  reconstruction from raw ingress fields rejects before any session effect.
  The relation is non-Clone and carries the same owner/cohort/entry stamp;
  late callback failure has no publication or local rollback path.

Smallest next slice:
  The source-only relation field and scoped accessor are now present in the
  existing S6C ingress/common envelope. Positive and foreign-owner gates are
  green; lifetime remains callback-scoped by construction. The seed
  materializer I0 is landed below.

Non-claims:
  No Const/Write/ValueId/read receipt, Bool/Compare, Length re-emission,
  branch/edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text,
  route, performance, production caller, fallback, or retry is opened here.
```

### `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-I0` — landed caller-zero effect slice 2026-08-17

```text
Decision:
  Issue exactly one unpublished `ConstI64(0)` in the canonical function entry
  and publish it through `publish_declaration_exact` using the transported
  source initializer relation. Return one callback-scoped seed/read receipt;
  do not emit Compare, branch, edge, or loop CFG.

Source authority + canonical issuer:
  The same `PreparedLoopV2InitialIndexSeedRelationV1` owns the BindingRef,
  declaration site, I64 type, Integer(0) witness, and logical carrier entry.
  `CanonicalSsaFunctionSessionV2` is the sole physical ValueId/type issuer,
  Const writer, and exact declaration/SSA publisher. The outer unpublished
  function transaction remains the only discard owner.

Non-authority:
  hardcoded zero without the relation, `LoopValueKey` alone, raw ValueId,
  detached `read_entry`, `activate_declaration_without_value`, MIR/type-map
  scans, current block inference, the old loop physicalizer, or a second
  canonical session cannot issue the seed.

Fail-fast boundary:
  Already-issued/re-entry, missing function, entry-block drift, owner or
  source-shape mismatch, value/type drift, declaration/BindingSSA failure,
  and callback escape reject before publication. A failure after reservation
  poisons the unpublished session rather than exposing a retry path; the
  outer transaction discards the Const and declaration together.

Smallest next slice:
  The Bool-result materializer I0 now consumes this receipt (after its scoped
  borrow is consumed), reads the Left binding at canonical entry, emits one
  `Less`, and publishes one Bool result. Branch/edge/terminator remain closed.

Non-claims:
  No branch/edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text,
  route, performance, production caller, fallback, or retry is opened here.
```

## `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-I0` — landed caller-zero effect slice 2026-08-17

`CanonicalLengthCallResultReceiptV1::consume_for_condition_bool` is the sole
bridge from the source Left ReadBinding and Length result to the physical
condition value. It validates the same owner, producer, two-row operand
inventory, seeded entry declaration, and I64 types; then the canonical session
issues one Bool destination, emits one `Less` Compare, and publishes its Bool
type. The result receipt is non-Clone and retains the same-session borrow, so a
second session or detached operand pair cannot be introduced. Missing seed,
wrong role/type, and late callback failure are covered by focused tests; late
failure discards the whole unpublished function.

No branch/edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text,
route, publication, fallback, retry, or production caller is opened. The
read-only G0 owner census is complete; the active bounded row is the
Generic `LOOP-PRECUTOVER-AUTHORITY-G0-D0` design stop.

Current blockers are deliberately explicit:

```text
NoSafeSlice::GenericG0EntrySourceCoverageParentUnsealed
```

## Decision

Close the post-Recipe boundary before physical implementation begins.

```text
resolver / source map
  -> versioned VerifiedLoopSemanticProgramV1 | V2
  -> versioned complete operation/source/control demand
  -> PreparedLoopOperationProgramV1 | target V2
  -> one thin prepared execution product
       PreparedCallableLoopPhysicalizationV1
       OR PreparedGenericG0LoopPhysicalizationV1
       OR scoped target PreparedLoopV2PreSessionEnvelopeV1<'loan>
  -> target LoopV2CanonicalSessionAdmissionV1
       fan-in of the neutral envelope, separately admitted route policy,
       and callable signature/residence demands
  -> one fresh unpublished function session
       completion moves here exactly once
  -> outer callable lowerer + one common recursive Loop physicalizer
  -> open After continuation
  -> existing completion / DraftSeal
  -> one unpublished function draft
```

The target canonical full-operation input is a private, move-only, AST-free,
and physical-ID-free semantic-program receipt which feeds
`VerifiedLoopOperationPhysicalDemandV1`. It co-seals the Core-bearing
operation/effect product with one common continuation capability issued by
that Core's own JoinSig; the two are never independently re-paired at the
physical boundary. Each thin
prepared product is move-only and physical-ID-free. Its source-backed input
must be issued by one existing-owner ingress receipt that pairs the exact
`ResolvedFunctionLoweringInputV1` with its resolver ledger view and, where the
profile requires it, the exact callable index/header. No prepare/physicalizer
policy may inspect or rematch its AST. The current
`NormalCallableSemanticLoanPortV1` is only the raw-lowering host and does not
yet issue this receipt; that source-loan expansion is the remaining D0 gate,
not a reason to add a second resolver or to remove `cfg(test)` from prepared
types prematurely.
A profile prepare consumes one inner demand plus already sealed boundary
capabilities and fixes their exact compatibility before the first Builder
effect. Only the common inner demand is consumed by the recursive Loop
physicalizer. Neither product is a new Recipe, selector, SSA, CFG, PHI,
transaction, Return writer, or publication owner.

Nested control can split one logical Recipe block into multiple physical
segments. Generic G0 is the counterexample: root block B1 contains a carrier
read, a nested Loop item, then the root update. Therefore logical-block-to-one-
physical-block mapping is not a sufficient execution contract. A private,
move-only `PreparedLoopPhysicalLayoutV1` target is mechanically derived from
the complete Recipe/JoinSig and exact operation coverage before Builder
effect. It owns only ordered segment placement and transfer compatibility:

```text
Recipe item -> exact segment
segment -> ordered operation rows + one verified transfer
nested After -> exact parent resume segment
```

Recipe/JoinSig remain the sole logical authority. The layout may not infer
control meaning, reorder by item key, accept a profile name, or survive as a
second Recipe. Unsupported structural items reject with typed `NoSafeSlice`
before Builder mutation. Canonical CFG remains the sole physical block/edge/
terminator owner after the layout is admitted.

### Pre-cutover authority correction (2026-08-08)

Decision: accepted after external review and independent code audits. The
direction above remains authoritative, but current caller-zero code has two
known gaps and must not be activated in production yet.

First, current `VerifiedLoopOperationPhysicalDemandV1::issue` accepts semantic
context, operation/effect product, and continuation as separate verified
arguments. Its checks establish owner/scope/root-key compatibility but do not
prove that all three came from the same resolver Loop site/frame and the same
Core-owned JoinSig. The final issuer is:

```text
resolver-issued Loop source capability
+ exact LoopNodeKey -> source relations
+ complete item/carrier source relations
+ one existing entry-source owner's complete-coverage receipt
+ Core-bearing operation/effect product
    -> require continuation from this Core's JoinSig
    -> VerifiedLoopSemanticProgramV1
    -> VerifiedLoopOperationPhysicalDemandV1
```

`VerifiedLoopSemanticProgramV1` owns only the relational proof that these
existing products describe one executable Loop program. It is not a second
Core, Recipe, source observer, selector, input owner, or Callable plan. The
actual initialized-local input set and Generic parameter input contract stay
typed and distinct; each may issue only an opaque coverage receipt over the
same Recipe inputs. Raw `VerifiedLoopSemanticContextV1::from_parts`, external
continuation `from_after`, and the three-argument physical-demand issue path
are compatibility debt and reach zero callers in the co-seal migration.

For Recipe V2 Dynamic, the same rule is stronger: the caller passes only the
exact source/Recipe/envelope aggregate. A common JoinSig engine is invoked
inside the issuer, and `After(L0,B0,Dynamic)` is requested from that exact
JoinSig before `VerifiedLoopSemanticProgramV2` exists. A V2 caller cannot pass
owner, root key, JoinSig, After, or Continuation separately. `V10/ch`, Dynamic
Fault, Callable Tail, and the two-site Completion remain external to the Loop
carrier/continuation identity.

#### S6C complete-V2 pre-session admission (2026-08-15)

S6C uses the same rule without borrowing the Selected-Dynamic fixed cursor or
coercing its Recipe to V1. The landed common recursive physicalizer is
test-only and its operation program/dispatcher is V1-shaped. It therefore
cannot become production merely because the caller-zero S6C ingress and one
TextEq route policy exist.

The missing contract is owned by one ordered task family:

```text
LOOP-S6C-INSTALLED-CHILD-COMPOSITION-D0
LOOP-S6C-INSTALLED-CHILD-COMPOSITION-I0
CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0/I0
LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
LOOP-S6C-COMMON-V2-PRESESSION-I0
```

The child row is the sole pre-install semantic admission seam. The package
issuer co-seals the selected role/identity with the retained S6C Facts/Recipe/
Join child and moves one non-`Clone` Completion seed into it exactly once. The
Port only verifies the already-issued role/identity and takes/lends the child
exactly once after install; it does not reclassify S6C or accept a caller slot,
key, ingress, or fixture.

The callable-signature Decision is accepted: one logical ExactText formal and
one semantic BindingRef expand to adjacent scalar `u64` lanes
`[slot,generation]`. Logical `/N` and `physical_formal_lane_count` are separate
authorities; a by-value 16-byte aggregate is rejected. The package-owned sole
issuer consumes same-brand selected/batch identity plus the complete parameter
contracts only. Header/Completion, `MirType::String`, C validator argument
order, raw `ValueId`, root residence, and call-edge origin are not signature
inputs. The active caller-zero I0 may implement only this total lane map and
its combined Installed S6C loan.

The parent common-V2 row then closes the shared callable-signature, single-
Completion, and operation/control envelope:

```text
LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0
LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0
LOOP-S6C-COMMON-V2-PRESESSION-I0

VerifiedNormalCallableSemanticPackageV1
  -> install consumes the verified product
InstalledNormalCallableSemanticPackageV1
  -> NormalCallableSemanticPackagePortV1 target extension
       exact lowering input and formal/header projections
       target TextFormal callable-signature mapping
       target VerifiedS6CSemanticChildV1 + one Completion (issued pre-install)
       target InstalledS6CChildAdmissionRefV1<'loan> (Port take/lend only)

AdmittedLoopTextEqRoutePolicyV1
  separate sibling branch; site-free; no Loop/item/value/source identity

PreparedLoopV2PreSessionEnvelopeV1<'loan>          target contract
  -> PreparedLoopOperationProgramV2
       every operation placement for the admitted V2 program
  -> Recipe + JoinSig + Layout control subproduct
       every If/Exit control placement for the admitted V2 program
  -> complete envelope receipt
       generic disjoint-union coverage

S6C adapter requirements
  exact 13 operations + one If + one Exit = all 15 placements

LoopV2CanonicalSessionAdmissionV1                 later fan-in
  consumes the neutral envelope + admitted route policy
  -> CanonicalSsaFunctionSessionV2
```

The verified package is consumed at install time; the catalog moves into
`CompilationContext`, while `InstalledNormalCallableSemanticPackageV1`
retains the same-brand batch/selection/formal/header state.
`NormalCallableSemanticPackagePortV1` is the only target lending seam. The
pre-session envelope is route-free and borrows that cohort only inside the
Port callback; it cannot retain a site borrowed from its own parent, clone
source/Completion ledgers, or accept raw keys from a caller. The installed S6C
child, one Completion ownership path, and the Completion-independent package
lane map are closed by their caller-zero I0 rows. The common-V2 BoxShape,
source-level transport R0, and the three-issuer I0 contract are now landed as
caller-zero source products. No generic V2 execution effect or Builder/session
admission is implied. Exact call-edge origin, Canonical composite adoption,
and Completion-backed lifecycle finish remain a later canonical-session
fan-in; they are never inferred from Recipe/MIR. The generic V2 exact-set
partition is the current common boundary.

### Common V2 pre-session BoxShape (accepted boundary; source implementation landed)

The D0 boundary is one non-splittable parent loan, not a public aggregate whose
fields can be independently reacquired. The target Port shape is a single
HRTB callback:

```text
installed package/Port
  -> with_s6c_common_v2_presession(|parent| { ... })

parent (private, same-brand, scoped)
  ├─ selected callable identity + catalog brand
  ├─ one Completion owner (borrowed only)
  ├─ package physical-signature sibling view
  ├─ retained S6C Facts/Recipe/Join/prephysical sibling view
  └─ neutral operation/control/coverage projections
```

The operation and control projections are sibling views of that parent. The
operation product covers every Operation placement for the admitted V2 input;
it does not hard-code the S6C cardinality. The control product owns only
`If`, `Exit`, and JoinSig-issued transfers. JoinSig, Completion, and callable
Tail add no placement. A passive coverage projection proves the disjoint union
of the operation and control sets. The S6C adapter alone must prove the exact
`13 + 1 + 1 = 15` coverage before any leaf is emitted.

The boundary is limited to same-brand ownership, scoped sibling lending,
generic operation/control partition, and passive exact coverage. The landed
adapter rejects missing/duplicate/overlapping/foreign placement, a detached
key/source ledger, and any need to infer the partition from MIR/Recipe order
before effect. It does not add Builder/session effects or issue a second
semantic authority.

### Common V2 transport R0 (accepted boundary; installed Port implementation landed)

Transport is a source-level projection, not a MIR/JSON carrier. The sole target
Port seam is one callback of the following shape:

```text
NormalCallableSemanticPackagePortV1
  -> with_s6c_common_v2_presession(|parent| { ... })
```

The callback consumes the same installed selected key once and lends a private
parent whose sibling views are selected identity/brand, the package-owned
physical signature, the one retained Completion owner, and the retained S6C
Facts/Recipe/Join ingress. The existing `with_s6c_child` and separate
signature/header accessors are not a second route: they are compatibility
substrates to be replaced or wrapped by this single parent seam in the I0.
No view may escape, be stored, or be recombined after the callback; the Port's
existing consumption ledger remains the exactly-once owner.

The Port callback now emits the three caller-zero sibling products from the
same retained cohort. JSON, MIR metadata, Builder/session, physical IDs, Text
route, lifecycle, fallback, and production caller remain out of scope.

The stable callable boundary is not the function-internal Text carrier. A
later non-splittable residence set couples the invocation lease-set token to
immutable UTF-8 root descriptors. Session-branded slices and backend-local
`TextPlan` values borrow those roots; raw pointer/length values are scoped
backend projections only. The common pre-session envelope owns none of these
physical identities.

### Common V2 I0 issuer contract (accepted and landed caller-zero, 2026-08-16)

All three issuers run inside the one installed-package/Port HRTB parent loan.
They are scoped sibling projections and cannot be independently acquired,
stored, or re-paired:

```text
same retained S6C source/Recipe/Join/prephysical cohort
  ├─ S6C operation adapter
  │    -> generic PreparedLoopOperationProgramV2<'loan>
  ├─ JoinSig control adapter
  │    -> generic If/Exit + transfer control product
  └─ passive coverage issuer
       -> disjoint-union/complete-coverage receipt
```

The operation issuer is the sole S6C profile adapter allowed to consume the
retained prephysical source view. It projects borrowed generic operation rows
(placement, operation payload, and existing execution-class evidence) for every
`Operation` placement in the admitted V2 program. It must not expose
`S6CPrephysicalOperationRoleV2`, the fixed `OPERATION_COUNT = 13`, S6C names,
`If`/`Exit`, JoinSig transfers, Completion, Builder, MIR/CFG/SSA/PHI, or route
policy. Only the S6C adapter may later assert its profile fact of 13
operations; the generic product never hard-codes 13.

The control issuer consumes the existing
`VerifiedLoopJoinClosureV2::logical_transfer_view()` as its sole JoinSig
logical source and co-seals only the matching Recipe `If`/`Exit` rows from the
same retained cohort. It does not reissue JoinSig, scan physical blocks, or
classify `If`/`Exit` as operations. Owner/loop/condition/exit-item,
branch-transfer, and After relations must agree before effect.

The coverage issuer is passive. It consumes the two already-issued sibling
products, rejects missing/duplicate/foreign/overlapping placement keys, and
issues only the disjoint-union/complete-coverage receipt. It does not rescan
Recipe or MIR and does not mint a third semantic product. The S6C adapter may
then prove `13 + 1 + 1 = 15`; the generic coverage product remains cardinality
neutral.

The source-level seams are implemented by the private/non-Clone
`issue_s6c_common_v2_pre_session_v1` adapter and its operation, control, and
passive coverage siblings. They may be issued only within the parent loan. Any
missing source anchor, foreign cohort, duplicate/overlap, HRTB escape,
detached row/key, or downstream re-pair is a typed rejection before effect.
The implementation does not open Builder/session, lifecycle, Text residence,
route, fallback/retry, production callers, or publication.

The complete envelope does not turn control into operations. The operation
program target covers every operation placement for the admitted V2 program;
the S6C adapter alone requires exactly 13. Recipe plus JoinSig remains the
logical owner of the S6C `If`, `Exit`, and their transfers; Layout only binds
that control to placement. A passive union receipt proves the generic
disjoint union, while the S6C adapter proves exact 15-placement coverage,
without creating a second control ledger.

Decision B applies to V2 as well: the complete V2 envelope is prepared and
checked before a private TextEq or CallSlot leaf is emitted. The existing
canonical CFG/Binding-SSA/Phi/Completion/finish/DraftSeal services are reused
inside one fresh unpublished session. V2-to-V1 adaptation, Dynamic-cursor
reuse, standalone leaf scheduling, and an S6C physicalizer are forbidden.
Actual handle/span/ValueId residence remains session-local and cannot be
issued by the pre-session product.

Second, current `physical_layout.rs` does not consume JoinSig transfers. It
reconstructs Predicate true/false, body backedge, nested entry, and child-After
resume from Recipe; `segment_allocator.rs` also rereads Recipe condition roles,
and `recursive_after.rs` emits the resulting transfer. This is caller-zero
evidence, not the accepted final transfer authority.

The corrected physical contract is:

```text
private Recipe traversal events
  -> item order and structural segment boundaries only
JoinSig-issued VerifiedLoopTransferV1
  -> logical role, exact control point, ports, payload, exit/After obligation
PreparedLoopPhysicalLayoutV1
  -> bind each verified transfer and operation to exact segments
CanonicalCfgSessionV1
  -> allocate and emit each admitted edge/terminator exactly once
```

The private traversal event stream may be retained inside
`PreparedLoopOperationProgramV1` and reused by schedule/layout preparation.
It carries no control target, is not public, and is never serialized as a
second Recipe. JoinSig must first gain exact item/control-point-keyed
capabilities for every admitted family. Therefore current typed
`UnsupportedAlways`, `UnsupportedIf`, and `UnsupportedExit` remain correct
until separate BoxCount rows land after the BoxShape authority cutover.

The existing `VerifiedLoopPhysicalDemandV1` is a closed topology-only P0
compatibility transport. It feeds only the historical caller-zero topology/
After probe, carries no complete operation/effect ledger, and cannot be
extended, renamed, or reused as the canonical operation input. The module-
split row moves the flat file into one directory facade, deletes the old flat
module, and quarantines that entry behind the topology-only test facade. Two
module entries or two topology authorities are forbidden.

`Admission` remains the semantic family-selection term. `Prepared...` means
only that already verified capabilities have been related into one executable
request. The prepared product owns exactly one new relational fact: its Loop,
Prelude/input, Tail, return ABI, Completion, owner, and frame may execute
together once. It does not own or copy the component semantic truths.

The existing owners remain authoritative:

| Concern | Sole owner |
| --- | --- |
| source membership, owner, frame, Scope/Region | resolver ledger and source map |
| logical operations, keys, recursive nesting | one recursive Loop Recipe algebra through its exact `LoopRecipeV1` or `LoopRecipeV2` projection; never V2-to-V1 coercion |
| logical ports, edges, carrier obligations | the JoinSig issued from that exact versioned Recipe/program cohort |
| source/effect/input relations | existing Core, initialized-local input, Generic parameter-input, item-source, and carrier-source products; none is replaced by the semantic program |
| cross-product source/Core/continuation compatibility | the version-matched semantic-program co-seal (landed/target `VerifiedLoopSemanticProgramV1` responsibility plus the D0-named V2 projection); relational co-seal only, never V2-to-V1 conversion |
| `BindingRef -> ValueId`, lexical SSA | `CanonicalSsaFunctionSessionV2.identity` |
| physical blocks, predecessors, sealing | `CanonicalCfgSessionV1` |
| provisional and patched PHI lifecycle | the function session's one `PhiTxn` |
| source completion evidence | `VerifiedFunctionCompletionV1`; remains owned by the installed cohort and is borrowed once to issue the session's physical consumer |
| mutable physical completion consumption | fresh `CanonicalSsaFunctionSessionV2.completion` / `ResolvedFunctionCompletionConsumptionV1` |
| common function-local finish terminal | `CanonicalSsaFunctionSessionV2::finish_for_draft_seal` target API |
| captured caller restore, unpublished discard, prepared close | `CanonicalFunctionLoweringSessionV1` |
| detached DraftSeal prepare and rejected-owner retention | `OpenFunctionDraftSealV1` |
| sole function commit terminal | `PreparedFunctionDraftSealV1::commit` through prepared session close |
| draft collection and module publication | `ModuleDraftCollectorV1` plus the existing module transaction / `ModuleBuilderInvocationSessionV1` |

## Why this boundary exists

Recipe completion is not physical completion. A verified Recipe proves the
logical program, but it does not prove:

- which already-resolved callable prelude supplies an external value;
- that every Recipe input can be materialized in the preheader;
- that the function return has an exact supported ABI;
- that the terminal source binding is the value returned;
- that one fresh session can finish CFG, SSA, PHI, completion, and DraftSeal;
- that late failure leaves the live caller unchanged.

These obligations must be sealed once before mutation. A physicalizer must not
rediscover them from AST, source names, route labels, or existing MIR.

### Repository audit receipt

| Observed code authority | Confirmed boundary |
| --- | --- |
| `CanonicalSsaFunctionSessionV2::new` consumes `VerifiedFunctionCompletionV1` into `ResolvedFunctionCompletionConsumptionV1` | Completion cannot remain owned by a prepared sibling after session open |
| `CanonicalDirectAccumSsaLowererV1::lower` finishes semantics/If/identity/Phi/binding/completion but omits `cfg.finish` | prose ordering alone is insufficient; the V2 finish terminal is required |
| `ReadyFunctionDraftSealV1::new` currently accepts only ready completion + current block | current Ready type does not prove common CFG/SSA/PHI closure by construction |
| `ResolvedFunctionLoweringInputV1` is an existing exact read-only source/function/forest/header view | prepared outer product may retain it; common Loop demand must not receive it |
| `NormalCallableSemanticLoanPortV1` currently forwards a raw body after consuming a loan, while `VerifiedNormalCallableSemanticLoanV1::into_parts()` retains only lineage + request-local lowering state | a source-loan expansion receipt must be issued before I0; AST re-walk, name lookup, and synthetic catalog/header pairing are `NoSafeSlice` |
| `CompilationContext::callable_declaration_catalog()` is installed before normal lowering | the catalog is an existing borrowed authority, not an automatic source/forest pairing; owner/frame/scope/index/header identity must be checked once |
| `loop_physical_prepare.rs::VerifiedCallableFunctionLoweringInputV1` is `cfg(test)` and static-header-profile-specific | it remains a canary witness; removing `cfg(test)` is not a production ingress design, and normal callables must not be forced through that header profile |
| `VerifiedCallableSingleLoopSourceMapV1` carries source roles, BindingRefs, loop context, and resolved exit evidence only | current co-seal cannot issue ABI or Completion authority |
| `PhiTxn::abort_on_err` sees only still-pending provisional PHIs | whole-session discard, not PHI rollback, owns atomicity |

### Current co-seal stop correction

`RECIPE-COSEAL-I0-R0` has no authority to issue an exact return ABI or a new
`VerifiedFunctionCompletionV1`; its current source map contains source-role and
resolved-exit evidence only. The current row therefore publishes these
disjoint caller-zero products:

```text
VerifiedLoopRecipeCoSealV1
  Core / Recipe / JoinSig
  operation-source and input-source relations
  semantic context
  VerifiedLoopContinuationContractV1

VerifiedCallablePreludeV1
VerifiedCallableTailV1
```

The existing exact ABI and Completion capabilities remain with their existing
issuers. `LOOP-PHYSICAL-PREPARE-P0` later consumes all components once and
either issues one prepared execution product or returns typed `NoSafeSlice`.
`VerifiedLoopAfterTailEnvelopeV1` is rejected: Loop continuation and callable
Tail must never be fused and then split again.

## Product boundary

### Common product

### Operation physical demand

The current caller-zero compatibility shape is:

```text
VerifiedLoopOperationPhysicalDemandV1 {
  context: VerifiedLoopSemanticContextV1,
  operation_effect: VerifiedLoopOperationEffectProductV1,
  continuation: VerifiedLoopContinuationContractV1,
  index: private LoopOperationPhysicalIndexV1,
}
```

The context owns only the resolver-issued semantic identity relation
(owner/origin/source-kind/loop-site/frame/Scope/Region); it is moved from the
existing source authority and is not re-derived from Recipe keys. The
operation/effect product owns the moved Core and item-keyed source/effect
ledger. The continuation owns only the logical Loop After capability. Callable
and Generic G0 adapters issue the common context and continuation by exact move
from their existing resolver/source products; they do not share source types,
compare counts, or pass two independent arguments to the physicalizer. The
index is a private key-only lookup aid and never a second semantic or physical
truth. The existing
`VerifiedLoopPhysicalBoundaryV1` remains topology-only and is invalid for the
operation program because it drops source anchors.

Decision B keeps whole-program preparation and leaf emission separate:

```text
VerifiedLoopOperationPhysicalDemandV1
  -> prepare_all
  -> PreparedLoopOperationProgramV1
       complete Recipe-derived operation schedule
       exact complete-coverage receipt

PreparedLoopOperationEmissionV1
  -> one private leaf emitter
```

The full demand exposes no first/select/filter/take-operation API. Recipe
Loop/Block/Item structure is the sole execution-order authority; an evidence
vector sorted by key is only storage order. `PreparedLoopOperationProgramV1`
retains the complete demand and common continuation. A leaf emission owns only
one already-prepared operation, source evidence, expected Loop, and expected
logical block; it never sees Recipe, profile, Tail, ABI, Completion, Return,
DraftSeal, publication, or continuation.

The first leaf canary may use a private test-only ConstI64 constructor, but it
must not obtain that row by extracting it from a seven-operation Callable or
fifteen-operation Generic G0 demand. A synthetic one-operation full Recipe is
not the first authority and may be added only as a later integration fixture.

The accepted conceptual shape has two layers:

```text
PreparedCallableLoopPhysicalizationV1
  input: exact ResolvedFunctionLoweringInputV1
  loop: VerifiedLoopOperationPhysicalDemandV1
  prelude: VerifiedCallablePreludeV1
  tail: VerifiedCallableTailV1
  return_abi: existing exact ABI capability
              (ExactTrivialReturnAbiV1 for the first profile)
  completion: VerifiedFunctionCompletionV1

PreparedGenericG0LoopPhysicalizationV1
  input: exact ResolvedFunctionLoweringInputV1
  loop: VerifiedLoopOperationPhysicalDemandV1
  tail: VerifiedGenericG0TailV1
  return_abi: existing exact ABI capability
              (ExactTrivialReturnAbiV1 for G0)
  completion: VerifiedFunctionCompletionV1
```

The exact Rust field split may remain private, but the following contract is
fixed.

The common prepare consumes the Loop co-seal once and moves it wholly into one
`VerifiedLoopOperationPhysicalDemandV1`. Callable Prelude/Tail remain separate
inputs; they are not
split back out of the co-seal. The prepare never borrows, clones, or re-catalogs
the co-seal. The inner receives, without duplication:

- verified Recipe/Core and JoinSig;
- the moved resolver-issued semantic context (including Scope/Region);
- semantic owner/origin/source-kind, loop source and execution frame;
- Scope/Region relation;
- exact operation-source and input-source relations;
- `VerifiedLoopContinuationContractV1`, which owns only the logical Loop After
  port/capability.

`LoopOperationPhysicalIndexV1` is a private, key-only search index over
existing logical keys:

- Recipe binding/value/item/block keys and their placement roles;
- Recipe input value -> logical preheader port;
- Recipe item -> owning Loop/Block + exact input/output value keys;
- JoinSig port/edge obligations -> logical placement roles.

`BindingRef`, source site, source/effect relation, semantic identity, Recipe,
and JoinSig truth remain solely in the moved co-seal owner held by the inner
demand. The private index cannot be independently constructed, published, or
verified; it refers to those logical keys and may be rebuilt only inside the
same prepare operation. The physicalizer consumes the demand as one product,
not Recipe plus a second public topology truth.

The physicalizer boundary is move-only. Prepare must issue a private consuming
operation for `VerifiedLoopOperationPhysicalDemandV1`; borrowing, cloning, a
second co-seal, or MIR reconstruction is invalid. This prevents logical demand
from being silently reused after it crosses into physical lowering.

The callable prepared product relates the non-Loop obligations:

- exact prelude caller/site/target/result contract when a prelude result is
  required;
- destination source `BindingRef` for that result;
- exact terminal return statement and value sites;
- exact terminal source `BindingRef`;
- exact supported return ABI capability;
- the matching `VerifiedFunctionCompletionV1`.

The prelude contract contains no `ValueId`. Its current source shape also does
not prove argument bindings: arity is not an argument materialization receipt.
The selected prerequisite is one move-only, AST-free
`VerifiedCallablePreludeArgumentListV1`. Each row carries an exact ordinal,
`SourceExprSiteV1`, resolver-issued `BindingRefV1`, and exact `i64` ABI. The
issuer reads `VerifiedResolvedFunctionV1.variable_ref(site)` and admits only
`ResolvedLexicalRefV1::Local` owned by the caller. Upvar, literal, nested
expression, unknown site, foreign binding, and unsupported ABI are typed
`NoSafeSlice`. No new resolver or semantic owner is introduced.

The outer lowerer consumes this list once, reads each BindingRef through the
canonical session identity, and materializes the external Prelude result. The
Loop input initializer is a separate exact source-site obligation: it is
resolved through the existing source view, emitted as the entry value, and
published under the co-sealed Loop input BindingRef. The Prelude result local
and Loop input binding must never be conflated. Only then does the adapter
issue one private `ReadyLoopEntryV1`. AST reread, name lookup, and arity-only
reconstruction are forbidden. It then opens the exact function session, moves
Completion into `CanonicalSsaFunctionSessionV2::new` exactly once, and retains
Prelude/Tail/ABI evidence only. The future full operation physicalizer consumes
the `VerifiedLoopOperationPhysicalDemandV1` plus that entry receipt and never
observes the callable boundary.

The Generic G0 prepared product wraps one instance of the same complete
operation-demand type but retains its existing `L0.After/b1` boundary
capability. It neither
reuses the callable prefix `value` Tail nor creates a G0 physicalizer.

The Generic G0 window lease is the source authority for its Scope/Region pair.
The lease therefore retains the pair alongside its existing owner/source/frame
brand, and the G0 product moves that context into the common demand. If a
profile cannot provide this exact pair, the demand is a typed `NoSafeSlice`; no
synthetic region or route-local context is permitted.

The G0 adapter must consume the existing S4 product into a common co-seal view
by a disjoint move of its already verified Core/relations/After evidence. If
that view cannot be issued without copying source truth or re-verifying the
Recipe, the G0 adapter is `NoSafeSlice` and parity remains parked.

### These are prepared execution products, not callable megaboxes

No profile prepared product becomes a universal callable semantic owner. It
implements no new Call, ABI, Loop, Return, or publication algorithm. It owns
only the relational compatibility proof that already sealed capabilities
belong to the same callable execution:

```text
source/target identity  -> existing resolver/catalog authority
argument/result ABI     -> existing verified ABI capabilities
Loop meaning            -> Recipe/JoinSig/co-seal
terminal disposition    -> existing completion capability
physical commit         -> existing DraftSeal owner

profile prepared product
  -> one exact owner/site/BindingRef compatibility proof
  -> one fixed Prelude -> Loop -> Tail -> Completion order
```

Prelude/Input, Loop, Tail/Return, and Completion stay typed sub-capabilities.
The two-layer product prevents the Loop physicalizer from observing the
callable boundary at all; only the outer callable lowerer sees both siblings.
They are not flattened into an opaque `CallablePlan` payload. The envelope
moves or borrows sealed evidence and cannot copy facts into a second catalog.
A non-Loop callable remains outside this Loop-specific prepared product, so
this D0 does not pre-empt the final general callable design.

### Completion and ABI are separate

`VerifiedFunctionCompletionV1` is necessary but insufficient. It seals exit
cardinality, terminal statement kind, target function, cleanup, and declared
result contract. It does not by itself carry the return value `BindingRef`,
the return expression site, or a concrete physical ABI. An unannotated
explicit return can therefore pass completion verification without being safe
for this physical row.

Each prepared profile with a value return requires both:

```text
exact terminal value site + BindingRef + exact return ABI
AND
matching VerifiedFunctionCompletionV1
```

For the first row the supported ABI is the already verified exact trivial
`i64` capability. Unannotated, dynamic, unknown, or inferred-by-name return
types reject before Builder effects. Later ABI profiles require separate
verified capabilities, not widening inside the physicalizer.

### Loop After and callable Tail are separate

The selected callable profile returns the prefix `value` binding:

```hako
local value = helper.to_i64(n)
local i = 0
loop i < 1 { i = i + 1 }
return value
```

Its terminal operand is not a Loop carrier After value. Generic G0 currently
returns `L0.After/b1`; that is a different profile adapter.

```text
logical Loop After capability != callable terminal Tail capability
```

The callable prepared product keeps both fields distinct. A profile adapter may prove
the same binding supplies both, but no consumer may infer that equality.
Generic G0's `VerifiedGenericAfterEffectG0` remains its boundary input and is
adapted beside the same common inner demand; it is neither the common Loop
authority nor the callable Tail authority.

## Forbidden contents

The inner demand and co-seal must be AST-free. The prepared profile product may
retain only the exact existing `ResolvedFunctionLoweringInputV1` source view;
it must not add independent raw source fields. Across these products the
following are forbidden:

```text
raw AST / StmtRef / ExprRef fields outside ResolvedFunctionLoweringInputV1
source or callable name selectors
path-suffix or ordinal rematching
legacy route ID or scheduler cursor
ValueId / BasicBlockId / PHI destination
MirBuilder or CanonicalSsaFunctionSessionV2
PhiTxn or rollback journal
ResolvedFunctionCompletionConsumptionV1
retry / fallback / reselection
commit or publication capability
```

The old DirectAccum-only `VerifiedLoopPhysicalInputV1` contains Recipe and
JoinSig only. It is a pilot input and must not be renamed or reused as the
final common demand; it lacks the co-sealed source/effect, continuation,
private-index, ABI, Tail, and prepared execution contract.

No session brand is added merely to pair either demand with a session. The
pre-effect issuer verifies semantic owner/frame/scope contracts; the consumer
then checks them against the freshly opened existing session. If the existing
session cannot expose the required prepare facts, the result is
`SessionPreparationUnavailable`, not a second session identity.

## Exact consumption

The common prepare consumes one `VerifiedLoopRecipeCoSealV1` plus the complete
operation/effect product and either issues one non-Clone
`VerifiedLoopOperationPhysicalDemandV1` or returns a typed rejection retaining
the sole unconsumed owner. A thin callable or G0 prepare then consumes exactly
one full demand plus the profile's disjoint boundary capabilities and issues
one prepared product. Neither step re-runs Recipe verification, mints keys, or
consults the legacy scheduler.

The outer profile entry consumes one prepared product to open the exact fresh
function session. The installed `VerifiedFunctionCompletionV1` remains owned
by its cohort; the session consumes one owned
`ResolvedFunctionCompletionConsumptionV1` issued from that scoped borrow. The
semantic Completion is not cloned or moved into a sibling boundary. The outer
lowerer retains only Prelude/Tail/ABI evidence, transfers the full operation
demand exactly once to the future full physicalizer, and later claims the
exact return operand through `session.completion`. Lowering by `&demand`,
cloning a split/prepared product, recreating one from MIR, or trying a second
route is forbidden.

Logical keys map to physical owners as follows:

| Logical evidence | Physical interpretation |
| --- | --- |
| `LoopBindingKey` + source `BindingRef` | canonical identity/BindingSSA |
| Recipe input + preheader relation | outer prelude/input materialization |
| `LoopItemKey` + owning block + value keys | common recursive physicalizer |
| JoinSig port/edge role | canonical CFG allocation and sealing |
| carrier obligation | canonical identity plus the one PHI transaction |
| Loop After capability | open allocation result first; sealed `ReadyLoopAfterContinuationV1` before any Tail read |
| terminal Tail capability | outer callable lowerer and completion consumer |

The topology physicalizer initially returns an open After/continuation receipt.
That receipt is not readable by Tail. A callable profile must first consume it,
issue the verified CFG edges, seal CFG and identity for every loop block, and
mint one session-local `ReadyLoopAfterContinuationV1`. Only that sealed receipt
may be passed to the outer Tail handoff. The physicalizer must not write
`Return`, take the function, publish a draft, or close the module.

### Session-local entry receipt

Opening a function session does not prove that Prelude/parameter/input values
have been installed. The outer profile lowerer must materialize every required
entry binding first and issue one private, session-local `ReadyLoopEntryV1`.
The future full operation physicalizer requires:

```text
PreparedLoopOperationProgramV1
+ ReadyLoopEntryV1
+ borrowed canonical CFG / Binding SSA / PhiTxn services
```

`ReadyLoopEntryV1` owns no source or callable semantics. It proves the
temporal fact that the exact logical input keys required by the demand, and
their resolver-issued BindingRef-to-entry materialization, are already
installed in this function session. It is non-Clone, cannot cross a session,
and is consumed once by the physicalizer. A receipt containing only arity or a
source-site label is insufficient.

The argument list is a Prelude product, not a Loop demand field. It is
consumed before `ReadyLoopEntryV1` is issued and is never passed to the common
physicalizer. This preserves the single common physical algebra while keeping
call argument source proof at the callable boundary.

## Fresh session and atomic failure law

Neither demand owns freshness or rollback. Existing transactions do.

```text
A. semantic prepare failure
  -> no Builder/session effect

B. fresh session open + exact owner binding
  -> reversible caller-capture/function-state effect
  -> no MIR / ValueId / BasicBlockId emission yet

C. physical emission failure
  -> rollback only still-pending unpatched provisional PHIs
  -> retain any PHI cleanup failure in the typed failure
  -> discard the complete unpublished function session
  -> restore the captured caller once
  -> no repair, retry, fallback, or route advance

fresh request
  -> open a new candidate/session from source authority
  -> allocate new physical IDs
  -> lower independently
```

Stage A must complete before B. Exact owner/session binding in B must complete
before C. Any B/C failure discards the whole session; it does not return to A
or select another route.

The sole owners are:

- fresh module/candidate state: existing `ModuleBuilderInvocationSessionV1`
  with the canonical Fresh seed policy;
- fresh function state and caller capture:
  `CanonicalFunctionLoweringSessionV1` over the existing function-owned state
  transaction;
- PHI-local provisional abort: the session's `PhiTxn`;
- whole unpublished function discard:
  `CanonicalFunctionLoweringSessionV1::discard_unpublished` or rejected
  DraftSeal discard;
- function commit: `PreparedFunctionDraftSealV1::commit` through the prepared
  function-session close;
- module commit: the existing module transaction / `ModuleBuilderInvocationSessionV1`
  terminal after `ModuleDraftCollectorV1` admission.

There is no Loop-local Builder clone, `LoopEmissionDraft`, undo log, second
transaction, or same-session retry. A fresh-session proof compares semantic
result and live-caller fingerprints; it must not require `ValueId` or
`BasicBlockId` numbers to match across sessions.

`PhiTxn::abort_on_err` rolls back only provisional PHIs that are still pending
and unpatched. It is best-effort local cleanup and diagnostic hygiene, not the
atomicity owner. It does not repair patched PHIs, other MIR instructions, or ID
allocation. Even if PHI cleanup itself reports a suppressed failure, the
poisoned unpublished function is still removed by whole-session discard.

## One typed function-finish terminal

The new common path must not rely on every lowerer remembering this order.
`CanonicalSsaFunctionSessionV2` gains one consuming target API:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal(...)
  -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1>
```

Profile-specific ledgers remain with their profile lowerer. Before entering the
common terminal, that lowerer must consume them and provide one private
`ReadyCanonicalProfileCloseV1`. This is a temporal receipt, not a semantic
owner. The common terminal then consumes every common function-local owner and
is the only issuer of `ReadyFunctionDraftSealV1` for
`CanonicalSsaFunctionSessionV2` paths.

The target order is:

```text
1. materialize verified callable prelude and Recipe inputs
2. physicalize the recursive Loop, leaving After open
3. close the fixed profile's CFG edges and seal the After continuation
4. materialize the verified Tail operand and claim completion once
5. consume profile-specific ledgers -> ReadyCanonicalProfileCloseV1
6. close semantic scopes and seal the terminal CFG
7. finish CanonicalCfgSessionV1
8. finish semantic, If-control, and identity/BindingSSA preconditions
9. commit the one PhiTxn
10. finish the remaining resolved-binding ledger and
   ResolvedFunctionCompletionConsumptionV1
11. issue ReadyFunctionDraftSealV1
12. prepare every detached DraftSeal check
13. commit DraftSeal once
```

The current production resolved DirectAccum lowerer is a parity oracle, not the
final common owner. The earlier census found a missing whole-function
`CanonicalCfgSessionV1::finish` call; the typed
`CanonicalSsaFunctionSessionV2::finish_for_draft_seal` terminal now owns that
finish for the V2 path. The omission must not be copied into a future common
path. Existing non-V2 direct construction is frozen compatibility debt: the
first R0 adds no caller there, and final retirement makes
`ReadyFunctionDraftSealV1::new` unavailable to every production lowerer. Tests
may then use only an explicit test factory. For every migrated V2 path the
invariant is:

```text
ReadyFunctionDraftSealV1 exists
  == common CFG / SSA / PHI / binding / Completion owners are closed
  && the profile-specific close receipt was consumed
```

### R0 audit lock (2026-08-07)

The repository audit fixes the migration boundary before implementation. The
canonical V2 session is constructed by exactly three profile lowerers:

```text
trivial_ssa/lowerer.rs
direct_accum_lowerer.rs
nested_predicate_lowerer.rs
```

The current production `ReadyFunctionDraftSealV1::new` census contains those
three V2 callers plus one non-V2 `CanonicalFunctionLowererV1` compatibility
caller. R0 migrates only the three V2 paths. The non-V2 caller is a named
compatibility debt and may not gain new callers; its later retirement is a
separate decision. Test-only constructors are allowed only through an explicit
test factory and are not production evidence.

The finish API must consume a typed terminal receipt rather than re-deriving
source facts at the end of lowering. The target shape is conceptually:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal(
    self,
    builder,
    profile_close: ReadyCanonicalProfileCloseV1,
) -> Result<ReadyFunctionDraftSealV1, CanonicalFunctionFinishErrorV1>
```

The exact Rust visibility may remain private, but the contract is fixed:

- `body`, `body_end`, `target_function`, `current_block`, source site, and
  return operand are not re-inferred from raw AST/source/MIR arguments at the
  terminal. Function/body identity and completion target are sealed when the
  V2 session opens; the profile close receipt carries the exact terminal block
  and already-claimed completion witness.
- `ReadyCanonicalProfileCloseV1` is move-only, non-cloneable, and contains only
  profile-ledger closure evidence. It is a temporal receipt, not a new
  semantic owner or a second Completion/CFG/PHI authority.
- the common terminal is the sole issuer of `ReadyFunctionDraftSealV1` for V2
  sessions. A direct V2 `ReadyFunctionDraftSealV1::new` caller count of zero
  is a guard, not a prose claim.
- a mismatch, duplicate close, missing close, or completion/body identity
  mismatch rejects before the terminal consumes the session. Any failure
  after the fresh session opens discards the whole unpublished function and
  restores the caller once; same-session repair/retry is forbidden.

The R0 acceptance pack therefore includes all of the following, with no
profile or MIR acceptance delta:

```text
DirectAccum omission: missing cfg.finish cannot issue Ready/DraftSeal
finish order: CFG/semantic/If/identity/binding/Phi/completion close once
profile receipt: missing/duplicate/foreign receipt rejects
completion identity: body/site/end/target mismatch rejects before effects
late failure: unpublished function is discarded and caller is unchanged
fresh reuse: a failed session cannot poison the next session
caller census: V2 direct Ready constructor callers = 0
non-V2 census: compatibility caller remains named and non-growing
source/README/reference/current-entry update in the same implementation commit
```

This audit lock is deliberately narrower than a universal function-finalizer
redesign. It does not migrate the non-V2 lowerer, add a semantic owner, change
accepted profiles, or open physical Loop lowering.

### Callable production-edge census (2026-08-08)

The new callable physical products remain test-only:

```text
loop_physical_prepare.rs
callable_loop_physical_canary.rs
loop_recipe_physicalizer/callable_canary.rs
```

No production caller currently supplies
`PreparedCallableLoopPhysicalizationV1 -> profile-close -> Completion ->
DraftSeal`. The nearest production host is
`NormalCallableSemanticLoanPortV1::lower_normal_top_level_function`, whose
loop child edge still enters
`RawInvocationChildPortV1::lower_loop ->
PreparedLocatedRawLoopChildEntryV1::lower_with_existing_route_v1 ->
lower_loop_or_freeze_v1`. Its current output is a legacy pending function
session and `LegacyReplaceWholePair`, not `CompletedFunctionDraftV1`.

Therefore `CALLABLE-LOOP-PRODUCTION-EDGE-D0` closes as `NoSafeSlice`. The
Admission D0 confirmed that `NormalCallableSemanticLoanPortV1` is only a
production host/outer orchestrator. The accepted source/facts bridge design
does not add a semantic owner: `CallableSemanticSourceLedgerView` remains the
resolver source authority, while neutral SyntaxFacts and SourceMap are split
from test fixtures and promoted in
`CALLABLE-LOOP-PRODUCTION-SOURCE-FACTS-ISSUER-S0`. That source/facts slice is
closed with bounded negatives, exact resolver parity, and caller-zero audit.
The resolver seam is
`CallableSemanticSourceLedgerView::only_loop_site()` and the observer seam is
`FunctionSourceViewV1::stmt_at(membership)`; zero/multiple sites are typed
`NoSafeSlice`. The neutral SyntaxFacts and SourceMap issuers now compile in
production scope; their bounded entry uses resolver `only_loop_site()` plus
branded `stmt_at`, and the SourceFacts -> SourceMap parity receipt preserves
resolver identity. They still have no production caller or physical consumer.
Recipe/Prepared issuance remains closed; the next stop is the bounded logical
Recipe/JoinSig/After issuer implementation. A by-name adapter, fixture copying,
selector, retry, fallback, Generic G0 substitution, or legacy deletion is not
authorized by this census.

### Production admission contract (design-only)

The future production chain is fixed, but not implemented:

```text
NormalCallableSemanticLoanPortV1
  -> production source/facts bridge
  -> PreparedCallableLoopPhysicalizationV1
  -> fresh CanonicalFunctionLoweringSessionV1
  -> CanonicalSsaFunctionSessionV2
       (one physical Completion consumer issued from one scoped semantic borrow)
  -> Prelude / common Loop / After / Tail
  -> finish_for_draft_seal
  -> DraftSeal prepare/commit
```

Before production activation, `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` replaces the
three separately supplied semantic fields with one consumed
`VerifiedLoopSemanticProgramV1`. The demand may retain a private lookup index,
but it cannot expose `first`/`select`/`filter`, split the semantic program, or
reconstruct context/continuation from matching keys. The old multi-argument
issuer and any caller that manufactures context or continuation from parts are
deleted in the same Refactor Series.

The source-facts step must promote the existing neutral
`VerifiedSourceSyntaxFactsV1` and `VerifiedCallableSingleLoopSourceMapV1`;
it must not create a new aggregate Bridge owner. It may consume only
resolver-backed source/facts/forest/projection and callable lineage products.
It must not re-walk AST, recover names from route labels, infer Recipe keys
from MIR, or remove `cfg(test)` from a fixture issuer. The SourceMap does not
own Recipe/JoinSig, ABI, Completion, physical IDs, CFG/SSA/PHI, DraftSeal,
collector, or module publication. Until S0 is accepted, the production
ingress returns typed `NoSafeSlice` before opening a function session.

The sole unpublished-function/discard owner remains
`CanonicalFunctionLoweringSessionV1::discard_unpublished`. Adapter failure is
pre-effect rejection; every later failure discards the whole unpublished
function and restores the caller once. Phi rollback is auxiliary cleanup, and
same-session repair/retry/fallback is forbidden.

## Typed rejection boundary

Before Builder effects, reject at least:

```text
foreign owner/origin/source-kind/frame/Scope/Region
missing, duplicate, foreign, or unconsumed logical key/relation
Recipe item/block owner mismatch
JoinSig port/edge mismatch
input without an exact preheader producer
prefix target/result ABI unavailable
prefix destination BindingRef mismatch
terminal value site/BindingRef mismatch
missing or unsupported exact return ABI
completion owner/site/result-kind mismatch
Loop After confused with callable Tail
unsupported logical operation, exit, or recursive depth
second Recipe/SSA/CFG/PHI/completion owner
physical ID or Builder capability present in the demand
```

These are typed `NoSafeSlice`/contract rejections. They do not fall back to a
profile-specific physicalizer or the 19-route scheduler.

After physical effects begin, any failure is terminal for that unpublished
session. It is not reclassified as a pre-effect decline.

## One recursive algebra; 19 is coverage only

The canonical full operation demand accepts one recursive Loop Recipe algebra
through an exact V1 or V2 projection. V2 adds typed operation/value vocabulary;
it is not converted into V1 and does not create another physicalizer. The
algebra does not contain `DirectAccum`, `GenericG0`, `LoopTrue`, `LoopCond`, or
the 19 legacy route labels as physical variants.

```text
source profiles/adapters: many bounded rows
portable Recipe algebra:  one, with exact V1/V2 projections
prepared profiles:        bounded callable/G0 compatibility products
full operation demand:    one
common physicalizer:      one
```

If the selected callable profile cannot later enter the existing family
selection envelope exactly, production selection returns `NoCandidate` and
parks it. Shape similarity must not relabel it as LoopV0 or Generic G0, and a
20th Recipe kind or second selector is forbidden.

## Finite implementation ladder

The bounded design is closed, but physical activation is intentionally split
into three mechanical commits. This is not a new semantic ladder: each row
consumes an existing owner and has one named temporal prerequisite. Do not
skip the After closure or reopen a Tail-only route.

| Order | Row | One claim | Stop line |
| ---: | --- | --- | --- |
| 0 | `RECIPE-COSEAL-I0-R0` | caller-zero common logical co-seal plus separate Prelude/Tail source contracts | closed caller-zero implementation; no ABI/Completion issuance |
| 1 | `CANONICAL-FUNCTION-FINISH-TERMINAL-R0` | migrate existing canonical V2 paths to one `finish_for_draft_seal` issuer; freeze non-V2 direct construction as compat debt | BoxShape-only; accepted profiles and MIR unchanged |
| 2 | `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0` | fix callable input/prelude/terminal/G0/lifetime pairings in the existing prepare design | design-only; no code, Builder, physicalizer, selector, or caller |
| 3 | `LOOP-PHYSICAL-PREPARE-P0` | caller-zero common demand plus callable prepared product; exact ABI/Completion are consumed from existing issuers | no physicalizer, Builder emission, selector, or I0 claim |
| 4 | `GENERIC-G0-PHYSICAL-PREPARE-P0` | exact-move G0 adapter issues the same inner demand plus distinct G0 Tail | `NoSafeSlice` if source truth must be copied or reverified |
| 5 | `LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` | resolver-issued variable-only i64 argument rows -> one move-only Prelude product | caller-zero; no Builder physicalizer or selector |
| 6 | `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` | closed test-only inner demand + `ReadyLoopEntryV1` + borrowed V2 services -> topology/After continuation | no production caller; operation MIR remains `NoSafeSlice` |
| 7 | `LOOP-RECIPE-OPERATION-EFFECT-PLAN-D0` | one neutral `LoopItemKey` + exact source-anchor effect projection before operation emission | closed preparation; no production caller |
| 8 | `CALLABLE-LOOP-AFTER-CLOSURE-P0` | complete fixed callable operation schedule, issue CFG edges, seal CFG/identity, and mint one `ReadyLoopAfterContinuationV1` | closed caller-zero; no production selection |
| 9 | `CALLABLE-LOOP-TAIL-COMPLETION-P0` | consume sealed After, read exact Tail binding, `mark_return`, and claim completion once | closed caller-zero; no selector, retry, or fallback |
| 10 | `CALLABLE-LOOP-DRAFT-SEAL-P0` | consume profile close, call only `finish_for_draft_seal`, then DraftSeal prepare/commit | closed caller-zero; production selection and legacy deletion remain closed |
| 11 | `LOOP-CALLER-ZERO-PARITY-G0-D0` | accepted design: compiler-side exact-input composite ingress, neutral S4 owner, common physicalizer, distinct G0 After/Tail | no source reconstruction, physical emission, or production selection |
| 12 | `LOOP-CALLER-ZERO-PARITY-G0-I0-R0` | exact G0 ingress -> common fifteen-row `prepare_all` with Builder effect zero | closed 2026-08-08; no physical emission, Completion/DraftSeal, selector, retry/fallback, or legacy deletion |
| 13 | `LOOP-CALLER-ZERO-PARITY-G0-I1-D0` | top-down counterexample fixes segment/resume as a common prerequisite | superseded historical design; R1/R2/R3-I0 closed |
| 14 | `LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` | Builder-free Recipe-derived segment/resume layout plus exact order/coverage | **closed 2026-08-08**; no Builder effect or new accepted structural family |
| 15 | `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` | exact segment-to-old-topology adapter and operation placement; Callable parity | **closed 2026-08-08**; not a segment allocator; no G0 physical |
| 16 | `LOOP-COMMON-RECURSIVE-AFTER-R3-I0` | exact segment allocator, retained completed program, complete transfer preflight, neutral After handoff | **closed 2026-08-08** for Callable caller-zero; G0 physical and production selection remain closed |
| 17 | `LOOP-CALLER-ZERO-PARITY-G0-I1-D1` | per-transfer Predicate receipts, neutral After boundary, and common DerivedCarrierEntry emitter contract | **accepted design 2026-08-08**; implementation is split into the common I0 row below and G0 I1 |
| 18 | `LOOP-COMMON-PREDICATE-CARRIER-I0-R0` | common per-transfer Predicate values plus profile-neutral DerivedCarrierEntry emission | **closed 2026-08-08**; no G0-specific owner or production selection |
| 19 | `LOOP-CALLER-ZERO-PARITY-G0-I1-R0` | exact parameters, five segments + root After, all fifteen operations, distinct Tail/Completion, finish/DraftSeal | **closed 2026-08-08** caller-zero; no G0-specific physicalizer |
| 20 | existing M8 S6A..S6G + M9 S7A..S7G | close all-19 ingress coverage and Rust/.hako portable producer parity | repository-wide convergence; not a prerequisite for the bounded selected H2 first cutover unless its unchanged source needs that family |
| 21 | `LOOP-SEMANTIC-PROGRAM-COSEAL-R0` | Callable-first BoxShape: consume one complete source-backed Callable parent into one private semantic-program envelope; generic G0/all-family issuer remains later | accepted design 2026-08-17; no production change |
| 21a | `LOOP-SEMANTIC-PROGRAM-COSEAL-CALLABLE-I0` | caller-zero implementation of the one-shot Callable envelope and mechanical physical-demand projection; remove split caller ingress | **landed 2026-08-17**; no CFG/SSA/PHI, session effect, lifecycle, Text, route, fallback, retry, or production caller |
| 22 | `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | one private traversal, JoinSig-issued transfers, Layout binding only, direct transfer inference deletion | **closed 2026-08-18**; implementation `542b3a794d` plus direct view/binder and allocator negatives; no old V1 caller migration |
| 22a | `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0` | make V1/V2 physical consumers borrow one complete ordered operation/source-effect ledger; remove repeated Recipe/evidence `find` scans | implementation present in `28c4bdd5c4`; behavior-preserving only, no V2-to-V1 adapter or new source/effect authority |
| 22b | `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` | move Callable profile-close/Tail/ABI/Completion out of the common Loop physicalizer; common stop is `ReadyLoopAfterContinuationV1` | implementation present in `46fbf8d0d7`; BoxShape only, no profile callback, selector, or production switch |
| 22c | `LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0` | parent BoxShape: order the installed child, TextFormal mapping, one Completion owner, and generic V2 operation/control envelope | closed design boundary 2026-08-16; one parent HRTB/sibling views, generic operation/control partition, and passive coverage are fixed; no session effect |
| 22c-a | `CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0/I0` | accepted mapping: one logical ExactText ordinal/BindingRef -> adjacent scalar `[slot,generation]` lanes; issue one complete/disjoint Completion-independent package cohort and transport it through one combined Installed S6C loan | closed caller-zero implementation; no call-edge actualization, `ValueId`, aggregate ABI, fallback, or retry |
| 22d | `LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0` | transport the generic parent/sibling boundary through one installed Port HRTB without emitting an execution product | closed caller-zero source transport 2026-08-16; one selected-key consumption seam; no JSON/MIR, route policy, Builder/session effect, or production caller |
| 22e | `LOOP-S6C-COMMON-V2-PRESESSION-I0` | implement the named source-backed operation adapter, JoinSig/Recipe control co-seal, and passive coverage issuer inside one caller-zero parent loan | closed caller-zero implementation 2026-08-16; focused positive/negative/duplicate tests green; no S6C physicalizer, Builder/session, lifecycle, route, or production caller |
| 22f | `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` | census fixed-role receipts versus segment receipts and publish the caller-zero deletion gate | guard/docs present in `1544d128d2`/`1e93ad6be9`; transitive old-edge census remains before deletion |
| 23 | `LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` | add one JoinSig-authorized Always physical family | one BoxCount commit; no fallback |
| 24 | `LOOP-PHYSICAL-IF-COVERAGE-I0` | consume one existing V2 branch-arm view with a named physical merge owner | design stop: NoSafeSlice until merge authority/consumer are named; no V2-to-V1 adapter or Layout inference |
| 24a | `LOOP-PHYSICAL-IF-CONTINUATION-RELATION-I0` | issue one JoinSig `NextItem` continuation for an Exit+Fallthrough branch and transport it through the existing V2 control view | landed; positive/negative pre-Layout evidence; no BlockEnd, two-normal-arm PHI, session, or physical effect |
| 24b | `LOOP-PHYSICAL-IF-CONTINUATION-CONSUMER-D0` | name the sole physical/session consumer and rollback owner for the borrowed `NextItem` relation | accepted placement-only BoxShape; target-placement I0 landed; branch-emission D0 is next |
| 24b-a | `LOOP-PHYSICAL-IF-CONTINUATION-TARGET-PLACEMENT-I0` | validate one existing segment and reserve one canonical unpublished continuation target block for the exact `NextItem` | landed 2026-08-18; focused positive/duplicate/late-discard gates green; no edge or instruction |
| 24b-b | `LOOP-PHYSICAL-IF-CONTINUATION-BRANCH-EMISSION-D0` | name item-to-split and one-sided terminal/continuation terminator authorities before physical emission | accepted NoSafeSlice; current placement view cannot issue split/FunctionExit meaning; no code or physical effect |
| 24b-c | `LOOP-PHYSICAL-IF-CONTINUATION-SPLIT-TERMINAL-AUTHORITY-D0` | co-seal the source-backed continuation item-to-split and one-sided Return/FunctionExit terminal authorities | accepted NoSafeSlice; separate source site/value and Exit item/block/value lack a canonical co-seal issuer |
| 24b-d | `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COSEAL-D0` | name the source issuer that binds Return site/value to the logical Exit and FunctionExit arm | accepted NoSafeSlice 2026-08-18; existing source/Recipe/JoinSig issuers remain separate; no code, receipt, or physical effect |
| 24b-e | `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-ISSUER-BOUNDARY-D0` | design and accept one same-cohort canonical issuer boundary before any Return relation receipt | accepted NoSafeSlice 2026-08-18; common aggregation sees the loan but lacks source-to-key provenance; no code, receipt, or physical effect |
| 24b-f | `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-D0` | define source-layer Return-to-Recipe/Join key binding and the sole issuer handoff | accepted BoxShape 2026-08-18; source co-seal retains region/binding evidence, Recipe producer owns the key relation; I0 below |
| 24b-f-I0 | `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-I0` | issue and transport one non-Clone source-to-Recipe/Join relation with negative drift guards | landed 2026-08-18; focused S6C suite 9/9 green plus existing call/row/domain negatives; no physical block/edge/Return/PHI, session, production, fallback, or retry |
| 24b-g | `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-AGGREGATE-D0` | name the sole common pre-session consumer and rollback boundary for the borrowed source-to-Recipe/Join relation | design stop: common envelope does not yet retain this relation; no aggregation, physical emission, session, production, fallback, or retry |
| 25 | `LOOP-PHYSICAL-EXIT-COVERAGE-I0` | add item-keyed Break/Continue/Return transfer capabilities and common physicalization | one BoxCount commit; no route-local exit writer |
| 25a | `LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-D0` | fix the two-stage admission BoxShape and census its three source authorities | accepted 2026-08-16; outer-If and Completion reuse existing issuers, and typed BlockExpr issuance/transport are now landed |
| 25a-a | `RESOLVED-BLOCK-EXPR-EXPECTATION-I0` | co-seal typed BlockExpr body-shape sites with the exact resolver scope/region pairs and store one non-Clone receipt in the callable batch row | landed 2026-08-17; no selected/package transport, raw count API change, or session effect |
| 25a-b | `CALLABLE-BLOCK-EXPR-EXPECTATION-TRANSPORT-I0` | lend the batch-owned expectation through the existing selected/package HRTB | landed 2026-08-17; transport only, no reissue, clone, Completion consumption, or session construction |
| 25a-c | `LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0` | co-seal exact Loop outer-If residual, typed BlockExpr expectation, common V2 envelope, and actual borrowed Completion in one callback-scoped admission | landed 2026-08-17; caller-zero/effect-free; no `CanonicalSsaFunctionSessionV2`, DraftSeal, lifecycle, Return rescan, or legacy-finalizer retrofit |
| 25b | `LOOP-COMMON-V2-PHYSICAL-SESSION-I0` | consume the accepted admission and open one caller-zero canonical session owner without exposing a second loan | landed 2026-08-17; typed expectation projects inside `new_common_v2`, borrowed Completion yields one owned physical consumer, and the envelope remains callback-scoped; no Builder/CFG effect, claim, DraftSeal, lifecycle, or physicalizer |
| 25b-a | `LOOP-S6C-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-D0/I0` | S6C-only same-cohort physical function-entry input and descriptor projection | landed caller-zero S6C seam; this does not issue Generic/common ABI and does not open skeleton, lane adoption, Loop CFG/block allocation, operation/control physicalization, PHI, Completion claims, DraftSeal, lifecycle, route, fallback, retry, or production caller |
| 25b-b | `LOOP-COMMON-V2-PHYSICAL-HEADER-COSEAL-D0` | accept one package/installed-loan issuer for S6C storage header, result, attrs/uses, source-backed effects, and physical signature relation | accepted BoxShape; caller-zero I0 is the only open effect; no skeleton or Builder effect |
| 25b-b-I0 | `LOOP-COMMON-V2-PHYSICAL-HEADER-COSEAL-I0` | issue/transport the same-brand S6C storage header and source-backed physical-effects projection beside the existing signature | landed 2026-08-17; focused package/S6C tests green; no session, skeleton, ValueId, ExactText adoption, Loop block, PHI, Completion claim, DraftSeal, lifecycle, route, fallback, retry, or production caller |
| 25b-c0 | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-D0` | carrier choice is fixed as package-owned `U64BitsOnI64` over the existing i64 mechanical carrier; define the same-loan physical-parameter descriptor/lane-role contract, including source ParamDecl, receiver, and ExactText pair policy | accepted BoxShape 2026-08-17; no skeleton, ValueId, lane adoption, Loop blocks, PHI, Completion claim, DraftSeal, lifecycle, route, fallback, or production caller |
| 25b-c0-I0 | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-INPUT-I0` | consume one accepted same-loan view and expose nonsemantic physical parameter descriptors for the later skeleton consumer | landed 2026-08-17; caller-zero transport only; no skeleton allocation, ValueId, BindingSSA, Completion consumption, Loop CFG, lifecycle, route, fallback, or production caller |
| 25b-c0-G0 | `LOOP-GENERIC-G0-PHYSICAL-ENTRY-SOURCE-PROJECTION-D0` | census a Generic G0 TopLevel declaration/header, result/ABI, function-effect, Completion, and source storage/lane cohort without borrowing S6C receipts | accepted source-only BoxShape 2026-08-17; body-shape transport, function-effect receipt, result-ABI transport, canonical Completion transport, and storage/lane source policy are landed/accepted; no physical signature, EffectMask, skeleton, ValueId, BindingSSA, CFG/PHI, Completion consumption, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-storage-lane | `LOOP-GENERIC-G0-STORAGE-LANE-SOURCE-PROJECTION-D0` | accept the same-parent source storage/header, receiver-policy, and explicit-row BoxShape; keep receiver separate from explicit formal arity and forbid S6C reuse | accepted 2026-08-17; next source-only caller-zero I0; no physical signature, EffectMask, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-storage-lane-I0 | `LOOP-GENERIC-G0-STORAGE-LANE-SOURCE-PROJECTION-I0` | retain one private/non-Clone Generic parent-owned source row: attrs/uses, receiver policy/BindingRef, dense explicit rows, and checked mechanical `ExistingCallableI64` carrier tag | landed 2026-08-17; six focused source-parent tests green; checked explicit/callable counts and instance/absent policy are source-only; next was the Generic entry BoxShape; no physical/session effect |
| 25b-c0-G0-entry | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-D0` | accept one Generic-only pre-effect entry-input BoxShape over the same source parent; forbid S6C descriptor/header/signature reuse and keep receiver prefix separate from explicit arity | accepted 2026-08-17 after issuer census; its caller-zero input I0 is landed; the next stop is Generic physical-function skeleton D0; no skeleton, ValueId, BindingSSA, EffectMask, Builder/session, Completion consumption, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-entry-I0 | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-I0` | project one same-parent Generic source row into private non-Clone mechanical entry descriptors with receiver policy, dense explicit rows, metadata, and existing i64 carrier | landed 2026-08-17; focused positive plus parent rejection/no-publication gates green; no S6C/common descriptor reuse, skeleton, ValueId, BindingSSA, EffectMask, Builder/session, Completion consumption, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-physical-effect | `LOOP-GENERIC-G0-PHYSICAL-EFFECT-PROJECTION-D0` | accept one Generic-only source-to-physical `EffectMask` mapping from the parent no-external-effect receipt before any skeleton effect | accepted BoxShape 2026-08-17; the finite five-variant local/pure operation contract and same-cohort target/frame parity are fixed; no skeleton, ValueId, BindingSSA, Builder/session, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-physical-effect-I0 | `LOOP-GENERIC-G0-PHYSICAL-EFFECT-PROJECTION-I0` | issue one private same-cohort physical-effect projection and focused no-publication/rejection gates | landed 2026-08-17; focused Generic suite 58/58 green; source/evidence parity and `EffectMask::PURE` projection only; skeleton, `MirFunction`, ValueId, Builder/session, CFG/PHI, lifecycle, Text, route, fallback, retry, and production caller remain closed |
| 25b-c0-G0-skeleton | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-D0` | accept the Generic-only detached skeleton reservation: source explicit-arity symbol, receiver-prefix/physical lanes, i64 result, same-cohort PURE effect, exact-empty metadata, and non-Clone rollback owner | accepted BoxShape 2026-08-17; next caller-zero I0 is allocation-only; no entry adoption, Builder/session, Completion, CFG/PHI, lifecycle, Text, route, fallback/retry, or production caller |
| 25b-c0-G0-skeleton-I0 | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-I0` | reserve one unpublished detached Generic `MirFunction` from the consumed same-parent entry/effect cohort and retain it in a non-Clone wrapper | landed 2026-08-17; two focused tests plus Generic suite 60/60 green; explicit `/N`, receiver-prefix ordering, PURE/i64 signature, exact-empty metadata, and descriptor preflight are covered; entry adoption remains closed |
| 25b-c0-G0-entry-adoption | `LOOP-GENERIC-G0-PHYSICAL-ENTRY-LANE-ADOPTION-D0` | accept one Generic callback-scoped admission that co-seals existing parent/BlockExpr/outer-If/Completion views with the detached shell and a Generic mechanical cohort stamp | accepted BoxShape 2026-08-17; S6C stamp/session reuse remains forbidden; the admission issues no new semantic fact and is consumed only by the next I0 |
| 25b-c0-G0-entry-adoption-I0 | `LOOP-GENERIC-G0-PHYSICAL-ENTRY-LANE-ADOPTION-I0` | consume one Generic admission, open one fresh unpublished transaction, install the detached shell, and atomically adopt receiver/ordinary declarations through the canonical SSA issuer | landed 2026-08-17; two focused entry-session tests plus Generic suite 62/62 green; no Loop CFG/operations/PHI/Completion claims, lifecycle, Text, route, fallback/retry, or production caller |
| 25b-c0-G0-effect-transport | `LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0` | transport the same-resolver body-shape product through the source unit/root input into the Generic cohort; no count-only effect receipt | landed 2026-08-17; owner/body-root checks and bare-input/foreign-cohort negatives green; no effect issuer, EffectMask, skeleton, session, or Builder |
| 25b-c0-G0-effect | `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-D0` | use the transported body-shape sibling for a resolver-owned census of body effects, calls, metadata-empty witness, and Generic structural facts; issue no physical EffectMask | accepted BoxShape 2026-08-17; next caller-zero I0 is the private source receipt; no physical EffectMask/session |
| 25b-c0-G0-effect-I0 | `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-I0` | issue one same-cohort private non-Clone Generic no-external-effect receipt before demand/product consumption | landed 2026-08-17; focused source-receipt and late-failure gates green; no physical/session effect |
| 25b-c0-G0-result | `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-D0` | transport the existing same-cohort Generic return ABI row before any Completion or physical entry | accepted BoxShape 2026-08-17; no new classifier, combined result/Completion receipt, default ABI, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-result-I0 | `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-I0` | retain one candidate-owned result ABI row in the Generic parent before demand/product consumption | landed 2026-08-17; focused exact/foreign transport tests green; no Completion, physical ABI, EffectMask, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-completion | `LOOP-GENERIC-G0-COMPLETION-PROJECTION-D0` | retain the canonical resolver Completion in the Generic parent after result-ABI transport, with Generic tail/result/cleanup parity | accepted BoxShape 2026-08-17; canonical verifier remains the sole issuer; no Completion consumption, physical ABI/lane, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-completion-I0 | `LOOP-GENERIC-G0-COMPLETION-PROJECTION-I0` | issue `verify_function_completion_v1(input)` once and lend the canonical non-Clone product through the parent callback | landed 2026-08-17; focused source-parent tests green; transport only, with no Completion consumer, physical/session effect, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-header | `LOOP-GENERIC-G0-TOPLEVEL-DECLARATION-HEADER-I0` | source-backed TopLevel declaration/header projection in the existing Generic cohort | landed 2026-08-17; parent physical-entry blocker remains; no result/lane/effect/Completion/skeleton/session |
| 25b-c0-converge | `MIRBUILDER-CANARY-CONVERGENCE-CHECKPOINT-R0` | read-only census of duplicate receipts, canary owners, retirement conditions, legacy edges, semantic-program tuple escape hatches, and S6C-only provenance adapters after the Generic physical-entry cohort | design-stop envelope; its concrete deliverable is the manifest below, with no new authority or production switch |
| 25b-c0-converge-manifest | `MIRBUILDER-CANARY-CONVERGENCE-MANIFEST-R0` | publish one owner/final-consumer/zero-caller deletion manifest for the six remaining seams before naming another physical owner | landed 2026-08-17; deletion owners and retirement gates are recorded, with no production switch |
| 25b-c0-structure-audit | `MIRBUILDER-STRUCTURAL-CONVERGENCE-AUDIT-R0` | classify intentional keep, waiting scaffolding, bounded retirement, and parked inventory; record scale as informational rather than authority | landed as design-only audit 2026-08-17; current design-stop blocker and work mode remain explicit in CURRENT_STATE |
| 25b-c0-semantic-consume-D0 | `MIRBUILDER-SEMANTIC-PROGRAM-CONSUME-D0` | close the source owner, direct-consumer shape, caller census, parity checks, and zero-caller retirement contract for the Callable six-tuple escape hatch | accepted BoxShape 2026-08-17; source-free prepared-demand parent, one-shot consumer, and no-six-tuple boundary are fixed |
| 25b-c0-semantic-consume-I0 | `MIRBUILDER-SEMANTIC-PROGRAM-CONSUME-I0` | replace the production six-tuple `VerifiedCallableSemanticProgramV1::into_prepared_parts` consumer with one direct source-free prepared-demand parent consumer | landed 2026-08-17; compiler 3/3 and Builder 1/1 focused tests, cargo check, format, diff, and zero-caller census are green |
| 25b-c0-lease-facade | `RUNTIME-END-AUTHORIZED-TEXT-FACADE-I0` | narrow the public `EndAuthorizedTextV1`/getter facade without changing lease semantics or the Generic MIR owner graph | parked in the runtime lane; no MIR receipt, Text route, or production caller change is authorized here |
| 25b-c0-disposition | `MIRBUILDER-LEGACY-DISPOSITION-R0` | fill the legacy disposition TSV only from observed owner/caller/parity evidence and attach a deletion gate per non-sentinel decision | parked; no bulk relabeling, route activation, or deletion by LOC |
| 25b-c0-helper-inventory | `MIRBUILDER-BYTE-HELPER-INVENTORY-R0` | inventory byte-identical helper groups and micro-seed families with an owner and evidence command, without semantic dedupe | parked informational census; merge/delete requires a separate source-backed owner and focused parity slice |
| 25b-c0-G0-operation-contract | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-CONTRACT-D0` | census a Generic-only operation contract over the finite five-variant source/evidence set without S6C provenance reuse or Builder effect | accepted BoxShape 2026-08-17; the bounded mechanical mapping I0 is landed, while operation MIR and production remain closed |
| 25b-c0-G0-operation-mapping-I0 | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-MAPPING-I0` | project the same-parent Generic operation/evidence product into one private mechanical five-variant mapping; preserve item/BindingRef/value-class identity and keep item 4/carrier/tail out | landed 2026-08-17; focused mapping test is green with 15-row coverage and item-4 exclusion; no Builder, ValueId, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-operation-emission | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-EMISSION-D0` | name the sole Generic/common physical operation emitter and its five variant lowering boundary from the landed mapping | accepted ownership BoxShape 2026-08-17: one prephysical admission, no detached `MirFunction`, session-owned shell/rollback, existing dispatcher only; leaf emission remains closed behind the later session-preflight D0 |
| 25b-c0-G0-operation-cohort | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-COHORT-D0` | resolve the borrowed Generic mapping versus owned `PreparedLoopOperationProgramV1` lifetime with one source-owned one-shot cohort/port; choose transient mapping or owned cohort without self-reference | accepted BoxShape 2026-08-17 after ownership census; next caller-zero cohort I0 only; no operation MIR, Builder, ValueId, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-operation-cohort-I0 | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-COHORT-I0` | consume the Generic source parent once, own the family-neutral program and independent source siblings, and lend only a callback-scoped transient mapping | landed 2026-08-17; focused cohort test green; program/mapping preflight only, with no operation leaf, Builder, ValueId, CFG/SSA/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-operation-emitter-cohort-D0 | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-EMITTER-COHORT-D0` | consume one Generic source parent exactly once and name the combined non-Clone prephysical admission owning layout/program, shell plan, entry-control facts, Completion, target, and one full stamp | accepted BoxShape 2026-08-17; actual `MirFunction`, raw IDs, parent borrow, stored mapping, S6C/old-V1 adapter, and second rollback owner are forbidden |
| 25b-c0-G0-emitter-facts-I0 | `LOOP-GENERIC-G0-PHYSICAL-EMITTER-FACTS-EXTRACTION-I0` | extract existing pure entry/effect/shell/control validators into private source-parts helpers, narrow the sole source-parent construction seam, and keep old canary behavior through delegation | landed 2026-08-17; borrowed source-parts view plus shared-axis parity tests/guards are green; no new admission, `MirFunction`, Builder/session, ValueId, layout, dispatcher, fallback/retry, or production caller |
| 25b-c0-G0-operation-emitter-admission-I0 | `LOOP-GENERIC-G0-PHYSICAL-EMITTER-ADMISSION-I0` | consume the existing operation cohort once into `PreparedGenericG0PhysicalEmitterAdmissionV1`, owning the neutral layout/program, shell plan, entry-control facts, Completion, target, and full stamp; lend mapping only inside HRTB | landed 2026-08-17; five focused and 68 Generic tests plus structural/size guards are green; detached canary dependencies were removed by the later retirement R0; no function/session/raw-ID/dispatcher effect or production caller |
| 25b-c0-G0-operation-emitter-session-D0 | `LOOP-GENERIC-G0-PHYSICAL-EMITTER-SESSION-PREFLIGHT-D0` | accept one whole-admission family-neutral unpublished consumer for shell materialization, lane adoption, canonical entry projection, layout-keyed segment preflight, and sole rollback | accepted BoxShape 2026-08-17; implementation remains bounded behind `LOOP-GENERIC-G0-SEALED-CONSUME-I0`; `ReadyLoopEntryV1::new_for_test`, S6C input, owner-only re-pairing, and leaf effect remain forbidden |
| 25b-c0-G0-sealed-consume-I0 | `LOOP-GENERIC-G0-SEALED-CONSUME-I0` | isolate the caller-zero production-visible `into_physical_boundary` split behind `cfg(test)` without changing the source-parent/cohort/admission path | landed 2026-08-17; Generic suite, cargo check, fmt, diff, and caller census are green; detached-canary tuple exits were subsequently removed by the parity retirement R0 |
| 25b-c0-G0-operation-emitter-session-I0 | `LOOP-GENERIC-G0-PHYSICAL-EMITTER-SESSION-PREFLIGHT-I0` | after the D0 and sealed-consume prerequisite are accepted, consume one admission into the sole unpublished session and prepare exact entry/segment dispatch inputs without emitting an operation | landed 2026-08-17; retained source rows, shell/adoption, canonical preheader reads, layout-keyed segment allocation, and late-discard tests are green; operation leaf, publication, retry, fallback, and production remain closed |
| 25b-c0-G0-entry-canary-retire | `GENERIC-G0-ENTRY-CANARY-RETIREMENT-R0` | after session-preflight parity, migrate focused tests, delete the detached skeleton/canary admission/session and their tuple exits, and share only reserved-parameter validation | landed 2026-08-17; old detached callers are zero, validators live in neutral facts modules, and the combined admission/session path is the only retained Generic probe |
| 25b-c0-G0-operation-emitter-I0 | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-EMITTER-I0` | consume the callback-scoped program/layout/entry/segment input through the canonical unpublished session and emit the existing five-variant operation rows | landed 2026-08-17; existing common dispatcher emits inside the outer unpublished transaction, late failure discards, and completion/publication/production remain closed |
| 25b-c0-G0-operation-dispatch-preflight-I0 | `LOOP-GENERIC-G0-PHYSICAL-OPERATION-DISPATCH-PREFLIGHT-I0` | bind admission-owned program/layout to session-issued canonical entry and segment receipts, then run the existing segment-dispatch preflight without a leaf effect | landed 2026-08-17; one callback-scoped mechanical view and no-leaf preflight are green; operation emission remains a separate row |
| 25b-c | `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON-I0` | reserve one fresh unpublished physical function skeleton from the accepted same-cohort entry input | landed 2026-08-17; detached mechanical-i64 shell and descriptor retention only; no Builder installation, ExactText adoption, Loop blocks, PHI, Completion claim, DraftSeal, lifecycle, route, fallback, or production caller |
| 25b-d | `LOOP-COMMON-V2-PHYSICAL-ENTRY-LANE-ADOPTION-D0` | accept the one-value BindingSSA plus private generation-sidecar adoption and its fresh-transaction rollback owner | accepted BoxShape 2026-08-17; slot-only publication and skeleton-bound sidecar are fixed; no Loop CFG/PHI, lifecycle, route, fallback, or production caller |
| 25b-d-I0 | `EXACT-TEXT-ENTRY-LANE-ADOPTION-I0` | consume one prepared skeleton for ordinary lanes and one logical ExactText slot lane plus adjacent private generation sidecar | landed caller-zero canary 2026-08-17; positive install/adopt and duplicate-adoption rejection are green, but atomic same-cohort/session ownership remains the next design stop |
| 25b-e | `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM-D0` | bind retained skeleton, descriptor cohort, common-V2 session, slot-only BindingSSA, sidecar, and one discard/poison owner into a consuming transaction | accepted BoxShape 2026-08-17; compiler-only consuming input and Builder rollback owner fixed; no Loop CFG/PHI, lifecycle, route, fallback, or production caller |
| 25b-e-I0 | `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM-I0` | consume one prepared input and one common-V2 admission, install/adopt once, and return only a callback-scoped success view | landed 2026-08-17; same-loan admission, fresh Builder transaction, slot-only BindingSSA plus generation sidecar, and outer discard/no-retry are covered by positive and late-failure tests; no Loop CFG/PHI, lifecycle, route, fallback, or production caller |
| 25b-f | `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-D0` | accept one source-backed V2-native physical-ID-free layout/placement BoxShape | accepted 2026-08-17; topology transport is the only next I0; no block/effect emission, Loop CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-f-I0 | `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0` | lend typed loop/block/item topology and JoinSig transfer bindings through the same common-V2 cohort | landed 2026-08-17; relation guard is green; no Builder/block allocation, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-f-I0-RG | `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-I0-RELATION-GUARD` | require each operation/If/Exit item to belong to its specified layout block and add focused negatives | landed 2026-08-17; positive transport plus operation/If/Exit block-drift negatives are green; no Builder/block allocation, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-g | `LOOP-COMMON-V2-PHYSICAL-ENTRY-EFFECTS-D0` | after layout input is sealed, name the first source-segment block allocation carrier and rollback boundary | accepted BoxShape 2026-08-17; monotonic unpublished-ID gaps are explicit, synthetic After allocation is a separate D0; no ReadBinding/effect emission, Loop CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-g-I0 | `LOOP-COMMON-V2-PHYSICAL-SEGMENT-BLOCK-ALLOCATION-I0` | consume one accepted segment allocation plan and allocate only ordered source-segment blocks under one outer discard owner | landed 2026-08-17; positive and late-discard gates are green; no synthetic After block, edges/terminators, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-h | `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0` | issue a source-backed synthetic After row with root/resume relation and its separate allocation owner | accepted BoxShape 2026-08-17; typed RootAfter transport I0 landed, ParentResume remains parked until its issuer input exists, and the allocation D0/I0 are now landed before any After edge/terminator, operation/read/Const, CFG/PHI, Completion claim, DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-h-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-I0` | transport the typed RootAfter/ParentResume boundary relation through the same common-V2 cohort | landed 2026-08-17; RootAfter is the only admitted S6C arm, ParentResume remains parked, and no block/edge/terminator/effect/CFG/PHI/Completion/DraftSeal/lifecycle/Text/route/production is open |
| 25b-i | `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0` | accept one RootAfter-only one-shot unpublished After placement and its outer discard owner | accepted BoxShape 2026-08-17; one prepared plan, canonical BasicBlockId issuance, exact segment coverage, and monotonic unpublished cursor gaps are fixed; the next I0 is allocation-only |
| 25b-i-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0` | consume the private plan and allocate exactly one unpublished After block | landed 2026-08-17; positive/one-shot/late-discard gates are green; no After edge, successor, operation, CFG/PHI, Completion, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-j | `LOOP-COMMON-V2-PHYSICAL-AFTER-EDGE-D0` | close the source-backed complete Predicate branch plan and condition-carrier admission for Header -> Body / Header -> RootAfter | accepted BoxShape 2026-08-17; the false-only edge is rejected because canonical `emit_branch` requires both successors; the next caller-zero I0 transports the physical-ID-free plan; no edge/terminator, operation, CFG/PHI, Completion, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-j-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-BRANCH-PLAN-I0` | transport one typed complete predicate branch plan plus condition-carrier requirement from the same S6C cohort | landed 2026-08-17; focused positive/duplicate/missing-boundary gates are green; no ValueId issuance, `emit_branch`, CFG mutation, operation/read/Const, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-k | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-CARRIER-D0` | name the source-backed physical condition carrier and its canonical issuer before any edge effect | accepted BoxShape 2026-08-17; logical CompareI64 producer relation is the next transport-only I0, while physical ValueId/operation/edge effects remain closed |
| 25b-k-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-PRODUCER-I0` | transport one exact source-backed CompareI64 producer relation for the root predicate | landed 2026-08-17; source/operation row, owner, block, operand/result/class drift and non-Compare negatives are green; no ValueId issuance, Compare emission, `emit_branch`, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-l | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-D0` | parent boundary for same-session operand receipts, stamp retention, and the canonical physical result | parent remains blocked; operand inventory and stamp retention are landed, while the result BoxShape below must name the single plan/receipt owner before any physical effect |
| 25b-l-a | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0` | co-seal the source Length contract, CallSlot/result/class, matching common operation row, and Compare-right key from one S6C ingress | accepted BoxShape 2026-08-17; fixed two-row physical-ID-free inventory only; no physical receipt, ValueId, call lowering, Compare, branch, CFG/PHI, fallback, retry, or production |
| 25b-l-a-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` | transport the typed Left ReadBinding/Right Length CallSlot inventory with wrong-role/op/result/block/class, duplicate, foreign, provenance, and late-failure negatives | landed 2026-08-17; three focused inventory tests are green; no physical emission, Builder/session mutation, or parent-result unlock |
| 25b-l-b | `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` | retain the existing physical-entry cohort stamp through the consuming canonical session without copy/reconstruction, then expose only a scoped borrow | accepted BoxShape 2026-08-17; caller-zero I0 landed 2026-08-17 with move-only session ownership and scoped borrow; no physical condition result, ValueId, edge, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-c | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0` | fix one session-local Bool result plan/receipt that borrows the producer/inventory/stamp, uses canonical ValueId/type issuance, and has one outer discard owner and one later branch consumer | superseded by the ordered receipt-lifetime and Bool-result rows below; no ValueId, Compare, edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-d | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` | name the sole same-session issuer for the Length CallSlot physical result required by the parent Bool receipt | accepted BoxShape 2026-08-17; the first consumer is a no-effect one-shot canary, with no ValueId, CallSlot lowering, Compare, edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-d-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` | consume the same-cohort Length relation/inventory/stamp exactly once as a Builder-neutral canary | landed 2026-08-17; positive, duplicate, missing-stamp, source-shape, and late-failure no-mutation gates are green; no physical Length result, CallSlot lowering, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-e-D0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-D0` | accept one source-backed StringLen target/receiver/zero-args/I64 plan before any canonical Call effect | accepted BoxShape 2026-08-17; the next I0 issues the plan once with no ValueId, Call, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-e-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` | issue and consume the source-backed target plan exactly once in the existing callback | landed 2026-08-17; same-cohort facts, canonical StringBox.length, plan/canary parity, duplicate, missing-stamp, and late-discard gates are green; no canonical Call/result receipt or parent Bool effect |
| 25b-l-f-D0 | `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-D0` | project the allocated source-segment receipt to the exact physical condition block through the same canonical session | accepted BoxShape 2026-08-17; callback-scoped owner/logical-block/physical-block/stamp view only; no Call, ValueId, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-f-I0 | `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-I0` | allocate source segments once and lend exactly one same-session condition-block target with late-discard and escape negatives | landed 2026-08-17; callback-scoped owner/logical-block/physical-block/stamp projection and late-discard canaries are green; no Length Call/result receipt, receiver ValueId, Compare, edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-g-D0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIVER-OPERAND-D0` | fix the same-session source receiver BindingRef → canonical read receipt boundary before any Call effect | accepted BoxShape 2026-08-18; existing resolver relation → `LengthReceiverBindingRefV1` mechanical projection → canonical read receipt; no Call/result/parent Bool/Compare/edge/CFG/PHI/lifecycle/Text/route/fallback/retry/production |
| 25b-l-g-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIVER-OPERAND-I0` | lend exactly one same-session receiver operand receipt with no Call or result emission | landed 2026-08-17; canonical read only, with owner/type/target/stamp drift, duplicate/re-entry, and late-discard tests green; no Call/result/parent Bool/Compare/edge/CFG/PHI/lifecycle/Text/route/fallback/retry/production |
| 25b-l-h-D0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-D0` | consume the receiver receipt and name the sole direct StringBox.length Call/result issuer | accepted BoxShape 2026-08-17; same-session target/receiver/condition/stamp co-seal, canonical Call/result issuer, and unpublished canary/discard boundary are fixed; no parent Bool/Compare/edge/CFG/PHI/lifecycle/Text/route/fallback/retry/production |
| 25b-l-h-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-I0` | emit exactly one canonical Length Call and one I64 receipt under the outer unpublished transaction | landed 2026-08-17; same-session target/receiver/condition/stamp checks, final generic-Call shape/effect checks, one-shot, and late-discard tests are green; no parent Bool/Compare/edge/CFG/PHI/lifecycle/Text/route/fallback/retry/production |
| 25b-l-e | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0` | close the parent physical Bool-result boundary after the Length Call/result canary | superseded by the ordered receipt-lifetime and Bool-result rows below; the direct canary and session-scoped Length receipt lifetime are landed |
| 25b-l-i | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-D0` | seal the full physical-entry/session stamp and issue one callback-scoped non-repairable Length result receipt | accepted BoxShape 2026-08-17; the receipt owns the exclusive canonical-session borrow, with no Compare, branch, edge, CFG/PHI, lifecycle, Text, route, publication, fallback, retry, or production caller |
| 25b-l-i-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-I0` | change the Length receipt to an exclusive callback-scoped session borrow and add lifetime/duplicate/late-discard gates | landed 2026-08-17; direct-length and full physical-entry suites green; no Bool ValueId/Compare, branch/edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, publication, fallback, retry, or production |
| 25b-l-j-D0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-D0` | accept one same-session condition-result materializer consuming Left read + Length result and issuing one canonical Bool receipt | accepted BoxShape 2026-08-17; receipt-owned same-session method, one canonical Left read, one Bool ValueId/type, one `Less` Compare, no branch/edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, publication, fallback, retry, or production |
| 25b-l-j-seed-D0 | `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-D0` | close the source-backed pre-loop index initializer relation needed before the Bool materializer can read the condition binding | accepted BoxShape 2026-08-17; `VerifiedS6CTypedInputRelationV1::initializer()` plus resolver/source literal evidence define one typed seed relation; no session effect, Bool/Compare, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-j-seed-transport-I0 | `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-SOURCE-TRANSPORT-I0` | carry the accepted source-only seed relation through the same S6C ingress/common envelope | landed caller-zero transport; positive/foreign-owner gates green; no Const/Write/ValueId/read receipt, Bool/Compare, branch/edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-j-seed-I0 | `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-I0` | emit one unpublished `ConstI64(0)` and exact declaration publication from the transported seed relation | landed caller-zero effect slice; positive/duplicate/missing-function/late-discard gates green; no Bool/Compare, branch/edge/terminator, CFG/PHI, lifecycle, Text, route, fallback, retry, or production |
| 25b-l-j-I0 | `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-I0` | emit one mechanical `Less` Compare and one Bool type/result receipt under the outer unpublished transaction | landed caller-zero effect slice; receipt-owned same-session consume, canonical entry seed/read, one Bool ValueId/type, one Compare, and missing-seed/type/late-discard gates are green; branch/edge/terminator, CFG/PHI, Completion/DraftSeal, lifecycle, Text, route, publication, fallback, retry, and production remain closed |
| 26 | `LOOP-PRECUTOVER-AUTHORITY-G0` | all-19 semantic-program/JoinSig/Layout/CFG coverage plus zero competing target-subtree authorities | downstream after `LOOP-SEMANTIC-PROGRAM-COSEAL-ALL-FAMILY-R0` and `LOOP-PRECUTOVER-AUTHORITY-G0-D0`; missing coverage blocks selection |
| 26a | `LOOP-PRECUTOVER-AUTHORITY-COVERAGE-D0` | read-only owner mapping and competing-authority census beneath G0; name the missing co-seal issuer without minting a semantic receipt | census complete; Callable-first R0 handoff accepted, while `NoSafeSlice::GenericG0EntrySourceCoverageParentUnsealed` remains for Generic G0; no CFG mutation, physicalizer, selection, fallback, or retry |
| 26b | `LOOP-PRECUTOVER-AUTHORITY-G0-D0` | design the Generic source parent, entry/source coverage retention, same-Core JoinSig continuation, and test-only split retirement boundary | accepted BoxShape; source-parent implementation is landed, while physical/session effects remain closed |
| 26c | `LOOP-PRECUTOVER-AUTHORITY-G0-SOURCE-COHORT-D0` | design one opaque same-cohort source view that retains the resolver input while issuing Generic demand and parent callback | accepted BoxShape; callback-scoped owner/origin/site/frame/region/forest and entry coverage are fixed; no physical demand, Builder/session, CFG/SSA/PHI, lifecycle, route, fallback, retry, or production caller |
| 26d | `LOOP-PRECUTOVER-AUTHORITY-G0-I0` | replace the cfg(test) Generic ingress with one same-cohort non-Clone source parent and callback-scoped common co-seal transport | landed 2026-08-17; production parent issuer, exact two-entry rows, foreign-input rejection, and callback-scoped loan are green; no physical demand, Builder/session, CFG/SSA/PHI, lifecycle, route, fallback, retry, or production caller |
| 27 | `LOOP-PRODUCTION-SELECTION-D0` | decide exact family admission after all required gates | human consultation stop; `NoCandidate` is valid |
| 27a | `LOOP-PRODUCTION-CANDIDATE-CENSUS-R0` | enumerate the production semantic arms and exact collector handoff before any Generic selection code | design-stop census 2026-08-17; selected-Dynamic is the only live package-loan -> DraftSeal -> collector path, Generic G0 remains caller-zero, and no selector/fallback/retry code is authorized |
| 27b | `MIRBUILDER-STRUCTURAL-DEBT-DISPOSITION-R0` | after production selection, join the sealing-escape, function-level S6C/Generic/common-V2 duplicate census, legacy TSV disposition, and byte-helper/micro-seed inventory under one owner/retirement manifest | parked behind `LOOP-PRODUCTION-SELECTION-D0`; inventory only until each row has a source-backed owner, parity gate, exact callers, and same-slice zero-caller deletion; no new receipt, selector, fallback, retry, or LOC-driven cleanup |
| 27c | `DYNAMIC-EXIT-PHYSICAL-SESSION-P0` | co-seal `VerifiedDynamicExitTransactionCoSealV1`, `PreparedDynamicLocalEntryV1`, the exact located Loop/method/frame/scope/region, site-keyed Completion claims, one unpublished session, DraftSeal exit projection, and the same collector handoff | parked after the no-switch D0; the selected-callable bridge is missing, so no production effect or old-edge deletion is allowed |
| 28 | existing `M10b-I0-R0` + R1/M11/M12/R2 | one production switch, same-commit old-edge deletion, direct Ready-constructor retirement, then manifest-led sole-authority proof | no fallback; cutover must be green before retirement |

### Selected Dynamic first-cutover overlay (2026-08-11)

The table above remains the repository-wide convergence order. The bounded
`ParserScanLoopBox.skip_while/4` replacement uses this smaller exact overlay so
the first production cutover does not wait for unrelated all-19 families:

```text
A-PRIME-PARAMETER-CONTRACT-I0
  -> exact pos/end source/binding/ABI relation
  -> A-PRIME-MIXED-RECIPE-SEMANTIC-RECUT-I0
  -> A-PRIME-PHYSICAL-INPUT-I0

LOOP-UNIFICATION-AFTER-DYNAMIC-D0 (BoxShape only)
  -> LOOP-SEMANTIC-PROGRAM-COSEAL-R0
  -> LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0

selected BoxCount only
  -> LOOP-PHYSICAL-IF-COVERAGE-I0
  -> LOOP-PHYSICAL-EXIT-COVERAGE-I0
  -> LOOP-PRECUTOVER-AUTHORITY-H2

backend siblings
  -> AOT/LLVM production exact-I64 capability
  -> LLVM-SELECTED-DYNAMIC-EXACT-I64-DIRECT-I0
  (Rust VM exact-I64 row is reference/smoke evidence only; it is not a
   production gate, provider consumer, or session prerequisite.)

outer callable owner
  -> FUNCTION-COMPLETION-SITE-KEYED-CLAIMS-R0
  -> DRAFT-SEAL-EXIT-PROJECTION-SPLIT-R0
  -> A-PRIME-I64-LOOP-PHYSICAL-SESSION-I0
  -> H2-SELECTED-DYNAMIC-LOOP-CUTOVER-I0

post-cutover
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0
```

`LOOP-UNIFICATION-AFTER-DYNAMIC-D0` never absorbs If/Exit BoxCount. `Always`,
unrelated G0 retirement, and broader all-family parity remain later unless the
unchanged selected source proves they are required. The topology D0 is census
and deletion-gate preparation only; fixed-role hard deletion occurs after the
selected old production edge is removed and remaining callers reach zero.

### Pre-cutover execution briefs

`LOOP-SEMANTIC-PROGRAM-COSEAL-R0`

```text
Change:
  issue one move-only semantic program from existing source/Core authorities;
  migrate caller-zero Callable/G0/all-route logical products; delete split issue
Contract:
  exact resolver site/frame and Core-owned JoinSig are co-branded once;
  profile input owners, Tail, ABI, Completion, and physical owners stay outside
Done:
  mixed-Core/context/continuation and wrong-node/source fixtures reject;
  raw from_parts/from_after and three-argument demand callers are zero
Stop:
  any need to copy input truth, infer source coordinates, or add a selector
  returns to design
```

`LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0`

```text
Change:
  share one private structural traversal; issue current-cohort transfers from
  JoinSig; bind them in Layout; delete Recipe-derived transfer inference
Contract:
  Recipe owns structure, JoinSig owns logical transfers, Layout owns placement,
  Canonical CFG owns physical edges; accepted shapes remain unchanged
Done:
  Callable/G0 layouts and MIR receipts retain parity; missing/duplicate/foreign/
  wrong-target transfer fixtures reject; direct Layout/allocator/writer inference
  callers are zero
Stop:
  If/Exit/Always support, profile-specific repair, or a public traversal Plan
  is a different row and cannot enter this Refactor Series
```

The same behavior-preserving series may include the ledger-bound consumer
cleanup `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0`: V1 and V2 consumers must
borrow one complete ordered operation/source-effect ledger instead of calling
`find` over Recipe/evidence arrays repeatedly. This is a consumer protocol, not
a V2-to-V1 adapter or a new source/effect authority. If the ledger cannot be
borrowed without re-pairing rows, stop with `NoSafeSlice` and keep the current
physical demand owner unchanged.

`LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` is the next BoxShape slice in the same
series. The common Loop physicalizer may consume only the neutral continuation
boundary and complete physical layout/ledger products. It must not import or
construct `ReadyCallableLoopProfileCloseV1`, inspect Callable-specific counts
such as `Pure/Read/Write`, or own Tail, ABI, Completion, Return, DraftSeal, or
callable symbols. `recursive_after.rs` stops at
`ReadyLoopAfterContinuationV1`; the callable owner consumes that receipt in a
separate adapter. A guard must prove zero Callable profile symbols and zero
hard-coded profile cardinalities in the common physicalizer. Moving a file is
not sufficient: the owner and import boundary must change together.

`LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` is a census gate, not an eager
deletion. It inventories production, test, and guard callers of the old
fixed-role receipts (`LoopPhysicalBlockReceiptV1` / role-indexed boundary) and
the newer segment receipts (`LoopPhysicalSegmentBlockReceiptV1`). The old path
is removable only after the segment path is the sole production route and its
remaining test callers are either migrated or explicitly allowlisted. Numeric
role, current-block, name, ordinal, or Recipe-order repair is never an
acceptable bridge. If the census cannot prove caller-zero ownership, leave the
old type in place and return `NoSafeSlice`.

### Post-Dynamic audit additions (2026-08-11)

The external review did not add a fifth Loop authority. It makes the existing
four-row BoxShape series mechanically checkable. The following file-level
responsibilities are part of the rows above, not independent execution cards:

| Row | Existing surface | Required final owner | Forbidden bridge |
| --- | --- | --- | --- |
| `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | `physical_layout.rs`, `recursive_after.rs` | JoinSig-issued transfer view bound to private Recipe placement | rebuilding Predicate/Jump/Backedge/nested resume from `LoopConditionV1` or `as_recipe()` |
| `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | `segment_allocator.rs` | verified segment-placement receipt | rereading Recipe condition roles to classify Header/Body, current-block repair |
| `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0` | V1/V2 physical-demand consumers | one complete ordered operation/source-effect ledger borrowed by the consumer | per-access `find` over Recipe/evidence/effect arrays, zip-by-order, V2-to-V1 adapter |
| `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` | `recursive_after.rs`, `tail_completion.rs` | common stop at `ReadyLoopAfterContinuationV1`; Callable adapter owns profile close/Tail/ABI/Completion | `ReadyCallableLoopProfileCloseV1`, callable symbols, or hard-coded `Pure/Read/Write` counts in common code |
| `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` | `operation_target.rs` and fixed-role/segment receipts | one proven segment production route, then caller-zero deletion | keeping old and new topology issuers live without a census, or repairing by role/name/ordinal |

The `tail_completion.rs` file location is itself part of the boundary audit:
moving a file is insufficient if the common physicalizer still imports or
constructs Callable profile products. The final common module may stop at the
neutral continuation receipt; the outer Callable owner consumes it and owns
Tail, ABI, Completion, Return, DraftSeal, and Callable symbols.

The ledger row is a consumer-protocol refactor, not a new semantic authority.
Each family may retain its own verified source/effect product and lend one
complete ordered view. The view must be complete before physical preparation,
must retain exact item/source/placement identity, and must make missing,
duplicate, foreign, or extra rows reject before Builder effects. If this cannot
be done without re-pairing rows, the row returns to design with `NoSafeSlice`.

The topology census must include both the old role-indexed entry points and the
new segment entry points, including the dual `operation_target.rs` issuers and
their tests/guards. Deletion is allowed only after the segment route is the
sole production route and all remaining tests are migrated or explicitly
allowlisted. This keeps retirement reversible and prevents a second topology
authority from surviving behind a compatibility wrapper.

These are structural acceptance rules only. They do not authorize a new Loop
shape, a production selector, a Builder/CFG change, a fallback/retry path, or
the current H2 parser execution lane.

These three rows are the canonical post-Dynamic unification series. They are
ordered as one BoxShape-only refactor boundary:

```text
LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0
  -> LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0
  -> LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0
  -> LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0
```

`LOOP-PHYSICALIZER-COMMON-OWNER-R0` in the portable-Recipe SSOT is a related
but separate historical Accum-owner split. It may not absorb these rows or
become a second authority for the Dynamic/Common physicalizer boundary. The
post-Dynamic series owns transfer/evidence consumption and the Callable
profile boundary; the older row owns only the behavior-neutral Accum service
split. If an implementation touches both owners, keep the changes in separate
refactor-series commits with independent guards.

The three structural-coverage I0 rows each use the same four-block contract:

```text
Change:
  add exactly one previously typed-unsupported structural family
Contract:
  Recipe + JoinSig + common physicalizer only; no new route or fallback
Done:
  one positive fixture, exact transfer/coverage negatives, common guards, and
  implementation-coupled README/reference update are green
Stop:
  a missing JoinSig vocabulary returns to design before Layout or CFG edits
```

### Closed implementation receipt: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`

```text
Change:
  add one consuming finish_for_draft_seal target to the V2 session;
  migrate existing V2 profile finish sequences through it;
  add no non-V2 ReadyFunctionDraftSealV1::new caller

Contract:
  profile-specific ledgers close into ReadyCanonicalProfileCloseV1;
  common CFG / semantics / If / identity / PhiTxn / resolved binding /
  Completion close exactly once; whole unpublished session remains the
  failure atomicity owner

Done:
  DirectAccum cannot reach DraftSeal without cfg.finish;
  V2 direct Ready constructor callers are zero; the one non-V2 compatibility
  caller is named and non-growing; profile close is move-only and completion
  body/site/target metadata is not re-inferred at finish; focused
  omission/order/receipt/identity/failure-discard/fresh-reuse tests and the
  existing canonical gates are green; loop/function-exit references and the
  owning README update in the same commit

Stop:
  any accepted-profile or MIR delta, new semantic owner, non-V2 migration,
  or same-session repair/retry returns to design
```

### Closed design correction receipt: `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0`

The existing prepare architecture is directionally accepted but has one
bounded BoxShape correction before implementation. The correction task fixes
the callable input brand, resolved Prelude target/result capability, one-shot
Tail/Completion/ABI compatibility receipt, G0 owner/ABI pairing, and the
borrowed `ResolvedFunctionLoweringInputV1` lifetime wording. It adds no code or
Builder authority.

The correction is accepted only when these facts are explicit:

```text
callable input = non-Clone brand over exact input + current header/index
Prelude        = resolved target/header/arity/result capability, not syntax shape
terminal       = one-shot Tail/Completion/ABI relation receipt
G0             = same owner/source-type/ABI/terminal relation check
lifetime       = owned AST-free demand/receipts separate from borrowed input
```

Missing/foreign header, target, arity, result ABI, owner, tail site/binding,
Completion site/value-kind, G0 source brand, duplicate receipt, or any physical
authority is a pre-effect typed `NoSafeSlice`. The detailed task and its
acceptance matrix were the correction checklist; that row is closed and the
current execution row is the caller-zero recursive physicalizer below.

The static-call fixture and Prelude argument receipt close the remaining
positive prepared-input prerequisites without opening a production caller.

### Closed implementation receipt: `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`

```text
Change:
  add one test-only caller-zero common recursive topology boundary that
  consumes the topology-only compatibility VerifiedLoopPhysicalDemandV1
  exactly once together with the
  private, single-use ReadyLoopEntryV1 receipt and opens one Loop After
  continuation without emitting operation MIR.

Contract:
  the physicalizer sees only the AST-free demand, ReadyLoopEntryV1, and
  borrowed CanonicalSsaFunctionSessionV2 services. It does not see callable
  Tail/ABI/Completion, profile names, legacy route labels, source AST/name
  lookup, or a second Recipe/CFG/SSA/PHI owner. Late failure discards the
  unpublished fresh session; retry and same-session repair are forbidden.

Done:
  the focused canary proves recursive child/root After topology, exact entry
  coverage, owner/binding checks, parent/preheader placement, and rejection
  before block allocation. The module is cfg(test), has no production caller,
  and keeps source/check files below 800 lines. Exact MIR references, the
  owning README, and the compact current-row receipt were updated together.

Stop:
  operation emission without the accepted operation physicalizer design and
  canary task, missing logical relation,
  copied/reverified source truth, a new Recipe kind, profile-specific
  physicalizer, public topology, Return/DraftSeal/publication, selector,
  fallback, retry, or legacy deletion returns to design.
```

The topology probe may allocate only the common logical child/header/body/
step/After structure and the existing session-local continuation receipt. It
does not claim that `ReadBinding`, `WriteBinding`, constants, comparisons, or
arithmetic have been physically emitted. Those operations need an AST-free,
item-keyed source/effect projection so repeated ordinals in nested loops
cannot be guessed or matched by name.

### Operation/effect design boundary

The operation/effect relation product and both profile adapters are now
closed caller-zero cells, and cross-profile parity is closed as a diagnostic
receipt. They remain one relation product, not a new operation owner:

```text
Recipe:
  sole logical owner of LoopItemKey -> LoopOperationV1 and operand values

profile source adapter:
  sole issuer of exact source anchor / BindingRef evidence

VerifiedLoopOperationEffectProductV1:
  move-only { co-sealed Core, item-keyed source evidence ledger }
  evidence row = item + exact anchor + optional Core BindingRef view
               + checked block/loop provenance
  no copied LoopOperationV1, no ordinal lookup, no second Recipe
```

The existing `VerifiedLoopBindingEffectRelationV1` remains a separate
binding-level read/write/carrier product. The callable test producer's
item/site/operation relation is evidence for the adapter, not the common
authority. Generic G0 must retain or issue its item-keyed source evidence at
the producer boundary before structural source facts are consumed; it may
not reconstruct anchors from source preorder after Core issuance.

The operation product joins against the Core's already-sealed effect rows. If
the current Core API lacks the anchor/class view needed for that join, the
implementation may add one non-authority accessor or a consuming join helper
at the Core boundary. A second effect catalog or copied effect rows are not
allowed.

Coverage is by Recipe operation item, not by every Core effect row. Each
`LoopRecipeItemV1::Operation` has one exact source-evidence row;
`ReadBinding`/`WriteBinding` rows may additionally reference their sealed Core
effect row, while literal/compare/binary rows need no binding-effect row.
Most structural carrier rows and callable Tail/After reads remain with their
existing owners. The nested Generic G0 item 3 is the explicit exception: its
`ReadBinding` operation uses the existing child-entry
`DerivedCarrierEntry` anchor for carrier 2, and the Core effect relation must
match that anchor exactly. Item 4, C0/C1 carriers, and Generic tail reads stay
outside the operation product.

### Operation physicalizer design closeout

Decision B is accepted: full-demand preparation and one-operation emission are
different proofs. The full demand bundles the complete operation/effect product
with one neutral continuation and exposes only `prepare_all`; the private leaf
emitter consumes `PreparedLoopOperationEmissionV1` and never sees continuation
or any profile/function terminal contract.

The full semantic preflight runs before Builder mutation. After topology
allocation, the Callable R2 adapter derives an owner-branded
`LoopPhysicalSegmentBlockReceiptV1` from the R1 layout and binds each exact
segment to one physical block before instruction emission. The leaf emitter may
borrow only the existing canonical CFG, BindingSSA, and PhiTxn services plus a
session-local `ReadyLoopEntryV1`. It creates no second CFG, SSA, PHI,
transaction, or retry owner. A post-emission failure poisons the unpublished
function and uses whole-session discard; local Phi rollback is diagnostic
cleanup only. The older logical block receipt remains only for pre-existing
test seams and is not a fallback for the selected Callable dispatcher.

Generic item 3 remains a normal parent-body `ReadBinding`, but its source
anchor is the child-entry `DerivedCarrierEntry` for carrier 2. It is **not
admitted by ReadBinding D0**: the row is rejected as
`CarrierSeedUnavailable` and belongs to a later carrier-seed row. That later
row must assert parent-block placement and issue a child-entry carrier-seed
receipt through canonical BindingSSA; it must never relabel the operation or
infer placement from the anchor. The bounded leaf canaries are ConstI64 and
ReadBinding only; they do not constitute full Loop physicalization.

Duplicate item keys, foreign or missing anchors, wrong block/loop membership,
and repeated-ordinal ambiguity are typed `NoSafeSlice`. No operation MIR is
opened by these passive rows. Each profile product must be issued before the
P0 topology-only `into_physical_boundary` path, which intentionally drops
source anchors; P0 cannot be reused as the operation source.

`LOOP-PHYSICAL-PREPARE-P0`, the static-call fixture/profile, and
`LOOP-PRELUDE-ARGUMENT-RECEIPT-P0` are closed caller-zero prerequisites. The
cross-profile parity receipt and reviewed Decision-B closeout are closed.
Callable has seven item rows and Generic G0 has fifteen, but parity compares
neither counts nor source order. The full-demand P0, behavior-neutral module
split, canonical physical block receipt, private ConstI64 leaf-emitter canary,
bounded ReadBinding I0, and the caller-zero full callable physical canary are
closed. G0 D0/I0 are accepted/closed. R1 is now closed with Builder effect
zero; `LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` and
`LOOP-COMMON-RECURSIVE-AFTER-R3-I0` are now closed by the Callable segment and
neutral After canaries. G0 physical parity,
production selection, M8/M9 coverage, and retirement remain separate gates.

### ReadBinding leaf D0 correction (2026-08-07; Decision: accepted and landed)

The broad B boundary remains accepted. Worker review closed the following
contracts, and the bounded ReadBinding I0 implementation landed with focused
tests. These constraints remain normative for the leaf:

- Project the row exactly once from a complete
  `PreparedLoopOperationProgramV1`. Its Recipe `binding`/`result`, verified
  effect row (`source_binding`, `anchor`, `role`), owner, and logical
  placement must agree. AST, name, ordinal, and ad-hoc full-demand
  re-extraction are forbidden.
- The ordinary expression-read leaf admits only
  `LoopBindingEffectAnchorV1::Expr`. `DerivedCarrierEntry` (including Generic
  G0 item 3) belongs to the separate common carrier-seed projection closed by
  `LOOP-COMMON-PREDICATE-CARRIER-I0-R0`.
- The raw `ValueId` from `ResolvedSsaIdentityStateV2::read_entry` must not
  become the public leaf receipt directly. A thin canonical seam must issue
  `CanonicalBindingReadReceiptV1 { owner, binding, physical_block,
  physical_value }` after canonical BindingSSA/PHI verification.
- Placement comes only from the sole `LoopPhysicalBlockReceiptV1` and the
  orchestrator's logical Loop/Block/role. `current_block` and ordinal
  inference are not authorities. All checks happen before the canonical
  read/PHI operation.
- The logical result key is alias publication only. The leaf returns one
  immutable receipt `{ owner, item, binding, result, block, value }`; the
  outer operation ledger owns publication. No second ValueId, BindingSSA map,
  PHI owner, Return, Completion, or DraftSeal is introduced.
- Identity and `PhiTxn` are borrowed through one explicit canonical read
  service bundle. The physicalizer does not become a second session or owner.
- Pre-effect rejects are typed `NoSafeSlice`. A post-read type/receipt
  mismatch is a late terminal: discard the whole unpublished function,
  retain only local Phi cleanup diagnostics, and never retry or fallback.

The required reject matrix is: operation-not-ReadBinding; missing or
mismatched expression source anchor/binding; Core effect/role mismatch;
owner, logical, or physical placement mismatch;
missing entry binding; canonical BindingRead failure; result-type mismatch;
terminated block; and late emission failure.

This D0/I0 boundary claims no full-demand extraction API, AST reread, second
CFG/SSA/PHI/catalog owner, derived/G0 carrier bridge, other operation kinds,
return/seal/module publication, selector, retry/fallback, legacy retirement,
or performance result. The bounded implementation is landed. The current
authorized row is `CALLABLE-LOOP-AFTER-CLOSURE-P0`; Tail-only lowering is a
NoSafeSlice until its sealed After receipt exists. Each subsequent Tail and
DraftSeal slice must update reference documentation in the same commit as
code and focused tests.

#### ReadBinding source/effect mapping matrix

The following table is the complete D0 mapping. The full prepared program is
the only projection input; a one-row test fixture is allowed because the
complete program itself contains one ReadBinding row, not because a single
operation is extracted from a demand.

| Recipe operation | Evidence item | Core effect / anchor | D0 admission | Canonical read | Result publication owner |
| --- | --- | --- | --- | --- | --- |
| `ReadBinding { binding: LoopBindingKeyV1, result: LoopValueKeyV1 }` | same `LoopItemKeyV1` | `SourceRead { ordinal }`, `source_binding: BindingRefV1`, `Expr(OwnedExprSiteV1)` | admit only when all keys, owner, block, and role match | claim exact `SourceExprSiteV1` for `BindingRefV1`, then issue `CanonicalBindingReadReceiptV1` | outer operation ledger maps `LoopValueKeyV1` to the immutable leaf receipt |
| same operation | same item | `DerivedCarrierEntry` anchor | ordinary expression leaf excludes it; common carrier-seed row owns it | canonical `read_entry_receipt` | outer operation ledger maps the logical result |
| any non-`ReadBinding` operation | same item | any effect row | reject `OperationNotReadBinding` | none | none |

The canonical receipt has exact field types and one issuer:

```text
CanonicalBindingReadReceiptV1 {
  owner: FunctionOwnerIdV1,
  binding: BindingRefV1,
  physical_block: BasicBlockId,
  physical_value: ValueId,
}
```

Only `CanonicalSsaFunctionSessionV2`'s borrowed read service may issue it.
The order is fixed: validate the prepared row and physical placement; claim
the exact `SourceExprSiteV1` with `claim_variable_use_binding`; call the
canonical `read_entry_receipt`; validate owner, block, and physical type;
then return the receipt. A raw `ValueId` from `read_entry` is never a leaf
receipt and cannot be fabricated or rewrapped by the physicalizer.

Before the loop block is sealed, canonical BindingSSA may return a provisional
PHI with `MirType::Unknown`. The verified Recipe class is the only permitted
publication evidence in that state: `Unknown -> exact class MirType` is
published once by the private operation-type owner, while a concrete conflict
or missing type rejects as `ResultTypeMismatch`. Block/identity sealing then
revalidates the now-concrete PHI inputs; this is not type inference or a
fallback route.

The leaf receipt uses distinct logical and physical names:

```text
ReadBindingEmissionReceiptV1 {
  owner: FunctionOwnerIdV1,
  item: LoopItemKeyV1,
  binding: BindingRefV1,
  result: LoopValueKeyV1,
  logical_block: LoopBlockKeyV1,
  physical_block: BasicBlockId,
  physical_value: ValueId,
}
```

`result` is an alias key only. The Recipe's `LoopValueClassV1` for `result`
and the binding/effect class are the logical type authority; the canonical
BindingSSA type fact is the physical observation and must match the class-to-
`MirType` mapping before the receipt is returned. The outer operation ledger,
not the leaf, publishes the result mapping. No second SSA/PHI/value map is
created.

#### Entry, placement, service, and failure contracts

`ReadyLoopEntryV1` is a **preheader seed receipt**, not a complete map of all
live bindings. The private ReadBinding projection carries an explicit
`entry_requirement: LoopReadEntryRequirementV1` with exactly two cases:
`PreheaderSeed` or `CanonicalLive`. The full-program orchestrator issues this
field from the existing Recipe input set and source-binding relations, then
checks `PreheaderSeed` against `ReadyLoopEntryV1`; the leaf never infers the
case. Body/step bindings use canonical SSA availability at their exact
physical block; absence from the preheader rows is not itself an error. A
required preheader seed missing from `ReadyLoopEntryV1` is the typed
pre-effect reject `EntryBindingMissing`.

The orchestrator supplies `expected_role: LoopPhysicalBlockRoleV1` together
with `LoopBlockKeyV1`; the sole placement authority is
`LoopPhysicalBlockReceiptV1`. The leaf never derives a role from
`current_block`, an ordinal, or source-anchor shape. The logical block and
expected role must resolve to the same `BasicBlockId` before claim/read.

The canonical borrowed bundle is the only physicalizer service boundary:

```text
CanonicalBindingReadServicesV1<'a> {
  builder: &'a mut MirBuilder,
  identity: &'a mut ResolvedSsaIdentityStateV2,
  phis: &'a mut PhiTxn,
}
```

It is created by the fresh canonical function session, borrowed for one
read, and never stores a second CFG/SSA/PHI owner. CFG placement is borrowed
separately from the sole `LoopPhysicalBlockReceiptV1`; simultaneous borrows
are sequenced so the receipt is fully validated before the canonical read.
There is no new type-fact owner or `TypeFactContext`: physical type
validation reads the existing `TypeContext` at
`MirBuilder::function_state.type_ctx`, and any publish/idempotence decision
uses the existing `TypeFactDecisionV1`/
`PreparedTypeFactPublicationV1` seam. The service bundle therefore borrows
only the canonical Builder/identity/Phi owners named above.

Phase ownership is fixed as follows:

| Phase | Allowed failure | Owner/action |
| --- | --- | --- |
| prepared-row/source/effect/entry/placement validation before claim | typed `NoSafeSlice` | no Builder/claim/PHI effect |
| canonical validation before the atomic claim/read service starts | typed `NoSafeSlice` | no claim/PHI effect |
| claim succeeds or canonical read starts, then any read/type/receipt error | terminal `Freeze` | whole unpublished function discard; caller restore once; PhiTxn abort is diagnostic only |
| post-read type/receipt/result mismatch or injected late failure | terminal `Freeze` | whole unpublished function discard; caller restore once; no retry/fallback |

This phase split is part of D0 acceptance and must be represented by focused
negative tests in the later implementation row.

## Implementation and documentation obligation

Every implementation row above must update its exact live references in the
same commit after code and focused tests land:

- `docs/reference/mir/loop-recipe-contract.md` for the landed co-seal/demand/
  physical boundary and sole-owner claims;
- `docs/reference/mir/generic-loop-stage-matrix.md` for caller-zero,
  canary, activation, and retirement status;
- `docs/reference/language/function-exit-and-entry-result.md` when the typed
  function-finish/Completion handoff lands;
- `src/mir/loop_recipe_contract/README.md` and the owning canonical lowering
  README (`src/mir/builder/resolved_lowering/README.md`) when their code
  contract changes;
- `docs/reference/mir/phi_policy.md` and `phi_invariants.md` only when a
  physical PHI contract actually changes;
- `CURRENT_STATE.toml` and the active rolling workstream for the next exact
  row and compact closeout;
- `docs/tools/check-scripts-index.md` only if a reusable public guard entry is
  added.

References must describe only landed behavior. This design SSOT may name the
accepted target now, but the reference pages must not claim physical,
production, backend, or retirement capability before the corresponding
implementation receipt exists.

For the new pre-cutover rows, the co-seal and transfer-authority implementation
commits each update the reference with their exact caller-zero status; each
Always/If/Exit BoxCount commit updates the supported structural matrix; M10b
then updates the same reference once more to record the real production caller
and removed legacy authorities. A design-only commit does not pre-announce any
of those capabilities in `docs/reference/**`.

## Current execution boundary

The architecture, `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`, bounded
`RECIPE-COSEAL-I0-R0`, callable static-prefix prepare, and Prelude argument
receipt are closed under the typed-receipt and no-reinference contract above.
The topology/After canary `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0` is closed.
The operation/effect plan, passive product, Callable adapter, Generic G0
15-row anchor ledger, cross-profile parity receipt, worker-reviewed
physicalizer Decision-B closeout, and Builder-free full-demand P0 are closed.
The `LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` boundary is now closed as a
private prepared ConstI64 leaf canary. It proves exact physical placement,
canonical Const/type-fact emission, typed pre-emission rejects, and
whole-session discard/fresh-session repeat. Full operation emission,
operation production activation, callable physical completion, production
selection, retry/fallback retirement, and legacy deletion remain closed. The
logical callable issuer S0 is closed without a production caller. R1 and R2 are
now closed by the receipts below; the next row is bounded neutral recursive
After R3. No single-item extraction API may be added to the full demand.

### Callable physical-canary preparation slice (2026-08-07)

The current preparation slice is mechanically green without claiming the full
callable physicalizer. The Prepared callable product has one private test
handoff that moves `input`, complete operation demand, Prelude, Tail, terminal
compatibility, and Completion exactly once. The full operation contract also
projects every WriteBinding row with its exact Recipe item, source
binding/site, class, and logical placement.

Private leaf bridges cover `ConstI64`, `BinaryI64`, and `CompareI64` through
the existing Builder/type emitters. Their schedule-local value map is only a
temporary `LoopValueKey -> ValueId` transport; it is not a second SSA or PHI
owner. A focused test proves the Const -> Binary -> Compare chain. A bounded
row-level dispatcher and full Recipe-order Builder-free prepare now join
Read/Const/Compare/Binary/Write leaf services with an opaque typed value
ledger. The physical operation boundary now issues one exact
logical-to-physical target receipt per row, validates all target blocks before
the first leaf effect, and separates target/pre-claim physical failure from
semantic preflight. The caller-zero full physical canary is now closed:
the exact resolved-module input/ledger enters S2 once, then reaches Prelude,
topology, all five operation families, sealed After, Tail/Completion,
`finish_for_draft_seal`, and DraftSeal prepare/commit. Its late-failure test
discards the whole unpublished function and reruns the same semantic fixture
in a fresh session. Production selection, Generic G0 parity, retry/fallback
retirement, module publication, and legacy deletion stay closed.

### Callable full physical canary closeout (2026-08-08)

`CALLABLE-LOOP-PHYSICAL-CANARY-P0` is a caller-zero-only integration receipt.
The test-only source bridge borrows the exact existing resolver ledger from
`ResolvedFunctionLoweringInputV1`; it does not resolve a second owner or clone
the source AST. The complete seven-row Recipe schedule is consumed once and
the existing owners remain sole authorities for CFG/SSA/PHI, completion,
DraftSeal, and unpublished-function discard. The focused positive and late
duplicate/discard/fresh-reuse tests are green. G0 D0 is accepted; the next
authorized row is the Builder-free
`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` exact-input composite gate.

### Generic G0 exact-ingress I0 closeout (2026-08-08)

`LOOP-CALLER-ZERO-PARITY-G0-I0-R0` now has a compiler-side `cfg(test)` ingress
at `src/mir/compiler/generic_g0_physical_prepare.rs`. It pairs the exact
resolver-issued `ResolvedFunctionLoweringInputV1` with the existing neutral
S4 product, validates source/owner/frame/forest/entry/tail relations, splits
`VerifiedGenericAfterEffectG0` once into the neutral continuation and the
distinct `VerifiedGenericG0TailCapabilityV1`, then issues the common demand
and `prepare_all` for all fifteen G0 Recipe items. The schedule is checked by
Recipe membership rather than Callable/G0 count or evidence order. Focused
positive, missing-input, foreign-input, and tail-separation tests are green;
existing demand/producer tests retain duplicate/missing-evidence coverage.
This remains Builder/MIR/physicalizer/selector/Retry/publication-free; later
negative expansion must use typed sealed-product rejection, not tampering or
reconstruction.

### Recursive segment plan R1 closeout (2026-08-08)

`LOOP-COMMON-RECURSIVE-SEGMENT-PLAN-R1` is closed as a Builder-free derived
product. `VerifiedLoopOperationPhysicalDemandV1::prepare_all` now traverses
the verified recursive Recipe preorder instead of flattening logical blocks.
`PreparedLoopPhysicalLayoutV1` consumes the complete prepared program and
records only mechanically derived segments, operation placement, and nested
After-to-parent-resume targets. It creates no `ValueId`, `BasicBlockId`, CFG,
SSA, PHI, function session, selector, retry, fallback, or legacy authority.

The exact fixtures are green:

```text
Callable: seven operation rows in Recipe preorder
Generic G0: [0,1,2,3,5,6,7,8,9,10,11,12,13,14,15]
Generic G0 segments: root B0, root B1-pre, child B2, child B3, root B1-resume
coverage: 16 items / 15 operations / 5 segments
```

R2 may bind these private segments to the already allocated old topology. Until
the R3 correction closes, true segment block allocation, recursive After emission, G0
physical parity, production selection, retry/fallback retirement, and legacy
deletion remain closed. The R2 task is
`investigations/loop-common-segment-block-cutover-r2-task-2026-08-08.md`.

### Segment block cutover R2 closeout (2026-08-08)

`LOOP-COMMON-SEGMENT-BLOCK-CUTOVER-R2` is closed for the Callable canary.
`LoopPhysicalSegmentBlockReceiptV1` is a private adapter receipt derived from
the closed R1 layout and the already allocated canonical topology. It verifies
exact segment coverage, owner/preheader branding, and unique physical blocks.
The selected Callable dispatcher builds one complete item-to-segment index from
that layout and issues each target through the exact segment key; it no longer
uses logical-block-only execution lookup. The existing canonical CFG,
BindingSSA, and PhiTxn services remain the only physical owners.

The R2 receipt is intentionally only a Callable adapter: segments that would alias
one current topology block reject rather than silently sharing a block. This
keeps Generic G0's parent pre-child/resume split closed until R3 supplies the
neutral recursive After/edge physicalization. The focused canary preserves the
seven-row `Pure=4 + Read=2 + Write=1` parity and covers exact placement,
foreign-owner, missing-segment, duplicate-block, late-failure discard, and
fresh-session reuse. No G0 physical emission, selector, fallback, retry,
collector/publication, or legacy retirement is claimed.

The implementation receipt is recorded below; R3-I0 is closed. The next task
is the bounded D1 common Predicate/carrier contract row, not production
selection.

### R3-I0 implementation receipt (2026-08-08; Decision: accepted)

R2 is an adapter over the old fixed topology, not the physical allocator for
the R1 segment graph. A neutral edge writer cannot consume it safely because
R1 transfers do not use the synthetic Step block. The corrected physical
boundary is implemented for the selected Callable caller-zero canary:

```text
PreparedLoopPhysicalLayoutV1 + ReadyLoopEntryV1
  -> one block per R1 segment + one root After (no Step)
  -> CompletedLoopSegmentProgramV1 retains layout, segment receipt,
     and completed operation receipts
  -> preflight entry plus every R1 transfer exactly once
  -> canonical CFG/identity/PhiTxn edge emission and sealing
  -> neutral ReadyLoopAfterContinuationV1
```

The layout carries an explicit sealed `entry_segment`; ordinal zero is not an
entry authority. `segment_allocator` allocates exactly one block per R1
segment plus one root After and no synthetic Step. The completed segment
program retains layout, entry, segment receipt, completed operation receipts,
and the value ledger. R3 preflights the entry edge and every R1
Jump/Predicate/OpenNestedLoop transfer, emits each once through canonical
CFG/identity/PhiTxn, seals all segment blocks plus root After, and returns one
neutral `ReadyLoopAfterContinuationV1`. Callable's seven-row coverage stays in
its thin profile wrapper; Tail/Completion meaning is unchanged. The old fixed
Callable close helper and `from_callable_layout` adapter are removed from the
selected path. G0 receives no physical allocation or operation emission, and
selector, fallback/retry, publication, and broad legacy retirement remain
later boundaries.

### G0 I1 D1 review closeout (2026-08-08; Decision: accepted)

The post-R3 worker audit found two common contracts that must be implemented
before G0 I1. The current Callable recursive writer cannot use the first
Predicate value for every transfer: `LoopPhysicalTransferV1::Predicate` must
resolve its own completed Bool value and physical source segment. The neutral
After receipt therefore carries only common owner/root/predecessor and
coverage facts; Callable's `7 = Pure4 + Read2 + Write1` remains profile-local.

The G0 child-carrier row is a `ReadBinding` with a `DerivedCarrierEntry`
anchor, not an expression anchor. The common operation family must add a
profile-neutral prepared carrier-seed variant which delegates to canonical
identity `read_entry_receipt`. It must not fabricate an expression site or
introduce a G0-specific dispatcher/SSA owner.

The next two commits are intentionally separated:

```text
LOOP-COMMON-PREDICATE-CARRIER-I0-R0
  common contracts + Callable regression; no G0 allocation

LOOP-CALLER-ZERO-PARITY-G0-I1-R0
  exact ingress, 5 segments + root After, 15 rows, per-Predicate values,
  G0 Tail/Completion/DraftSeal, whole-session discard/fresh rerun
```

Both remain cfg(test) caller-zero evidence. Production selection, M8/M9,
M10b/M11/M12, retry/fallback retirement, collector publication, and broad
legacy deletion remain closed. Each implementation commit updates the exact
reference page, README, tests/guards, current pointers, and workstream.

### Common Predicate/carrier I0 closeout (2026-08-08; Decision: accepted)

`LOOP-COMMON-PREDICATE-CARRIER-I0-R0` is closed. The neutral After receipt no
longer carries a profile-specific condition key or operation counts. Recursive
After validates one completed Bool receipt per Predicate transfer, including
owner, type, and physical source-segment placement; Callable's coverage and
condition proof remain in its outer profile close.

The common operation demand now has a separate full-program
`PreparedLoopDerivedCarrierSeedRowV1` for `DerivedCarrierEntry` anchors. The
private `CarrierSeed` emitter delegates to canonical identity
`read_entry_receipt`, so no fake expression source site, G0-name dispatch, or
second SSA owner is introduced. The focused Callable gate is green (25/25),
the Generic demand fixture identifies exactly one item-3 carrier row, and all
touched source files remain below 800 lines. The next implementation row is
`LOOP-CALLER-ZERO-PARITY-G0-I1-R0`; physical G0, production selection,
fallback/retry retirement, publication, and legacy deletion remain closed.

The earlier matrix rows that described `CarrierSeedUnavailable` as the final
DerivedCarrier boundary are historical for this cell and are superseded by
this receipt; expression-anchor reads keep their original contract.

### Generic G0 I1 caller-zero receipt (2026-08-08; Decision: accepted)

`LOOP-CALLER-ZERO-PARITY-G0-I1-R0` is closed as a profile wrapper around the
same common physical services. The exact resolver-issued G0 ingress moves once
into the full common operation program and the separate G0 Tail. The canary
opens a fresh unpublished function session, publishes the resolver-declared
receiver and two parameters through canonical identity, allocates five R1
segments plus root After, and dispatches the fifteen prepared rows exactly
once. The structural nested Loop item remains a control/layout row rather
than a fabricated operation.

The carrier row uses the profile-neutral `CarrierSeed` emitter and canonical
`read_entry_receipt`; an unsealed PHI value is typed only through the existing
`ensure_provisional_value_class` contract. Each Predicate transfer consumes
its own completed Bool receipt, so root and child conditions have distinct
physical values and source segments. The G0 `L0.After/b1` Tail read is
canonical, exact I64 Completion is claimed once, and
`finish_for_draft_seal`/DraftSeal reaches one unpublished completed draft.

The late duplicate fixture fails after earlier emission, discards the whole
unpublished session, and a fresh session reproduces the same semantic receipt.
No G0-specific CFG/SSA/PHI owner, production selector, caller switch,
retry/fallback, collector publication, backend/performance claim, M8/M9
coverage, or M10b/M11/M12 retirement is opened. The behavior-preserving
`LOOP-INPUT-SOURCE-RELATION-SET-R0` is now closed: callable consumes the common
exact-coverage initialized-local input set and Generic parameter inputs remain
separate. S6A's caller-zero Facts/producer and typed Main C/D/U/R ingress are
landed; its exact identity/source-coherence negative closeout remains current.
After M8/M9, the semantic-program and transfer-authority rows above are the
mandatory production-selection prerequisites. Current source task order is
owned by the Loop pipeline SSOT and `CURRENT_STATE.toml`.

### Return source-to-Recipe/Join binding I0 closeout (2026-08-18; Decision: accepted)

`LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-I0` is landed as
one caller-zero transport-only BoxShape. The resolver-owned S6C Exit/Tail
co-seal now retains the exact nested-If region, Return region, index
`BindingRef`, Return site, and Return value. The sole S6C Recipe producer issues
one non-Clone relation that binds those source facts to the fixed If/then
blocks, `ReadBinding` Return item/result, logical Exit key, and the JoinSig
Return/FunctionExit arm plus matching Body-to-FunctionExit summary. The
relation is lent through the existing product, logical JOINIR, and logical
output/prephysical façades; no second semantic issuer or Recipe key owner is
introduced.

Focused `s6c_scan_with_init` tests are green (9/9), including the positive
source/Recipe/Join binding and existing swapped-call, swapped-argument, and
domain-drift negatives. `current_state_pointer_guard.sh`,
`loop_physical_transfer_authority_guard.sh`, formatting, and diff checks are
green. No block/edge/terminator, Return emission, CFG/SSA/PHI, session,
production, fallback, retry, or publication effect is opened. The next design
stop is `LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COMMON-AGGREGATE-D0`:
the common pre-session envelope must first be named as the sole consumer and
rollback owner before this relation can affect physical demand.
