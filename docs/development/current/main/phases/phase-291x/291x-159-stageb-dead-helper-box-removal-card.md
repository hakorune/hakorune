---
Status: Landed
Date: 2026-04-24
Scope: Remove dead Stage-B helper test box from the entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-158-stageb-dead-comment-strip-helper-removal-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-159 Stage-B Dead Helper Box Removal Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by removing the unused
`StageBHelperBox.test_loop(...)` scaffold from `compiler_stageb.hako`.

The helper has no references outside its own definition, so keeping it in the
entry adapter is pure surface noise.

## Boundary

- BoxShape/dead-code cleanup only.
- No parser invocation changes.
- No Stage-B main/compile behavior changes.
- No CoreMethodContract, `.inc`, or runtime lowering changes.

## Implementation

- Removed `StageBHelperBox` from `compiler_stageb.hako`.
- Verified the only references were the definition itself.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Validation Notes

- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
