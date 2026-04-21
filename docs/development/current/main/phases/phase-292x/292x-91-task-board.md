---
Status: Active
Date: 2026-04-22
Scope: Phase 292x task board for `.inc` thin tag cleanup.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
---

# 292x-91: Task Board

## Guardrails

- [x] G1 `.inc` analysis-debt no-growth guard
  - command: `tools/checks/inc_codegen_thin_shim_guard.sh`
  - integrated into: `tools/checks/dev_gate.sh quick`
- [ ] G2 route-family deletion accounting
  - when a C analyzer is retired, reduce
    `tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv` in the same commit

## Active Card

- [ ] A1 `array_rmw_window` MIR-owned route tag
  - design: `292x-93-array-rmw-window-route-card.md`
  - current owner leak: `analyze_array_rmw_window_candidate`
  - desired state: `.inc` reads metadata first and treats the old C analyzer as
    temporary fallback only

## Follow-up Cards

- [ ] A2 `array_string_len_window` MIR-owned route tag
- [ ] A3 generic method route policy metadata
- [ ] A4 string concat / direct-set windows metadata-only consumption
- [ ] A5 exact seed ladders to function-level backend route tags

## Done Definition

- `.inc` has no active route-legality owner for the migrated family
- MIR JSON carries the pre-decided route tag
- `.inc` validates and emits, but does not rediscover the shape
- smoke covers both behavior and route selection
- no new `.inc` analysis-debt baseline rows are introduced
