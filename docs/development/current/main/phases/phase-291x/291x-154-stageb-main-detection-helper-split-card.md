---
Status: Landed
Date: 2026-04-24
Scope: Split Stage-B main/body pattern detection out of the Stage-B entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-153-stageb-args-source-resolver-split-card.md
  - lang/src/compiler/entry/compiler.hako
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_main_detection_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-154 Stage-B Main Detection Helper Split Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by moving main/body pattern
detection out of `compiler_stageb.hako`:

```text
compiler_stageb.hako inline MainDetectionHelper
  -> lang.compiler.entry.stageb_main_detection_box
```

This keeps the Stage-B entry file focused on orchestration and lets Stage-A
fallback and Stage-B same-source scanning consume the same helper without
keeping the helper body inside the adapter.

## Design

Create `stageb_main_detection_box.hako` with:

```text
MainDetectionHelper.findMainBody(src)
MainDetectionHelper.findLegacyMainBody(src)
MainDetectionHelper.findPattern(src, pat, offset)
MainDetectionHelper.extractBodyFromPosition(src, pos)
```

The moved box keeps existing compatibility behavior:

```text
static method main
method main
static box Main { main(...) { ... } }
box Main { main(...) { ... } }
```

## Boundary

- BoxShape only.
- No parser invocation changes.
- No same-source defs scan split in this card.
- No Stage-B args/source resolution changes.
- No CoreMethodContract, `.inc`, or runtime lowering changes.
- Do not change legacy main-body fallback semantics.

## Implementation

- Added `lang/src/compiler/entry/stageb_main_detection_box.hako`.
- Moved `MainDetectionHelper` out of `compiler_stageb.hako`.
- Added direct imports for the helper in `compiler_stageb.hako` and
  `compiler.hako` so Stage-B same-source scanning and Stage-A fallback share
  the same compatibility helper.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Validation Notes

- Additional PASS:
  `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- Additional PASS:
  `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh`
- Additional PASS:
  `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh`
- Additional PASS:
  `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh`
- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
- Note: `stageb_min_emit.sh` with its default 10s timeout hit the timeout
  boundary on this WSL run. The same route emitted MIR JSON with a 20s timeout,
  so this card does not change smoke timeout policy.
- Additional PASS: `bash tools/checks/dev_gate.sh quick`
