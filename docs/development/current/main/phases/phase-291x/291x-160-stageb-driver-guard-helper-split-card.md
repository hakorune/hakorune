---
Status: Landed
Date: 2026-04-24
Scope: Split Stage-B driver entry trace/depth guard helpers out of the entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-159-stageb-dead-helper-box-removal-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_driver_guard_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-160 Stage-B Driver Guard Helper Split Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by moving Stage-B driver entry
trace/depth-guard boilerplate out of `compiler_stageb.hako`.

The Stage-B adapter should sequence entry handling, not own repeated
`HAKO_STAGEB_TRACE` and `HAKO_STAGEB_DRIVER_DEPTH` mechanics.

## Design

Create `stageb_driver_guard_box.hako` with:

```text
StageBDriverGuardBox.trace_enter()
StageBDriverGuardBox.trace_entry_marker()
StageBDriverGuardBox.enter_depth_guard()
StageBDriverGuardBox.trace_depth_ok()
StageBDriverGuardBox.clear_depth_guard()
StageBDriverGuardBox.trace_after_emit()
```

The moved helper preserves existing tags and environment variables exactly.

## Boundary

- BoxShape only.
- No trace tag changes.
- No env var changes.
- No parser invocation changes.
- No CoreMethodContract, `.inc`, or runtime lowering changes.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Implementation

- Added `lang/src/compiler/entry/stageb_driver_guard_box.hako`.
- Moved Stage-B entry trace, depth guard, guard clearing, and after-emit trace
  helpers out of `compiler_stageb.hako`.
- Preserved existing `HAKO_STAGEB_TRACE`, `HAKO_STAGEB_DRIVER_DEPTH`, and log
  tags exactly.

## Validation Notes

- Additional PASS:
  `HAKO_STAGEB_TRACE=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
- Additional PASS: `bash tools/checks/dev_gate.sh quick`
