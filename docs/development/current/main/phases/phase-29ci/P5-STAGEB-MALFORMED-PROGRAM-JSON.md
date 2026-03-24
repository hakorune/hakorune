---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `tools/selfhost/selfhost_build.sh` の default / BuildBox emit-only lane が `apps/tests/hello_simple_llvm.hako` でどう壊れているかを producer-owner ごとに固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - tools/selfhost/selfhost_build.sh
  - lang/src/compiler/entry/compiler.hako
  - lang/src/compiler/entry/compiler_stageb.hako
  - lang/src/compiler/entry/stageb_body_extractor_box.hako
  - lang/src/compiler/build/build_box.hako
  - apps/tests/hello_simple_llvm.hako
---

# P5 Stage-B Malformed Program JSON

## Status Summary

`apps/tests/hello_simple_llvm.hako` については close した。

- default Stage-B lane: healthy
- BuildBox emit-only keep lane: healthy
- downstream `--json-file` / `--run` / `--exe`: healthy

この文書は「hello fixture では malformed producer debt が消えた」証拠を残す closeout note として保持する。
ただし、これだけで `HAKO_USE_BUILDBOX=1` keep を global delete してよい、という意味ではない。
W2 以降、`tools/selfhost/selfhost_build.sh --json` は wrapper/public surface としては retired なので、下の `--json` command lines は historical evidence として読むこと。

## Goal

`apps/tests/hello_simple_llvm.hako` を使って、

- `tools/selfhost/selfhost_build.sh --json`
- `HAKO_USE_BUILDBOX=1 tools/selfhost/selfhost_build.sh --json`
- downstream `--run` / `--exe`

の current failure が shell extract ではなく upstream Program(JSON v0) production 側にあることを exact evidence で固定する。

## Exact Observations

### 1. default Stage-B emit-only lane

- command:
  - `bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json /tmp/phase29ci_hello_program.json`
- observed result:
  - exit `0`
  - emitted file is now healthy Program(JSON v0)
- payload head:
  - `{"version":0,"kind":"Program","body":[{"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Int","value":42}]},{"type":"Return","expr":{"type":"Int","value":0}}]}`
- downstream success:
  - `target/release/hakorune --json-file /tmp/phase29ci_hello_program.json`
  - prints `42`
- interpretation:
  - `StageBBodyExtractorBox.build_body_src()` full-source fallback debt is closed for this fixture
  - the body-only parse path is also healthy again for this fixture

### 2. BuildBox emit-only keep lane

- command:
  - `HAKO_USE_BUILDBOX=1 bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json /tmp/phase29ci_hello_program_buildbox.json`
- observed result:
  - exit `0`
  - emitted file is now healthy Program(JSON v0)
- payload head:
  - `{"version":0,"kind":"Program","body":[{"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Int","value":42}]},{"type":"Return","expr":{"type":"Int","value":0}}]}`
- downstream success:
  - `target/release/hakorune --json-file /tmp/phase29ci_hello_program_buildbox.json`
  - prints `42`
- interpretation:
  - `hello_simple_llvm` no longer reproduces the old full-source malformed payload on the BuildBox keep lane either
  - the owner-local fix was to split `scan_src` (defs/imports scan) from `parse_src` (Main.main body when available)

### 3. downstream consumer split

- `--mir`
  - green
  - reason: `emit_mir_json_from_source()` is source-direct and now agrees with the repaired default Stage-B lane
- `--run`
  - green
  - `bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --run`
  - prints `42`
- `--exe`
  - green
  - `bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --exe /tmp/phase29ci_hello_exe`
  - emitted EXE runs and prints `42`

## Raw Evidence

### default compiler lane

- raw snapshot:
  - `NYASH_SELFHOST_KEEP_RAW=1 NYASH_SELFHOST_RAW_DIR=/tmp/phase29ci_selfhost_raw bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json ...`
- observed raw log:
  - command tag is `compiler.hako --stage-b --stage3`
  - healthy Program(JSON v0) is printed in raw stdout before shell extraction
  - current raw payload starts with `Extern(log 42)` and then `Return(Int 0)`

### BuildBox lane

- raw snapshot:
  - `NYASH_SELFHOST_KEEP_RAW=1 NYASH_SELFHOST_RAW_DIR=/tmp/phase29ci_selfhost_raw_buildbox HAKO_USE_BUILDBOX=1 bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json ...`
- observed raw log:
  - command tag is `BuildBox.emit_program_json_v0 via compiler build_box`
  - healthy Program(JSON v0) is now printed in raw stdout before shell extraction
  - current raw payload starts with `Extern(log 42)` and then `Return(Int 0)`

## Current Root-Cause Pin

The current failure is not:

- shell `extract_program_json_v0_from_raw()`
- `selfhost_build.sh` top-level lane routing
- default Stage-B body-only parse on `hello_simple_llvm`
- direct JoinIR freeze on `hello_simple_llvm`

The current failure is pinned as:

- no malformed Program(JSON v0) producer remains on `hello_simple_llvm`
- both owner-local debts that this fixture exposed are closed:
  - `lang/src/compiler/entry/stageb_body_extractor_box.hako` + parser whitespace path
  - `lang/src/compiler/build/build_box.hako`

### Strongest local evidence

- `HAKO_STAGEB_DEBUG=1 HAKO_SRC="$(cat apps/tests/hello_simple_llvm.hako)" target/release/hakorune --backend vm lang/src/compiler/entry/compiler.hako -- --stage-b --stage3`
- plus the skip/parse probe:
  - `HAKO_SRC="$(cat apps/tests/hello_simple_llvm.hako)" target/release/hakorune --backend vm lang/src/compiler/entry/compiler.hako -- --stage-b --stage3`
  - trace now shows `parser/stmt enter j=15 ch="r"` and `kind=return`
- closed default-lane root cause:
  - `StageBBodyExtractorBox.build_body_src()` now reuses `BodyExtractionBox.extract_main_body(src)` before falling back to full source
  - `ParserControlBox._skip_ws_with_fallback()` now validates `ctx.skip_ws()` output and falls back to explicit local scanning if the skipped region contains non-whitespace
- closed BuildBox owner-local root cause:
  - `BuildBox.emit_program_json_v0(src, ...)` now keeps `scan_src` for defs/imports scanning
  - `BuildBox.emit_program_json_v0(src, ...)` now derives `parse_src` from `BodyExtractionBox.extract_main_body(scan_src)` and only falls back to full-source parse when no `Main.main` body exists

## Interpretation

- default Stage-B lane and BuildBox keep lane no longer reproduce malformed Program(JSON v0) on `hello_simple_llvm`
- the fixture proved two separate owner-local bugs, and both were closed without reopening shell-helper routing
- this removes `hello_simple_llvm` as evidence for a producer-side retreat in P3 helper audit
- delete-order work can now return to caller inventory / helper thinning instead of malformed-producer pinning for this fixture

## Guardrails

- do not cite this document as proof that all BuildBox inputs are globally healthy
- do not reopen default Stage-B or BuildBox malformed-output debt for this fixture unless a new regression reproduces it
- do not reopen shell-helper cleanup as if `extract_program_json_v0_from_raw()` were the cause
- do not mix this producer bugfix with `test_runner.sh` / smoke-tail retirement

## Next Safe Slice

1. keep both fixes closed with `phase29ci_stageb_body_extract`
2. return to caller/delete-order work in `P3-SHARED-SHELL-HELPER-AUDIT.md`
3. only reopen producer-side analysis if another fixture reproduces malformed Program(JSON v0)
