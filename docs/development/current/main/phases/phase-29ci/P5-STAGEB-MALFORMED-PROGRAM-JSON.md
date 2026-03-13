---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `tools/selfhost/selfhost_build.sh` の default / BuildBox emit-only lane が `apps/tests/hello_simple_llvm.hako` で malformed Program(JSON v0) を出す current root-cause evidence を固定する。
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
  - emitted file is malformed Program(JSON v0)
- payload head:
  - `{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Var","name":"static"}} ... ]}`
- downstream failure:
  - `target/release/hakorune --json-file /tmp/phase29ci_hello_program.json`
  - `❌ JSON v0 bridge error: undefined variable: static`

### 2. BuildBox emit-only keep lane

- command:
  - `HAKO_USE_BUILDBOX=1 bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json /tmp/phase29ci_hello_program_buildbox.json`
- observed result:
  - exit `0`
  - emitted file is the same malformed Program(JSON v0) shape
- interpretation:
  - `hello_simple_llvm` is no longer evidence that `HAKO_USE_BUILDBOX=1` rescues output correctness

### 3. downstream consumer split

- `--mir`
  - green
  - reason: `emit_mir_json_from_source()` is source-direct and bypasses the malformed Program(JSON) payload
- `--run`
  - fails downstream on the emitted JSON payload
  - hidden helper exit reproduced by direct core command:
    - `NYASH_GATE_C_CORE=1 ... target/release/hakorune --json-file /tmp/phase29ci_hello_program.json`
    - `❌ JSON v0 bridge error: undefined variable: static`
- `--exe`
  - fails downstream on the same payload
  - `❌ Program(JSON v0) parse error: undefined variable: static`

## Raw Evidence

### default compiler lane

- raw snapshot:
  - `NYASH_SELFHOST_KEEP_RAW=1 NYASH_SELFHOST_RAW_DIR=/tmp/phase29ci_selfhost_raw bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json ...`
- observed raw log:
  - command tag is `compiler.hako --stage-b --stage3`
  - malformed Program(JSON v0) is printed in raw stdout before shell extraction

### BuildBox lane

- raw snapshot:
  - `NYASH_SELFHOST_KEEP_RAW=1 NYASH_SELFHOST_RAW_DIR=/tmp/phase29ci_selfhost_raw_buildbox HAKO_USE_BUILDBOX=1 bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json ...`
- observed raw log:
  - command tag is `BuildBox.emit_program_json_v0 via compiler build_box`
  - the same malformed Program(JSON v0) is printed in raw stdout before shell extraction

## Current Root-Cause Pin

The current failure is not:

- shell `extract_program_json_v0_from_raw()`
- `selfhost_build.sh` top-level lane routing
- direct JoinIR freeze on `hello_simple_llvm`

The current failure is pinned as:

- malformed Program(JSON v0) production upstream of the shell helper
- with two contributing owners:
  - `lang/src/compiler/entry/compiler_stageb.hako` / `lang/src/compiler/entry/stageb_body_extractor_box.hako`
  - `lang/src/compiler/build/build_box.hako`

### Strongest local evidence

- `HAKO_STAGEB_DEBUG=1 HAKO_SRC="$(cat apps/tests/hello_simple_llvm.hako)" target/release/hakorune --backend vm lang/src/compiler/entry/compiler.hako -- --stage-b --stage3`
- debug shows:
  - `k0`, `k1`, `k2`, `k3` are found for `Main.main`
  - balanced scan reaches the closing `}`
  - but `body_src` remains `null`
  - `StageBBodyExtractorBox.build_body_src()` falls back to full source
- in parallel, `BuildBox.emit_program_json_v0(src, ...)` calls `ParserBox.parse_program2(body_src)` with full-source text as `body_src`

## Interpretation

- default Stage-B lane and BuildBox keep lane are no longer separated by a JoinIR freeze/rescue split
- they are currently unified by a malformed Program(JSON v0) producer shape
- therefore this is a compiler-owner bug, not a shell-helper deletion blocker by itself

## Guardrails

- do not cite `hello_simple_llvm` as evidence that `HAKO_USE_BUILDBOX=1` still rescues emit-only output
- do not reopen shell-helper cleanup as if `extract_program_json_v0_from_raw()` were the cause
- do not mix this producer bugfix with `test_runner.sh` / smoke-tail retirement

## Next Safe Slice

1. pin a smallest owner-local proof for `StageBBodyExtractorBox.build_body_src()` on `static box Main { main() { ... } }`
2. if needed, separately pin `BuildBox.emit_program_json_v0(...)` as “full-source parse” instead of body-source parse
3. only after producer shape is fixed, re-evaluate whether `HAKO_USE_BUILDBOX=1` is a meaningful live keep on `hello_simple_llvm`
