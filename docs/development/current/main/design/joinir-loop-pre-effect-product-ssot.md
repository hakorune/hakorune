---
Status: Active design stop
Date: 2026-08-02
Decision: provisional — `JOINIR-LOOP-ALL-ROUTE-PRE-EFFECT-PRODUCT0-D0`
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

This is a target contract, not permission to introduce a wrapper or an I0.
The producer/type name and physicalizer signature remain unselected until the
effect-order proof below makes every route's logical demand and effect boundary
explicit.

## Effect-order inventory

| Route IDs | Current first route-side action | First physical boundary | Retry risk |
| --- | --- | --- | --- |
| `LoopBreakRecipe`, `IfPhiJoin`, `LoopContinueOnly`, `LoopTrueEarlyExit` | route-specific facts / contract gate | RecipeComposer then `parts::entry::lower_loop_v0` | handler `None` or route error policy must be classified |
| `LoopSimpleWhile`, `LoopCharMap`, `LoopArrayJoin` | route/nested gate and facts | RecipeComposer then `parts::entry::lower_loop_v0` | `None` can advance ordered execution |
| `ScanWithInit`, `SplitScan`, `BoolPredicateScan`, `AccumConstLoop` | facts / contract gate | recipe or direct composer using Builder | `None` can advance ordered execution |
| `NestedLoopMinimal`, `LoopTrueBreakContinue` | release/nested eligibility gate | Builder-backed composer | compose/lower decline needs pre-effect disposition |
| `LoopCondBreakContinue`, `LoopCondContinueOnly`, `LoopCondContinueWithReturn`, `LoopCondReturnInBody` | release/nested eligibility gate | Builder-backed loop-cond pipeline | decline must not be learned after mutation |
| `GenericLoopV0`, `GenericLoopV1` | facts and release eligibility | Builder-backed skeleton/pipeline and lowerer | current release maps verifier/lower failure to `None`, then tries the next route |

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
