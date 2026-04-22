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
- [x] G2 route-family deletion accounting
  - when a C analyzer is retired, reduce
    `tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv` in the same commit

## Completed Cards

- [x] A1 `array_rmw_window` MIR-owned route tag
  - design: `292x-93-array-rmw-window-route-card.md`
  - state: MIR metadata is emitted; `.inc` reads metadata first and treats the old C analyzer as
    temporary fallback only
  - trace proof: `[llvm-route/trace] stage=array_rmw_window result=hit reason=mir_route_metadata`

- [x] A2a `array_string_len_window` len-only MIR-owned route tag
  - design: `292x-94-array-string-len-window-route-card.md`
  - state: MIR metadata is emitted for len-only get/copy*/length windows;
    `.inc` reads metadata and the old analyzer was retired in A2d
  - trace proof: `[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata`

- [x] A2b `array_string_len_window` keep-live MIR-owned route tag
  - design: `292x-95-array-string-len-keep-live-route-card.md`
  - state: `keep_get_live` mode is MIR metadata-owned; `.inc` emits
    slot-load + string-len from metadata
  - trace proof: `[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata ... keep_get_live=1`

- [x] A2c `array_string_len_window` source-only direct-set route tag
  - design: `292x-96-array-string-len-source-only-route-card.md`
  - state: `source_only_insert_mid` mode is MIR metadata-owned; source-only
    insert-mid and piecewise direct-set smokes require
    `reason=mir_route_metadata`
  - trace proof: `[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata ... source_only_insert_mid=1`

- [x] A2d delete legacy `array_string_len_window` C analyzer
  - design: `292x-97-array-string-len-c-analyzer-deletion-card.md`
  - state: `analyze_array_string_len_window_candidate` and its fallback branch
    are deleted; `.inc` keeps metadata validation / emit / skip / fail-fast
    for the migrated family

## Active Card

- [ ] A1b delete legacy `array_rmw_window` C analyzer
  - design: `292x-98-array-rmw-c-analyzer-deletion-card.md`
  - desired state: `.inc` keeps only metadata validation / emit / skip /
    fail-fast for the migrated `array_rmw_window` family

## Follow-up Cards

- [ ] A3 generic method route policy metadata
- [ ] A4 string concat / direct-set windows metadata-only consumption
- [ ] A5 exact seed ladders to function-level backend route tags

## Done Definition

- `.inc` has no active route-legality owner for the migrated family
- MIR JSON carries the pre-decided route tag
- `.inc` validates and emits, but does not rediscover the shape
- smoke covers both behavior and route selection
- no new `.inc` analysis-debt baseline rows are introduced
