# 293x-594 MIMAP-097A Segment Allocation Modeled Ledger Release Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-097A` is the allocator row selected by `MIMAP-096A`.

The modeled segment allocation ledger can record accepted modeled consume
tokens. This row should add a narrow modeled release route that marks exactly
one live ledger token as released and exposes scalar release facts.

This remains a ledger-only proof route. It is not real segment free execution.

Result:

```text
landed:
  segment allocation modeled ledger release route

selected next row:
  MIMAP-098A segment allocation modeled ledger release closeout guard
```

## Scope

Allowed:

- extend the existing modeled ledger owner with one token-release method;
- add one release report shape if needed;
- add one proof app and one local-run guard;
- validate live-token, duplicate-release, missing-token, and unsupported
  substrate reasons;
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
| `097A.1` | Add release SSOT and ledger owner method. | one live modeled token can be released, counters update. | no real free |
| `097A.2` | Add proof app. | release / duplicate / missing / unsupported reasons are stable. | no substrate |
| `097A.3` | Add guard/index/manifest docs. | VM/MIR/EXE route and stop lines are guarded. | no `.inc` matcher |
| `097A.4` | Select closeout row. | current pointers move to `MIMAP-098A`. | no cleanup bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh
tools/checks/run_proof_app.sh --only MIMAP-097A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
