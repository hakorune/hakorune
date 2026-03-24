---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `apps/tests/hello_simple_llvm.hako` の malformed producer debt closeout note。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - tools/selfhost/selfhost_build.sh
---

# P5 Stage-B Malformed Program JSON

## Status Summary

`apps/tests/hello_simple_llvm.hako` の malformed producer debt は close 済み。
この文書は `tools/selfhost/selfhost_build.sh --json` の historical evidence を残す closeout note だよ。

## What Was Fixed

- default Stage-B lane は healthy
- BuildBox emit-only keep lane も healthy
- downstream `--json-file` / `--run` / `--exe` も green
- owner-local fix は 2 点
  - `StageBBodyExtractorBox.build_body_src()` と parser whitespace path
  - `BuildBox.emit_program_json_v0(...)` の `scan_src` / `parse_src` split

## Reading Rule

- `tools/selfhost/selfhost_build.sh --json` は wrapper/public surface としては retired
- この文書の `--json` command lines は historical evidence として読む
- `hello_simple_llvm` を理由にした producer-side retreat はもう再開しない

## Next Safe Slice

1. caller/delete-order work を `P3-SHARED-SHELL-HELPER-AUDIT.md` に戻す
2. もし別 fixture が malformed Program(JSON v0) を再現したら、そのときだけ producer-side を再調査する
