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
| `32xC phase2100 role split plan` | active | mixed aggregator を role buckets へ切る | selfhost / probe / product / experimental / shared の sub-runner 形が固定される |
| `32xD top-level orchestrator rehome prep` | queued | `bootstrap_selfhost` / `plugin_v2` の caller drain を固定する | top-level keep surfaces の canonical next home が読める |
| `32xE direct-route takeover prep` | queued | child/stage1 shell residues を core route へ寄せる準備をする | `core_executor` takeover seam と direct shell gap が固定される |
| `32xF shared helper follow-up gate` | queued | helper family を別 phase へ回す gate を決める | shared helpers are either explicit keep or reopened under a dedicated phase |
| `32xG raw default/token gate` | deferred | default/token rewrite の可否を最後に判定する | source split 後まで `args.rs` / `dispatch.rs` が untouched のまま保たれる |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `32xA1` | landed | `build.rs` mixed ownership inventory | object emit / feature build / link flow の product vs engineering 同居が exact に読める |
| `32xA2` | landed | `phase2100` mixed aggregator inventory | selfhost / llvmlite probe / crate product / native experimental reps が exact に読める |
| `32xB1` | landed | `build.rs` split target lock | product build owner, engineering build owner, shared prelude が docs で分かれる |
| `32xB2` | landed | `build.rs` implementation slice order | helper-first / owner-split / caller-preserve の順が固定される |
| `32xC1` | landed | `phase2100` role bucket lock | selfhost / probe / product / experimental / shared bucket と exact script set が固定される |
| `32xC2` | active | `phase2100` thin meta-runner plan | top-level aggregator が meta-runner only に縮む計画を固定する |
| `32xD1` | queued | `bootstrap_selfhost_smoke.sh` caller drain map | rehome blocker と canonical future home が読める |
| `32xD2` | queued | `plugin_v2_smoke.sh` caller drain map | rehome blocker と canonical future home が読める |
| `32xE1` | queued | `child.rs` / `stage1_cli` direct-route gap inventory | direct `--backend vm` shell residues の exact gap が読める |
| `32xE2` | queued | `core_executor` takeover seam lock | direct MIR/core route に寄せる seam が固定される |
| `32xF1` | queued | shared helper follow-up gate | `hako_check*` / `hakorune_emit_mir.sh` は dedicated helper phase まで keep のままと固定する |
| `32xG1` | deferred | raw backend default/token decision remains last | `args.rs` / `dispatch.rs` are still do-not-flip-early |

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

- active macro wave: `32xC phase2100 role split plan`
- active micro task: `32xC2 phase2100 thin meta-runner plan`
- next queued micro task: `32xD1 bootstrap_selfhost_smoke.sh caller drain map`
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

## Delete / Archive Gate

- do not archive/delete `vm-rust` surfaces while mixed-owner source files still remain.
- do not delete `bootstrap_selfhost_smoke.sh` or `plugin_v2_smoke.sh` until caller drain is explicit.
- do not touch `args.rs` / `dispatch.rs` before the mixed-owner split tasks are complete.
