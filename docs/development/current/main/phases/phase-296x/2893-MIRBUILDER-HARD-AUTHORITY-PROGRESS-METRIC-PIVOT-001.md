---
Status: Landed
Date: 2026-07-05
Scope: MirBuilder Rust-to-Hako progress metric pivot.
---

# MIRBUILDER-HARD-AUTHORITY-PROGRESS-METRIC-PIVOT-001

## Decision

Narrow Rust-oracle parity pilots remain valid leaf cleanup, but they are no
longer counted as hard MirBuilder authority migration progress.

Hard authority progress starts only when a native `.hako` slice owns one of
these contracts with a fixture-backed parity gate:

```text
Fact owner: input snapshot -> fact DTO
Plan rule: facts -> recipe / plan DTO
Command producer: recipe -> symbolic command list
Allocator: symbolic command list -> allocated command list
```

## Reason

The current pilot lane has mostly adopted formatter, label, tag, and classifier
owners. Those reduce vocabulary drift, but route collection, route selection,
planner order, backend lowering, MIR mutation, region construction, observer
contracts, FastMemory fact construction, closure rewrite, and allocation
authority still remain Rust.

Therefore pilot count is not a reliable selfhost-distance metric.

## Next

Stop selecting another pure formatter/classifier rerun as the default next
step. Select the smallest Fact-owner or REGISTRY-rule contract that can be
tested as JSON input/output without MIR mutation or ID allocation.

## Boundaries

- Source Selfhost remains unclaimed.
- Backend lowering and MIR mutation remain Rust.
- ID allocation remains Rust until symbolic command ordering is stable.
- Failed `exit_analysis_is_single_exit_group` WIP is not adopted; it is parked
  as failed parity-gate work.
