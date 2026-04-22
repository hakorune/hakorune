---
Status: Active
Date: 2026-04-22
Scope: next cleanup card for moving exact seed ladders toward function-level backend route tags.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
---

# 292x-101: Exact Seed Ladder Function Route Tags

## Problem

Several exact seed ladders still enter `.inc` through helper-specific matcher
families. Even when the active route already has MIR-owned metadata, the
function boundary can still read like a list of backend-local exact shape
attempts instead of a single function-level route decision.

## Decision

Do not widen accepted MIR shapes in this card. Pick one exact seed ladder that
already has a MIR metadata owner, then move the function-level selection to a
single backend route tag.

`.inc` may keep only:

- metadata reader / field validation
- selected helper emission
- fail-fast on inconsistent route metadata

## Acceptance

Define the exact seed ladder before code edits, then pin it with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
cargo test -q <focused-route-test>
bash <focused-boundary-smoke>
```
