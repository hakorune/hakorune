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
