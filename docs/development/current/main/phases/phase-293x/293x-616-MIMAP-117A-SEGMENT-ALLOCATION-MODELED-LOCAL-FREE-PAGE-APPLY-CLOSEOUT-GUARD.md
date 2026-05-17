# 293x-616 MIMAP-117A Segment Allocation Modeled Local-Free Page-Apply Closeout Guard

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-117A` is the closeout row selected by `MIMAP-116A`.

It should freeze:

```text
MIMAP-115A local-free page-model apply route
```

and the upstream scalar lane it consumes:

```text
MIMAP-107A released-span ledger
MIMAP-109A local-free candidate ledger
MIMAP-111A local-free apply-plan ledger
MIMAP-113A scalar lane closeout
```

This row must not add allocator behavior. It closes the proof/docs/guard shape
before a later row decides whether to connect this page-local apply seam to a
larger segment owner.

## Validation Cadence

Cadence level:

```text
L4 closeout row
```

Expected evidence:

```text
bash tools/checks/run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_apply_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Scope

Allowed:

- add a closeout SSOT for the page-model apply seam;
- add a manifest-backed closeout guard implementation;
- add a public `k2_wide_*` wrapper;
- add a `guard_rows.toml` entry and check-script index row;
- verify the already-landed owner / proof / guard / stop-line set for
  `MIMAP-115A` plus the upstream scalar lane.

Stop lines:

- no new `.hako` allocator behavior;
- no direct page array mutation;
- no raw pointer residence;
- no segment-map pointer membership or lookup;
- no arena backing allocation;
- no atomic bitmap execution;
- no page-source or OSVM execution;
- no real thread scheduling or worker spawning;
- no source-level concurrency feature change;
- no provider activation, hook, host allocator replacement, or
  `#[global_allocator]`;
- no backend `.inc` app/name matcher;
- no broad allocator gate by default.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `117A.1` | Add page-apply closeout SSOT. | SSOT lists row set and stop lines. | no behavior |
| `117A.2` | Add manifest-backed closeout guard. | `run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout` passes. | no proof reroute |
| `117A.3` | Add public wrapper and docs/index wiring. | public wrapper delegates to manifest runner. | no broad gate |
| `117A.4` | Update current pointers and closeout evidence. | pointer guard passes. | no implementation bundle |
