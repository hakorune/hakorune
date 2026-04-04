---
Status: SSOT
Date: 2026-04-04
Scope: make the final decision on full rust-vm retirement versus residual explicit keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-61x/61x-90-residual-rust-vm-caller-zero-audit-rerun-ssot.md
  - docs/development/current/main/phases/phase-62x/62x-90-rust-vm-delete-ready-removal-wave-ssot.md
---

# 63x-90 Rust-VM Final Retirement Decision SSOT

## Intent

- collect the corridor evidence in one place
- decide whether full retirement is supported by source-backed facts
- otherwise freeze a residual explicit keep set and stop-line

## Current Starting Read

- mainline retirement is already achieved
- full source retirement is not yet proven
- `62x` removal wave was a no-op because delete-ready candidates did not materialize

## Decision Boundary

- full retirement requires:
  - no broad rust-vm source remaining in active route ownership
  - no compat/proof keep that still has unavoidable callers
  - no required backend override surface for `vm`
- otherwise:
  - declare residual explicit keep
  - stop widening it
  - hand off a later reevaluation point instead of forcing deletion
