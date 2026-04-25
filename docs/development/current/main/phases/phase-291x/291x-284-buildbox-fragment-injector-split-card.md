---
Status: Landed
Date: 2026-04-26
Scope: BuildBox defs/imports fragment injector split.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-281-buildbox-remaining-cleanup-order-card.md
  - lang/src/compiler/build/build_box.hako
  - lang/src/compiler/build/build_program_fragment_box.hako
---

# 291x-284: BuildBox Fragment Injector Split

## Goal

Move Program(JSON v0) enrichment details out of `BuildBox`.

`BuildBox` remains the outer sequencing authority:

```text
prepare scan_src
  -> parse Program(JSON v0)
  -> BuildProgramFragmentBox.enrich(ast_json, scan_src)
```

`BuildProgramFragmentBox` owns only defs/imports fragment construction and JSON
fragment injection.

## Boundary

- `BuildProgramFragmentBox` owns:
  - `FuncScannerBox.scan_all_boxes(...)` handoff for defs metadata
  - defs JSON construction
  - `UsingCollectorBox.collect(...)` handoff for imports metadata
  - using-to-imports conversion
  - JSON fragment injection
  - `HAKO_STAGEB_FUNC_SCAN=0` defs-scan gate
- `BuildBox` owns:
  - source preparation
  - parser invocation
  - freeze-tag pass-through before enrichment
  - parse-source narrowing via `BodyExtractionBox`

## Non-Goals

- Do not change Program(JSON v0) shape.
- Do not change defs/imports scan inputs.
- Do not change parser fallback semantics.
- Do not change bundle behavior.
- Do not touch CoreMethodContract fallback rows.

## Acceptance

```bash
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh
bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_shape_canary_vm.sh
bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```

Result: PASS.

Additional bundle coverage:

```bash
SMOKES_ENABLE_STAGEB=1 bash tools/smokes/v2/profiles/integration/stageb/stageb_bundle_require_ok_vm.sh
```

Result: PASS.

Additional gate:

```bash
tools/checks/dev_gate.sh quick
```

Result: PASS.
