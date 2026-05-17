# 293x-608 MIMAP-109A Segment Allocation Modeled Local-Free Candidate Ledger Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-109A` is the allocator behavior row selected by `MIMAP-108A`.

The row adds a scalar local-free candidate ledger downstream of `MIMAP-107A`:

```text
successful released-span ledger report
  -> page / segment / token / block span
  -> record a modeled local-free candidate row
```

This is not real free-list mutation. It is a scalar handoff ledger for a later
row that can decide whether and how a page-local free-list should be updated.

## Validation Cadence

Cadence level:

```text
L2 proof row
```

Expected evidence:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-109A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_candidate_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

L3 compatibility guards are required only if this row changes the existing
`MIMAP-107A` released-span report shape.

## Scope

Allowed:

- add a new local-free candidate ledger owner under `lang/src/hako_alloc/memory/`;
- consume a `HakoAllocSegmentAllocationModeledReleasedSpanLedgerReport`-shaped
  value;
- validate accepted released-span ledger facts and block span shape;
- append deterministic scalar local-free candidate rows;
- reject invalid, source-rejected, duplicate, and unsupported execution
  requests;
- add one focused proof app, manifest entry, SSOT, README/export wiring, and
  local guard.

Stop lines:

- no real segment allocation/free execution;
- no free-list mutation;
- no page state mutation outside the new scalar candidate ledger;
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
| `109A.1` | Add the local-free candidate ledger SSOT and owner boundary. | owner and stop lines are documented. | no free-list mutation |
| `109A.2` | Implement scalar local-free candidate ledger owner. | accepted/rejected rows are deterministic. | no page state mutation |
| `109A.3` | Add focused proof app and manifest entry. | `run_proof_app.sh --only MIMAP-109A` passes. | no broad gate |
| `109A.4` | Add public guard and current closeout docs. | dedicated guard and pointer guard pass. | no allocator-wide default growth |

