# 293x-632 MIMAP-126A Segment Allocation Modeled Local-Free Reuse Route

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-126A` is the allocator behavior row selected by `MIMAP-125A`.

It proves the next narrow behavior after the modeled local-free integration
closeout:

```text
released-span report
  -> local-free candidate ledger
  -> local-free apply-plan ledger
  -> HakoAllocPageModel.releaseLocal(block_id)
  -> HakoAllocPageModel.acquire(size) reuses the collected local_free block
```

This row stays page-local. It does not open real segment allocation/free,
segment-map pointer lookup, arena backing allocation, atomic bitmap execution,
page-source/OSVM calls, worker scheduling, or provider activation.

## Scope

Owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako
```

Proof app:

```text
apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_guard.sh
```

The row may compose the existing `MIMAP-119A` local-free integration owner and
the existing `HakoAllocPageModel.acquire` path. It may expose scalar observer
facts for before/after `used`, `free`, `local_free`, collection count, selected
block id, and reuse reason.

## Acceptance Shape

The proof output should make the reuse behavior explicit:

```text
reuse=1,0,<block_id>,<used_before>,<used_after>,<free_before>,<free_after>,<local_before>,<local_after>,<collect_count_after>
integration=1,0,<token>,<page_id>,<start>,<end>,<applied_blocks>
inactive=0,0,0,0,0,0,0,0,0,0
summary=ok
```

Exact field order can be adjusted in the row implementation, but it must be
stable in the guard and must distinguish:

- local-free integration success;
- page-local reuse success;
- local-free collection occurred before reuse;
- inactive stop-line families remain zero.

## Stop Lines

- No real segment allocation/free execution beyond the existing page-local
  model.
- No direct page array mutation outside `HakoAllocPageModel`.
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
| `126A.1` | Add the local-free reuse owner. | owner composes existing local-free integration and page-local acquire. | no real segment free |
| `126A.2` | Add the proof app. | proof prints stable reuse/inactive/summary lines. | no source workaround |
| `126A.3` | Add the public guard and manifest/index wiring if needed. | dedicated guard validates proof and stop-line leaks. | no broad guard bundle |
| `126A.4` | Update current pointers. | current pointer guard and diff check pass. | no provider activation |

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --only MIMAP-126A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
