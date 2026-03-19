Hybrid Selfhost Build (80/20)

Purpose
- Provide a minimal, fast path to compile Hako source via Hakorune Stage‑B to Program(JSON v0), and optionally run it via Core‑Direct (in‑proc).
- Future: add MIR emit and ny-llvmc EXE build in small increments.

Script
- tools/selfhost/run.sh
  - Unified thin entrypoint for day-to-day selfhost operations.
  - Modes:
    - `--gate`: run selfhost gate (`phase29bq_selfhost_planner_required_dev_gate_vm.sh`)
    - `--runtime`: run runtime selfhost route (`NYASH_USE_NY_COMPILER=1`)
      - `--runtime-mode stage-a|exe` (default `stage-a`)
    - `--direct`: run Stage-B wrapper route (`run_stageb_compiler_vm.sh`)
  - Examples:
    ```bash
    tools/selfhost/run.sh --gate --max-cases 5
    tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako
    tools/selfhost/run.sh --runtime --runtime-mode exe --input apps/examples/string_p0.hako
    tools/selfhost/run.sh --direct --source-file apps/tests/phase29bq_selfhost_cleanup_only_min.hako
    ```
- tools/selfhost/selfhost_build.sh
  - --in <file.hako>: input Hako source
  - --json <out.json>: write Program(JSON v0); default: /tmp/hako_stageb_$$.json
  - --mir <out.json>: emit MIR(JSON) from source (runner path)
  - --exe <out>: build native executable via ny-llvmc (llvmlite harness)
  - --run: run via Gate‑C/Core Direct (in‑proc). Exit code mirrors program return.
  - Env:
    - NYASH_BIN: path to hakorune/nyash binary (auto-detected)
    - NYASH_ROOT: repo root (auto-detected)
    - HAKO_USE_BUILDBOX=1: use BuildBox for emit-only (no run/exe)
- tools/selfhost/promote_tier2_case.sh
  - Parser handoff Tier-2 の 1件PROMOTEを 1コマンドで同期するヘルパー。
  - 同期対象:
    - `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
    - `docs/development/current/main/phases/phase-29bq/29bq-93-parser-handoff-tier2-backlog.md`
    - `CURRENT_TASK.md`（legacy compatibility block がある場合のみ）
  - 必須引数:
    - `--fixture`
    - `--expected`
    - `--backlog-id`
    - `--next-task`
  - 例:
    ```bash
    tools/selfhost/promote_tier2_case.sh \
      --fixture apps/tests/phase29bq_selfhost_local_expr_compare_rel_mixed_logic_cleanup_min.hako \
      --expected 2477 \
      --backlog-id T2-CMP-REL-MIX \
      --next-task "! + unary - + 比較 + &&"
    ```
- tools/selfhost/run_lane_a_daily.sh
  - lane A（JoinIR compiler-meaning）の日次ワンコマンド入口。
  - 実行内容:
    - `tools/checks/phase29bq_joinir_port_sync_guard.sh`
    - `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
  - オプション:
    - `--guards-only`: sync/promotion guard のみ実行
  - 例:
    ```bash
    bash tools/selfhost/run_lane_a_daily.sh
    bash tools/selfhost/run_lane_a_daily.sh --guards-only
    ```
- tools/selfhost/sync_lane_a_state.sh
  - lane A 状態 mirror の同期ヘルパー（single-entry）。
  - 入力SSOT:
    - `CURRENT_TASK.md` の compiler lane block（active/done/next）
  - 同期先:
    - `docs/development/current/main/10-Now.md`
    - `docs/development/current/main/design/de-rust-lane-map-ssot.md`
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
  - 使い方:
    ```bash
    bash tools/selfhost/sync_lane_a_state.sh
    bash tools/checks/phase29bq_joinir_port_sync_guard.sh
    ```
- tools/selfhost/sync_wasm_lane_state.sh
  - wasm lane pointer 同期ヘルパー（`CURRENT_TASK.md` / `phase-29cc/README.md` / `10-Now.md`）。
  - 使い方:
    ```bash
    bash tools/selfhost/sync_wasm_lane_state.sh \
      --lock-doc docs/development/current/main/phases/phase-29cc/29cc-163-wsm-p5-min4-hako-lane-bridge-shrink-lock-ssot.md \
      --next-id WSM-P5-min5 \
      --next-task-text ".hako emitter/binary writer 実体路を 1 shape 接続し、bridge fallback 非依存 case を lock する。" \
      --readme-note "1-shape real `.hako` emitter/binary writer route lock"
    ```

Examples
```bash
# Emit JSON only (Stage‑B)
tools/selfhost/selfhost_build.sh --in apps/demo/main.hako --json /tmp/demo.json

# Run and use exit code
tools/selfhost/selfhost_build.sh --in apps/demo/return7.hako --run; echo $?
```

Notes
- Stage‑B emit uses either the Stage‑B entry or BuildBox（HAKO_USE_BUILDBOX=1 for emit-only）
- Runner executes Core‑Direct in-proc under HAKO_CORE_DIRECT_INPROC=1.
- PyVM は historical / direct-only 扱い（既定導線は Rust VM）。legacy parity が必要な場合は `tools/historical/pyvm/*.sh` を使う。
- For heavier cases (bundles/alias/require), keep Stage‑B canaries opt‑in in quick profile.

---

Stage1 Selfhost Binary (Phase 25.1 — initial wiring)

Purpose
- Provide concrete selfhost artifacts built from Hako sources.
- Separate artifact contracts to avoid route drift between `launcher-exe` and `stage1-cli`.
- The `stage1-cli` artifact is a runnable bootstrap output; payload proof stays on the stage0 bootstrap route.

Script
- tools/selfhost/build_stage1.sh
  - Builds a selfhost executable from a Nyash/Hako entry point.
  - Artifact kinds:
    - `launcher-exe` (default): run-oriented launcher artifact
      - entry: `lang/src/runner/launcher.hako`
      - output: `target/selfhost/hakorune`
    - `stage1-cli`: bootstrap output artifact for the reduced Stage1 lane
      - entry: `lang/src/runner/stage1_cli_env.hako`
      - output: `target/selfhost/hakorune.stage1_cli`
  - Writes sidecar metadata: `<out>.artifact_kind`

Usage
```bash
# Build launcher-exe artifact (default)
tools/selfhost/build_stage1.sh

# Build stage1-cli artifact explicitly
tools/selfhost/build_stage1.sh --artifact-kind stage1-cli

# Custom output path
tools/selfhost/build_stage1.sh --out /tmp/hakorune-dev

# Custom entry (experimental)
tools/selfhost/build_stage1.sh --entry apps/selfhost-minimal/main.hako --out /tmp/hako_min
```

How it works
- Pipeline:
  1) Stage‑B + MirBuilder:
     - `tools/hakorune_emit_mir.sh <entry.hako> <mir.json>`
     - internal-only mainline route: `tools/hakorune_emit_mir_mainline.sh <entry.hako> <mir.json>`
  2) LLVM EXE build:
     - `tools/ny_mir_builder.sh --in <mir.json> --emit exe -o <exe>`
- The Rust binary (Stage0) is resolved via the existing helpers inside `hakorune_emit_mir.sh` / `ny_mir_builder.sh`:
  - Prefers `target/release/hakorune` when present.
  - Falls back to `target/release/nyash` otherwise.

Notes
- `launcher-exe` is still a run artifact and does not satisfy G1 identity emit contract by itself.
- `stage1-cli` is a runnable bootstrap output; success is defined by stage0 bootstrap payload proof plus reduced artifact liveness, not by reduced artifact payload emission.
- `stage0` bootstrap proof stays on the payload/file materialization route.
- current proven closure is `stage3 launcher -> stage4 stage1-cli -> stage5 launcher -> stage6 stage1-cli -> stage7 launcher`
- `tools/selfhost_identity_check.sh` keeps the stage0 / stage1 compare contract in full mode as a separate diagnostics lane; the reduced artifact itself is not the payload-emitting contract.
- Prefer explicit artifact kind in scripts and CI to avoid accidental contract mismatch.

Helper — G1 Identity Check
- `tools/selfhost_identity_check.sh`
  - Orchestrates Stage1/Stage2 build+compare flow (argument parsing and gate flow only).
  - `--cli-mode auto|stage0` is compatibility-only and requires `--allow-compat-route` explicit opt-in.
  - Route/emit helpers are split into:
    - `tools/selfhost/lib/identity_routes.sh`
    - `tools/selfhost/lib/identity_compare.sh`
  - This split keeps route policy and compare policy centralized, while preserving existing CLI behavior.
  - MIR canonical compare helper/test:
    - `tools/selfhost/lib/mir_canonical_compare.py`
    - `python3 -m unittest tools.selfhost.lib.tests.test_mir_canonical_compare`

Helper — Legacy Main Readiness
- `tools/selfhost/legacy_main_readiness.sh`
  - Runs producer inventory + consumer inventory + identity smoke in one command.
  - Inventory count ignores comment-only matches.
  - Default command:
    - `bash tools/selfhost/legacy_main_readiness.sh`
  - Strict gate command:
    - `bash tools/selfhost/legacy_main_readiness.sh --strict`
  - Compatibility probe command:
    - `bash tools/selfhost/legacy_main_readiness.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune`
  - Exit code contract:
    - `0`: flow completed (`--strict` の場合は readiness 条件も満たす)
    - `1`: readiness 条件未達（`--strict` のみ）
    - `2`: smoke 失敗 or usage/setup error
- Promotion rule:
  - Start legacy-literal removal only when strict gate returns `0`.

Helper — Legacy Main Removal Pre-PROMOTE Gate
- `tools/selfhost/pre_promote_legacy_main_removal.sh`
  - Dedicated pre-promote gate for commits that remove legacy literals from `compiler_stageb.hako` / `compiler.hako`.
  - Default command:
    - `bash tools/selfhost/pre_promote_legacy_main_removal.sh`
  - Compatibility probe command:
    - `bash tools/selfhost/pre_promote_legacy_main_removal.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune`
  - Exit code contract:
    - `0`: ready to start legacy literal removal commit
    - `1`: not ready (strict readiness failed)
    - `2`: usage/setup/smoke error
  - Removal sequence decision:
    - tests-side producers first, compiler literals second (see migration-order SSOT).

Helper — Stage1 CLI Runner
- `tools/selfhost/run_stage1_cli.sh`
  - Wraps a Stage1 binary (default `target/selfhost/hakorune`) with the required runtime env:
    - `NYASH_NYRT_SILENT_RESULT=1`（Result 行を抑止して JSON stdout を維持）
    - `NYASH_DISABLE_PLUGINS=1`, `NYASH_FILEBOX_MODE=core-ro`（FileBox などのコア実装を強制）
  - For `emit program-json` / `emit mir-json`, translate the raw CLI surface into the compatibility env contract (`stage1_contract_exec_mode`); this is a compatibility wrapper, not the bootstrap proof route.
  - Non-`emit` arguments are passed verbatim to the Stage1 binary:
    ```bash
    tools/selfhost/run_stage1_cli.sh emit program-json apps/tests/minimal.hako
    tools/selfhost/run_stage1_cli.sh --bin /tmp/hakorune-dev emit mir-json apps/tests/minimal.hako
    ```
  - Use this helper (or set the env vars manually) whenever CLI output is consumed by compatibility scripts. The bootstrap acceptance path is `stage1_contract_verify_stage1_cli_bootstrap_capability()`.
