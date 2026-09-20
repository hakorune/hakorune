---
Status: design_stop__2026-09-20__ParserLoopBreakSourceConsumer
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-CONSUMER-D0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-CONSUMER-D0
Date: 2026-09-20
Parent: mir-call-parser-source-to-mir-package-acceptance-window-d0-2026-09-20.md
Implementation permission: false; this row fixes the missing owner contract before code or fixture changes
---

# Parser LoopBreak source consumer D0

## Six-line brief

```text
Decision: keep the parser package acceptance stop closed until the same
  invocation's LoopBreak dependency has a source-lineage consumer that keeps
  whole-package coverage intact.
Source authority + canonical issuer: the resolver forest/exit ledger and the
  existing LoopBreakFacts/Recipe outcome for the finite dependency methods;
  no source-aware LoopBreak issuer currently exists, so this row must name
  the extension owner before implementation.
Non-authority: selected-only lowering, method-name or ordinal filtering,
  package cloning, AST/MIR rescans, LoopRouteContext reconstruction, GenericLoop
  retry, compatibility fallback, or a synthetic Cataloged/Selected receipt.
Fail-fast boundary: reject before Builder effects when the source caller,
  exact exit/forest relation, LoopBreak shape, package brand, or physical
  obligation is missing, foreign, duplicated, or mismatched.
Smallest next slice: inventory the LoopBreakRecipe front in the same merged
  parser package, then choose one existing LoopBreak Facts/Recipe/physical
  owner for a move-only source input and an exclusive old-edge delete-set.
Non-claims: no parser source-to-MIR success, publication, production switch,
  old-edge deletion, fallback, VM/AOT parity, or warning cleanup.
```

## Exact frontier

The selected package remains the bounded parser invocation:

```text
caller       = ParserProgramBox.parse/2
target       = ParserStringUtilsBox.starts_with/3
target site  = resolver-issued site for parser_program_box.hako:102
route        = LoopCondBreakContinue with the selected LoopTrue child
result       = ExactI64, required ordinal `[1]`
package stop = [freeze:contract][callable-loop/route-not-front-selected]
               GenericLoopV1NotSelected
               raw front = [LoopBreakRecipe]
```

The stop is dependency evidence. It is not permission to lower only the
selected `starts_with/3` method. The existing root work plan iterates every
immediate and deferred method, and the installed package's selected-call APIs
are scoped loans whose `complete()` requires all selected coverage.

## Finite dependency census

The parent nested-loop design already fixes the only reviewed source topology
that can produce this package boundary. The source witness is one caller,
`ParserProgramBox.parse/2`, with these loop members:

| Member | Source witness | Exit witness |
| --- | --- | --- |
| root state-machine loop | `:81 loop(cont_prog == 1)` | `break` at `:85/:95`; returns at `:104/:109/:127/:142/:161/:165/:194` |
| static-semicolon child | `:131 loop(static_semis == 1)` | condition-only child boundary |
| semicolon-scan child | `:182 loop(true)` | `continue :186` and `break :188` |

The parent card records the resolver-owned root/child indices, frame keys, and
all exit records; its line numbers are source witnesses only. Two terminal
observations are currently visible and must not be conflated: the lifecycle
raw front reports `[LoopBreakRecipe]`, while the source bridge normalizes its
stop to `GenericLoopV1NotSelected`. Neither observation is a source-to-route
slot map, and no co-sealed relation currently proves that they identify the
same dependency rows. Therefore this D0 must not infer either label from
names, line numbers, or AST shape.

### Static merged-source candidate inventory (read-only)

The source import closure rooted at
`lang/src/compiler/parser/program/parser_program_box.hako` is now bounded to
ten files: `ParserProgramBox`, `ParserStringUtilsBox`, `RuneContractBox`,
`ParserDeclarationBox`, `GrammarContractProjection`, `ParserFromRejectBox`,
`ParserDelegateExposesBox`, `ParserRecordDeclarationBox`,
`ParserBrandDeclarationBox`, and `ParserBoxWeakFieldBox`. A source-text census
finds five methods in that closure whose bodies contain both a `loop` and a
`break`:

| Candidate method | Source sites observed | Status |
| --- | --- | --- |
| `ParserProgramBox.parse/2` | root `:81`, children `:131` and `:182` | caller topology; route still resolver-owned |
| `ParserStringUtilsBox.trim/1` | `:87` | candidate only; no LoopBreak route claim |
| `ParserStringUtilsBox.to_int/1` | `:105` | candidate only; no LoopBreak route claim |
| `ParserDelegateExposesBox._parse_delegate/3` | `:54` | candidate only; no LoopBreak route claim |
| `ParserRecordDeclarationBox.parse/3` | `:19` | candidate only; no LoopBreak route claim |

This narrows the next audit from an unspecified package to a finite set, but
it does not select a route: a textual `break` is not a `LoopBreakFacts` or
`LoopRouteId` receipt. The resolver rows and route-first facts for these
candidates must still be recovered from the same invocation. Until that
observation exists, the raw `[LoopBreakRecipe]` and normalized
`GenericLoopV1NotSelected` labels remain dependency terminals, and no method
may be admitted or skipped by name, line, or shape.

The existing LoopBreak design explicitly rejects the parser nested profile as a
new route: its logical product is generic-direct-only, while its physicalizer
is `NoSafeSlice` because the legacy composer enters `lower_loop_v0` after
Builder-bound mutation. This row may reopen that owner only with a source-aware
move-only input and a builder-free pre-effect consumer contract; it may not
promote a parser-specific LoopBreak route.

## Resolver/consumer bridge audit

The resolver-side API is intentionally source-complete but route-neutral:
`VerifiedResolvedFunctionV1::loop_sites()` exposes the finite Loop-site
inventory, `resolved_loop_source_forest(root)` co-seals parent indices, and
`resolved_loop_source_context(site)` lends the exact Scope/Region pair. None of
these products records which physical `LoopRouteId` later classified the site.
`only_loop_site()` is a singleton guard and cannot select one member from this
package's three-member forest.

The current source bridge has the complementary boundary: its route issuer
accepts only Generic/LoopCond/LoopTrue source products. The LoopBreak path
starts at `LoopBreakFacts`, enters `route_loop_break_recipe` with
`LoopRouteContext`, and its composer reaches `lower_loop_v0` after it has
already received a mutable `MirBuilder`. There is no source-port argument or
package route inventory at that boundary. The package root therefore cannot
name the exact method behind the raw `[LoopBreakRecipe]` observation without
adding a new route observer or inferring from names/AST, both prohibited here.

The nearby `InvocationRouteMatrixV1`/`RouteOwnedInvocationInventoryV2` does not
close this gap: it inventories root invocation families (`RawStaticChild`,
`CallableModuleBatch`, and similar publication/drain lanes), not per-callable
`LoopRouteId` rows or source loop sites. Reusing it as a LoopBreak selector
would cross its authority boundary and silently turn a root-family observation
into loop meaning.

This is a confirmed missing-owner condition, not a missing line-number lookup.
The next slice must first name one existing package-scoped route inventory
authority (or explicitly reject that owner) before any LoopBreak source issuer
or physical adapter is designed. Until then, `GenericLoopV1NotSelected` stays
the terminal and the selected parser tuple cannot advance to Cataloged.

## Existing-owner decision

The audit of nearby products is finite and does not reveal a reusable
package-scoped LoopBreak owner:

| Candidate | What it actually carries | Decision for this row |
| --- | --- | --- |
| resolver batch / selected package map | callable identity, owner, forest, batch slot, and role | no `LoopRouteId` or package LoopBreak rows; do not turn selected-map transport into route meaning |
| callable source ledger | source-call observations and forest/context relations | explicitly does not issue Loop policy; retain as source evidence only |
| `LoopRouteContext` / route execution witness | AST-local route context and raw route schedule/attempt | no source identity or package coverage; cannot be the source issuer |
| `CallableLoopSourceBridgeV1` / source-route token | per-callable Generic/LoopCond/LoopTrue projections | no LoopBreak product and no package-wide inventory; cannot be widened by inference |
| `InvocationRouteMatrixV1` / `RouteOwnedInvocationInventoryV2` | root invocation families and publication/drain lanes | wrong authority: root-family inventory is not a per-callable loop-route map |

Decision: no current candidate owns the LoopBreak meaning relation. The
smallest safe design slice is one source-scoped LoopBreak product in the
existing LoopBreak owner, co-sealing callable identity, exact source
site/forest/exits, route outcome, package brand, and a builder-free physical
input. The package issuer below may observe complete coverage, but it must not
become a second LoopBreak meaning issuer. That product must also name the
exclusive old-edge delete set. Until that contract exists, do not extend the
selected package map, reuse the root invocation matrix, or issue a new
semantic receipt.

### Package issuer as parent-observer candidate (read-only, 2026-09-20)

`issue_normal_callable_semantic_package_with_brand_catalog_v1` already owns the
same-package co-seal boundary. It consumes one resolved batch, iterates every
declaration for existing Dynamic/ordinary/direct-call products, validates the
catalog/batch relation, and stores the batch in the non-splittable semantic
package. This makes the package issuer the natural place to aggregate a
route-neutral LoopBreak candidate or a typed absence for every batch row.

That observation does not exist today: the package model has no LoopBreak
coverage field, and the issuer must not infer one from method names, AST shape,
selected keys, or the `LoopRouteContext` consumer. The issuer can therefore be
reused as the parent coverage owner only after the same-owner Facts issuer
defines the source candidate and its reject vocabulary. The lowering port stays
the selected-consumption checker; it is not the observer.

D1 boundary: package issuer = complete-row coverage observer and brand
co-seal; existing `CallableGenericLoopSourceFactsIssuerV1` = per-callable
LoopBreak source/Facts co-seal; existing LoopBreak physical owner = consumer.
If any of these three contracts cannot be joined without a new semantic
authority, retain `NoSafeSlice` rather than adding an aggregate guess.

### Direct terminality candidate (read-only recheck)

The ordered registry does contain a narrower LoopBreak candidate:
`DirectLoopBreakTerminalityV1` in
`control_flow/joinir/route_entry/registry/direct_loop_break_terminality.rs`,
and its `VerifiedDirectLoopBreakLogicalProductV1` wrapper in
`live_ordered_terminality/logical_product.rs`. The candidate proves only the
direct three-statement scheduler shape and records the four logical roles
(`LoopCondition`, `BreakIfSubtree`, `CarrierUpdate`, and
`LoopBackContinuation`). It is reusable evidence for the future adapter's
existing LoopBreak Facts/Recipe owner.

It does not close this row: the terminality proof receives `LoopFacts` and the
borrowed AST body, but issues no `FunctionOwnerIdV1`, resolver forest/exit
relation, package brand, source-call item inventory, or exclusive deletion set.
The logical product exposes only `route()` and the unreached legacy tail; it
cannot identify which callable row in the merged parser package owns the
product. Treating its four role tags as source lineage would cross the same
authority boundary as inferring from names or AST shape. The candidate is
therefore a reusable downstream proof, not the missing source issuer or
package acceptance observer.

The next design slice may wrap this existing terminality/product only after a
source-scoped issuer co-seals the exact resolver row and verifies that the
direct topology is the same row consumed by the terminality proof. No change
to terminality predicates or logical role vocabulary is authorized in D0.

### Bridge ownership correction (worker recheck)

`CallableLoopSourceBridgeV1` must not become a merged-package `LoopRouteId`
inventory. It is constructed from one `ResolvedFunctionLoweringInputV1` and is
therefore a per-callable source-evidence provider only. It does not issue route
policy, terminality, catalog brand, or package-completeness meaning.

The viable D1 premise is narrower: retain a route-neutral LoopBreak candidate in
that bridge, then extend the existing
`CallableGenericLoopSourceFactsIssuerV1` boundary to co-seal the candidate with
the existing planner `PlanBuildOutcome`, `LoopBreakFacts`/
`LoopBreakSourceTopologyV1`, `DirectLoopBreakTerminalityV1`, exact source
items/exit rows, and the catalog brand. The resulting move-only product may
feed the existing `CallableLoopSourceParts` located block and
`lower_raw_loop_v0`/`lower_loop_v0_core`; it must not add a second Recipe,
JoinSig, or route ledger.

This still does not satisfy the D0 acceptance boundary. A parent-level
observer must aggregate every callable's route-neutral candidate from the same
merged package and prove complete coverage before `Cataloged`/`Selected`,
caller-zero, or an old-edge delete-set can be claimed. The bridge remains
source evidence, the Facts issuer remains the semantic co-seal point, and
package completeness remains an explicit downstream obligation.

### Parent observer audit (read-only, 2026-09-20)

The existing `VerifiedResolvedCallableSemanticBatchV1::with_declaration_semantics`
is a viable enumeration substrate: it validates complete batch-row coverage,
slot/identity/owner/body-shape parity, and lends every resolver-backed callable
row from one package. It is transport and coverage evidence only. The batch row
does not carry a `LoopRouteId`, a LoopBreak candidate, route terminality, or an
old-edge relation, so it cannot issue the missing source meaning.

`NormalCallableSemanticPackagePortV1::complete()` is narrower still. Its
`consumed` set proves selected-key lowering coverage, while the other checks
cover declared-instance locators, ordinary-new claims, and object definitions.
It does not compare all batch rows with LoopBreak candidates and therefore
cannot be promoted into the package observer by interpreting selected-key
consumption as loop coverage. Doing so would create a second semantic
authority at the install boundary.

Decision: reuse the batch callback as the finite parent enumeration in D1, but
keep the missing observer explicit. A same-package observer must join each
route-neutral source candidate (or a typed absence) to that enumeration before
`Cataloged`/`Selected` and before any caller-zero or delete-set claim. This
narrows the design gap; it does not close the current `NoSafeSlice`.

### Existing issuer route audit (read-only, 2026-09-20)

`CallableGenericLoopSourceFactsIssuerV1` is the closest existing source owner,
but its production disposition is closed over `Ready`, `LoopCondReady`, and
`LoopTrueReady`. `issue_once` verifies the Generic route first, then dispatches
only the LoopCond and LoopTrue source issuers; a `LoopBreakRecipe` selection
therefore reaches the named `RouteNotFrontSelected` terminal. The direct
LoopBreak terminality proof is not currently wired into this issuer.

Decision: the missing source issuer is a bounded extension of this existing
Facts owner, provided it co-seals the resolver source relations and the direct
terminality/product before the physical port. It is not permission to add a
parallel LoopBreak issuer, to reinterpret `RouteNotFrontSelected`, or to make
the legacy composer source-aware by inference. The parent observer and this
same-owner issuer remain separate obligations in the next design slice.

### Source projection shape for D1 (read-only contract)

The existing LoopCond/LoopTrue projectors establish the safe shape for this
extension. A LoopBreak projector would start from the resolver-issued root
`SourceStmtSiteV1`, open its exact condition/body through
`ResolvedFunctionLoweringInputV1::source()`, and obtain parentage/exits only
from `function().resolved_loop_source_forest(...)` and the resolved exit index.
It must never rebuild a loop member from `LoopSourceBodySiteV1` coordinates.

The bounded direct product therefore needs the same source identity tuple as
the sibling projections (`FunctionOriginV1`, `SemanticOwnerSourceKindV1`,
owner, and root frame key), plus exact sites for the loop, condition,
break-if, carrier-update, step, and the explicit break exit/record. Its forest
binding and exit rows must remain paired with those sites. The existing
`LoopBreakSourceTopologyV1` can be checked against this product for the
direct three-statement indices, but cannot serve as the resolver identity by
itself.

The projector must reject foreign owner, missing forest/exit, non-loop member,
break targeting another loop, ScopeBox-expanded direct topology, duplicate or
out-of-root sites, and any specialized `LoopBreakFacts` row whose
`source_topology` is absent. This is a D1 contract only; no new receipt or
source projection is issued in D0.

## D1 bounded task tuple (design handoff; implementation still closed)

1. **Source projection owner** — extend the existing compiler source-projector
   family with the direct LoopBreak shape above. Its only inputs are one
   `ResolvedFunctionLoweringInputV1` and one resolver-issued root site; its
   output is source evidence, never a Recipe or physical ID.
2. **Facts co-seal owner** — extend
   `CallableGenericLoopSourceFactsIssuerV1` with one exclusive LoopBreak branch
   that joins the projection, existing `PlanBuildOutcome`/
   `LoopBreakFacts`, and `DirectLoopBreakTerminalityV1`. Generic, LoopCond, and
   LoopTrue behavior remains unchanged.
3. **Package coverage owner** — in the existing package issuer, enumerate every
   batch declaration and join one candidate or typed absence by batch slot,
   owner, source identity, and catalog brand. Missing/duplicate/foreign or
   unclassified rows reject before catalog selection; selected-key consumption
   is not used as a substitute.
4. **Physical consumer** — adapt the existing source-bound LoopBreak physical
   owner to consume the move-only co-sealed product before Builder allocation,
   then reuse `CallableLoopSourceParts`, cleanup, verifier, and
   `lower_loop_v0_core`. The legacy composer remains untouched.
5. **Acceptance and retirement** — prove the full parser package reaches the
   named Cataloged/Selected terminal, then record the exact production caller,
   old edge, and exclusive delete-set before any switch or deletion. Positive
   evidence must cover the direct parser member; negative evidence must cover
   missing/foreign/duplicate/out-of-root/ScopeBox and specialized-topology
   rejection. No task closes on a selected-method-only green.

This tuple is the smallest implementation-ready design result. It does not
authorize code, fixture, fallback, backend parity, or production-switch work
until the parent observer's typed candidate/absence relation is accepted by
the existing package owner.

## Physical adapter premise audit

The physical owner is narrower than the route name suggests. The shared
`loop_v0_core` frame/carrier/backedge implementation is reusable, but the
current `loop_break_composer` cannot be called from a located source path:

| Observed owner | Boundary | Decision |
| --- | --- | --- |
| `LoopBreakFacts` + `LoopBreakSourceTopologyV1` | existing planner facts; topology is currently only the direct three-statement shape | candidate source authority; specialized LoopBreak subsets remain rejected until they issue source topology |
| `loop_break_composer` / `build_loop_break_recipe` | creates dummy AST nodes and synthetic `StmtRef` bodies before raw `lower_loop_v0` | reject as source consumer; synthetic recipe rows cannot prove source alignment |
| `CallableLoopSourceParts` + `lower_raw_loop_v0` | already validates located recipe/source bodies and enters `lower_loop_v0_core` through source closures | selected physical boundary for a future direct-generic adapter |
| `LoopCond` source token | exact route token is hard-coded to `LoopCondBreakContinue` | cannot be reused for LoopBreak |

The required design is therefore a source-bound direct-generic LoopBreak
adapter, not a call into the legacy composer. Its source Facts issuer must
co-seal the existing `LoopBreakFacts`/Recipe outcome with the direct
`break-if -> carrier-update -> step` topology, source identity, package brand,
and the exact source item/target rows. The physical adapter must validate all
of those relations before `reseal_branch_bindings` or `lower_loop_v0_core`,
construct a located `CallableLoopSourcePartsBlockV1`, and then reuse the
existing `lower_raw_loop_v0`/`loop_v0_core` owner. Missing, foreign, duplicate,
out-of-root, order-mismatched, and non-direct-specialized rows remain named
pre-effect rejects.

This audit keeps the row at `NoSafeSlice`: the missing source-aligned Recipe
construction and LoopBreak route token are design work, not permission to add
a synthetic receipt or to make the legacy composer source-aware by inference.

## Owner audit and NoSafeSlice decision

The existing package exposes source AST/catalog loans, selected callable loans,
physical-signature rows, and result contracts. None is a package-scoped
acceptance observer that can consume the selected tuple while retaining the
LoopBreak dependency as a named terminal. The LoopBreak path still uses
`LoopBreakFacts`, mutation-first `loop_break_composer`, and
`route_loop_break_recipe` with `LoopRouteContext`/`MirBuilder`; it has no
source-lineage co-seal or source-port consumer.

Therefore this row stays `design_stop` with `NoSafeSlice`. The package
acceptance-window card is handed off without a parser acceptance claim. The
next design work is bounded to the finite LoopBreak dependency inventory below;
it must not reopen the parked route generally.

## Required next-design inventory

| Item | Required evidence |
| --- | --- |
| Dependency callers | exact methods represented by the same merged package's `LoopBreakRecipe` front |
| Source issuer | resolver forest/exit ledger relation co-sealed with the existing LoopBreak facts, or an explicit missing-issuer decision |
| Physical consumer | one existing LoopBreak owner receiving one move-only source input and reusing cleanup/verifier |
| Failure boundary | named pre-effect reject for missing, foreign, duplicate, out-of-root, or mismatched source rows |
| Retirement | finite old caller/edge inventory and an exclusive delete-set; no caller-zero claim before cutover |

If the finite inventory cannot satisfy these conditions in an existing owner,
record the missing owner and keep `GenericLoopV1NotSelected` as the terminal.
Do not add a second Recipe, route, ledger, or compatibility fallback to make
the parser tuple appear green.

## Acceptance boundary

This design row can advance only after the inventory names one source issuer,
one physical consumer, one pre-effect terminal, and one exclusive deletion set.
Focused positive/negative evidence must cover exact source site, package brand,
exit/forest relation, duplicate/foreign rows, and dependency terminality. Until
then the parent parser tuple remains a design dependency and all production,
publication, and legacy-retirement claims stay closed.
