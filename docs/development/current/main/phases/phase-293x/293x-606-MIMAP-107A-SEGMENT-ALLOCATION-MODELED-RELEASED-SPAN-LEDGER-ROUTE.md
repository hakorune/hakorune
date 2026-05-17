# 293x-606 MIMAP-107A Segment Allocation Modeled Released-Span Ledger Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-107A` is the allocator behavior row selected by `MIMAP-106A`.

The row adds a scalar released-span ledger downstream of `MIMAP-104A` release
span facts:

```text
successful modeled ledger release report
  -> release_span_present == 1
  -> record token / segment / page / released block span
  -> expose deterministic released-span lookup/read facts
```

This is not real segment free execution. It does not mutate a free-list, page
bitmap, segment map, raw pointer residence, page-source/OSVM state, scheduler,
or provider state.

## Result

`MIMAP-107A` landed by adding:

- `lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako`
- `docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md`
- `apps/hako-alloc-segment-allocation-modeled-released-span-ledger-proof/`
- `tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_released_span_ledger_guard.sh`

It selects:

```text
MIMAP-108A post-released-span-ledger row selection
```

## Validation Cadence

Cadence level:

```text
L2 proof row
```

Expected evidence:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-107A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_released_span_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

L3 compatibility guards are required only if this row changes the existing
`MIMAP-104A` release report shape or released-token recycle-visible behavior.

## Scope

Allowed:

- add a new released-span ledger owner under `lang/src/hako_alloc/memory/`;
- consume a `HakoAllocSegmentAllocationModeledLedgerReleaseReport`-shaped value;
- validate `did_release`, `reason`, `release_span_present`, scalar token, page,
  segment, block start/end, and released block count;
- append deterministic scalar released-span rows;
- reject invalid, span-missing, duplicate, and unsupported execution requests;
- add one focused proof app, manifest entry, SSOT, README/export wiring, and
  local guard.

Stop lines:

- no real segment allocation/free execution;
- no free-list mutation;
- no page state mutation outside the new scalar ledger;
- no arena backing allocation;
- no raw pointer residence;
- no segment-map pointer membership or lookup;
- no atomic bitmap execution;
- no page-source or OSVM execution;
- no real thread scheduling or worker spawning;
- no source-level concurrency feature change;
- no provider activation, hook, host allocator replacement, or
  `#[global_allocator]`;
- no backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `107A.1` | Add the released-span ledger SSOT and owner boundary. | owner and stop lines are documented. | no behavior outside scalar ledger |
| `107A.2` | Implement scalar released-span ledger owner. | accepted/missing/duplicate/unsupported rows are deterministic. | no free-list mutation |
| `107A.3` | Add focused proof app and manifest entry. | `run_proof_app.sh --only MIMAP-107A` passes. | no broad gate |
| `107A.4` | Add public guard and current closeout docs. | dedicated guard and pointer guard pass. | no allocator-wide default growth |

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-released-span-ledger-proof/main.hako
bash tools/checks/run_proof_app.sh --only MIMAP-107A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_released_span_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
