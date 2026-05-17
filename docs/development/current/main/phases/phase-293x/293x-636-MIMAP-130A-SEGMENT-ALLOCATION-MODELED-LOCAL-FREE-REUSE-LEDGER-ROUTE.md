# 293x-636 MIMAP-130A Segment Allocation Modeled Local-Free Reuse Ledger Route

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-130A` records successful modeled local-free reuse as a dedicated scalar
reuse allocation ledger row.

The row is intentionally separate from `segment_allocation_modeled_ledger_box`.
That ledger owns bump-shaped modeled consumes. Local-free reuse returns a
specific `reused_block_id`, so the new owner must record the reused block
without changing the bump ledger contract.

## Scope

Owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

Proof app:

```text
apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh
```

The owner may consume a successful
`HakoAllocSegmentAllocationModeledLocalFreeReuseReport`, derive a deterministic
scalar reuse token from `(segment_id, page_id, reused_block_id)`, record one
live row, reject live duplicates, and expose deterministic scalar reads.

## Acceptance Shape

The proof output should make the ledger behavior explicit:

```text
first=1,0,<row>,<existing>,<token>,<segment>,<page>,<reused_block_id>,<used_before>,<used_after>,<count>,<live_count>
duplicate=0,4,<existing>
inactive=0,0,0,0,0,0,0,0,0
summary=ok
```

Exact field order can be adjusted during implementation, but it must be stable
in the guard and distinguish:

- successful reuse row recording;
- duplicate live reuse rejection;
- deterministic read facts for token/page/block;
- inactive stop-line families remain zero.

## Stop Lines

- No change to the bump-shaped modeled allocation ledger contract.
- No real segment allocation/free execution.
- No direct page array mutation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No arena backing allocation.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling or worker spawning.
- No source-level concurrency feature changes.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No compiler acceptance broadening in this row; split a sidecar if the proof
  exposes a real compiler blocker.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `130A.1` | Add the reuse ledger owner. | owner records successful reuse reports and rejects duplicates. | no bump ledger widening |
| `130A.2` | Add the proof app. | proof prints stable ledger/inactive/summary lines. | no source workaround |
| `130A.3` | Add the public guard and manifest/index wiring. | dedicated guard validates proof and stop-line leaks. | no broad guard bundle |
| `130A.4` | Update current pointers. | current pointer guard and diff check pass. | no provider activation |

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --only MIMAP-130A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
