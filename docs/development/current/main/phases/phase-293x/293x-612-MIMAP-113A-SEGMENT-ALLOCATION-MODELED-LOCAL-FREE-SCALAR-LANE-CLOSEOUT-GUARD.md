# 293x-612 MIMAP-113A Segment Allocation Modeled Local-Free Scalar Lane Closeout Guard

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-113A` is the closeout row selected by `MIMAP-112A`.

It should freeze the scalar local-free lane through:

```text
MIMAP-107A released-span ledger
MIMAP-109A local-free candidate ledger
MIMAP-111A local-free apply-plan ledger
```

This row must not add allocator behavior. It closes the proof/docs/guard shape
before a later row decides whether to open real page-local free-list mutation.

## Validation Cadence

Cadence level:

```text
L4 closeout row
```

Expected evidence:

```text
bash tools/checks/run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_scalar_lane_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Scope

Allowed:

- add a closeout SSOT for the scalar local-free lane;
- add a manifest-backed closeout guard implementation;
- add a public `k2_wide_*` wrapper;
- add a `guard_rows.toml` entry and check-script index row;
- verify the already-landed owner / proof / guard / stop-line set for
  `MIMAP-107A`, `MIMAP-109A`, and `MIMAP-111A`.

Stop lines:

- no new `.hako` allocator behavior;
- no free-list mutation;
- no page state mutation;
- no real segment allocation/free execution;
- no arena backing allocation;
- no raw pointer residence;
- no segment-map pointer membership or lookup;
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
| `113A.1` | Add scalar local-free lane closeout SSOT. | SSOT lists row set and stop lines. | no behavior |
| `113A.2` | Add manifest-backed closeout guard. | `run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout` passes. | no proof reroute |
| `113A.3` | Add public wrapper and docs/index wiring. | public wrapper delegates to manifest runner. | no broad gate |
| `113A.4` | Update current pointers and closeout evidence. | pointer guard passes. | no implementation bundle |
