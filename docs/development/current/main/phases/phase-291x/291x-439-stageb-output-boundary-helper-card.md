---
Status: Landed
Date: 2026-04-27
Scope: Stage-B output boundary helper extraction
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-438-stageb-adapter-thinning-inventory-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_output_box.hako
---

# 291x-439: Stage-B Output Boundary Helper

## Goal

Keep `compiler_stageb.hako` adapter-shaped by moving Program(JSON v0) output
boundary handling into a small helper.

This is BoxShape-only. It must not change source -> Program(JSON v0)
authority or accepted source shapes.

## Implementation

- Add `StageBOutputBox`.
- Move null result, freeze tag, list/error result, and normal Program(JSON v0)
  printing into that helper.
- Move `_starts_with` / `_is_freeze_tag` with the output boundary.
- Keep `StageBDriverBox.main(...)` as orchestration:
  args -> options -> compile -> output helper -> success trace.

## Non-Goals

- Do not move parser/body/defs/import logic.
- Do not change `BuildBox.emit_program_json_v0(...)`.
- Do not prune the disabled FuncScan harness in this slice.
- Do not add CoreMethodContract/CoreOp or MapGet metadata work.

## Verification

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: PASS.
