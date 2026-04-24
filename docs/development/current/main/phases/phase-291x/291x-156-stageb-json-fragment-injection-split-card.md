---
Status: Landed
Date: 2026-04-24
Scope: Move Stage-B JSON fragment injection out of the Stage-B entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-155-stageb-same-source-defs-scan-split-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb/stageb_json_builder_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-156 Stage-B JSON Fragment Injection Split Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by removing the inline JSON fragment
injection helper from `compiler_stageb.hako`:

```text
compiler_stageb.hako _inject_json_fragment(...)
  -> StageBJsonBuilderBox.inject_json_fragment(...)
```

This keeps the entry adapter from owning JSON string surgery.

## Design

Use the existing `StageBJsonBuilderBox.inject_json_fragment(json, fragment)`
helper. The Stage-B driver remains responsible for sequencing:

```text
parse Program(JSON v0)
  -> StageBSameSourceDefsBox.build_fragment(clean)
  -> StageBJsonBuilderBox.inject_json_fragment(...)
```

## Boundary

- BoxShape only.
- No parser invocation changes.
- No same-source defs scanning changes.
- No keyword expr stripping changes.
- No CoreMethodContract, `.inc`, or runtime lowering changes.

## Implementation

- Added a `StageBJsonBuilderBox` import to `compiler_stageb.hako`.
- Replaced the inline `_inject_json_fragment(...)` call with
  `StageBJsonBuilderBox.inject_json_fragment(...)`.
- Removed the inline JSON fragment injection helper from the Stage-B adapter.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Validation Notes

- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
