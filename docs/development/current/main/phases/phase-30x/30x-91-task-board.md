---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: `phase-30x backend surface simplification` の concrete task order と evidence command をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-30x/README.md
  - docs/development/current/main/phases/phase-30x/30x-90-backend-surface-simplification-ssot.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/artifact-policy-ssot.md
---

# 30x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `30xA role taxonomy lock` | landed | role labels and active lane wording |
| 2 | `30xB smoke taxonomy split` | landed | role-first smoke buckets and suite reading |
| 3 | `30xC rust-vm dependency inventory` | active | internal `--backend vm` pressure by category |
| 4 | `30xD dangerous-early-flip lock` | queued | launcher/default/orchestrator denylist |
| 5 | `30xE user-facing main switch prep` | queued | README/help/examples move to `llvm/exe` first |
| 6 | `30xF backend default decision gate` | queued | decide raw CLI default only after the above |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `30xA1` | landed | root mirrors use `product / engineering / reference / experimental` |
| 2 | `30xA2` | landed | design role SSOT alignment |
| 3 | `30xB1` | landed | `vm-hako` reference smoke lock |
| 4 | `30xB2` | landed | `wasm` experimental smoke lock |
| 5 | `30xB3` | landed | `llvm/exe` product vs `llvmlite` probe boundary lock |
| 6 | `30xB4` | landed | smoke matrix/guide cleanup |
| 7 | `30xC1` | landed | `rust-vm` bootstrap/selfhost pressure |
| 8 | `30xC2` | landed | `rust-vm` plugin/macro/tooling pressure |
| 9 | `30xC3` | landed | `rust-vm` smoke/test pressure |
| 10 | `30xC4` | landed | `rust-vm` docs/help pressure |
| 11 | `30xD1` | landed | default/dispatch do-not-flip-early lock |
| 12 | `30xD2` | active | selfhost/bootstrap freeze |
| 13 | `30xD3` | queued | plugin/orchestrator freeze |
| 14 | `30xE1-30xE4` | queued | user-facing main switch prep |
| 15 | `30xF1-30xF2` | queued | backend default decision last |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
rg -n -- '--backend vm' src lang tools Makefile
rg -n 'rust-vm|vm-hako|llvm-exe|ny-llvm|ny-llvmc|compile-wasm|wasm-backend' \
  README.md README.ja.md docs/development/current/main docs/tools
```

## Role Touchpoints

### Product

- `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`
- `docs/development/current/main/design/artifact-policy-ssot.md`
- `docs/development/current/main/05-Restart-Quick-Resume.md`

### Engineering

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/modes/common_util/selfhost/child.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run.sh`
- `tools/plugin_v2_smoke.sh`
- `tools/bootstrap_selfhost_smoke.sh`
- `tools/selfhost_smoke.sh`
- `src/macro/macro_box_ny.rs`

### Plugin / macro / tooling keep vs watch

- keep:
  - `src/macro/macro_box_ny.rs`
  - `tools/bootstrap_selfhost_smoke.sh`
  - `tools/plugin_v2_smoke.sh`
  - `tools/run_vm_stats.sh`
  - `tools/hako_check.sh`
  - `tools/hako_check_deadcode_smoke.sh`
  - `tools/hakorune_emit_mir.sh`
  - `tools/parity.sh`
- watch:
  - `tools/ny_stage1_asi_smoke.sh`
  - `tools/ny_stage3_bridge_accept_smoke.sh`
  - `tools/async_smokes.sh`

Plugin/macro/tooling archive/delete result:

- `none`
- hard delete/archive is blocked in `30xC2`; `30xD` and `30xE` must land first

### Smoke / test keep vs watch

- keep:
  - `tools/selfhost_smoke.sh`
  - `tools/selfhost_vm_smoke.sh`
  - `tools/selfhost_stage3_accept_smoke.sh`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
  - `tools/smoke_aot_vs_vm.sh`
- watch:
  - `tools/cross_backend_smoke.sh`
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
  - `tools/selfhost_stage2_smoke.sh`

Smoke/test archive/delete result:

- `none`
- hard delete/archive is blocked in `30xC3`; `30xD` and `30xE` must land first

### Docs / help keep vs rewrite vs watch

- rewrite in `30xE`:
  - `README.md`
  - `README.ja.md`
  - `docs/development/selfhosting/quickstart.md`
  - `docs/guides/selfhost-pilot.md`
- keep as engineering docs:
  - `docs/tools/cli-options.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
  - `docs/guides/testing-guide.md`
- watch:
  - `docs/tools/nyash-help.md`

Docs/help archive/delete result:

- `none`
- root README/help rewrites belong to `30xE`; stale help snapshot stays watch-only until replacement exists

### Default / dispatch freeze

- `src/cli/args.rs`
  - raw backend token/default stays `vm` for now
  - no early flip before `30xF`
- `src/runner/dispatch.rs`
  - central dispatch still carries `vm`, `vm-hako`, and `llvm`
  - no early route rewrite before `30xF`

Default/dispatch freeze result:

- `30xD1` is docs-first only
- no code edits in this slice
- raw token/default change is explicitly deferred to `30xF`

### Bootstrap/selfhost keep details

- `src/cli/args.rs`
  - raw default token remains `vm`
- `src/runner/dispatch.rs`
  - backend selector still carries `vm`, `vm-hako`, `llvm`
- `src/runner/modes/common_util/selfhost/child.rs`
  - child capture route is explicit `--backend vm`
- `lang/src/runner/stage1_cli/core.hako`
  - raw stage1 compat route still accepts `vm|pyvm`
- `tools/selfhost/run.sh`
  - runtime/direct selfhost paths still force `--backend vm`
- `tools/selfhost/selfhost_build.sh`
  - build wrappers still use `--backend vm`
- `tools/selfhost/run_stageb_compiler_vm.sh`
  - shared Stage-B compiler route is explicit Rust VM keep
- `Makefile`
  - `run-minimal` still uses `--backend vm`

Bootstrap/selfhost archive/delete result:

- `none`
- all current hits are live bootstrap/selfhost pressure and stay keep surfaces in `30xC1`

### Reference

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md`
- `docs/development/current/main/design/artifact-policy-ssot.md`

### Experimental

- `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/README.md`
- `tools/smokes/v2/lib/wasm_g3_contract.sh`
- `tools/smokes/v2/configs/matrix.conf`

### Compat / probe keep

- `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/README.md`
- `tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt`
- `docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md`

## Do-Not-Flip-Early Set

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

## Exit Condition For Phase Entry

- root docs read `phase-30x backend surface simplification` as the active lane
- role-first taxonomy is explicit in mirrors and child SSOTs
- `phase-29x` is read as landed precursor, not the current optimization front
