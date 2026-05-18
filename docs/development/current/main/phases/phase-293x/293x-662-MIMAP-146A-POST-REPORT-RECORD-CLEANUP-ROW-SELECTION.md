# 293x-662 MIMAP-146A Post-Report-Record-Cleanup Row Selection

Status: selected current
Date: 2026-05-18

## Decision

Select exactly one next allocator, Hakorune core, or source cleanup row after
the local-free integration report record boundary cleanup.

## Inputs

```text
HAKO-ALLOC-REPORT-RECORD-002
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
```

## Scope

- Review the HAKO-ALLOC-REPORT-RECORD-002 evidence.
- Decide whether the next smallest row is allocator behavior, Hakorune language
  ergonomics, compiler acceptance, or another BoxShape cleanup.
- Keep the next row narrow enough to land with one focused guard/proof bundle.

## Stop Lines

- No allocator behavior change.
- No compiler route behavior.
- No source syntax change.
- No broad report cleanup sweep.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
