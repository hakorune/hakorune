# 293x-760 MIMAP-237A Segment Arena Backing Readiness Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add a narrow diagnostics row for the MIMAP-236A segment arena backing readiness
inventory before opening an arena-readiness closeout pack.

## Context

MIMAP-236A records scalar/model arena backing readiness from the lifecycle-keyed
apply/recycle continuation summary. The next row should make the reject surface
durable enough that later arena backing, no-escape raw pointer residence, real
segment-map mutation, and atomic bitmap rows do not inherit ambiguous failure
states.

This row should focus on observer/report diagnostics only.

## Landed Scope

MIMAP-237A added the observer-only diagnostics owner:

```text
lang/src/hako_alloc/memory/segment_arena_backing_readiness_diagnostic_box.hako
```

The owner observes MIMAP-236A readiness counters and the last readiness report,
then publishes scalar diagnostic summary facts for missing inventory, invalid
shape, and blocked requirement categories. It does not classify readiness or
open arena/raw/segment-map/atomic execution.

Row SSOT:

```text
docs/development/current/main/design/hako-alloc-segment-arena-backing-readiness-diagnostics-ssot.md
```

Proof app:

```text
apps/hako-alloc-segment-arena-backing-readiness-diagnostics-proof
```

Guard:

```text
tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_diagnostics_guard.sh --level L2
```

## Candidate Scope

- Missing or not-applicable lifecycle continuation summary.
- Invalid segment/arena/slice/alignment/page-size shapes.
- Explicit blocked requirement flags:
  - arena backing allocation would be required
  - raw pointer residence would be required
  - real segment-map mutation would be required
  - atomic bitmap execution would be required
  - OSVM/page-source execution would be required
  - provider activation would be required
- Counter/report observer fields for the above categories.

## Stop Lines

- No real arena backing allocation.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_readiness_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-237A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Next Row

MIMAP-237A selects:

```text
MIMAP-238A segment arena backing readiness closeout pack
```

MIMAP-238A should provide representative L3 evidence for MIMAP-236A and
MIMAP-237A before any later arena backing or raw pointer residence bridge.
