# 293x-618 MIMAP-119A Segment Allocation Modeled Local-Free Integration Route

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-119A` is the allocator behavior row selected by `MIMAP-118A`.

The scalar local-free lane and the explicit page-model apply seam are now
closed through:

```text
MIMAP-107A released-span ledger
MIMAP-109A local-free candidate ledger
MIMAP-111A local-free apply-plan ledger
MIMAP-115A page-model local-free apply
MIMAP-117A page-apply closeout guard
```

This row should add one narrow integration owner that composes those existing
owners so proof apps no longer have to hand-wire the full local-free chain.

## Result

`MIMAP-119A` landed by adding:

- `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako`
- `apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/`
- `docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-integration-ssot.md`
- `tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh`

It selects:

```text
MIMAP-120A post-local-free-integration row selection
```

## Scope

Allowed:

- add one `.hako` owner under `lang/src/hako_alloc/memory/`;
- consume a successful
  `HakoAllocSegmentAllocationModeledReleasedSpanLedgerReport`;
- require an explicit `HakoAllocPageModel`;
- internally compose:

```text
recordLocalFreeCandidate(...)
recordLocalFreeApplyPlan(...)
recordLocalFreePageApply(...)
```

- expose a scalar integration report with candidate / plan / page-apply row
  indices, final page used/local-free/free counters, and inactive stop-line
  flags;
- add one focused proof app and one dedicated guard.

Forbidden:

- real segment allocation/free execution beyond the existing page-local model;
- direct page array mutation outside `HakoAllocPageModel.releaseLocal`;
- segment-map pointer membership or lookup;
- raw pointer residence;
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
L2 proof row
```

Required evidence:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-119A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `119A.1` | Add local-free integration SSOT and owner boundary. | owner and stop lines are documented. | no raw pointer / segment-map |
| `119A.2` | Implement the integration owner and scalar report. | owner composes existing ledger/apply owners only. | no direct page array mutation |
| `119A.3` | Add proof app and manifest entry. | `run_proof_app.sh --only MIMAP-119A` passes. | no broad proof bundle |
| `119A.4` | Add dedicated guard and index wiring. | dedicated guard passes and forbids stop-line leaks. | no backend matcher |
| `119A.5` | Update current pointers and next-row selection. | pointer guard passes. | no extra behavior |

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 120 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/main.hako
bash tools/checks/run_proof_app.sh --only MIMAP-119A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
