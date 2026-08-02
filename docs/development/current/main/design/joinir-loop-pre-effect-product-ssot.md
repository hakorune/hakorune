---
Status: Active design stop
Date: 2026-08-02
Decision: accepted — `JOINIR-LOOP-ALL-ROUTE-PRE-EFFECT-PRODUCT0-D0`
Scope: all-route Loop qualification and the future logical-product / physicalizer boundary
Related:
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
  - src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs
---

# JoinIR Loop Pre-effect Product

## Authority

One raw Loop enters `lower_loop_or_freeze_v1 -> try_cf_loop_joinir -> route_loop`.
`try_build_loop_facts_inner` owns source observation, and the ordered
19-entry `ENTRIES` registry plus predicate/suppression policy owns candidate
qualification. `RecipeFirstRouteSelectionV1` is a pure candidate schedule;
it is not a selected logical lowering product.

`try_execute_recipe_first_selection` is the present effect boundary. It passes
`&mut MirBuilder` to every `RouteFn`; composers allocate physical IDs/blocks or
lower a physical `CorePlan`. `CorePlan`, `LoopRouteKind`, located GenericV1
proof products, and dropped raw-loop child receipts are not a replacement
authority for this boundary.

## Decision

The final Loop path needs one non-Clone, Builder-free product, issued once
from the existing Facts/registry authority and consumed once by one typed
physicalizer. It must contain exactly one selected `LoopRouteId`, or a typed
NoRoute / contract-reject / ambiguity disposition, plus ordered located
condition/body/cleanup/suffix demands, transferred/opaque boundaries, and
logical binding/carrier roles.

It must not contain `MirBuilder`, `CorePlan`, real `ValueId` or
`BasicBlockId`, PHIs, `Frag`, MIR instructions, or a retry capability. Its
producer may not use an AST rewrite or turn `LoopRouteKind` into a second
selector. Decision B-prime remains: no raw/reference-profile reopening and no
universal ingress.

This is not permission to introduce a wrapper or an I0. The D0-selected shape
is `LoopQualificationDispositionV1`, whose only success payload is a non-Clone
`VerifiedLoopRouteDemandV1`. Its single producer consumes the existing source
view, facts, predicates, and suppression policy once. Its selected payload has:

```text
route id
+ located condition / ordered body / cleanup / suffix demands
+ explicit transferred and opaque subtree boundaries
+ logical binding, carrier, exit, and continuation roles
+ route-local structural payload (never a rebuilt AST)
```

`Rejected` and `Ambiguous` are typed pre-effect dispositions. Physicalization
may consume only `VerifiedLoopRouteDemandV1`; it cannot choose a route, read
facts/AST, or return `None` for another candidate to try. The exact Rust module
and constructors are S0 work, but this ownership and field boundary is fixed.

## Effect-order inventory

| Route | Pre-effect logical payload | First physical boundary | Current retry/required disposition |
| --- | --- | --- | --- |
| `LoopBreakRecipe` | header/break conditions, update placement, carrier/step roles | `lower_loop_v0` frame allocation | all decline must be typed before lowering |
| `IfPhiJoin` | header/if conditions, then/else updates, step and carrier roles | `lower_loop_v0` frame allocation | all decline must be typed before lowering |
| `LoopContinueOnly` | header, continue condition, ordered carrier updates, step | `lower_loop_v0` frame allocation | nested/contract gates are pre-effect |
| `LoopTrueEarlyExit` | true header, exit kind/value, carrier update, step | `lower_loop_v0` frame allocation | absent contract is pre-effect reject |
| `LoopSimpleWhile` | located sole step assignment and loop role | `lower_loop_v0` frame allocation | nested decline is pre-effect |
| `LoopCharMap` | located three source children and transform roles | `lower_loop_v0` frame allocation | never use its synthetic `ch` AST as payload |
| `LoopArrayJoin` | located separator branch, append, step, and roles | `lower_loop_v0` frame allocation | never use rebuilt AST as payload |
| `ScanWithInit` | condition, exit body, step, dynamic-needle roles | `lower_loop_v0` frame allocation | recipe contract/shape reject is pre-effect |
| `SplitScan` | condition, join branches, result/i/start carriers | `lower_loop_v0` frame allocation | recipe contract/shape reject is pre-effect |
| `BoolPredicateScan` | condition, predicate call, false exit, step | `lower_loop_v0` frame allocation | receiver role remains logical/source-bound |
| `AccumConstLoop` | condition, ordered accumulator update and increment | `lower_loop_v0` frame allocation | shape/contract reject is pre-effect |
| `NestedLoopMinimal` | outer/inner conditions, init, updates, accumulator | direct block/Value allocation in composer | no-value-join/exit gates are pre-effect |
| `LoopTrueBreakContinue` | classified body recipe, exit/continue and carrier policy | loop-true skeleton allocation | nested release decline is pre-effect |
| `LoopCondBreakContinue` | complete recipe, accept kind, policy, exit/continue semantics | Standard5 block allocation | no post-effect decline |
| `LoopCondContinueOnly` | recipe, carrier demand, continue cleanup | coreloop-frame allocation | nested release decline is pre-effect |
| `LoopCondContinueWithReturn` | recipe, carrier, return/continue topology | coreloop-frame allocation | nested release decline is pre-effect |
| `LoopCondReturnInBody` | recipe, precomputed carrier demand, return/fallthrough topology | Standard5 allocation | carrier fallback must move before effects |
| `GenericLoopV0` | loop variable, condition, increment, body, carrier role | generic skeleton allocation | release verifier/lower `None` is forbidden at cutover |
| `GenericLoopV1` | V0 fields plus body policy, exit/no-exit recipe, step disposition | generic skeleton allocation | release verifier/lower `None` is forbidden at cutover |

The table is the 19-row membership inventory. `P0` must ratchet individual
rows against `ENTRIES`, including suppression, every pre-effect decline,
first Builder effect, and whether post-effect `None` is presently possible.
The existing scheduler unit that proves first-candidate `None` followed by a
second candidate is the mandatory counterexample; runtime smoke output alone
is not evidence of pre-effect uniqueness.

## Task order

1. `JOINIR-LOOP-ROUTE-EFFECT-ORDER-MATRIX0-P0`
   - Add one table-driven test/manifest ratchet for every registry row and the
     source-backed minimal Counter loop fixture. Reuse the registry test module
     and `mirbuilder_inplace_replacement_guard.sh`; add no row-specific shell
     guard and change no route behavior.
2. `JOINIR-LOOP-ALL-ROUTE-LOGICAL-PRODUCER0-S0`
   - After P0 proves the input, implement one Builder-free producer over all
     19 rows. Partial family coverage has no production consumer.
3. `JOINIR-LOOP-SINGLE-PHYSICALIZER-CUTOVER0-I0-R0`
   - Switch the named production caller only after the product is all-route.
     Delete the ordered `RouteFn -> Option` retry loop and every selected old
     physicalization edge atomically.

## Gates and stop line

P0's focused gate is `cargo test --lib route_entry::registry::`; its shared
structural gate is `bash tools/checks/mirbuilder_inplace_replacement_guard.sh`.
The representative runtime backstops are
`phase29bq_fast_gate_vm.sh --only entry_ambiguous_break_min` and
`phase29bq_fast_gate_vm.sh --only generic_loop_v1_recipe_nested_if_loop_min`.

Stop immediately if a row needs Builder state, physical IDs, CorePlan
composition, environment reclassification, or route execution to establish
logical demand; if `raw_execution` and diagnostic projection are conflated;
or if an alleged proof relies only on runtime output/tag parity. Do not use a
transaction/rollback as a substitute: it would preserve mutable fallback.

## D0 closeout

The source inventory is complete: all 19 registry rows have an identifiable
pre-effect logical payload and a first physical boundary. The shared product
must carry located source demands rather than current reconstructed recipe ASTs;
CharMap and ArrayJoin prove this is essential. Generic V0/V1 prove that a
post-effect verifier/lower failure cannot remain `None` at cutover. Next row is
`JOINIR-LOOP-ROUTE-EFFECT-ORDER-MATRIX0-P0`; it changes only registry proof
surfaces and leaves route behavior unchanged.

## P0 closeout

`registry/effect_order_matrix_tests.rs` now ratchets all 19 `ENTRIES` rows in
production order, their qualification boundary, first physical boundary, and
post-effect-`None` class. It names Generic V0/V1 as the only explicit release
conversion of verifier/lower failure to `None`; the existing scheduler retry
test remains the counterexample. Registry tests and the shared MirBuilder guard
are green. Next is all-route S0 only.

## S0 Refactor Series contract

Purpose: establish the source-identity substrate required by the selected
all-route product. The series is BoxShape-only and each commit must build.

1. `JOINIR-LOOP-SOURCE-RECEIPT-FOUNDATION0-S0`
   - Mint a loop-relative source receipt before `flatten_scope_boxes`, carry it
     through `LoopFacts`, and prove raw body order remains observable. No
     selector, producer, physicalizer, or caller changes.
2. `JOINIR-LOOP-LOGICAL-DEMAND-VOCABULARY0-S1`
   - Move `LoopRouteId` and its stable keys from physical `types.rs` into a
     neutral registry sibling, then add private registry-local source/role/product
     vocabulary. It imports no Builder, CorePlan, other physical ID, or lowering
     API and has no route adapter.
3. `JOINIR-LOOP-ALL-ROUTE-LOGICAL-PRODUCER0-S2`
   - Add one all-19-route producer over existing selection/Facts. It returns a
     typed selected/rejected/ambiguous disposition without invoking a composer.
4. `JOINIR-LOOP-LOGICAL-DEMAND-CONTRACT0-P0`
   - Ratchet source order, all-route correspondence, and forbidden physical
     imports through the existing shared guard. Production consumers remain 0.

The future I0/R0 is a separate decision after this series. Generic and nested
routes that cannot establish physical admissibility before effects must return a
typed pre-effect rejection in S2; they must never preserve a `None` retry.

### S0 series 1 evidence

`LoopSourceReceiptV1` is minted from the original loop body before
`flatten_scope_boxes` and carried by `LoopFacts`. It records only original
condition/body ordinal coordinates; it owns no AST reference, route, planner,
or lowerer. Synthetic `LoopFacts` have an unavailable receipt, so a later
producer must reject rather than infer source coordinates. Its ScopeBox test
proves one original body statement remains ordinal one when the analysis body
has flattened to two statements. No production caller or routing behavior moved.

### S0 series 2 evidence

`LoopRouteId` and its stable keys now live in neutral `route_id.rs`; registry
selection, execution order, and P0's 19-row matrix remain unchanged. Private
`logical_demand` vocabulary owns source views, logical role order, and typed
`Selected`/`NoRoute`/`Rejected`/`Ambiguous` dispositions. The selected product
is non-Clone, source nodes are borrowed rather than rebuilt, and the subtree has
no Builder, CorePlan, physical-ID, composer, lowerer, selector, or caller import.
S2 alone may issue the product; it must stay pre-effect and disconnected.

### S0 series 3 evidence

`qualify_selected_loop_route_v1` consumes `raw_execution_routes()` once and
issues `NoRoute`, `Ambiguous`, or a typed rejection before any route execution.
All 19 exact-one routes currently reject as
`RouteSourceTopologyUnavailable { route }`: the receipt deliberately lacks the
per-extractor raw source topology needed for a truthful selected demand. This is
the required fail-fast boundary; it does not clone/re-match AST, invoke a
composer/lowerer, alter the legacy retry path, or have a production caller.

### S0 series 4 evidence and D1 handoff

The existing shared MirBuilder guard now calls a small library helper that
checks the logical-demand subtree for forbidden physical/selection imports and
enforces the per-file limit. The 19-row producer ratchet is green. S0 is closed.
The next blocker is D1: Facts need a minimal, route-local pre-flatten
provenance contract. It must preserve source topology without cloned AST, a
second selector, or a reusable raw-source ingress.

## D1 route-source provenance decision

Decision: accepted. `LoopSourceReceiptV1` remains a count/ordinal receipt; it
cannot authorize a selected demand. Add a facts-private
`LoopSourceProjectionV1` alongside the one existing ScopeBox flatten operation:
each flattened analysis statement receives an opaque original-body plus
ScopeBox-child coordinate. It owns no AST and has no path-to-AST resolver.

The projection is common infrastructure only. Route Facts own their own small,
typed topology later; `LoopFacts` must not grow a route-to-topology map. The
first BoxCount series is:

1. `JOINIR-LOOP-SOURCE-PROJECTION0-S0`: issue and retain aligned ScopeBox
   lineage; no route behavior changes.
2. `JOINIR-LOOP-SIMPLE-WHILE-PROVENANCE0-S1`: add only the simple-while
   condition/step topology, permitting this one logical demand to become
   selected while every other route remains rejected.
3. `JOINIR-LOOP-SIMPLE-WHILE-PROVENANCE-CONTRACT0-P0`: fixture and shared-guard
   ratchet, then repeat one route = one topology = one fixture/gate/commit.

Source authority is `try_build_loop_facts_inner` before flattening. Registry,
logical product/producer, and physical layers cannot issue coordinates, match
AST, resolve a generic path, select a route, or lower. Typed provenance failure
remains rejection, never `None` retry.

### Source projection S0 evidence

`flatten_scope_boxes_with_projection` is now the sole top-level ScopeBox
flatten operation for loop Facts. It returns the existing cloned analysis body
and a coordinate-only `LoopSourceProjectionV1`; `LoopFacts` carries the latter.
Nested ScopeBox lineage is covered by a focused fixture. No route topology,
producer disposition, runtime path, or source resolver changed. S1 may add only
the `LoopSimpleWhileFacts` step coordinate.

### LoopSimpleWhile provenance S1 evidence

`LoopSimpleWhileFacts` now optionally carries one opaque step coordinate when
the single flattened increment aligns with the facts-side projection. Direct
extractor and production Facts-builder fixtures cover nested ScopeBox lineage.
The topology retains no AST and is not consumed by the producer; all 19 current
logical dispositions remain the same typed rejections. P0 only ratchets this
boundary before any selected-demand connection is considered.

### LoopSimpleWhile provenance P0 handoff

The shared logical-demand helper now covers the projection and simple-while
Facts sources; direct/nested ScopeBox and builder-carriage fixtures are green.
The next blocker is D2: decide whether the one existing topology can issue a
truthful borrowed `Selected` demand without creating a path resolver or a
runtime connection. All other routes remain rejected.

## D2 direct SimpleWhile selection decision

Decision: accepted for direct top-level step only. A receipt gains an opaque
condition/body-frame identity stamp, checked before source borrowing. For an
exact-one `LoopSimpleWhile` selection, a topology whose step has empty
ScopeBox lineage may borrow only the raw condition and that raw body statement;
the demand owns the coordinate and logical role. Nested ScopeBox lineage remains
`Rejected` rather than being resolved. This preserves source-frame identity
without storing AST, re-matching, generic path resolution, selector re-run, or
runtime connection. All other 18 routes remain typed rejected.

### Direct SimpleWhile logical-demand S0 evidence

The producer now issues `Selected` only for exact-one `LoopSimpleWhile` with a
matching source frame and an empty ScopeBox lineage. Its non-Clone, route-local
payload exposes just the verified raw condition and raw step statement, plus the
owned topology, loop-binding name, and `LoopBinding`/`LoopBackContinuation`
roles; it cannot expose an arbitrary body slot. Nested ScopeBox lineage rejects
as `RouteSourceTopologyNotDirectlyBorrowable`; a same-length foreign frame
rejects as `SourceFrameMismatch`; no-route, overlap, unavailable topology, and
the other 18 routes retain terminal typed outcomes. The shared guard forbids
producer AST matching, projection traversal, extractor invocation, and selector
re-entry. There is still no physicalizer or production caller.

## Next-route provenance D0 decision

Decision: accepted `AccumConstLoop` as the next one-route S0. Its existing
extractor admits exactly two flattened statements: an accumulator update and a
loop increment. S0 may retain only their two aligned `LoopSourceBodySiteV1`
coordinates when the common projection proves the complete flattened body; it
does not issue `Selected`. A later selection row may borrow raw `Condition`,
`BodyStatement(acc_update)`, and `BodyStatement(step)` only when the receipt
frame matches, both lineages are empty, and their raw indices differ.

`LoopCharMap` remains parked despite its explicit three-child payload because
its synthetic `ch` boundary and competing generic shape require a separate
exact-selection proof. `LoopTrueEarlyExit` remains parked because its direct
raw `if` must be treated as an opaque exit subtree, including separate
return/break and carrier policy. No AST clone, path resolver, selector re-run,
physicalizer, runtime caller, or second route is authorized in Accum S0.

### AccumConstLoop provenance S0 evidence

`AccumConstLoopFacts` now retains optional route-local accumulator-update and
step coordinates only when the existing projection covers the complete two-item
analysis body. Direct and nested ScopeBox fixtures prove those coordinates are
aligned and carried through `LoopFacts`; the latter retains lineage rather than
resolving it. The producer explicitly proves this new topology remains
`RouteSourceTopologyUnavailable { AccumConstLoop }`, so this S0 changes no
logical disposition, selector, runtime route, physicalizer, or caller. D1 must
now decide whether the direct two-site case can issue a truthful borrowed
product.

### AccumConstLoop logical-selection D1 decision

Decision: `NoSafeSelected` with the current receipt. Direct Accum topology is
structurally sufficient only under a trusted live source transaction: both
lineages must be empty, the two raw indices must differ, and the three borrows
would be condition/update/step with `LoopBinding`, `AccumulatorBinding`, and
`LoopBackContinuation` roles. However, `LoopSourceReceiptV1` records address
integers and length without a lifetime/brand relationship to `LoopFacts` or
the existing selection. After an original source frame drops, address reuse can
falsely satisfy the stamp. The concurrent foreign-frame test does not prove
this temporal identity.

Before further Selected issuance, D2 must design one non-forgeable live
source-frame capability binding the original raw borrows, Facts, and the one
existing selection transaction. It must revalidate the already-disconnected
direct SimpleWhile product too. The capability must not become generic AST
ingress, path resolver, second selector, physicalizer, runtime connection, or
retry mechanism. Until then, Accum remains
`RouteSourceTopologyUnavailable { AccumConstLoop }` and the existing direct
SimpleWhile result has no production caller.

## Live source-frame capability D2 decision

Decision: accepted a consuming, factory-issued live transaction; rejected all
address/nonce repairs and any constructor that accepts source, facts, and
selection as independent values. The existing facts builder is the sole issuer
of a private non-Clone live pair retaining `&condition`, `&body`, and the
`LoopFacts` derived from those same borrows. A later registry-side transaction
(not `logical_demand/`) consumes that pair, canonicalizes its retained facts,
calls the existing selector exactly once, and consumes itself to qualify a
route-local product. It exposes no raw view, facts, selection, generic slot
lookup, or parts constructor.

The first implementation box is `JOINIR-LIVE-LOOP-FACTS0-S0`: issue the live
pair only. S1 then adds the registry transaction and reissues SimpleWhile
through it; Accum remains rejected until its own selected row. The guarantee is
per capability transaction (one selection and one terminal consumption), not
global uniqueness for a loop. There is no runtime/physicalizer caller in this
series.

### LiveLoopFacts S0 evidence

`LiveLoopFactsV1<'src>` is now an opaque, non-Clone pair issued only by the
facts builder's test-facing foundation. It retains the original condition/body
borrows and exactly their derived `LoopFacts`, with no fields or parts accessor
outside its facts module. Its source-lifetime fixture proves the retained frame
is the input frame; registry logical-disposition tests remain unchanged. The
shared logical-demand guard covers this new source-authority file and its line
limit. S1 may consume this pair only in a registry-side transaction.

### Live logical-transaction S1 result

Decision: `NoSafeSelected` without a logical uniqueness decision. The direct
SimpleWhile fixture's actual existing `raw_execution_routes()` is
`[LoopSimpleWhile, GenericLoopV0]`; its earlier Selected test used a test-forged
single-route selection and therefore cannot prove a transaction-issued result.
The live capability does not authorize changing legacy ordered execution or
silently selecting its first entry. D3 must decide a pre-effect uniqueness
boundary that accounts for the full raw execution list without using the
diagnostic projection, rerunning predicates, or adding fallback. Until then,
the live transaction and reissued SimpleWhile product are not implemented.

## Logical selection uniqueness D3 decision

Decision: accepted an ordered pre-effect terminality certificate as the only
future route-winner boundary. A logical transaction consumes the existing raw
schedule once. It may select route `r` only after every earlier entry is proven
pre-effect ineligible and `r` is proven guaranteed to terminate under the same
legacy conditions; later entries remain an `UnreachedLegacyTail`, not rejected
or deleted. Any post-effect-dependent/unknown earlier or current route blocks
pre-effect issuance rather than falling through. This preserves legacy order
without treating priority as success.

The currently safe terminal boundary remains full raw-list exact-one;
diagnostic-effective routes and first-entry selection are non-authority. Since
direct SimpleWhile is `[LoopSimpleWhile, GenericLoopV0]` and its route still
uses Builder-bound composition/lowering, `JOINIR-LOOP-SIMPLE-WHILE-TERMINALITY0-D0`
must inventory whether its every decline can be classified without effects.
Until then, SimpleWhile is not reissued and GenericLoopV0 is not silently
chosen.

### SimpleWhile terminality D0 decision

Decision: accepted a bounded direct non-nested terminality certificate. The
existing handler's only `Ok(None)` is its `detect_nested_loop(ctx.body)` pre-gate.
For the direct source topology already accepted by SimpleWhile Facts, that gate
is ineligible before composition. Its `StandardEntry` has both None-returning
configuration flags disabled; missing contract and every composer/lower failure
propagate `Err`, which terminates the legacy scheduler. The composed SimpleWhile
path is `CorePlan::Loop`; loop completion returns `Some(void)` or `Err`, not
`None`. This proves only scheduler terminality—not builder readiness, lowering
success, rollback, or runtime behavior.

S0 may encode that narrow certificate with direct/nested fixtures and an
existing shared guard ratchet. It may not issue a product or select a route;
the ordered terminality transaction remains a later box and must retain
GenericLoopV0 as an unreached legacy tail when the certificate is consumed.

### Direct SimpleWhile terminality S0 evidence

`DirectSimpleWhileTerminalityV1` now issues only for exactly one raw direct
statement, empty ScopeBox lineage, and non-nested Facts. It carries no AST or
product and proves only that the existing handler's nested None pre-gate is
unreachable. Direct and ScopeBox fixtures distinguish terminality from
ineligibility. The existing shared guard statically ratchets that the handler
has exactly one `Ok(None)` return and it is the nested pre-gate; later route
behavior remains `Some` or `Err`. D1 must design ordered consumption with the
live source frame and full raw schedule tail.

### Live ordered terminality transaction D1 decision

Decision: accepted one registry-owned parent module containing the private
`LiveLoopFactsV1<'src>` carrier and its consuming transaction child. This is the
only arrangement that lets the transaction consume the same live source/Facts
pair without exposing a raw-parts accessor or a generic callback portal: Rust
sibling privacy cannot express a facts-owned carrier that only registry may
open. `try_build_loop_facts_inner` remains the sole AST observation authority;
the facts builder may call the registry parent’s narrow binding bridge exactly
once, and the parent’s child alone may read the private carrier fields.

The transaction consumes the capability by value, canonicalizes that exact owned
Facts value, calls `select_recipe_first_routes(Some(&canonical))` exactly once,
and uses only `raw_execution_routes()` as schedule authority. It issues no
logical product and no success claim. Its positive disposition is only
`PreEffectSchedulerTerminal`: the selected route is proven to stop the legacy
scheduler with either `Some` or `Err`, and it retains the later raw suffix as
an ordered `UnreachedLegacyTail`. For the actual direct SimpleWhile shape, raw
order is `[LoopSimpleWhile, GenericLoopV0]`; the positive proof therefore keeps
`GenericLoopV0`, in that order, as tail evidence. It neither deletes, rejects,
executes, nor selects that route.

A candidate requires every earlier raw route to have a route-specific
pre-effect-ineligible certificate and itself to have a scheduler-terminality
certificate. Current direct SimpleWhile is first, so its earlier proof set is
empty but its terminality certificate remains mandatory. Unknown earlier routes,
unknown current terminality, ScopeBox/nested source, and unsupported routes fail
closed with a typed non-selection; empty raw order is `NoRoute`. The result has
no raw/Facts/selection getter, no reusable request or slot, and no Builder,
physicalizer, runtime, retry, or fallback behavior.

S0 fixture and guard contract: actual derived direct facts must produce the
SimpleWhile/GenericLoopV0 order and terminal proof with exact tail; ScopeBox and
nested fixtures must fail closed; a synthetic unknown earlier prefix must block
SimpleWhile; raw empty is `NoRoute`. The shared guard must ratchet the sole
binding bridge, exactly one raw selection call, no diagnostic projection or AST
re-match, and no public raw/Facts/selection accessors. The old
`logical_demand::producer` remains test-only legacy evidence and is not a caller
or dependency of this transaction; a later row may retire it after replacement.

### Live ordered terminality transaction S0 evidence

The registry now owns `live_ordered_terminality/` as one narrow parent with a
private non-Clone carrier and a consuming transaction child. The facts builder
is the sole caller of `bind_live_loop_facts_v1`; the transaction canonicalizes
that exact owned Facts value and invokes raw selection once. Its only positive
result is `PreEffectSchedulerTerminalV1`, which exposes the selected route and
ordered unreached tail, never AST/Facts/selection. Actual direct facts prove
SimpleWhile with `[GenericLoopV0]` tail; ScopeBox fails current certification,
an unknown earlier route blocks, and empty raw order is `NoRoute`. The shared
guard ratchets one bridge, one selection call, no diagnostic projection, no AST
re-match, and no legacy logical-demand dependency. There remains no production
consumer and no logical product.

### Direct SimpleWhile logical product D0 decision

Decision: accepted a private `live_ordered_terminality/logical_product/` sibling
whose issuer consumes `LiveOrderedTerminalityDispositionV1` by value and nothing
else. The transaction must place a private direct source lease (condition and
the direct raw step) only in its positive terminality proof; therefore the issuer
does not receive raw source, Facts, selection, receipt, topology, a slot, or a
callback. It issues a non-Clone direct-SimpleWhile product retaining route roles
and the ordered `UnreachedLegacyTail`. This remains a pre-effect admission
request, not composition/lowering/execution success.

There is no safe existing consumer: all current route paths start with Builder
bound composition/lowering, and `located_loop` already consumes a physical
CorePlan. The only consumer is a future design boundary
`DirectSimpleWhilePhysicalizerV1`; S0 creates neither that API nor a production
caller. Final all-route policy is unchanged: this partial product stays
disconnected and is not cutover authority.

S0 replaces—not supplements—the forged `logical_demand` test-only producer.
Its new actual-order fixtures and shared guard must land before the old
source/roles/product/producer subtree and address-stamped receipt APIs are
deleted in the same commit, eliminating dual issuance and every independent
source ingress. ScopeBox/nested/unknown-earlier/empty remain fail-closed;
Accum remains non-issued.

### Direct SimpleWhile logical product S0 evidence

`logical_product.rs` now consumes only the terminality disposition and issues a
non-Clone direct-SimpleWhile product with private source lease, fixed logical
roles, selected route, and ordered GenericLoopV0 tail. Its actual-source fixture
proves issuance only after the real `[LoopSimpleWhile, GenericLoopV0]` schedule;
the transaction retains the existing fail-closed ScopeBox, unknown-earlier, and
empty cases. The former `logical_demand` subtree was deleted in the same change,
including its forged selection, independent source view, roles vocabulary, and
address-stamped receipt APIs. The receipt is now raw-body arity only. No Builder,
CorePlan, physical ID, runtime, or production caller entered this box.

### Direct SimpleWhile physicalizer D0 closeout

Decision: rejected as `NoSafeSlice`. The opaque product exports neither source
lease nor Facts/CorePlan, while every existing SimpleWhile path is Builder-bound:
`route_standard` requires route context/outcome/environment, the composer rebuilds
AST from Facts, and `lower_loop_v0` allocates frame/PHI/block state before later
effect emission. Existing wrappers do not provide Builder rollback. Reusing that
path would discard product authority and retain the legacy `Result<Option<_>>`
retry scheduler, so it cannot be a disconnected consumer.

Future cutover requires an all-route pre-effect product and a sealed exact
physicalization context (Builder, source/facts/outcome/environment, and policy),
a Builder-free compose/preflight proof before the first mutation, one commit
boundary, and terminal `Result<ValueId, Freeze>` errors with no `None`/retry.
Only then may the named production caller replace the full ordered scheduler and
delete its old physical edges atomically. Direct SimpleWhile remains caller-zero;
its GenericLoopV0 tail is audit evidence, never fallback.

### All-route physicalization D0 closeout

Decision: rejected as `NoSafeSlice` for current `CorePlan` reuse. All 19
composers take `&mut MirBuilder` before returning a `CorePlan` that already owns
physical `ValueId`/`BasicBlockId`; no all-Builder rollback exists. The current
families are 11 LoopV0-frame recipe routes, Nested direct blocks, LoopTrue direct
blocks, four LoopCond frames, and two Generic skeletons. Their existing local
variable-map restores and poisoned sessions do not cover core/type/MIR mutation.

The accepted future architecture is: (1) an all-19 Builder-free logical
preflight/product dialect with exact source/facts/policy ownership and no AST
reconstruction; (2) a separately designed all-state `LoopEmissionDraftV1` (or
equivalent journal) with abort/publish-once proof; and only then (3) a sealed
context committing `Result<ValueId, Freeze>` once, with no `None`/retry and an
atomic scheduler replacement. P0 next fixes static mutation/None evidence only;
it does not implement any of these boxes.

### Compose mutation census P0 evidence

The registry matrix now preserves the existing exact 19-row order and additionally
distinguishes five first-mutation families: eleven LoopV0-frame routes, one Nested
block-ID route, one LoopTrue skeleton route, four LoopCond-frame routes, and two
Generic-skeleton routes. Its cardinality test prevents collapsing Nested or
LoopTrue into a generic direct-block label. This is observation only: it changes
no predicate, route, composer, lowerer, or caller, and it is the input inventory
for the next Builder-free preflight design stop.

### Builder-free all-route preflight D0 decision

Decision: accepted one registry-owned, private all-route preflight issuer. It
will bind the original live frame, exact owned Facts/policy snapshot, and one
`raw_execution_routes()` schedule; it cannot re-match AST, rebuild source,
re-read environment policy, use diagnostic selection, or expose a generic raw
slot. The future non-Clone result is `NoCandidate`, typed `Rejected { route,
reason }`, or `Qualified` with route-local borrowed operands, ordered schedule
evidence, logical roles/boundaries, and a mutation-family label—but no Builder,
CorePlan, physical IDs, composer/lowerer, or consumer.

All 19 routes are mandatory membership, not mandatory current qualification.
Current truthful raw operands exist only for direct SimpleWhile (condition and
step) and direct AccumConstLoop (condition, accumulator update, step). Accum
remains policy/terminality-blocked; Simple remains the disconnected direct
product and is not an all-route issuance exception. The other 17 routes lack
route-local raw topology and must reject `SourceTopologyUnavailable`; ScopeBox
lineage rejects separately. Generic V0/V1 additionally retain post-effect retry
debt and must not qualify. P0 next records this exact schema/membership/source/
policy matrix before the preflight module exists.

The migration rule is no dual authority: the future all-route issuer subsumes
the direct issuer only once all membership/operands/policy proof exists. Legacy
composers remain the only execution path until an all-route Builder-free compose
proof, `LoopEmissionDraftV1` publish/abort proof, and atomic scheduler cutover
are separately complete.

### Preflight schema matrix P0 evidence

The 19-row registry matrix now fixes all current preflight states: 17 routes
reject for missing route-local source topology, AccumConstLoop rejects for missing
policy/terminality proof despite its operands, and direct SimpleWhile is explicitly
direct-only rather than all-route-qualified. The matrix has zero qualified rows.
This separation prevents a generic raw-body fallback or accidental promotion of
the disconnected direct product when the private preflight vocabulary is added.

### Builder-free preflight vocabulary S0 evidence

`loop_preflight.rs` now owns only non-Clone `NoCandidate`/typed rejection
vocabulary. It distinguishes source topology, ScopeBox lineage, policy/terminality,
scheduler order, and post-effect retry debt without importing AST/Facts/selection,
Builder, CorePlan, physical IDs, composer, or lowerer. Its unit fixture fixes those
reasons as separate pre-effect outcomes; there is no producer, qualified variant,
consumer, or caller. D1 must now decide how one live facts/policy/schedule
transaction reaches this vocabulary without a second authority.

### All-route preflight producer D1 decision

Decision: accepted `live_ordered_terminality/all_route_preflight.rs` as a private
child beside the historical direct transaction/product. It consumes the existing
non-Clone `LiveLoopFactsV1` by value, so no second facts bridge, raw source
accessor, callback, or widened field visibility exists. Direct and all-route paths
cannot consume one capability sequentially; they coexist only as caller-zero,
disconnected evidence until the all-route issuer can preserve direct operands/tail
and retire the direct child atomically.

The transaction canonicalizes the owned Facts and invokes existing selection once,
using only `raw_execution_routes()`. Empty schedule is `NoCandidate`. Otherwise it
diagnoses only the raw front in order: missing topology, then non-borrowable
ScopeBox lineage, then policy/terminality, then explicit post-effect retry debt.
It never skips a rejected front to select a later candidate. `SchedulerOrderBlocked`
is reserved for a future self-qualified later candidate with an undischargeable
prefix; current S0 has zero qualified rows. RouterEnv/environment is not read or
captured because no truthful dynamic all-route policy proof exists yet.

### All-route preflight producer S0 evidence

`all_route_preflight.rs` now lives beside the direct historical transaction and
consumes the same private `LiveLoopFactsV1` capability by value. It canonicalizes
once, selects the raw schedule once, and emits only `NoCandidate` or the raw
front's typed rejection. Direct SimpleWhile is policy/terminality-rejected rather
than qualified; ScopeBox lineage precedes that policy diagnosis. The shared guard
ratchets this child’s one selection, no direct-product dependency, and unchanged
sole source bridge. No all-route product, consumer, or caller exists.

### AccumConstLoop terminality D0 decision

Decision: accepted a narrow direct scheduler-terminality certificate. The canonical
raw shape selects exactly `[AccumConstLoop]`: SimpleWhile requires one body step,
GenericV0 rejects non-loop accumulator writes, and GenericV1 is predicate-suppressed
by Accum. The route has no `Ok(None)` path: contract absence, composer failure, and
verify/lower failure are `Err`; its `CorePlan::Loop` completion is `Some` or `Err`.
This proves no lower success, mutation rollback, Builder readiness, or runtime result.

S0 requires existing topology to show raw body arity two, accumulator-update index
zero, step index one, distinct sites, and empty ScopeBox lineage, plus an actual
raw-schedule fixture. ScopeBox/topology/index mismatch issues no certificate. The
certificate does not qualify the all-route preflight or issue a product/caller.

### AccumConstLoop terminality S0 evidence

`DirectAccumConstLoopTerminalityV1` now requires raw body arity two, distinct
direct update/step sites at indices zero/one, and empty ScopeBox lineage. Its actual
fixture fixes raw selection to `[AccumConstLoop]` and the terminal certificate;
ScopeBox receives none. The shared guard ratchets that the Accum handler has no
`Ok(None)` return. This is scheduler terminality only and does not modify all-route
preflight's zero-qualified policy, product issuance, physicalization, or caller.

### LoopBreakRecipe provenance D0 decision

Decision: accepted one narrow source-topology box only for the generic fallback's
direct three-statement branch. `LoopBreakFacts` currently carries cloned and, in
some subsets, derived expressions with no source sites; `LoopSourceProjectionV1`
cannot resolve AST and preflight must not reconstruct it. S0 may retain only three
opaque whole-statement sites observed in that existing branch: break-if, carrier
update, and step. ScopeBox lineage stays in the topology so preflight can reject
borrowability later. All specialized/local/prelude/parse subsets remain topology
absent. Raw tail and scheduler terminality are deliberately unclaimed; existing
composer is mutation-first and remains NoSafeSlice for logical consumption.

S0 implementation record: the generic branch retains those coordinates only when
the break-if is the first flattened statement. The local-prelude variant has no
distinct carrier-update/step schedule and remains topology absent. Direct and
ScopeBox fixtures fix the coordinate and lineage behavior; all-route preflight
still has zero qualified routes.

### LoopBreakRecipe preflight classification D0 decision

Decision: the existing all-route preflight owns this classification and keeps its
single canonical raw-schedule selection. For raw front `LoopBreakRecipe`, S0 reads
only `LoopBreakFacts.source_topology`, with strict precedence: absent facts or
topology is `SourceTopologyUnavailable`; any nonempty site lineage is
`ScopeBoxLineageNotBorrowable`; only an all-direct three-site topology reaches
`PolicyAndTerminalityUnavailable`. The last outcome is deliberately not Qualified:
the three coordinates do not prove raw tail, handler terminality, builder-free
logical consumption, or physical safety. Fixtures must fix the raw schedule front
and each reject. S0 adds no AST re-match, selector rerun, product, caller, route
skip, or composer/physical authority.

S0 implementation record: the preflight fixture set fixes generic direct to the
policy/terminality rejection, generic ScopeBox to the lineage rejection, and a
read-digits specialized LoopBreak to source-topology unavailable. Each fixture
also asserts the actual raw front is `LoopBreakRecipe`. The production selector
still runs exactly once; the shared guard now excludes test-only selector calls
when checking that production boundary.

### LoopBreakRecipe terminality D0 decision

Decision: accept a scheduler-terminality-only S0 for the generic direct
three-statement topology. Its fixture proves the exact raw schedule is
`[LoopBreakRecipe]`; direct source receipt/sites are `0/1/2` with empty ScopeBox
lineage. `route_loop_break_recipe` has no `Ok(None)` return: its pre-compose
contract failure, compose/verification/lowering failures are `Err`, and a
successful loop lower finalizes as `Some(void)`. ScopeBox receives no
certificate. This is not a pre-effect proof: `lower_loop_v0` begins physical
work after its precondition checks, first allocating carrier PHI ValueIds, and
later errors have no rollback. S0 therefore adds no product, qualified arm,
caller, composer reuse, or physical claim.

S0 implementation record: the direct fixture fixes the exact singleton raw
schedule and its certificate; ScopeBox remains fail-closed. The shared guard
ratchets the LoopBreak handler's `Ok(None)` count to zero. This makes the
existing effect-order matrix stale: direct LoopBreak now belongs to the
policy/terminality-unavailable bucket, ScopeBox to lineage-unavailable, and
specialized LoopBreak remains source-topology-unavailable. The next BoxShape
card changes that observation only, from 17/1/1 to 16/2/1.

Matrix-realignment S0 implementation record: `effect_order_matrix_tests.rs` now
records `LoopBreakRecipe` as policy/terminality unavailable and ratchets the
current preflight totals to 16 source-unavailable, 2 policy/terminally
unavailable, and 1 direct-only. No production classifier, selector, route, or
qualification changed.

### IfPhiJoin provenance D0 decision

Decision: accept a two-site S0 sourced only from extractor-observed top-level
indices: the whole if-else statement and the loop step statement. The extractor
may accept raw `if; step` or `step; if`; it must therefore carry those observed
indices rather than infer `0/1` or use composer order. Projection has no nested
AST resolver, so if-condition and then/else carrier updates remain clone/derived
facts, never source operands. The existing flattened projection retains direct
empty lineage and ScopeBox lineage for both whole statements. S0 adds no raw-tail
or terminality claim, all-route classification/product/caller, or physical work.

### IfPhiJoin provenance S0 implementation record

`IfPhiJoinFacts` now retains `IfPhiJoinSourceTopologyV1` only when the existing
flattened projection aligns with the accepted extractor body. The extractor
carries the observed if-else and step indices, so direct `if; step`, direct
`step; if`, and ScopeBox-wrapped `if; step` retain their actual coordinates and
lineage. The production loop-facts builder forwards that same projection. The
legacy no-projection facts entry leaves topology absent. No nested branch
expression gains a source resolver or source authority. The next card is the
all-route-preflight classification D0; no terminality, product, physical, or
caller claim has been added.

### IfPhiJoin preflight classification D0 decision

Decision: the existing all-route preflight remains the sole classifier and
single raw-schedule selector. Its `IfPhiJoin` raw-front branch may read only
optional `IfPhiJoinFacts.source_topology` and its opaque if-else/step sites.
The strict rejection order is: absent facts or topology gives
`SourceTopologyUnavailable`; any nonempty ScopeBox lineage gives
`ScopeBoxLineageNotBorrowable`; two direct sites give
`PolicyAndTerminalityUnavailable`. The last case is not Qualified: provenance
does not prove raw tail, handler terminality, Builder-free logical consumption,
or physical safety. Actual if/step source order is retained provenance only and
is not a preflight policy input.

S0 adds this one classifier branch and fixtures that first assert the actual
raw front is `IfPhiJoin`, then assert direct, ScopeBox, and topology-absent
rejections. It updates the effect-order observation from `16/2/1` to `15/3/1`.
It adds no AST re-match, source projection access, selector rerun, route skip,
product, caller, handler/composer/lowerer/Builder change, or terminality claim.

### IfPhiJoin preflight classification S0 implementation record

`classify_front` now has one `IfPhiJoin` branch. It reads only the optional
IfPhiJoin topology and the two opaque whole-statement sites: missing facts or
topology reject for source unavailability, ScopeBox lineage rejects before
policy, and direct topology rejects for unavailable policy/terminality. The
fixtures assert the actual raw front before each direct, ScopeBox, or
topology-absent result. The effect-order matrix now records `15` source,
`3` policy/terminality, and `1` direct-only rows. No route is Qualified and no
selector, source bridge, AST observer, composer, lowerer, Builder, product,
caller, or terminality behavior changed.

### IfPhiJoin terminality D0 decision

Decision: accept a direct scheduler-terminality-only S0. `route_if_phi_join`
has contract and composition error arms, then delegates to the verified lower;
there is no route-local `Ok(None)` path. The composed `LoopV0` ultimately
returns `Some(void)` or an error. The direct certificate requires actual raw
selection exactly `[IfPhiJoin]`, two distinct direct topology sites, raw body
arity two, and observed coordinates either `(if_else, step) = (0, 1)` or
`(1, 0)`. This retains observation order rather than assuming composer order.
ScopeBox receives no certificate.

This is strictly scheduler terminality. `lower_loop_v0` begins physical work
by allocating its loop frame and PHI ValueIds before later failure points, and
the composer synthesizes its canonical recipe from cloned facts. S0 must not
claim pre-effect safety, rollback, raw tail, all-route policy, semantic-order
equivalence, product, Qualified, caller, or runtime success. It adds only the
opaque certificate, direct/reversed/ScopeBox fixtures, and a guarded
`route_if_phi_join` `Ok(None)`-count ratchet.

### IfPhiJoin terminality S0 implementation record

`DirectIfPhiJoinTerminalityV1` now accepts only an exact direct two-statement
receipt whose distinct if-else and step sites retain one of the two observed
source orders, `(0, 1)` or `(1, 0)`. Both fixtures prove the singleton raw
schedule `[IfPhiJoin]`; ScopeBox is fail-closed. The shared guard ratchets
`route_if_phi_join` to zero `Ok(None)` returns. This certificate says only
that the selected scheduler route ends `Some(void)` or `Err`: it adds no
pre-effect/product/Qualified/caller/rollback/physical/runtime claim.

The next design stop is logical-product eligibility. Because the existing
composer rebuilds a canonical if-then-step recipe from cloned facts while the
accepted raw source order can be reversed, scheduler terminality alone cannot
provide an ordered borrowed raw tail. D0 must decide whether that boundary is a
NoSafeSlice before any product is considered.

### IfPhiJoin logical-product D0 decision

Decision: NoSafeSlice. The existing logical-product issuer accepts only its
private direct-SimpleWhile terminality disposition, which owns a borrowed
condition/step lease and SimpleWhile-specific product roles. The IfPhiJoin
certificate is intentionally opaque and owns no borrowed source lease. More
importantly, accepted IfPhiJoin provenance permits raw `(step, if_else)` while
the existing composer synthesizes its canonical if-then-step recipe from
cloned facts. There is no source-order-preserving raw tail or semantic-order
equivalence proof to adapt the SimpleWhile issuer.

No product, Qualified, caller, selector, composer, lowerer, Builder, or
physical behavior changes. The next card must select another route from fresh
topology/terminality/product-readiness evidence rather than retrying this
boundary under a renamed IfPhiJoin implementation.

### Next-route selection D0 decision

Decision: select `AccumConstLoop` logical-product eligibility D0. Its direct
facts already retain projection-aligned accumulator-update and step sites at
raw indices `0/1`; the direct fixture proves singleton raw schedule
`[AccumConstLoop]`, and the direct scheduler certificate has no `Ok(None)`
path. Unlike IfPhiJoin, its source ordering does not conflict with a synthetic
canonical composer order. The former address-stamped receipt identity blocker
is now bound by the existing non-Clone `LiveLoopFactsV1` bridge.

D0 must still establish a route-local borrowed condition/update/step lease and
its logical roles/order. It must not widen the SimpleWhile transaction by
analogy, treat terminality as all-route qualification, or touch AST matching,
selection, composer/lowerer/Builder, physicalization, runtime, or callers.

### AccumConstLoop logical-product D0 decision

Decision: accept a narrow route-specific S0. `LiveLoopFactsV1` remains the
only non-Clone owner consuming source, Facts, and the one canonical selection.
Its terminality disposition gains an Accum-specific direct source lease for the
raw condition, accumulator-update, and step; `logical_product` receives only
that proof and issues the corresponding product variant. The source roles are
in raw order: condition/`LoopBinding`, update/`AccumulatorBinding`, then
step/`LoopBackContinuation`.

The transaction must prove Accum is the first raw route and must fail closed
for an earlier route, failed direct certificate, ScopeBox lineage, non-two-item
body, or update/step indices other than distinct `0/1`. S0 may not re-match
AST/Facts, traverse projection, reselect, alter the SimpleWhile variant,
qualify all routes, or touch composition/lowering/Builder/physical/runtime
paths or callers. Issuance remains a disconnected pre-effect request only.

### AccumConstLoop logical-product S0 implementation record

The one-shot live transaction now has an Accum-specific direct source lease for
the raw condition, accumulator update, and step. It reaches the terminality
disposition only when Accum is the first and sole raw route and the existing
direct certificate accepts its `0/1` topology. `logical_product` consumes that
disposition by value and issues the route-specific three-role product; it takes
no source, Facts, projection, or selection input. Direct and ScopeBox fixtures
fix issuance and fail-closed behavior. No physical consumer, Builder/CorePlan,
route cutover, runtime effect, or production caller has been added.

### AccumConstLoop physicalizer D0 decision

Decision: NoSafeSlice. The only existing Accum execution path enters
`route_accum_const_loop`, composes a builder-bound recipe, and calls
`lower_loop_v0`. That lowerer mutates through `build_coreloop_frame` and
`alloc_typed` before later header/body lowering failures are excluded. The
opaque product cannot reach that path without discarding product authority and
reintroducing the legacy retry scheduler; no rollback boundary exists.

No physicalizer, consumer, caller, selector, composer, lowerer, Builder, or
runtime behavior changed. The next logical-product D0 is LoopBreak, whose
direct topology and scheduler certificate exist but whose break-if subtree and
carrier placement roles still require an independent source-bound design.

### LoopBreak logical-product D0 decision

Decision: accept a generic-direct-only S0. The existing direct topology and
certificate prove raw arity three, distinct direct sites at `0/1/2`, singleton
raw schedule `[LoopBreakRecipe]`, and no route-local `Ok(None)`. The one-shot
transaction may therefore own a source lease of raw condition, whole break-if
statement, carrier update, and step. Product roles remain in raw order:
`LoopCondition`, opaque `BreakGate`, `CarrierUpdate`, and
`LoopBackContinuation`.

The break-if is a whole opaque statement only: its nested condition and any
internal update receive no source authority. S0 may not re-match AST, traverse
projection, rerun selection, alter another route, or touch composition,
lowering, Builder, physicalization, callers, or runtime. The product is
disconnected and does not claim physical safety.

### LoopBreak logical-product S0 implementation record

The one-shot transaction now adds a generic-direct LoopBreak branch only after
the direct `0/1/2` certificate and a sole raw `[LoopBreakRecipe]` schedule. Its
private lease retains raw condition, opaque break-if statement, carrier update,
and step; the product consumes that proof by value with the four fixed roles.
An injected earlier LoopBreak without actual LoopBreak facts remains
`BlockedEarlier`, preserving the existing raw-order rule. Direct issuance is
fixture-fixed; ScopeBox and specialized forms remain fail-closed. No AST
inspection of the break-if subtree, physical consumer, caller, or runtime
behavior was added.

### LoopBreak physicalizer D0 decision

Decision: NoSafeSlice. The direct LoopBreak product is opaque and its only
legacy execution path composes cloned facts into a Builder-bound recipe then
enters `lower_loop_v0`. That lowerer allocates its frame and ValueIds before
later header/body failures, with no rollback boundary. Reusing it would discard
the product authority and revive the retry scheduler.

No consumer, physicalizer, caller, selector, composer, lowerer, Builder, or
runtime behavior changes. The next card returns to one-route readiness
selection rather than retrying LoopBreak physicalization.

### Next-route selection D1 decision

Decision: select `LoopContinueOnly` provenance D0. It is the next registry
route with a bounded top-level continue-if/update/step shape, but it currently
has no route-local source topology. D0 must first establish only
extractor-observed whole-statement coordinates and ScopeBox lineage. Nested
continue-if contents and cloned or derived facts remain non-source; terminality,
raw tail, product, physical safety, and callers remain separate cards.

### LoopContinueOnly provenance D0 decision

Decision: NoSafeSlice. The extractor observes only the continue-if index and
requires it to be zero. Carrier updates are collapsed into a `BTreeMap`, losing
raw statement indices, duplicates, and source order; the loop step is retained
only as a cloned expression, with no observed statement index. Therefore a
continue-if-only coordinate would be incomplete and misleading, and no
truthful whole-statement topology can be issued.

The prerequisite is a separate extractor-topology D0 that defines complete
observed indices together with update cardinality and duplicate policy, without
changing route acceptance. No projection plumbing, selection, terminality,
product, physicalization, caller, or runtime behavior changed here.

### LoopContinueOnly extractor-topology D0 decision

Decision: accept behavior-neutral optional source topology S0. Existing facts
retain their current BTreeMap and cloned-step behavior. A projection-aware
extractor may retain opaque role/site entries only when the whole flattened body
is exactly: continue-if at index `0`, one or more distinct carrier updates in
observed order, and one final top-level loop step. The entries are `ContinueIf`,
`CarrierUpdate { carrier }`, and `Step`; each maps through the existing
projection and carries no AST/expression resolver. Duplicate carriers, a
nested-only/missing/non-final step, another top-level statement, or projection
mismatch makes topology absent, not a new facts rejection.

The old extractor wrapper supplies a default projection and remains topology
absent. No selection, terminality, product, physicalizer, caller, or runtime
behavior is added. Later cards must separately decide whether a direct topology
may be borrowed; ScopeBox lineage here is observation only.

### LoopContinueOnly extractor-topology S0 implementation record

The projection-aware facts path now retains an optional ordered opaque schedule
of continue-if, carrier updates, and final step only for complete top-level
observations. The legacy wrapper uses the default projection and stays topology
absent; facts acceptance and cloned semantic fields are unchanged. The next
card must decide direct/ScopeBox provenance consumption separately.

### LoopContinueOnly terminality D0 decision

Decision: accept a narrow direct pre-effect scheduler certificate S0. The
certificate may be issued only from a facts-owned opaque proof of a complete
direct schedule: `ContinueIf@0`, one or more distinct `CarrierUpdate`s, and a
final `Step`, with no ScopeBox lineage and the sole raw selection
`[LoopContinueOnly]`. Missing or malformed topology, ScopeBox lineage, or an
earlier route must fail closed.

This proves only that the selected route does not normally decline into the
legacy tail before effects. It must not claim a successful route value:
`PlanLowerer` may return `Ok(None)` for a void loop. It also must not claim raw
source order or semantic equivalence, because the composer reconstitutes
carrier updates from a `BTreeMap`; nor may it grant AST authority below the
opaque whole statements.

`lower_loop_v0` mutates frame/block/ValueId state before later failures are
excluded, so physical safety and rollback remain unproven. S0 adds no logical
product, composer/lowerer/Builder change, physicalizer, caller, or runtime
behavior. Its fixtures must cover direct issuance, ScopeBox rejection,
malformed/duplicate/non-final topology rejection, and injected-earlier-route
rejection.

### LoopContinueOnly terminality S0 implementation record

The facts-owned topology now has one opaque complete-direct-schedule predicate.
It verifies matching raw arity, direct indices and empty ScopeBox lineage,
`ContinueIf@0`, one or more distinct interior carrier updates, and a final
step. A route-local certificate consumes that predicate only with the sole raw
selection `[LoopContinueOnly]`; malformed, ScopeBox, and injected-earlier
schedules fail closed. It keeps no AST source lease and changes no execution
path, so it claims neither lower success nor physical safety.

### LoopContinueOnly logical-product D0 decision

Decision: accept a narrow disconnected S0. The terminality proof must retain
an opaque direct lease of only the loop condition and whole direct body. The
logical-product issuer then consumes terminality by value and may issue exactly
four aggregate roles: `LoopCondition`, opaque `ContinueIfSubtree`, aggregate
`CarrierUpdateSequence`, and `LoopBackContinuation`. The carrier sequence is
one-or-more as a unit; it grants neither per-carrier source order nor a nested
AST resolver.

The complete-direct topology predicate and singleton raw selection remain the
only admission proof. S0 may not re-match AST/Facts, rerun selection, traverse
projection, borrow source independently, alter another route, or create a
consumer. Existing composition is explicitly out of scope because it rebuilds
carrier updates from a key-sorted `BTreeMap` and enters mutating `lower_loop_v0`.
No composer/lowerer/Builder/physicalization/rollback/caller/runtime claim is
authorized. Fixtures must fix direct issuance and ScopeBox/current rejection.

### LoopContinueOnly logical-product S0 implementation record

The terminality capability now retains only the opaque direct condition/body
lease required to preserve source authority. Its by-value issuer creates a
caller-zero product with the four fixed aggregate roles; it never exposes a
per-carrier site, source-order promise, or AST resolver. Direct issuance and
ScopeBox rejection are fixture-fixed. No execution consumer was introduced.

### LoopContinueOnly physicalizer D0 decision

Decision: NoSafeSlice. The only execution chain is
`route_loop_continue_only → compose_loop_continue_only_recipe → lower_loop_v0
→ lower_verified_core_plan/PlanLowerer`. After initial checks,
`lower_loop_v0` allocates blocks and carrier/after PHI ValueIds, and updates
`variable_map`, before header verification and body lowering can still fail.
Continue-with-PHI construction and subsequent physical emission can fail too.
There is no rollback capability around those mutations.

The opaque aggregate product cannot safely call this legacy path: doing so
would discard its authority boundary and leave partial Builder state on an
error. A builder-free validation plus transactional commit/snapshot boundary is
a separate BoxShape series, not a physicalizer slice. No consumer, composer,
lowerer, Builder, caller, or runtime behavior changed.

### Next-route selection D2 decision

Decision: select `IfPhiJoin` provenance S0. Its accepted complete body has two
top-level sites, the whole if-else and loop step, and the existing projection
can represent both with ScopeBox lineage. S0 may retain only those opaque whole
statement sites; branch-internal conditions and updates remain cloned semantic
facts. Terminality, product, physicalization, callers, and runtime remain
separate cards.

### Current-pointer correction and next-route selection D3 decision

Decision: `IfPhiJoin` provenance S0 was already closed in `0c0eb73a14`; its
topology, preflight, terminality, and later logical-product NoSafe boundary are
recorded above. The stale current pointer selected it again, so no duplicate
implementation is authorized.

Select `LoopTrueEarlyExit` provenance D0. It is the next registry route with a
bounded extractor shape and no existing topology: Return has whole top-level
`exit-if, step`; Break has `exit-if, carrier-update, step`. D0 must decide only
whether those flattened observations can be represented through the existing
projection. The nested exit-if payload/condition and cloned carrier expression
remain non-authoritative. Terminality, selection use, product, physical safety,
caller, and runtime behavior are out of scope.

### LoopTrueEarlyExit provenance D0/S0 implementation record

Decision: accept behavior-neutral optional topology S0. The projection-aware
extractor retains Return as opaque `exit-if, step`, and Break as opaque
`exit-if, carrier-update, step`, only when the accepted flattened body has the
exact corresponding arity and a complete aligned projection. Default-projection
calls remain topology-absent; any mismatch leaves existing facts acceptance
unchanged. ScopeBox lineage is retained only as observation. Direct Return and
Break plus ScopeBox fixtures fix the contract.

Nested exit-if condition/payload and the cloned carrier expression have no
source authority. No selector/preflight consumer, terminality, product,
composer/lowerer/Builder, physicalizer, caller, or runtime behavior changed.
The next card is preflight D0 only.

### LoopTrueEarlyExit preflight D0/S0 implementation record

Decision: accept a raw-front-only preflight S0. Its single route branch reads
only optional opaque topology: absent facts/topology reject as source
unavailable; any ScopeBox lineage rejects as not borrowable; direct topology
rejects as policy-and-terminality unavailable. Direct Return and Break plus a
ScopeBox Break fixture fix those outcomes.

The generic handler can normally return `Ok(None)` without a planner contract,
so this classification grants no terminality, qualification, product, source
lease, physical-safety, caller, or runtime claim. The next card is terminality
D0 and must audit that contract boundary rather than infer it from topology.

### LoopTrueEarlyExit terminality D0 decision

Decision: NoSafeSlice. `route_loop_true_early_exit` uses `route_standard` with
`skip_without_contract`. When planner-required mode is off and no recipe
contract is present, that route normally returns `Ok(None)` and the registry
continues to its raw-tail candidates. The direct Return/Break topology records
neither `RouterEnv` nor recipe-contract state, so it cannot exclude that arm.

Strict planner-required missing-contract errors and successful compose/lower
paths do not repair this proof gap: they are environment-specific branches, not
a topology-derived route guarantee. Moreover, compose/lower mutates Builder
state before all later failures are excluded, so physical safety is separately
unproven. No terminality certificate, product, physicalizer, caller, or runtime
behavior changed. A revival needs an upstream non-Clone route-execution witness
that binds raw schedule, environment, and contract disposition before Builder
effects.

### Next-route selection D4

Select no implementation yet. Re-inventory the registry after the completed
LoopBreak/IfPhiJoin/ContinueOnly/LoopTrueEarlyExit paths and choose one unlanded
bounded route with its source authority and failure boundary written first.

### Next-route selection D4 decision

Decision: select `LoopCharMap` provenance D0. Its facts accept one exact
flattened three-statement shape—substring local, result update using that
binding, and final `+1` step—but retain only cloned or derived semantic fields.
D0 may decide an optional projection-aware opaque topology for those three
whole statements only. The recipe's reconstructed body, including synthetic
`ch`, is not source authority and cannot be validated from this topology.

No preflight, selector, terminality, product, composer/lowerer, Builder,
physicalizer, caller, or runtime change is authorized. The planner-required
route's ordinary `Ok(None)` behavior makes terminality a later independent D0.

### LoopCharMap provenance D0/S0 implementation record

Decision: accept behavior-neutral optional topology S0. The projection-aware
facts path retains only the opaque flattened sites at `0/1/2` for substring
local, result update, and final step when the complete three-statement body is
aligned. The legacy wrapper supplies the default projection and stays topology
absent; facts acceptance and synthetic-recipe behavior are unchanged.

No source resolver, synthetic `ch` authority, preflight, terminality, product,
composer/lowerer/Builder, physicalizer, caller, or runtime behavior was added.
The next card is preflight D0.

### LoopCharMap preflight D0 decision

Decision: accept raw-front-only classification S0. The all-route preflight may
read only optional opaque CharMap topology and reject as source unavailable,
ScopeBox lineage not borrowable, or direct policy-and-terminality unavailable.
It may add only an aggregate lineage accessor; synthetic `ch`, reconstructed
recipe AST, source resolvers, selection changes, and execution are forbidden.

The route is planner-required-only and can `Ok(None)` outside that mode, so no
terminality/product/physical claim is authorized. Keep the production
classifier below the existing file cap; place direct/ScopeBox/topology-absent
fixtures in its sibling test module.

### LoopCharMap preflight S0 implementation record

The raw-front classifier now consumes only the opaque aggregate lineage check.
Sibling fixtures prove direct policy rejection, ScopeBox rejection, and a
manually absent topology source rejection; the production parent and test child
remain separately below 800 lines. No synthetic recipe inspection, selector,
terminality, product, physicalizer, caller, or runtime behavior changed.

The next card is terminality D0. Planner-required-only `Ok(None)` behavior
must be audited from route environment/contract evidence, never inferred from
topology or preflight.

### LoopCharMap terminality D0 decision

Decision: NoSafeSlice. `route_loop_char_map` is planner-required-only, so
`route_standard` normally returns `Ok(None)` when that environment flag is off
and the registry continues its raw tail. Existing live terminality binds only
source/Facts and recomputes selection; it does not bind `RouterEnv` or the
dynamic recipe-contract disposition. Direct topology and raw-front preflight
therefore cannot prove scheduler terminality.

Contract-accepted compose/lower success does not close this gap, and its
Builder-bound lowering can fail after effects. No terminality certificate,
product, physicalizer, caller, or runtime behavior changed. A future revival
needs an execution-scoped environment-and-contract witness upstream of Builder
effects. Select the next unlanded route through fresh D5 inventory.

### Next-route selection D5 decision

Decision: select `LoopArrayJoin` provenance D0. Its extractor admits one exact
flattened three-statement body: separator guard, array append, and final step.
D0 may decide an optional opaque projection-aligned topology for those whole
statements only. Nested guard contents, get receiver/argument, cloned semantic
fields, and reconstructed recipe AST are not source authority.

No selector/preflight, terminality, product, composer/lowerer, Builder,
physicalizer, caller, or runtime behavior is authorized. Its `route_standard`
contract behavior makes terminality a later separate D0.

### LoopArrayJoin provenance D0/S0 implementation record

Decision: accept behavior-neutral optional topology S0. The projection-aware
facts path retains the opaque sites at `0/1/2` for separator guard, array
append, and final step only when the exact flattened body aligns. The legacy
wrapper keeps default-projection topology absent and facts acceptance unchanged.

Nested guard contents, get receiver/argument, cloned semantic fields, and
reconstructed recipe AST remain non-authoritative. No preflight, terminality,
product, execution, physicalizer, caller, or runtime behavior changed. The
next card is preflight D0.

### LoopArrayJoin preflight D0/S0 implementation record

Decision: accept raw-front-only classification S0. The classifier reads only
the opaque aggregate lineage check and distinguishes topology absent, ScopeBox
lineage, and direct policy-and-terminality unavailable. Sibling fixtures fix
each result and keep the production parent below 800 lines.

No guard-internal inspection, source borrowing, reconstructed recipe validation,
terminality, product, physicalizer, caller, or runtime behavior was added. The
next card is terminality D0; `Ok(None)` contract behavior must be audited from
route execution evidence rather than inferred from topology.

### LoopArrayJoin terminality D0 decision

Decision: NoSafeSlice. `route_loop_array_join` uses `route_standard` with
`skip_without_contract`; in non-planner mode without a recipe contract it
returns `Ok(None)` and raw route execution continues. The live terminality
capability retains only source/Facts, not `RouterEnv` or contract disposition,
so direct topology and raw-front selection cannot exclude this arm.

No terminality certificate, product, physicalizer, caller, or runtime behavior
changed. Compose/lower remains separately Builder-bound and may mutate before
later failures. Fresh D6 inventory must select the next unlanded route.
