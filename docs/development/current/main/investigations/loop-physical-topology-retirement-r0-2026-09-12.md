# LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0

Status: closed; implementation complete
Task: LOOP-PHYSICAL-TOPOLOGY-RETIREMENT-R0
Date: 2026-09-12
Priority: delete the superseded fixed-role Loop physical route after caller-zero proof
Parent: `mirbuilder-loop-g0-normal-package-function-consumer-i0-2026-09-11.md`
Authority: `docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Six-line brief

```text
Decision: retire only the superseded fixed Header/Body/Step/After route; preserve the segment route as the sole physical route.
Source authority + canonical issuer: PreparedLoopPhysicalLayoutV1 and its segment receipt are the placement authorities; segment_dispatcher issues operation targets by exact segment key.
Non-authority: old topology tests, fixed-role block receipts, logical-block target lookup, storage order, current_block, fallback, retry, and any new Recipe/JoinSig/physical owner.
Fail-fast boundary: absence of every old fixed-route issuer/wrapper/caller is checked before deletion; retained segment production callers must remain explicit and typed.
Smallest next slice: remove fixed-role receipt/physicalizer APIs, old logical target issuer and dispatcher wrappers, then replace the old tests/docs/guard census with segment-route evidence.
Non-claims: no new Loop shape, semantic source change, CFG/SSA/PHI owner, selector change, backend parity, Hako cutover, or whole-MIRBuilder completion.
```

## Caller census and authorized cells

The pre-change census is finite and was independently audited before coding:

```text
physicalize_topology_v1                         0 production / 7 test callers
physicalize_topology_for_operation_demand_v1   0 production / 0 test callers
LoopPhysicalBlockReceiptV1 and its row/reject   0 production / old test fixtures only
VerifiedLoopOperationTargetBlockV1::issue      only old fixed-route wrappers
prepare_loop_operation_dispatch_v1              0 production / 2 test callers
segment allocator/dispatcher route              Callable + Generic production consumers
VerifiedLoopOperationTargetBlockV1::issue_for_segment  segment dispatcher only
```

The worker audit confirmed that the remaining production path consumes the
layout-keyed segment receipt and that no new semantic design is needed. The
authorized cells are the fixed-route definitions, their old test-only
fixtures/wrappers, the physicalizer module README/reference, and the
consolidated transfer guard. Callable/Generic lowerers, segment topology,
segment allocator, segment dispatcher, canonical CFG/SSA/PHI, and publication
owners are preserved.

## Required implementation contract

1. Delete the old topology allocation entry points, fixed-role block receipt,
   and only the old `VerifiedLoopOperationTargetBlockV1::issue` compatibility
   issuer. Keep `LoopPhysicalBlockRoleV1`, `ReadyLoopEntryV1`, and
   `LoopPhysicalServicesV1` because the segment route still uses them.
2. Delete old logical-block dispatcher preparation and wrapper emission APIs
   that have no production caller. Keep the shared at-target leaf emitters and
   `LoopOperationDispatchServicesV1` consumed by segment dispatch.
3. Remove fixed-route tests rather than preserve a disconnected observer.
   Segment positives and negatives remain the evidence for current behavior;
   add a direct wrong-target negative only if the existing segment relation can
   express it without a new authority.
4. Strengthen the guard to prove old-symbol absence and allow only the known
   Callable/Generic segment consumers. Update the owning README and canonical
   reference so historical canary text cannot be read as the current route.

## Acceptance evidence

- [x] Old fixed topology/target/dispatcher symbols have zero Rust callers and
      are absent where deletion is authorized.
- [x] Callable and Generic production segment consumers compile and retain
      exact segment target validation; no fallback/retry or second owner exists.
- [x] Segment positive, missing, foreign, duplicate/alias, and wrong-target
      evidence is green, or the last item is recorded as an explicit bounded
      non-claim when the relation cannot be represented safely.
- [x] The consolidated transfer guard, current-pointer guard, focused
      physicalizer tests, formatting/diff checks, and quick-profile check pass.
- [x] All changed Rust source remains below 800 lines; no unrelated baseline
      formatting drift is rewritten.

## Non-claims and stop

This row does not change Recipe, JoinSig, physical layout meaning, recursive
After, Tail/Completion, selector, publication, backend, or source-to-exe
acceptance. Return to `design_stop` if a retained production caller requires
fixed logical placement, if a wrong-target proof needs a new relation/receipt,
or if deleting a wrapper would move semantic ownership instead of removing a
caller-zero compatibility edge.

## Worker consultation receipt

Read-only worker James audited the fixed-topology/segment caller boundary on
2026-09-12. The report found no new semantic design requirement and selected
this bounded retirement slice. The worker was closed before implementation;
there were no edits, Cargo runs, or worktree mutations from the worker.

## Implementation receipt

The fixed-role topology receipt/row/reject types, fixed topology issuers, old
logical target issuer, old dispatcher preparation/emission wrappers, and three
disconnected test modules were deleted. The facade now exposes only the
current segment route and shared services; Ledger remains owned by its own
module instead of becoming a second facade authority. All physicalizer Rust
sources remain below 800 lines; the current largest source is the 704-line
Callable production canary, while the production segment dispatcher is 464
lines.

Evidence on 2026-09-12:

```text
CARGO_BUILD_JOBS=1 cargo check --profile quick -j1
  passed; existing baseline warnings only (1,792 warnings)
CARGO_BUILD_JOBS=1 cargo test --profile quick -j1 \
  'loop_recipe_physicalizer' -- --nocapture
  22 passed / 0 failed; 7,876 filtered out
bash tools/checks/loop_physical_transfer_authority_guard.sh
  passed
bash tools/checks/current_state_pointer_guard.sh
  passed
git diff --check
  passed
```

The focused suite covers current segment positive production canaries,
missing exact segment, foreign entry owner, duplicate/alias physical
placement, late duplicate publication, fresh-session replay, and the
Callable PHI/backedge assertion. A direct wrong-target mutation is a bounded
non-claim: `issue_for_segment` receives an item and a segment receipt, but the
current segment receipt does not own the item-to-segment relation. Adding that
relation would be a new semantic/physical receipt outside this retirement
slice, so no unsafe synthetic negative was added.
