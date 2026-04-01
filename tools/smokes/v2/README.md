Smokes v2 — Minimal Runner and Policy

## Profiles

### quick
- **Purpose**: Fast feedback for development
- **Target**: ~45 seconds, ~400 tests (Phase 287 P2: 45.85s / 413 tests ✅)
- **Contents**: Minimal gate tests - basic syntax, using resolution, essential control flow
- **Excluded**: Heavy selfhost/Stage-B, crate exe, S3/LLVM integration, long-running tests (>0.4s)

### integration
- **Purpose**: Curated integration and heavier tests
- **Contents**: Selfhost canaries, Stage-B, crate exe, S3/LLVM, phase-specific comprehensive tests and suites
- **Run**: `./tools/smokes/v2/run.sh --profile integration`

### strict
- **Purpose**: Narrow fail-fast gate for policy-sensitive checks
- **Contents**: live strict pins and blocker-focused coverage
- **Run**: `./tools/smokes/v2/run.sh --profile strict`

### plugins
- **Purpose**: Plugin-specific tests (dynamic loading, etc.)
- **Run**: `./tools/smokes/v2/run.sh --profile plugins`

### archive
- **Purpose**: Manual replay / retired pins
- **Run**: `./tools/smokes/v2/run.sh --profile archive`

Policy
- Use [SKIP:<reason>] prefix for environment/host dependent skips.
  - Examples: [SKIP] hakorune not built, [SKIP:env] plugin path missing
  - Keep reasons short and stable to allow grep-based canaries.
- Prefer JSON-only output in CI: set `NYASH_JSON_ONLY=1` to avoid noisy logs.
- Diagnostics lines like `[provider/select:*]` are filtered by default in `lib/test_runner.sh`.
  - Toggle: set `HAKO_SILENT_TAGS=0` to disable filtering and show raw logs. `HAKO_SHOW_CALL_LOGS=1` also bypasses filtering.
- Flake detection: set `SMOKES_REPRO=N` to rerun a failing test up to N times under the same conditions.
  - Keeps logs under `/tmp/hakorune_smoke_*.log` and `/tmp/hakorune_smoke_retry_*.log`.

Helpers
- `tools/smokes/v2/lib/mir_canary.sh` provides:
  - `extract_mir_from_output` — between [MIR_BEGIN]/[MIR_END]
  - `assert_has_tokens`, `assert_skip_tag`, `assert_order`, `assert_token_count`
- `tools/lib/canary.sh` provides minimal, harness-agnostic aliases:
  - `extract_mir_between_tags` — same as `extract_mir_from_output`
  - `require_tokens token...` — fail if any token missing

Notes
- Avoid running heavy integration smokes in CI by default. Use `--profile quick`.
- When a test depends on external tools (e.g., LLVM), prefer `[SKIP:<reason>]` over failure.
- Stage‑B/selfhost canaries（`stage1_launcher_*`, `phase251*` など）は Stage‑3 デフォルト環境で安定しないため、quick プロファイルでは `[SKIP:stageb]` として扱い、必要に応じて別プロファイル（integration/strict/archive）で個別に実行する。
- Selfhost quick カバレッジは最小 1 本（`core/selfhost_minimal.sh`）に絞り、Stage‑3 + JoinIR 前提で Stage‑B→VM を通るかだけを確認する。
- S3 backend 向けの長尺テスト群も quick 向きではないため、timeout を短く保ちたい場合は `[SKIP:slow]` にして別途ローカルで回すことを推奨する。
- `full` is legacy compatibility vocabulary in older docs/configs; current live profile roots are `quick`, `integration`, `strict`, `plugins`, and `archive`.

Quick tips
- EXE-heavy cases (e.g., `phase2100/*`) may take longer. When running quick with these tests, pass a larger timeout like `--timeout 120`.
- Smokes v2 auto-cleans temporary crate EXE objects created under `/tmp` (pattern: `ny_crate_backend_exe_*.o`) after the run.
- VM trace: `./tools/smokes/v2/run.sh --trace-vm` sets `NYASH_VM_TRACE=1` and `HAKO_SILENT_TAGS=0`.

Developer Notes
- **JoinIR If/Select (Phase 33)**: A/B test with `NYASH_FEATURES=stage3 HAKO_JOINIR_IF_SELECT=1 ./target/release/hakorune apps/tests/joinir_if_select_simple.hako`（dev-only、CI対象外。NYASH_JOINIR_CORE は deprecated/無視）
