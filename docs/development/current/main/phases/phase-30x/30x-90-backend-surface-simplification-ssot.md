---
Status: SSOT
Decision: provisional
Date: 2026-04-02
Scope: backend surface simplification の role taxonomy、fixed order、dangerous early flips を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-30x/README.md
  - docs/development/current/main/phases/phase-30x/30x-91-task-board.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
---

# 30x-90 Backend Surface Simplification

## Goal

- backend を `product / engineering / reference / experimental` の 4 役に固定する。
- `llvm/exe` を user-facing main に寄せる。
- `rust-vm` は bootstrap/recovery/compat lane として explicit keep にする。
- `vm-hako` は reference/conformance lane として残し、mainline と混ぜない。
- `wasm` は experimental target として扱い、promotion 判定を分離する。

## Fixed Role Taxonomy

| Surface | Role | Fixed reading |
| --- | --- | --- |
| `llvm/exe` / `ny-llvm` / `ny-llvmc` | `product` | daily mainline / CI / distribution target |
| `rust-vm` (`--backend vm`) | `engineering` | bootstrap / recovery / compat lane |
| `vm-hako` | `reference` | semantic witness / conformance / debug |
| `wasm` / `--compile-wasm` | `experimental` | feature-gated compile target |

## Fixed Rules

- `rust-vm` を phase 冒頭で剥がさない。
- raw CLI backend token/default は `30xF` まで変えない。
- `vm-hako` は reference lane のままにし、co-main にしない。
- `wasm` は experimental のままにし、promotion は別 gate を要求する。
- selfhost/bootstrap/plugin/macro/smoke orchestration の `--backend vm` 直打ちは inventory 後にしか触らない。

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `30xA role taxonomy lock` | landed | docs と mirrors の backend role labels を揃える | active lane と role-first reading が root docs で一致する |
| `30xB smoke taxonomy split` | active | smoke を `product / engineering / reference / experimental` の見え方へ寄せる | role-first buckets と suites の方針が固定される |
| `30xC rust-vm dependency inventory` | queued | internal `--backend vm` pressure を category ごとに固定する | bootstrap/selfhost/plugin/macro/smoke/doc の pressure map が揃う |
| `30xD dangerous-early-flip lock` | queued | 先に変えると壊れる launcher/default/orchestrator を固定する | early-flip denylist が task board で explicit |
| `30xE user-facing main switch prep` | queued | README/help/examples を `llvm/exe` first に寄せる準備をする | default を変えずに main narrative だけ切り替える差分範囲が固まる |
| `30xF backend default decision gate` | queued | CLI default/backend flip の可否を最後に判定する | taxonomy、smoke split、dependency inventory が landed している |

## Micro Tasks

### `30xA` role taxonomy lock

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xA1` | landed | root mirrors use the same role-first labels | `CURRENT_TASK`, `05`, `10`, and `15` read `product / engineering / reference / experimental` |
| `30xA2` | landed | design role SSOT alignment | `artifact-policy` and `execution-lanes` agree on the same four-role reading |

### `30xB` smoke taxonomy split

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xB1` | landed | reference smoke lock | `vm-hako` suites/readmes read as `reference`, not active mainline |
| `30xB2` | landed | experimental smoke lock | `wasm` suites/readmes read as `experimental`, not co-main |
| `30xB3` | landed | product/probe boundary lock | `llvm/exe` product lane and `llvmlite` compat/probe keep are not mixed |
| `30xB4` | landed | matrix/guide cleanup | smoke discovery docs and matrix config use the same role-first reading |

### `30xC` rust-vm dependency inventory

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xC1` | active | bootstrap/selfhost inventory | launcher, stage1, selfhost wrappers are grouped explicitly |
| `30xC2` | queued | plugin/macro/tooling inventory | macro child, plugin smoke, bridge accept, parity tools are grouped explicitly |
| `30xC3` | queued | smoke/test inventory | vm-backed smoke/test orchestrators are listed separately from product/reference suites |
| `30xC4` | queued | docs/help inventory | README/help/guides that still center `--backend vm` are explicit |

### `30xD` dangerous-early-flip lock

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xD1` | queued | default/dispatch freeze | CLI default and central dispatch are marked `do not flip early` |
| `30xD2` | queued | selfhost/bootstrap freeze | selfhost/stage1 wrappers and scripts are explicit no-touch-first surfaces |
| `30xD3` | queued | plugin/smoke orchestrator freeze | plugin and smoke orchestrators are explicit no-touch-first surfaces |

### `30xE` user-facing main switch prep

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xE1` | queued | README/README.ja prep | product main reads `llvm/exe` first while `rust-vm` stays engineering keep |
| `30xE2` | queued | CLI/help wording prep | `docs/tools/*` stop reading `vm` as the main narrative |
| `30xE3` | queued | stage1/runtime guide prep | runtime/stage1 guides stop implying `rust-vm` is the product main |
| `30xE4` | queued | vm-hako/wasm wording prep | `vm-hako` stays reference and `wasm` stays experimental in user-facing docs |

### `30xF` backend default decision gate

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `30xF1` | queued | backend default gate checklist | raw default flip is blocked until `30xB-30xE` are landed |
| `30xF2` | queued | backend token/default decision | decide whether docs-only demotion is enough or a raw default flip is justified |

## Current Focus

- active macro wave: `30xC rust-vm dependency inventory`
- next queued wave: `30xD dangerous-early-flip lock`
- current blocker: `none`
- predecessor lane: `phase-29x backend owner cutover prep` is landed enough and no longer the active docs front

## Internal Pressure Buckets

### Bootstrap / selfhost

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/modes/common_util/selfhost/child.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `lang/src/runner/stage1_cli/config.hako`
- `lang/src/runner/stage1_cli/raw_subcommand_input.hako`
- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `Makefile`

### Plugin / macro / dev tooling

- `src/macro/macro_box_ny.rs`
- `tools/bootstrap_selfhost_smoke.sh`
- `tools/plugin_v2_smoke.sh`
- `tools/ny_stage1_asi_smoke.sh`
- `tools/ny_stage3_bridge_accept_smoke.sh`
- `tools/run_vm_stats.sh`
- `tools/parity.sh`
- `tools/hako_check.sh`
- `tools/hako_check_deadcode_smoke.sh`
- `tools/async_smokes.sh`
- `tools/hakorune_emit_mir.sh`

### Smoke / test

- `tools/selfhost_smoke.sh`
- `tools/cross_backend_smoke.sh`
- `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
- `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`

### Docs / help / taxonomy

- `README.md`
- `README.ja.md`
- `docs/tools/cli-options.md`
- `docs/tools/nyash-help.md`
- `docs/development/runtime/cli-hakorune-stage1.md`
- `docs/guides/testing-guide.md`

## Dangerous Early Flips

Do not change these before `30xB-30xD` land.

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/modes/common_util/selfhost/child.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run.sh`
- `tools/selfhost/selfhost_build.sh`
- `tools/bootstrap_selfhost_smoke.sh`
- `tools/plugin_v2_smoke.sh`
- `tools/selfhost_smoke.sh`
- `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`

## Worker Re-Inventory Notes

- keep the docs label `rust-vm`; do not introduce `vm-rust` as the primary docs label in this phase
- `vm-hako` already has an explicit reference/conformance smoke home:
  - `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md`
- `wasm` already has an explicit experimental smoke/tooling home:
  - `tools/smokes/v2/profiles/integration/phase29cc_wsm/README.md`
  - `tools/smokes/v2/lib/wasm_g3_contract.sh`
- current docs/help still over-read `--backend vm` in:
  - `README.md`
  - `README.ja.md`
  - `docs/tools/cli-options.md`
  - `docs/tools/nyash-help.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
  - `docs/guides/testing-guide.md`

## Exact Read Order

1. `docs/development/current/main/phases/phase-30x/README.md`
2. `docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md`
3. `docs/development/current/main/phases/phase-30x/30x-91-task-board.md`
4. `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
5. `docs/development/current/main/design/artifact-policy-ssot.md`
6. `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`
