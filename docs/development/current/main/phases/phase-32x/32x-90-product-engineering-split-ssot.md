---
Status: SSOT
Decision: provisional
Date: 2026-04-02
Scope: product / engineering mixed-owner source-surface と smoke-aggregator surface の split order を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-32x/README.md
  - docs/development/current/main/phases/phase-32x/32x-91-task-board.md
  - docs/development/current/main/phases/phase-31x/31x-90-engineering-lane-isolation-ssot.md
---

# 32x-90 Product / Engineering Split

## Goal

- `llvm/exe` product ownership と `rust-vm` engineering(stage0/bootstrap + tooling keep) residue が同居している source/smoke を split する。
- `vm-rust` delete ではなく owner separation を先に進める。
- `phase-31x` で rehome した engineering homes を前提に、next actual cleanup targets を exact に固定する。

## Fixed Rules

- keep `vm-rust` as `engineering(stage0/bootstrap + tooling keep)`.
- keep `vm-hako` as `reference/conformance`.
- keep `wasm` as `experimental/monitor-only`.
- prefer `split/rehome/drain` over forced deletion.
- keep raw default/token/dispatch freeze on:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
- do not start from `src/runner/modes/vm.rs`; start from mixed-owner surfaces:
  - `src/runner/build.rs`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `32xA mixed-owner inventory` | landed | exact mixed-owner surfaces を inventory する | `build.rs` と `phase2100/run_all.sh` の mixed roles が docs で読める |
| `32xB build.rs split plan` | landed | product build と engineering build の split target を固定する | `build.rs` の shared / product / engineering seams が分かれた計画になる |
| `32xC phase2100 role split plan` | landed | mixed aggregator を role buckets へ切る | selfhost / probe / product / experimental / shared の thin meta-runner 形が固定される |
| `32xD top-level orchestrator rehome prep` | landed | `bootstrap_selfhost` / `plugin_v2` の caller drain を固定し canonical home へ repoint する | current/public callers が canonical home へ切り替わり top-level は shim-only になる |
| `32xE direct-route takeover prep` | landed | child/stage1 shell residues を core route へ寄せる準備をする | `core_executor` takeover seam と direct shell gap が固定される |
| `32xF shared helper follow-up gate` | landed | helper family を別 phase へ回す gate を決める | shared helpers are either explicit keep or reopened under a dedicated phase |
| `32xG raw default/token gate` | landed | default/token rewrite の可否を最後に判定する | source split 後まで `args.rs` / `dispatch.rs` が untouched のまま保たれ、rewrite は later lane へ deferred される |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `32xA1` | landed | `build.rs` mixed ownership inventory | object emit / feature build / link flow の product vs engineering 同居が exact に読める |
| `32xA2` | landed | `phase2100` mixed aggregator inventory | selfhost / llvmlite probe / crate product / native experimental reps が exact に読める |
| `32xB1` | landed | `build.rs` split target lock | product build owner, engineering build owner, shared prelude が docs で分かれる |
| `32xB2` | landed | `build.rs` implementation slice order | helper-first / owner-split / caller-preserve の順が固定される |
| `32xC1` | landed | `phase2100` role bucket lock | selfhost / probe / product / experimental / shared bucket と exact script set が固定される |
| `32xC2` | landed | `phase2100` thin meta-runner plan | top-level aggregator が meta-runner only に縮み、role sub-runners が live になる |
| `32xD1` | landed | `bootstrap_selfhost_smoke.sh` caller drain map | canonical home が `tools/selfhost/bootstrap_selfhost_smoke.sh` に固定され current/public caller が repoint される |
| `32xD2` | landed | `plugin_v2_smoke.sh` caller drain map | canonical home が `tools/plugins/plugin_v2_smoke.sh` に固定され current/public caller が repoint される |
| `32xE1` | landed | `child.rs` / `stage1_cli` direct-route gap inventory | direct `--backend vm` shell residues と stage1 raw compat branches の exact gap が読める |
| `32xE2` | landed | `core_executor` takeover seam lock | direct MIR/core route に寄せる seam が固定される |
| `32xF1` | landed | shared helper follow-up gate | shared helper family is fixed as `keep here`, `family-home rehome`, or `dedicated phase keep` |
| `32xG1` | landed | raw backend default/token decision remains last | `args.rs` / `dispatch.rs` are still do-not-flip-early |

## 32xA Result

### `build.rs`

- current `build_aot` default is still `cranelift`.
- core build chooses cargo features by owner:
  - `llvm` -> `--features llvm`
  - else -> `--features cranelift-jit`
- object emit is mixed:
  - product path: `--backend llvm`
  - engineering path: `--backend vm`
- link stage is shared after owner-specific object emission.

Read as:
- product build ownership and engineering/bootstrap build ownership still coexist in one file.
- first source split should start here, not in `vm.rs`.

### `phase2100/run_all.sh`

- current aggregator mixes:
  - engineering selfhost canaries
  - deprecated/opt-in llvmlite probe reps
  - crate `ny-llvmc` product canaries
  - native experimental reps
  - always-on/shared reps

Read as:
- live home is correct, but the file is a thick mixed aggregator.
- next cleanup should split by role bucket, not by deleting the profile home.

## Current Focus

- active macro wave: `phase-32x closeout review`
- active micro task: `phase-32x closeout review`
- next queued micro task: `dedicated helper phase` (only if caller drain becomes exact)
- current blocker: `none`

## 32xB1 Result

### Shared seam to keep together first

- config/env load from `hako.toml` / build config
- plugin build loop
- app selection and candidate discovery
- platform link step after object emission

### Product seam to split out

- core build with `--features llvm`
- product object emit via `--backend llvm`
- product artifact ownership for `llvm/exe`

### Engineering seam to split out

- core build with `--features cranelift-jit`
- engineering object emit via `--backend vm`
- stage0/bootstrap build ownership

Read as:
- first cut is not file deletion. It is shared-vs-owner separation inside `build.rs`.
- owner split should happen before any default/token discussion.

## 32xB2 Result

- helper-first extraction landed inside the same file:
  - `load_build_doc`
  - `apply_env_overrides`
  - `build_plugins`
  - `build_core`
  - `resolve_app_entry`
  - `emit_object`
  - `emit_llvm_object`
  - `emit_engineering_object`
  - `ensure_object_exists`
- behavior stayed the same:
  - product emit still uses `--backend llvm`
  - engineering emit still uses `--backend vm`
  - link/output flow stayed shared

Read as:
- `build.rs` is now thinner without changing default/token policy.
- next cleanup should move to the smoke side (`phase2100`) before reopening deeper direct-route work.

## 32xC1 Result

### Fixed role buckets

- `engineering-selfhost`
  - `phase2100/selfhost_canary_minimal.sh`
  - `phase2048/s1s2s3_repeat_const_canary_vm.sh`
  - `phase2048/s1s2s3_repeat_compare_cfg_canary_vm.sh`
  - `phase2048/s1s2s3_repeat_threeblock_collect_canary_vm.sh`
  - `phase2051/selfhost_v1_primary_rc42_canary_vm.sh` when `HAKO_PHASE2100_ENABLE_HV1=1`
  - `phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh` when `HAKO_PHASE2100_ENABLE_HV1=1`
  - `tools/exe_first_smoke.sh` when `SMOKES_ENABLE_SELFHOST=1` and LLVM18 exists
  - `tools/exe_first_runner_smoke.sh` when `SMOKES_ENABLE_SELFHOST=1` and LLVM18 exists
- `probe-llvmlite`
  - `phase2049/s3_link_run_llvmlite_map_set_size_canary_vm.sh`
  - `phase2049/s3_link_run_llvmlite_print_canary_vm.sh`
  - `phase2049/s3_link_run_llvmlite_ternary_collect_canary_vm.sh`
  - gated by `NYASH_LLVM_RUN_LLVMLITE=1` and LLVM18 availability
- `product-crate-exe`
  - `phase2100/s3_backend_selector_crate_exe_return42_canary_vm.sh`
  - `phase2100/s3_backend_selector_crate_exe_compare_eq_true_canary_vm.sh`
  - `phase2100/s3_backend_selector_crate_exe_binop_return_canary_vm.sh`
  - gated by `target/release/ny-llvmc` probe success
- `experimental-native`
  - `phase2120/native_backend_return42_canary_vm.sh`
  - `phase2120/native_backend_binop_add_canary_vm.sh`
  - `phase2120/native_backend_compare_eq_canary_vm.sh`
  - gated by `llc` presence
- `always-on/shared`
  - `phase2211/ssot_relative_unique_canary_vm.sh`

### Fixed reading

- `phase2100/run_all.sh` path stays as the public aggregator entry.
- `phase2100/run_all.sh` should shrink to a thin meta-runner, not disappear.
- bucket split is role-first:
  - selfhost
  - probe
  - product
  - experimental
  - shared

### Direct caller surface

- direct live caller pressure is low.
- current public references are mostly:
  - `docs/releases/21.0-full-selfhosting.md`
  - `tools/smokes/v2/README.md`
  - `tools/smokes/v2/run.sh`
- read as:
  - keep the existing `phase2100/run_all.sh` path
  - split the body behind that path first

## 32xC2 Plan

- target shape is one thin meta-runner plus role sub-runners in the same directory:
  - `phase2100/run_engineering_selfhost.sh`
  - `phase2100/run_probe_llvmlite.sh`
  - `phase2100/run_product_crate_exe.sh`
  - `phase2100/run_experimental_native.sh`
  - `phase2100/run_always_on_shared.sh`
- `run_all.sh` remains the stable public entry and only orchestrates:
  - quick/timeout guard
  - env gate summaries
  - role sub-runner dispatch
  - final done line
- exact implementation order:
  1. add the five role sub-runners without changing `run_all.sh` contract
  2. move exact smoke filters into the sub-runners
  3. reduce `run_all.sh` to guard + dispatch only
  4. keep direct caller path unchanged until `32xD` starts

## 32xC2 Result

- added role sub-runners:
  - `phase2100/run_engineering_selfhost.sh`
  - `phase2100/run_probe_llvmlite.sh`
  - `phase2100/run_product_crate_exe.sh`
  - `phase2100/run_experimental_native.sh`
  - `phase2100/run_always_on_shared.sh`
- `phase2100/run_all.sh` now keeps only:
  - quick/timeout guard
  - role sub-runner dispatch
  - final done line
- public path stayed the same:
  - direct callers still use `phase2100/run_all.sh`
- validation:
  - `bash -n` on `run_all.sh` plus the five sub-runners passed
  - `SMOKES_CURRENT_PROFILE=quick bash .../phase2100/run_all.sh` kept the expected quick skip
  - `bash .../phase2100/run_always_on_shared.sh` passed
  - `HAKO_PHASE2100_ENABLE_HV1=0 SMOKES_ENABLE_SELFHOST=0 bash .../phase2100/run_engineering_selfhost.sh` passed

## 32xD1 Result

### `bootstrap_selfhost_smoke.sh`

- live/public caller surface before drain:
  - `Makefile`
  - `docs/guides/selfhost-pilot.md`
  - `dev/selfhosting/README.md`
- historical/private refs remain in older phase/archive docs and are not rewritten in this slice.
- canonical home is now:
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
- old top-level path:
  - `tools/bootstrap_selfhost_smoke.sh`
  - reduced to a compatibility shim only

Read as:
- selfhost bootstrap smoke is no longer a top-level live owner.
- current/public callers now point to the selfhost home.

## 32xD2 Result

### `plugin_v2_smoke.sh`

- live/public caller surface before drain:
  - `src/runner/modes/common_util/plugin_guard.rs`
- current phase docs were also repointed as part of this slice.
- canonical home is now:
  - `tools/plugins/plugin_v2_smoke.sh`
- old top-level path:
  - `tools/plugin_v2_smoke.sh`
  - reduced to a compatibility shim only

Read as:
- plugin smoke is no longer a top-level live owner.
- plugin hint surface now points at the plugin home.

## 32xE1 Result

### `child.rs`

- shell residue is concentrated in:
  - `run_ny_program_capture_json_v0`
- current responsibilities inside that one function:
  - spawn `nyash --backend vm <program>`
  - apply selfhost compiler env
  - manage timeout loop
  - capture stdout/stderr via temp files
  - select first `program_line` / `mir_line` from stdout
- thin wrappers on top:
  - `run_ny_program_capture_json`
  - `run_ny_program_capture_mir_json`

Read as:
- `child.rs` still owns too much execution logic for a helper.
- first cut should move spawn/timeout/capture away before touching route names.

### `stage1_cli/core.hako`

- direct-route residue is concentrated in:
  - `run_program_json`
  - `_mode_run`
  - `_run_raw_request`
  - `_cmd_emit_mir_json`
- exact compat residue inside `run_program_json`:
  - `backend == null` defaults to `vm`
  - `llvm` is explicitly rejected
  - only `vm|pyvm` are accepted
  - vm/pyvm path prints MIR(JSON) and returns instead of owning execution
- exact raw compatibility residue inside `_run_raw_request`:
  - parses backend/source request
  - mutates `NYASH_SCRIPT_ARGS_JSON`
  - emits Program(JSON)
  - then calls `run_program_json`

Read as:
- `run_program_json` is the thickest raw compat branch.
- adding thread/runtime branching here would widen the compat matrix in the wrong layer.

### Thread-growth warning

- do not add thread branching to:
  - `run_program_json`
  - `_run_raw_request`
  - `_cmd_emit_mir_json`
- if thread support is needed later, it should enter below raw compat routing, not inside stage1 raw mode selection.

### First-cut takeaway for `32xE2`

- move execution responsibilities before route naming:
  1. spawn/timeout/capture out of `child.rs`
  2. keep raw line selection above the executor seam
  3. define a narrow `core_executor` entry for already-materialized MIR(JSON)

## 32xE2 Result

### `core_executor`

- `core_executor` now exposes a narrow direct-MIR seam:
  - `execute_mir_json_text(runner, json, source_label)`
- terminal execution remains:
  - `execute_loaded_mir_module(...)`
- generic artifact intake stays separate:
  - `execute_json_artifact(...)`
  - `json_artifact::load_json_artifact_to_module(...)`

Read as:
- `json_artifact` keeps family classification and compat lowering ownership.
- `core_executor` owns `MIR(JSON)` / `MirModule` execution, not `Program(JSON)` import routes.

### direct MIR caller repoint

- `runner/mod.rs` direct `--mir-json-file` path now delegates to:
  - `core_executor::execute_mir_json_text(...)`
- this removes one ad hoc direct-MIR parse/execute path from the runner body.

Read as:
- direct MIR entry is now routed through the same narrow owner as future core takeovers.
- `stage1_cli` and `child.rs` stay archive-later compat and were not widened.

## 32xF1 Result

| Helper | Disposition | Read as |
| --- | --- | --- |
| `tools/hako_check.sh` | keep here | shared analysis entry with live smoke and docs callers |
| `tools/hako_check/deadcode_smoke.sh` | family-home rehome | deadcode smoke belongs with the `tools/hako_check/**` family |
| `tools/hako_check_deadcode_smoke.sh` | shim-only | compatibility path kept while historical/current docs drain |
| `tools/hakorune_emit_mir.sh` | keep here | shared route/helper with broad live script/docs integration |

- `hako_check_deadcode_smoke.sh` moved to:
  - `tools/hako_check/deadcode_smoke.sh`
- live family caller updated:
  - `tools/hako_check_loopless_gate.sh`
- `hako_check.sh` stays top-level because live script/docs callers still treat it as the canonical analysis entry.
- `hakorune_emit_mir.sh` stays top-level because `emit_mir_route`, perf, selfhost docs, and runner diagnostics still integrate through that path.

## 32xG1 Result

- `src/cli/args.rs` still exposes:
  - backend help text: `vm (default), vm-hako (S0 frame), llvm, interpreter`
  - default value: `vm`
- `src/runner/dispatch.rs` still exposes:
  - routed tokens: `mir`, `vm`, `vm-hako`, `jit-direct`, `llvm`
  - unknown-backend hint: `vm`, `vm-hako`, `llvm`
- `lang/src/runner/stage1_cli/core.hako` still exposes raw compat semantics:
  - null backend defaults to `vm`
  - raw route accepts `vm|pyvm`
  - `llvm` remains rejected in the raw stage1 path

Read as:
- backend token/help/default surfaces are still intentionally inconsistent across CLI, dispatch, and stage1 raw compat.
- `phase-32x` closes with that mismatch explicitly deferred.
- any truthification of backend token/default/help must happen on a later lane that edits `args.rs`, `dispatch.rs`, and stage1 raw compat together.

## Delete / Archive Gate

- do not archive/delete `vm-rust` surfaces while mixed-owner source files still remain.
- do not delete `bootstrap_selfhost_smoke.sh` or `plugin_v2_smoke.sh` until caller drain is explicit.
- do not touch `args.rs` / `dispatch.rs` before the mixed-owner split tasks are complete.
