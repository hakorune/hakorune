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
  A-prime lifecycle
  activation remains parked until its boundary owns
  `PreparedFunctionExitSetV1`.
- **Next ordered task:**
  `LOOP-GENERIC-G0-PHYSICAL-ENTRY-SOURCE-PROJECTION-D0` is the next design
  stop.  The S6C-only physical-entry projection, Generic TopLevel
  declaration/header, body-shape transport, function-effect receipt,
  result-ABI transport, and direct canonical Completion transport are landed;
  storage/lane projection remains an independent source census.  Transfer
  migration, CFG/PHI, lifecycle, Text, route, fallback, retry, and production
  remain closed.
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

### `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-D0` (next design stop)

```text
Decision:
  Keep the landed Generic entry-input row as a source-to-mechanical product
  and stop before any skeleton effect.  The next BoxShape must decide how one
  fresh unpublished Generic physical function skeleton is issued from this
  same cohort without importing S6C or inferring layout from `/N`, JSON, or
  descriptor length.

Source authority + canonical issuer:
  The Generic entry-input product remains the sole source-backed input.  A
  future skeleton issuer must co-seal its symbol/mode, ordered physical lane
  rows/types, result ABI, source-backed effect/attrs/uses projection, and
  unpublished transaction owner before Builder state is opened.

Non-authority:
  S6C skeleton/header/signature rows, `MirFunction` parameter order,
  `ValueId` numbering, `/N`, JSON vector length, raw `ParamDecl`/AST rescans,
  current Builder blocks, `EffectMask` defaults, and `new_selected_dynamic`
  cannot issue the Generic skeleton contract.

Fail-fast boundary:
  Missing symbol/mode/result/effect/attrs/uses projection, foreign parent or
  frame, lane order/type/count drift, duplicate/absent receiver, or any need
  to install a function, publish a `ValueId`, open BindingSSA/CFG/PHI, consume
  Completion, or use a legacy finalizer keeps the row at
  `NoSafeSlice::GenericG0PhysicalSkeletonInputUnsealed`.

Smallest next slice:
  Read-only issuer census for a fresh unpublished Generic skeleton input;
  implementation must remain stopped until the full source-backed input and
  rollback owner are named.

Non-claims:
  No skeleton allocation, Builder/session effect, entry-lane adoption,
  Completion consumption, CFG/SSA/PHI, lifecycle, Text, route,
  fallback/retry, production caller, or main integration.
```

### Generic G0 source-projection child tasks (ordered; next row is Generic skeleton D0)

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
| 22 | `LOOP-PHYSICAL-TRANSFER-AUTHORITY-R0` | one private traversal, JoinSig-issued transfers, Layout binding only, direct transfer inference deletion | BoxShape Refactor Series; current Predicate/nested cohort only |
| 22a | `LOOP-COMMON-TRANSFER-BOUND-SEGMENT-INPUT-R0` | make V1/V2 physical consumers borrow one complete ordered operation/source-effect ledger; remove repeated Recipe/evidence `find` scans | behavior-preserving consumer refactor only; no V2-to-V1 adapter or new source/effect authority |
| 22b | `LOOP-PHYSICALIZER-BOUNDARY-CLEANUP-D0` | move Callable profile-close/Tail/ABI/Completion out of the common Loop physicalizer; common stop is `ReadyLoopAfterContinuationV1` | BoxShape only; no accepted shape, profile callback, selector, or production switch |
| 22c | `LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0` | parent BoxShape: order the installed child, TextFormal mapping, one Completion owner, and generic V2 operation/control envelope | closed design boundary 2026-08-16; one parent HRTB/sibling views, generic operation/control partition, and passive coverage are fixed; no session effect |
| 22c-a | `CALLABLE-TEXT-FORMAL-PHYSICAL-SIGNATURE-D0/I0` | accepted mapping: one logical ExactText ordinal/BindingRef -> adjacent scalar `[slot,generation]` lanes; issue one complete/disjoint Completion-independent package cohort and transport it through one combined Installed S6C loan | closed caller-zero implementation; no call-edge actualization, `ValueId`, aggregate ABI, fallback, or retry |
| 22d | `LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0` | transport the generic parent/sibling boundary through one installed Port HRTB without emitting an execution product | closed caller-zero source transport 2026-08-16; one selected-key consumption seam; no JSON/MIR, route policy, Builder/session effect, or production caller |
| 22e | `LOOP-S6C-COMMON-V2-PRESESSION-I0` | implement the named source-backed operation adapter, JoinSig/Recipe control co-seal, and passive coverage issuer inside one caller-zero parent loan | closed caller-zero implementation 2026-08-16; focused positive/negative/duplicate tests green; no S6C physicalizer, Builder/session, lifecycle, route, or production caller |
| 22f | `LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-CENSUS-D0` | census fixed-role receipts versus segment receipts and publish the caller-zero deletion gate | independent census before cutover; never a prerequisite for issuing V2 meaning and delete only after production/test callers reach zero |
| 23 | `LOOP-PHYSICAL-ALWAYS-COVERAGE-I0` | add one JoinSig-authorized Always physical family | one BoxCount commit; no fallback |
| 24 | `LOOP-PHYSICAL-IF-COVERAGE-I0` | add exact branch/merge transfer capabilities and common physicalization | one BoxCount commit; no Layout inference |
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
| 25b-c0-G0-entry | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-D0` | accept one Generic-only pre-effect entry-input BoxShape over the same source parent; forbid S6C descriptor/header/signature reuse and keep receiver prefix separate from explicit arity | accepted 2026-08-17 after issuer census; its caller-zero input I0 is landed and the next stop is Generic skeleton D0; no skeleton, ValueId, BindingSSA, EffectMask, Builder/session, Completion consumption, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-entry-I0 | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-I0` | project one same-parent Generic source row into private non-Clone mechanical entry descriptors with receiver policy, dense explicit rows, metadata, and existing i64 carrier | landed 2026-08-17; focused positive plus parent rejection/no-publication gates green; no S6C/common descriptor reuse, skeleton, ValueId, BindingSSA, EffectMask, Builder/session, Completion consumption, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-skeleton | `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-D0` | census the source-backed symbol/mode/result/effect/attrs/uses and rollback owner required before a fresh Generic physical skeleton | next design stop; keep `NoSafeSlice::GenericG0PhysicalSkeletonInputUnsealed`; no skeleton allocation, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-effect-transport | `LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0` | transport the same-resolver body-shape product through the source unit/root input into the Generic cohort; no count-only effect receipt | landed 2026-08-17; owner/body-root checks and bare-input/foreign-cohort negatives green; no effect issuer, EffectMask, skeleton, session, or Builder |
| 25b-c0-G0-effect | `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-D0` | use the transported body-shape sibling for a resolver-owned census of body effects, calls, metadata-empty witness, and Generic structural facts; issue no physical EffectMask | accepted BoxShape 2026-08-17; next caller-zero I0 is the private source receipt; no physical EffectMask/session |
| 25b-c0-G0-effect-I0 | `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-I0` | issue one same-cohort private non-Clone Generic no-external-effect receipt before demand/product consumption | landed 2026-08-17; focused source-receipt and late-failure gates green; no physical/session effect |
| 25b-c0-G0-result | `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-D0` | transport the existing same-cohort Generic return ABI row before any Completion or physical entry | accepted BoxShape 2026-08-17; no new classifier, combined result/Completion receipt, default ABI, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-result-I0 | `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-I0` | retain one candidate-owned result ABI row in the Generic parent before demand/product consumption | landed 2026-08-17; focused exact/foreign transport tests green; no Completion, physical ABI, EffectMask, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-completion | `LOOP-GENERIC-G0-COMPLETION-PROJECTION-D0` | retain the canonical resolver Completion in the Generic parent after result-ABI transport, with Generic tail/result/cleanup parity | accepted BoxShape 2026-08-17; canonical verifier remains the sole issuer; no Completion consumption, physical ABI/lane, skeleton, ValueId, BindingSSA, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-completion-I0 | `LOOP-GENERIC-G0-COMPLETION-PROJECTION-I0` | issue `verify_function_completion_v1(input)` once and lend the canonical non-Clone product through the parent callback | landed 2026-08-17; focused source-parent tests green; transport only, with no Completion consumer, physical/session effect, CFG/PHI, lifecycle, Text, route, fallback, retry, or production caller |
| 25b-c0-G0-header | `LOOP-GENERIC-G0-TOPLEVEL-DECLARATION-HEADER-I0` | source-backed TopLevel declaration/header projection in the existing Generic cohort | landed 2026-08-17; parent physical-entry blocker remains; no result/lane/effect/Completion/skeleton/session |
| 25b-c0-converge | `MIRBUILDER-CANARY-CONVERGENCE-CHECKPOINT-R0` | read-only census of duplicate receipts, canary owners, retirement conditions, legacy edges, semantic-program tuple escape hatches, and S6C-only provenance adapters after the parent cohort | parked cleanup checkpoint; no new authority or production switch; unresolved storage/lane issuer keeps the current NoSafeSlice |
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
