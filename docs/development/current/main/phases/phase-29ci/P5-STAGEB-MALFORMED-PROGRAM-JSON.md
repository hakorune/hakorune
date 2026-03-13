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
  - emitted file is still malformed Program(JSON v0), but no longer the old full-source `static/ox/ain` shape
- payload head:
  - `{"version":0,"kind":"Program","body":[{"type":"Extern","iface":"env.console","method":"log","args":[{"type":"Int","value":42}]},{"type":"Expr","expr":{"type":"Var","name":"eturn"}}]}`
- downstream failure:
  - `target/release/hakorune --json-file /tmp/phase29ci_hello_program.json`
  - `❌ JSON v0 bridge error: undefined variable: eturn`
- interpretation:
  - `StageBBodyExtractorBox.build_body_src()` is no longer falling straight back to full source for this fixture
  - remaining debt is now downstream of body extraction, on the body-only parse path

### 2. BuildBox emit-only keep lane

- command:
  - `HAKO_USE_BUILDBOX=1 bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json /tmp/phase29ci_hello_program_buildbox.json`
- observed result:
  - exit `0`
  - emitted file is still the old full-source malformed Program(JSON v0) shape
- payload head:
  - `{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Var","name":"static"}},{"type":"Expr","expr":{"type":"Var","name":"ox"}},{"type":"Expr","expr":{"type":"Var","name":"ain"}} ... ]}`
- interpretation:
  - `hello_simple_llvm` now proves the opposite split: default Stage-B and BuildBox keep no longer emit the same bad payload
  - BuildBox remains on the full-source parse boundary and still needs a separate owner-local fix

### 3. downstream consumer split

- `--mir`
  - green
  - reason: `emit_mir_json_from_source()` is source-direct and bypasses the malformed Program(JSON) payload
- `--run`
  - fails downstream on the emitted JSON payload
  - hidden helper exit reproduced by direct core command:
    - `NYASH_GATE_C_CORE=1 ... target/release/hakorune --json-file /tmp/phase29ci_hello_program.json`
    - `❌ JSON v0 bridge error: undefined variable: eturn`
- `--exe`
  - fails downstream on the same payload
  - `❌ Program(JSON v0) parse error: undefined variable: eturn`

## Raw Evidence

### default compiler lane

- raw snapshot:
  - `NYASH_SELFHOST_KEEP_RAW=1 NYASH_SELFHOST_RAW_DIR=/tmp/phase29ci_selfhost_raw bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json ...`
- observed raw log:
  - command tag is `compiler.hako --stage-b --stage3`
  - malformed Program(JSON v0) is printed in raw stdout before shell extraction
  - after the current fix, that raw payload starts with `Extern(log 42)` and then `Var("eturn")`

### BuildBox lane

- raw snapshot:
  - `NYASH_SELFHOST_KEEP_RAW=1 NYASH_SELFHOST_RAW_DIR=/tmp/phase29ci_selfhost_raw_buildbox HAKO_USE_BUILDBOX=1 bash tools/selfhost/selfhost_build.sh --in apps/tests/hello_simple_llvm.hako --json ...`
- observed raw log:
  - command tag is `BuildBox.emit_program_json_v0 via compiler build_box`
  - the old full-source malformed Program(JSON v0) is still printed in raw stdout before shell extraction

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
- but they are no longer the same malformed shape on `hello_simple_llvm`

### Strongest local evidence

- `HAKO_STAGEB_DEBUG=1 HAKO_SRC="$(cat apps/tests/hello_simple_llvm.hako)" target/release/hakorune --backend vm lang/src/compiler/entry/compiler.hako -- --stage-b --stage3`
- debug shows:
  - `k0`, `k1`, `k2`, `k3` are found for `Main.main`
  - balanced scan reaches the closing `}`
  - but `body_src` remains `null`
  - `StageBBodyExtractorBox.build_body_src()` falls back to full source
- current code now reuses `BodyExtractionBox.extract_main_body(src)` before falling back to full source, which is enough to remove the `static/ox/ain` shape on the default Stage-B lane
- in parallel, `BuildBox.emit_program_json_v0(src, ...)` still calls `ParserBox.parse_program2(body_src)` with full-source text as `body_src`

## Interpretation

- default Stage-B lane and BuildBox keep lane are no longer separated by a JoinIR freeze/rescue split
- they are no longer unified by the same malformed payload either
- default Stage-B now proves the extractor-side full-source fallback was one owner-local bug
- BuildBox still proves the full-source parse boundary is a separate owner-local bug
- therefore this remains a compiler-owner bug, not a shell-helper deletion blocker by itself

## Guardrails

- do not cite `hello_simple_llvm` as evidence that `HAKO_USE_BUILDBOX=1` still rescues emit-only output
- do not cite `hello_simple_llvm` as evidence that default Stage-B output is fully healthy yet; it still fails downstream with `eturn`
- do not reopen shell-helper cleanup as if `extract_program_json_v0_from_raw()` were the cause
- do not mix this producer bugfix with `test_runner.sh` / smoke-tail retirement

## Next Safe Slice

1. pin the next owner-local debt on the default Stage-B lane: why `parse_block2("{" + body_src + "}", 0)` still turns `return` into `eturn`
2. separately keep `BuildBox.emit_program_json_v0(...)` pinned as “full-source parse” instead of body-source parse
3. only after both producer shapes are fixed, re-evaluate whether `HAKO_USE_BUILDBOX=1` is a meaningful live keep on `hello_simple_llvm`
