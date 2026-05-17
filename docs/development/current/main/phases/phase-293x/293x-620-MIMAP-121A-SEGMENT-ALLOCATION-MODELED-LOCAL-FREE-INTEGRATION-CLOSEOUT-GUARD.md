# 293x-620 MIMAP-121A Segment Allocation Modeled Local-Free Integration Closeout Guard

Status: selected current
Date: 2026-05-18

## Decision

`MIMAP-121A` is the closeout row selected by `MIMAP-120A`.

The MIMAP-119A integration route now owns:

```text
released-span report
  -> local-free candidate ledger
  -> local-free apply-plan ledger
  -> explicit page-model local-free apply
```

This row should add one manifest-backed closeout guard that freezes that seam
and prevents drift before the next allocator behavior row is selected.

## Scope

Allowed:

- add one closeout SSOT for the local-free integration route;
- add one `guard_rows.toml` row;
- add one public `k2_wide_*` wrapper that delegates to
  `tools/checks/run_row_guard.sh`;
- add one implementation script under `tools/checks/impl/`;
- update `docs/tools/check-scripts-index.md`;
- update current pointers and select the next planning row.

Forbidden:

- new `.hako` allocator behavior;
- proof app rerouting;
- raw pointer residence;
- segment-map pointer membership or lookup;
- arena backing allocation;
- atomic bitmap execution;
- page-source or OSVM execution;
- real thread scheduling or worker spawning;
- source-level concurrency feature changes;
- provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`;
- backend `.inc` app/name matcher.

## Validation Cadence

Cadence level:

```text
L4 closeout row
```

Required evidence:

```text
bash tools/checks/run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-integration-closeout
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `121A.1` | Add integration closeout SSOT. | SSOT lists row set and stop lines. | no behavior |
| `121A.2` | Add manifest-backed closeout guard. | `run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-integration-closeout` passes. | no proof reroute |
| `121A.3` | Add public wrapper and docs/index wiring. | public wrapper delegates to manifest runner. | no broad gate |
| `121A.4` | Update current pointers and closeout evidence. | pointer guard passes. | no implementation bundle |
