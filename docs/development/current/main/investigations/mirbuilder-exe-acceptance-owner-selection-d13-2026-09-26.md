# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D13

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1); D12/S0 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary):
  [plan/freeze:contract] loop winner selection declined:
  zero selected family candidates
  fn=StringHelpers.trim/1
  site=SourceStmtSiteV1(SourceNodeSiteV1([Body(2)]))
  cond=BinaryOp{And, Less{i,n}, Or{Equal{substring," "},
        Equal{substring,"\t"}}}

VM (debug binary):
  same decline
  fn=JsonStreamAggregator.ingest/1
  site=[Body(2)]
  cond=BinaryOp{Less, start, n}
```

`StringHelpers.starts_with/3` (return-in-body) now lowers via
the composite LoopBreak owner (S0). The next terminals are
exit-free `loop(cond)` bodies — no `Break`, `Continue`, or
`Return` anywhere under the root.

## Census findings

### `StringHelpers.trim/1` (EXE pure-first, callable lane)

Instrumented `NonGenericOrOverlapping` arm:

```text
D13CENSUS routes=[LoopBreakRecipe]
```

- `loop(i<n && (s.substring==" "||s.substring=="\t")){i=i+1}`:
  `try_extract_loop_break_trim_whitespace_subset` →
  `try_extract_trim_header_condition_subset` matches the
  whitespace-header condition and SYNTHESIZES `break_condition`
  (`build_not_whitespace_condition`) into plan-lane
  `LoopBreakFacts` — designed vocabulary for the retired
  plan-lane LoopBreak pipeline.
- `matched=[LoopBreakRecipe]`; `LoopBreakRecipe` is
  `RouteWireCoverageV1::TypedDeclined` — physically owner-less.
- `pred_loop_cond_break_continue` is suppressed by
  `!pred_loop_break_recipe` (predicates.rs:61), so the
  designed no-exit owner never enters `matched` even though
  `try_extract_loop_cond_no_exit_facts` produces
  `LoopCondBreakContinueFacts{NoExitBody}` for `[i=i+1]`.
- The no-exit S0 card (landed 2026-09-25) records the owner
  chain: `CallableGenericLoopSourceFactsIssuerV1` →
  `loop_cond::issue` → `lower_loop_cond_break_continue_source`
  — "the intended promotion" for `loop(var<bound){stmt*}`.
- So the dead `LoopBreakRecipe` route both (a) occupies
  `matched` making `sole_family` unreachable and (b) actively
  blocks the live designed route. Route-vocabulary stale
  guard, not a missing owner.
- Expected downstream: `s.substring` calls live in the loop
  CONDITION; the route token's call-coverage gate
  (`SourceCallOutsideSelectedFamily`/`SourceItemsMissing`, D5
  family) may be the next honest terminal.

### `JsonStreamAggregator.ingest/1` (VM: ledger-less; EXE: F2)

- `loop(start<n){local; if end<0{end=n}; local; start=...}`:
  `if end<0{end=n}` reads `end` in its condition and writes
  `end` in its body → `ConditionalUpdateIf` — the no-exit
  extractor's declared F2 boundary (`Ok(None)`), and the
  no-exit S0 card inventories the `ConditionalUpdateIf` parts
  arm as the next candidate (D4 F2).
- VM lane: `callable_ledger` absent for this instance-method
  frame → `lower_non_callable_loop_route_v1` → `route_loop` →
  five-family spine (DirectAccum/NestedPredicate/LoopTrue/
  LoopCond/GenericG0) — all shape-decline → `Declined`. The
  ledger absence on the VM lane is a separate authority
  (instance-method ledger admission), not this card's fix.
- EXE lane: sole LoopCond front-selects and declines at
  `loop-cond-item-unsupported` (`ConditionalUpdateIf`) — the
  designed F2 terminal (prior `s0` log evidence).

### Other shapes reachable in compile order

- `trim`'s second loop `loop(j>i && (...)){j=j-1}` same class
  (trim-header subset, `direction`/`delimiters` variants).
- `binary_trees.iterationCheck` / `mimalloc seedBlocks`:
  `route-not-front-selected SourceCallOutsideSelectedFamily`
  — the D5 call-coverage family, already inventoried; not
  this card's boundary.

## Decision

```text
Decision: `trim`-class decline is a stale-route-suppression
  bug, not a missing owner — the TypedDeclined
  `LoopBreakRecipe` route (owner retired with the plan-lane
  pipeline) suppresses `pred_loop_cond_break_continue`, which
  is the designed sole family for `NoExitBody` facts
  (no-exit S0). Unblock the live designed route; keep
  TypedDeclined routes out of sole-family contention.
Source authority + canonical issuer:
  `CallableLoopRouteMatchV1` (route predicates, predicates.rs)
  → `sole_family()` (front-selection,
  normal_callable_loop_source_route.rs) →
  `issue_callable_loop_source_facts_v1` → `loop_cond::issue` →
  `lower_loop_break_continue_source` (existing physical
  owner, NoExitBody-ready).
Non-authority: `LoopBreakFacts` subset extractors
  (`trim_header_condition` etc. — provenance facts, not route
  authority); `LoopBreakRecipe` route ID (TypedDeclined, no
  physical owner); winner spine; composite LoopBreak
  (exit-evidence invariant unchanged).
Fail-fast boundary: `ConditionalUpdateIf` items (ingest) stay
  the F2 parts boundary; continue-only/exit-bearing shapes
  keep existing routes; ledger-less lanes (VM ingest) keep
  `route_loop` spine; TypedDeclined routes remain in
  `matched` for diagnostics but must not gate `sole_family`.
Smallest next slice (S1): drop `!pred_loop_break_recipe`
  from `pred_loop_cond_break_continue`/`pred_loop_true_
  break_continue` (dead-route suppression) + `sole_family()`
  selects exactly one FRONT-SELECTABLE family (LoopCond/
  LoopTrue) ignoring TypedDeclined route IDs + pins:
  trim-header loop → `matched` includes LoopCondBreakContinue
  and sole family front-selects; `LoopSimpleWhile` (V1Parity,
  not TypedDeclined) overlap still yields sole=None
  (no-exit S0's recorded zero-behavior-change boundary
  preserved); then re-run EXE smoke for the next honest
  terminal (expected: `SourceCallOutsideSelectedFamily` for
  condition `substring` calls or `trim` loop 2 / `ingest`
  ConditionalUpdateIf).
Non-claims: does not fix `ingest`'s F2 ConditionalUpdateIf
  parts boundary or its VM ledger absence; does not admit
  continue-only or ConditionalUpdateIf-bearing bodies into
  no-exit vocabulary; does not extend sole-family vocabulary
  beyond the existing two front-selectable families; does
  not touch the winner spine or composite lane.
```

## Exit

- [x] Failing loops identified; lane and route observations
  recorded (`matched=[LoopBreakRecipe]`, TypedDeclined).
- [x] Owner audit: designed no-exit owner exists; blocking
  is stale dead-route suppression.
- [x] One bounded S1 emitted (route-vocabulary hygiene).
