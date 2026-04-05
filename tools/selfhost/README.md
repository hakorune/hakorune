Hybrid Selfhost Build (80/20)

Purpose
- Provide a minimal, fast path to compile Hako source via Hakorune Stage‑B to Program(JSON v0), and optionally run it via Core‑Direct (in‑proc).
- `Program(JSON v0)` routes are compat/internal keep, not the preferred external/bootstrap boundary.
- Future: add MIR emit and ny-llvmc EXE build in small increments.
- file-level responsibility inventory:
  - `docs/development/current/main/design/selfhost-authority-facade-compat-inventory-ssot.md`
- shell split reading:
  - `tools/selfhost/mainline/build_stage1.sh` = strategy shell
  - `tools/selfhost/lib/stage1_contract.sh` = contract shell
- Stage/lane vocabulary note:
  - canonical stage/route/backend/lane/kernel reading lives in `docs/development/architecture/selfhost_execution_ssot.md`
  - `stage0` = bootstrap keep
  - `stage1` = current bootstrap artifacts / proof line
  - `stage2-mainline` = daily mainline lane
  - `stage2+` = future mainline / distribution umbrella
  - `stage3` in this README is only the same-result compare/sanity label, not a standalone build-conduit family
  - new docs should prefer role-first labels such as `bridge-cli`, `proof-runner`, and `mainline-bundle`; historical stage-numbered artifact names remain compat labels

Script
- tools/selfhost/run.sh
  - Unified thin entrypoint for day-to-day selfhost operations.
  - Modes:
    - `--gate`: run selfhost gate (`phase29bq_selfhost_planner_required_dev_gate_vm.sh`)
    - `--runtime`: run runtime selfhost route (`NYASH_USE_NY_COMPILER=1`)
      - `--runtime-route mainline|compat` is canonical
      - `--runtime-mode exe|stage-a-compat` remains a compatibility alias
      - `runtime-route compat` stays explicit and now requires `NYASH_VM_USE_FALLBACK=1` before entering raw `--backend vm`
    - `--direct`: run Stage-B direct/source route (proof-oriented; VM wrapper is kept explicit-only)
  - Examples:
    ```bash
    tools/selfhost/run.sh --gate --max-cases 5
    tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako
    tools/selfhost/run.sh --runtime --runtime-route mainline --input apps/examples/string_p0.hako
    tools/selfhost/run.sh --runtime --runtime-route compat --input apps/examples/string_p0.hako
    tools/selfhost/run.sh --direct --source-file apps/tests/phase29bq_selfhost_cleanup_only_min.hako
    ```
- tools/selfhost/proof/run_stageb_compiler_vm.sh
  - Optional public proof gate for explicit Stage-B VM keep.
  - Use this only when you need the proof-only Stage-B compiler route on purpose.
- tools/selfhost/proof/selfhost_vm_smoke.sh
  - Optional public proof smoke for selfhost-minimal on the explicit VM keep.
  - Historical top-level alias remains a compatibility facade; do not read this as the day-to-day runtime route.
- Internal proof helpers (engineering keep)
  - `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/proof/selfhost_smoke.sh`
  - `tools/selfhost/proof/selfhost_stage3_accept_smoke.sh`
  - keep these for bootstrap/acceptance engineering proof; they are not the general front-door proof surface.
- tools/selfhost/selfhost_build.sh
  - --in <file.hako>: input Hako source
  - --json <out.json>: retired wrapper surface; use `--mir` for day-to-day flow and raw compat probes/flags for Program(JSON)
  - --mir <out.json>: emit MIR(JSON) from source (preferred runner path)
  - --exe <out>: build native executable via ny-llvmc mainline route
  - --run: run via Gate‑C/Core Direct (in‑proc). Exit code mirrors program return.
  - `--exe` now keeps context resolution behind `resolve_emit_exe_context()` and Program(JSON)->MIR->EXE execution behind `emit_exe_from_program_json_v0_with_context()`, so EXE lane cleanup stays separate from the top-level route tail.
  - Env:
    - NYASH_BIN: path to hakorune/nyash binary (auto-detected)
    - NYASH_ROOT: repo root (auto-detected)
    - BuildBox emit-only is retired from the default caller path; use the direct/source route instead
- tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh
  - Parser handoff Tier-2 の 1件PROMOTEを 1コマンドで同期するヘルパー。
  - 同期対象:
    - `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
    - `docs/development/current/main/phases/phase-29bq/29bq-93-parser-handoff-tier2-backlog.md`
    - `CURRENT_TASK.md`（legacy compatibility block がある場合のみ）
  - archived helper; use the archive path for historical replay only.
  - 必須引数:
    - `--fixture`
    - `--expected`
    - `--backlog-id`
    - `--next-task`
  - 例:
    ```bash
    tools/archive/legacy-selfhost/engineering/promote_tier2_case.sh \
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
# Exact helper-local Program(JSON)->MIR->EXE consumer proof
bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh

# Day-to-day selfhost entrypoint
tools/selfhost/run.sh --runtime --input apps/examples/string_p0.hako
```

Explicit compat boundary probe:
```bash
bash tools/dev/phase29ch_program_json_compat_route_probe.sh
```

Historical pure selfhost helper:
```bash
bash tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_selfhost.sh <mir.json> [exe_out]
bash tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh
```

- `tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_selfhost.sh` is the archived compat wrapper and preserves the historical shell contract while materializing the payload onto `vm-hako`.
- the old `tools/selfhost/run_compat_pure_selfhost.sh` path is retired.
- treat `tools/archive/legacy-selfhost/compat-codegen/hako_llvm_selfhost_driver.hako` as the archive-later payload and the wrapper as transport only.
- `tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh` is the historical compat pure-pack entry that shells into `phase2120/run_pure_capi_canaries.sh` and then the transport wrapper above.
- read the stack in this order:
  - payload: `tools/archive/legacy-selfhost/compat-codegen/hako_llvm_selfhost_driver.hako`
  - transport wrapper: `tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_selfhost.sh`
  - pack orchestrator: `tools/archive/legacy-selfhost/compat-codegen/run_compat_pure_pack.sh`
- the old `tools/selfhost/run_compat_pure_*` paths are retired; use the archive-codegen entrypoints above for historical runs.
- `run_compat_pure_pack.sh` is pack orchestration only, not a separate proof surface.
- old alias `run_all.sh` is retired; keep the compat pack entry singular.
- The owner-lane replacement proof is `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`.
- The compat wrapper now also runs on `--backend vm-hako`, but it still proves the provider stop-line rather than the pure owner lane.
- `phase-29x` cleanup bands are mirrored in `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`; the proof/example driver stays archive-later until the compat wrapper gains a root-first equivalent or is retired as a whole.

Notes
- Stage‑B emit uses the Stage‑B entry by default; BuildBox emit-only stays explicit compat/proof only and is retired from the day-to-day caller path
- raw `selfhost_build.sh --in ...` whole-script routes are not the current helper-local acceptance line; use the focused probe above for the EXE consumer seam while upstream Stage-B source-route freezes remain
- Runner executes Core‑Direct in-proc under HAKO_CORE_DIRECT_INPROC=1.
- PyVM は historical / direct-only 扱い（既定導線は mainline direct/core）。legacy parity が必要な場合は `tools/historical/pyvm/*.sh` を使う。
- For heavier cases (bundles/alias/require), keep Stage‑B canaries opt‑in in quick profile.

---

Stage1 Bootstrap Artifacts (Phase 25.1 — artifact-kind split)

Purpose
- Provide concrete Stage1 bootstrap artifacts built from Hako sources.
- Separate artifact contracts to avoid route drift between the Stage1 artifact kinds `launcher-exe` and `stage1-cli`.
- The `stage1-cli` artifact is a runnable bootstrap output; payload proof stays on the stage0 bootstrap route.
- In this section, `launcher-exe` / `stage1-cli` are artifact kinds for the Stage1 line, not stage numbers.

Script
- tools/selfhost/mainline/build_stage1.sh
  - Builds a selfhost executable from a Nyash/Hako entry point.
  - Artifact kinds:
    - `launcher-exe` (default): run-oriented launcher artifact
      - entry: `lang/src/runner/entry/launcher_native_entry.hako`
      - output: `target/selfhost/hakorune`
    - `stage1-cli`: bootstrap output artifact for the reduced Stage1 lane
      - entry: `lang/src/runner/entry/stage1_cli_env_entry.hako`
      - output: `target/selfhost/hakorune.stage1_cli`
  - These entry files are thin run-only stubs; the logical CLI owners stay in
    `lang/src/runner/launcher.hako` and `lang/src/runner/stage1_cli_env.hako`.
  - Writes sidecar metadata: `<out>.artifact_kind`

Usage
```bash
# Build launcher-exe artifact (default)
tools/selfhost/mainline/build_stage1.sh

# Build stage1-cli artifact explicitly
tools/selfhost/mainline/build_stage1.sh --artifact-kind stage1-cli

# Custom output path
tools/selfhost/mainline/build_stage1.sh --out /tmp/hakorune-dev

# Custom entry (experimental)
tools/selfhost/mainline/build_stage1.sh --entry apps/selfhost-minimal/main.hako --out /tmp/hako_min
```

How it works
- Pipeline:
  1) Stage‑B + MirBuilder:
     - `tools/hakorune_emit_mir.sh <entry.hako> <mir.json>`
     - top-level thin mainline preset wrapper: `tools/hakorune_emit_mir_mainline.sh <entry.hako> <mir.json>`
     - operational route SSOT for new scripts stays on `tools/smokes/v2/lib/emit_mir_route.sh`
  2) LLVM EXE build:
     - `tools/ny_mir_builder.sh --in <mir.json> --emit exe -o <exe>`
- The Rust binary (Stage0) is resolved via the existing helpers inside `hakorune_emit_mir.sh` / `ny_mir_builder.sh`:
  - Prefers `target/release/hakorune` when present.
  - Falls back to `target/release/nyash` otherwise.

Notes
- `launcher-exe` is still a run artifact and does not satisfy G1 identity emit contract by itself.
- `stage1-cli` is a runnable bootstrap output; success is defined by stage0 bootstrap payload proof plus reduced artifact `run` liveness, not by reduced artifact payload emission.
- `stage0` bootstrap proof stays on the payload/file materialization route.
- `selfhost_build.sh` keeps its post-emit final output selection behind `dispatch_stageb_primary_output()`, and its `--exe` lane keeps context resolution behind `resolve_emit_exe_context()` plus Program(JSON)->MIR->EXE execution behind `emit_exe_from_program_json_v0_with_context()`, so `--exe` / `--run` / path-result routes stay owner-local instead of inline in the main tail.
- current proven closure is `stage3 launcher -> stage4 stage1-cli -> stage5 launcher -> stage6 stage1-cli -> stage7 launcher`
- `tools/selfhost_identity_check.sh` keeps the stage0 / stage1 compare contract in full mode as a separate diagnostics lane; the reduced artifact itself is not the payload-emitting contract.
- Prefer explicit artifact kind in scripts and CI to avoid accidental contract mismatch.

Helper — G1 Identity Check
- `tools/selfhost_identity_check.sh`
  - Orchestrates Stage1/Stage2 build+compare flow (argument parsing and gate flow only).
  - In this helper, Stage1/Stage2 are compare-pair labels; Stage2 distribution packaging is a separate future SSOT.
  - `--cli-mode auto|stage0` is compatibility-only and requires `--allow-compat-route` explicit opt-in.
  - Route/emit helpers are split into:
    - `tools/selfhost/lib/identity_routes.sh`
    - `tools/selfhost/lib/identity_compare.sh`
  - This split keeps route policy and compare policy centralized, while preserving existing CLI behavior.
  - MIR canonical compare helper/test:
    - `tools/selfhost/lib/mir_canonical_compare.py`
    - `python3 -m unittest tools.selfhost.lib.tests.test_mir_canonical_compare`

Helper — Stage3 Same-Result Check
- `tools/selfhost/stage3_same_result_check.sh`
  - Stage3 is the bootstrap same-result sanity check, not the parser/bridge `Stage3` acceptance smoke.
  - `stage2-bin` / `stage3-bin` in this helper are compare-artifact labels only; they are not separate artifact-kind families.
  - Build lane: re-emit Program(JSON v0) and MIR(JSON) snapshots twice from a known-good seed, then compare the snapshots plus `.artifact_kind`.
  - `--artifact-kind stage1-cli` is the working build lane today; `--seed-bin` can override the payload seed.
  - `--skip-build` compares an explicit prebuilt Stage2/Stage3 pair only.
  - Use this helper when you want to confirm bootstrap reproducibility without touching G1 Program/MIR identity comparison.

Archived Helper — Legacy Main Readiness
- `tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh`
  - Runs producer inventory + consumer inventory + identity smoke in one command.
  - Inventory count ignores comment-only matches.
  - Default command:
    - `bash tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh`
  - Strict gate command:
    - `bash tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh --strict`
  - Compatibility probe command:
    - `bash tools/archive/legacy-selfhost/engineering/legacy_main_readiness.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune`
  - Exit code contract:
    - `0`: flow completed (`--strict` の場合は readiness 条件も満たす)
    - `1`: readiness 条件未達（`--strict` のみ）
    - `2`: smoke 失敗 or usage/setup error
- Promotion rule:
  - Start legacy-literal removal only when strict gate returns `0`.

Archived Helper — Legacy Main Removal Pre-PROMOTE Gate
- `tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh`
  - Dedicated pre-promote gate for commits that remove legacy literals from `compiler_stageb.hako` / `compiler.hako`.
  - Default command:
    - `bash tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh`
  - Compatibility probe command:
    - `bash tools/archive/legacy-selfhost/engineering/pre_promote_legacy_main_removal.sh --cli-mode auto --bin-stage1 target/release/hakorune --bin-stage2 target/release/hakorune`
  - Exit code contract:
    - `0`: ready to start legacy literal removal commit
    - `1`: not ready (strict readiness failed)
    - `2`: usage/setup/smoke error
  - Removal sequence decision:
    - tests-side producers first, compiler literals second (see migration-order SSOT).

Helper — Stage1 CLI Runner
  - `tools/selfhost/compat/run_stage1_cli.sh`
  - Wraps a Stage1 binary (default `target/selfhost/hakorune`) with the required runtime env:
    - `NYASH_NYRT_SILENT_RESULT=1`（Result 行を抑止して JSON stdout を維持）
    - `NYASH_DISABLE_PLUGINS=1`, `NYASH_FILEBOX_MODE=core-ro`（FileBox などのコア実装を強制）
  - For `emit mir-json`, translate the raw CLI surface into the compatibility env contract (`stage1_contract_exec_mode`); this is a compatibility wrapper, not the bootstrap proof route.
  - `emit program-json` is retired from the wrapper surface. Use the explicit compat probe instead.
  - Non-`emit` arguments are passed verbatim to the Stage1 binary:
    ```bash
    tools/selfhost/compat/run_stage1_cli.sh --bin /tmp/hakorune-dev emit mir-json apps/tests/minimal.hako
    tools/dev/phase29ch_program_json_compat_route_probe.sh --bin /tmp/hakorune-dev apps/tests/minimal.hako
    ```
  - Use this helper (or set the env vars manually) whenever CLI output is consumed by compatibility scripts. The bootstrap acceptance path is `stage1_contract_verify_stage1_cli_bootstrap_capability()`.
  - current mainline smoke:
    ```bash
    tools/selfhost/mainline/stage1_mainline_smoke.sh
    tools/selfhost/mainline/stage1_mainline_smoke.sh --bin target/selfhost/hakorune.stage1_cli.stage2 apps/tests/hello_simple_llvm.hako
    ```
  - legacy embedded bridge smoke moved to `tools/archive/legacy-selfhost/stage1_embedded_smoke.sh` and is not the daily/mainline proof route.
