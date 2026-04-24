---
Status: Landed
Date: 2026-04-24
Scope: Move Stage-B keyword expression stripping out of the Stage-B entry adapter.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-156-stageb-json-fragment-injection-split-card.md
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_keyword_expr_strip_box.hako
  - tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
---

# 291x-157 Stage-B Keyword Expr Strip Split Card

## Goal

Continue HCM-8 Stage-B thin-adapter work by moving keyword-expression JSON
cleanup out of `compiler_stageb.hako`:

```text
compiler_stageb.hako _strip_keyword_expr_nodes(...)
  -> StageBKeywordExprStripBox.strip(...)
```

This leaves the Stage-B adapter sequencing the parse/enrichment steps without
owning JSON cleanup string matching.

## Design

Create `stageb_keyword_expr_strip_box.hako` with:

```text
StageBKeywordExprStripBox.strip(json_text, keyword)
```

The implementation is moved unchanged. It removes exact JSON expression nodes
for a keyword variable, currently called for `local` before and after defs
fragment injection.

## Boundary

- BoxShape only.
- No parser invocation changes.
- No JSON fragment injection changes.
- No keyword set changes; only `local` remains used by Stage-B.
- No CoreMethodContract, `.inc`, or runtime lowering changes.

## Implementation

- Added `lang/src/compiler/entry/stageb_keyword_expr_strip_box.hako`.
- Moved the exact keyword expression JSON cleanup helper out of
  `compiler_stageb.hako`.
- Updated `StageBDriverBox.compile(...)` to call
  `StageBKeywordExprStripBox.strip(ast_json, "local")` before and after defs
  fragment injection.

## Acceptance

- `bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- `bash tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Validation Notes

- Additional PASS:
  `HAKO_BUILD_TIMEOUT=20 bash tools/smokes/v2/profiles/quick/core/stageb_min_emit.sh`
- Additional PASS: `bash tools/checks/dev_gate.sh quick`
