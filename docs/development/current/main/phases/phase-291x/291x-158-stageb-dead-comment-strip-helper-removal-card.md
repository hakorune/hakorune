---
Status: Landed
Date: 2026-04-24
Scope: Remove dead inline Stage-B comment stripping helper from the entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-157-stageb-keyword-expr-strip-split-card.md
  - lang/src/compiler/entry/comment_stripper_box.hako
  - lang/src/compiler/entry/compiler_stageb.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-158 Stage-B Dead Comment Strip Helper Removal Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by removing the unused inline
`_strip_comments(...)` helper from `compiler_stageb.hako`.

Comment stripping is already owned by `CommentStripperBox`, and the Stage-B
driver calls that box directly. Keeping a dead duplicate in the adapter weakens
the owner boundary.

## Boundary

- BoxShape/dead-code cleanup only.
- No parser invocation changes.
- No comment stripping behavior changes.
- No JSON enrichment changes.
- No CoreMethodContract, `.inc`, or runtime lowering changes.

## Implementation

- Removed the unused inline `_strip_comments(...)` helper from
  `compiler_stageb.hako`.
- Kept `CommentStripperBox.strip_comments(source)` as the only Stage-B entry
  comment-strip call site.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Validation Notes

- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
