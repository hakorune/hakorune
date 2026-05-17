# 293x-588 MIMAP-091A Segment Allocation Modeled Consume Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-091A` is the next allocator behavior row after the segment allocation
readiness closeout.

It adds one modeled scalar consume route:

```text
accepted segment allocation-readiness fact
  -> modeled allocation consume report
  -> new page-used / available-block scalar facts
```

This is still an in-memory `.hako` / `hako_alloc` proof route. It does not
allocate real segments, allocate arena backing, claim bitmaps, look up raw
pointers, call OSVM/page-source seams, run workers, activate providers, or
replace the process allocator.

## Scope

Allowed:

- add one narrow `.hako` owner for modeled segment allocation consume
- consume scalar inputs compatible with `MIMAP-088A` readiness facts
- return scalar observer/report fields:
  - accepted
  - reason
  - segment id
  - page id
  - old page-used
  - request blocks
  - new page-used
  - remaining blocks
  - modeled allocation token / block start
- add one proof app and one guard
- update `hako_alloc` module wiring and check-script index

Forbidden:

- real segment allocation/free execution
- arena backing allocation
- raw pointer residence
- segment-map pointer membership or lookup
- atomic bitmap claim/unclaim execution
- page-source call
- OSVM execution, unreserve, or release
- real thread scheduling
- worker spawning
- source-level concurrency feature change
- provider activation, hook, host allocator replacement, or
  `#[global_allocator]`
- backend `.inc` app/name matcher
- broad cleanup bundle

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `091A.1` | Add design SSOT for modeled consume contract. | owner/inputs/reasons/stop-lines are explicit. | no behavior outside row |
| `091A.2` | Add `.hako` owner and module export. | owner returns scalar report and counters. | no raw substrate |
| `091A.3` | Add proof app. | VM/MIR/EXE observable output covers success and rejection. | no app workaround |
| `091A.4` | Add guard/index/manifest wiring. | row guard passes locally. | no guard bundle |
| `091A.5` | Select next row. | current pointers move to a closeout or next narrow row. | no extra behavior |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_consume_guard.sh
tools/checks/run_proof_app.sh --only MIMAP-091A
bash tools/checks/proof_app_manifest_test_entry_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
