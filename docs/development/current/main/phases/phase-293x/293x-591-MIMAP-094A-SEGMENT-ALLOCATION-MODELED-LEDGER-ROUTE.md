# 293x-591 MIMAP-094A Segment Allocation Modeled Ledger Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-094A` is the allocator row selected by `MIMAP-093A`.

The previous row can model one accepted allocation consume result from scalar
readiness facts. This row should add a narrow scalar ledger that records those
modeled consume results so later rows can reason about allocation tokens without
opening real segment allocation/free execution.

## Scope

Allowed:

- add one `.hako` owner for a modeled segment allocation ledger;
- add one proof app and one local-run guard;
- record accepted `MIMAP-091A` consume result facts as scalar ledger rows;
- expose scalar append/read/find/report counters for modeled allocation tokens;
- keep all rows proof-only and deterministic.

Forbidden:

- real segment allocation/free execution;
- arena backing allocation;
- raw pointer residence;
- segment-map pointer membership or lookup;
- atomic bitmap claim/unclaim;
- page-source or OSVM calls;
- real thread scheduling or worker spawning;
- source-level concurrency feature changes;
- provider activation, hook, host allocator replacement, or
  `#[global_allocator]`;
- backend `.inc` app/name matcher;
- closeout bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `094A.1` | Add ledger SSOT and owner. | owner stores scalar modeled consume rows and inactive flags. | no real allocation |
| `094A.2` | Add proof app. | accepted row append/read/find and rejection reasons are stable. | no substrate |
| `094A.3` | Add guard/index/manifest/module docs. | VM/MIR/EXE route and stop lines are guarded. | no `.inc` matcher |
| `094A.4` | Select closeout row. | current pointers move to `MIMAP-095A`. | no cleanup bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_guard.sh
tools/checks/run_proof_app.sh --only MIMAP-094A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
