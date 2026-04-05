Self‑Hosting Pilot — Quick Guide (Phase‑15)

Overview
- Goal: keep backend roles explicit while the selfhost mainline uses direct/core routes:
  - `llvm/exe` = product main
  - `rust-vm` = compat/proof keep
  - `vm-hako` = reference/conformance
  - `PyVM` = historical opt-in
- Mainline stays on direct/core routes; explicit `--backend vm` examples below are compat/proof keep and CI still runs smokes to build confidence.

Recommended daily route
- Day-to-day selfhost runtime: `tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako`

Recommended Flows
- Product native path: `tools/build_llvm.sh apps/... -o app && ./app`
- Product EXE-first (crate path): `bash tools/crate_exe_smoke.sh apps/tests/ternary_basic.hako`
- Compat/proof bootstrap E2E (legacy VM keep): `NYASH_USE_NY_COMPILER=1 ./target/release/hakorune --backend vm apps/examples/string_p0.hako`
- Emit‑only: `NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=1 ...`
- Compat/proof bootstrap with parser EXE (legacy VM keep): `tools/build_compiler_exe.sh && NYASH_USE_NY_COMPILER=1 NYASH_USE_NY_COMPILER_EXE=1 ./target/release/hakorune --backend vm apps/examples/string_p0.hako`

CI Workflows
- Selfhost Bootstrap (always): `.github/workflows/selfhost-bootstrap.yml`
  - Builds nyash (`cranelift-jit`) and runs `tools/selfhost/bootstrap_selfhost_smoke.sh`.
- Selfhost EXE‑first（optional）
  - crate 直結（ny-llvmc）で JSON→EXE→実行までを最短経路で確認できるよ。
  - 手順（ローカル）:
    1) MIR(JSON) を出力: `./target/release/hakorune --emit-mir-json tmp/app.json --backend mir apps/tests/ternary_basic.hako`
    2) EXE 生成: `./target/release/ny-llvmc --in tmp/app.json --emit exe --nyrt target/release --out tmp/app`
    3) 実行: `./tmp/app`（戻り値が exit code）
  - ワンコマンドスモーク: `bash tools/crate_exe_smoke.sh apps/tests/ternary_basic.hako`
  - CLI で直接 EXE 出力: `./target/release/hakorune --emit-exe tmp/app --backend mir apps/tests/ternary_basic.hako`
  - Installs LLVM 18, builds `ny-llvmc`, then runs `tools/exe_first_smoke.sh`.

Useful Env Flags
- `NYASH_USE_NY_COMPILER=1`: Enable selfhost compiler pipeline.
- `NYASH_NY_COMPILER_EMIT_ONLY=1`: Print JSON v0 only (no execution).
- `NYASH_NY_COMPILER_TIMEOUT_MS=4000`: Child timeout (ms). Default 2000.
- `NYASH_USE_NY_COMPILER_EXE=1`: Prefer external parser EXE.
- `NYASH_NY_COMPILER_EXE_PATH=<path>`: Override EXE path.
- `NYASH_SELFHOST_READ_TMP=1`: Child reads `tmp/ny_parser_input.ny` when supported.

Troubleshooting (short)
- No Python found: install `python3` (llvmlite harness / legacy PyVM tools).
- No `llvm-config-18`: install LLVM 18 dev (see EXE‑first workflow).
- llvmlite import error: only relevant for explicit compat/probe keep lanes; daily selfhost/product routes do not require it.
- Parser child timeout: raise `NYASH_NY_COMPILER_TIMEOUT_MS`.
- EXE‑first bridge mismatch: re‑run with `NYASH_CLI_VERBOSE=1` and keep `dist/nyash_compiler/sample.json` for inspection.

Notes
- JSON v0 schema is stable but not yet versioned; validation is planned.
- Raw CLI default `vm` maps to Rust VM, but this is a compat/proof keep rather than product ownership.
- Historical PyVM checks are direct-only (`tools/historical/pyvm/*.sh`).
