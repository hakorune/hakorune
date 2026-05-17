# 293x-610 MIMAP-111A Segment Allocation Modeled Local-Free Apply Plan Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-111A` is the allocator behavior row selected by `MIMAP-110A`.

The segment allocation modeled lane now has:

```text
modeled consume
  -> allocation ledger
  -> release span facts
  -> released-span ledger
  -> local-free candidate ledger
```

This row adds a scalar apply-plan ledger downstream of `MIMAP-109A`:

```text
successful local-free candidate report
  -> page / segment / token / block span
  -> record a modeled local-free apply-plan row
```

This is still not real free-list mutation. It is the final scalar handoff
before a later row may open a page-local free-list update contract.

## Result

`MIMAP-111A` landed by adding:

- `lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako`
- `docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md`
- `apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/`
- `tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_apply_plan_guard.sh`

It selects:

```text
MIMAP-112A post-local-free-apply-plan row selection
```

## Validation Cadence

Cadence level:

```text
L2 proof row
```

Expected evidence:

```text
bash tools/checks/run_proof_app.sh --only MIMAP-111A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_apply_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

L3 compatibility guards are required only if this row changes the existing
`MIMAP-109A` local-free candidate report shape.

## Scope

Allowed:

- add a new local-free apply-plan owner under `lang/src/hako_alloc/memory/`;
- consume a successful `HakoAllocSegmentAllocationModeledLocalFreeCandidateLedgerReport`-shaped value;
- validate accepted candidate rows and block span shape;
- append deterministic scalar apply-plan rows;
- expose scalar proof fields for row index, source candidate index, token,
  segment id, page id, block span, candidate block count, and plan state;
- reject invalid, source-rejected, duplicate, and unsupported execution
  requests;
- add one focused proof app, manifest entry, SSOT, README/export wiring, and
  local guard.

Stop lines:

- no real segment allocation/free execution;
- no free-list mutation;
- no page state mutation outside the new scalar apply-plan ledger;
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
| `111A.1` | Add the local-free apply-plan SSOT and owner boundary. | owner and stop lines are documented. | no free-list mutation |
| `111A.2` | Implement scalar local-free apply-plan ledger owner. | accepted/rejected rows are deterministic. | no page state mutation |
| `111A.3` | Add focused proof app and manifest entry. | `run_proof_app.sh --only MIMAP-111A` passes. | no broad gate |
| `111A.4` | Add public guard and current closeout docs. | dedicated guard and pointer guard pass. | no allocator-wide default growth |

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/debug/hakorune --backend vm apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/main.hako
bash tools/checks/run_proof_app.sh --only MIMAP-111A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_apply_plan_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
