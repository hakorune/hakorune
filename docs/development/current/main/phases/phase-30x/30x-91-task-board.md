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
| 3 | `30xC rust-vm dependency inventory` | landed | internal `--backend vm` pressure by category |
| 4 | `30xD dangerous-early-flip lock` | landed | launcher/default/orchestrator denylist |
| 5 | `30xE user-facing main switch prep` | landed | README/help/examples move to `llvm/exe` first |
| 6 | `30xF backend default decision gate` | landed | keep raw default stable and finish ownership flip first |
| 7 | `30xG legacy disposition sweep` | active | archive/delete residual manual surfaces after main switch |

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
| 12 | `30xD2` | landed | selfhost/bootstrap freeze |
| 13 | `30xD3` | landed | plugin/orchestrator freeze |
| 14 | `30xE1` | landed | README/README.ja prep |
| 15 | `30xE2` | landed | CLI/help wording prep |
| 16 | `30xE3` | landed | stage1/runtime guide prep |
| 17 | `30xE4` | landed | remaining user-facing main switch prep |
| 18 | `30xF1` | landed | backend default flip is still blocked after 30xE |
| 19 | `30xF2` | landed | docs-only demotion is enough; raw token/default change stays later |
| 20 | `30xG1` | landed | low-blast manual smoke residues archived or reclassified |
| 21 | `30xG2` | landed | stale help snapshot replacement/archive |
| 22 | `30xG3` | active | compare/manual helper archive pass |
| 23 | `30xG4` | queued | post-switch docs cleanup |

## Evidence Commands

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
rg -n -- '--backend vm' src lang tools Makefile
rg -n 'rust-vm|vm-hako|llvm-exe|ny-llvm|ny-llvmc|compile-wasm|wasm-backend' \
  README.md README.ja.md docs/development/current/main docs/tools
rg -n 'selfhost_stage2_smoke|cross_backend_smoke|async_smokes|ny_stage1_asi|ny_stage3_bridge_accept|smoke_aot_vs_vm|nyash-help' \
  tools tests docs
```

## 30xF1 Result

- landed prerequisites:
  - `30xB`
  - `30xC`
  - `30xD`
  - `30xE`
- still-blocking default-flip surfaces:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/modes/common_util/selfhost/child.rs`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/selfhost_build.sh`
  - `tools/bootstrap_selfhost_smoke.sh`
  - `tools/plugin_v2_smoke.sh`
  - `tools/selfhost_smoke.sh`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- `30xF1` conclusion:
  - raw default flip is still blocked
  - active next is `30xF2`

## 30xF2 Decision

- phase-30x closes without a raw backend token/default flip
- fixed reading:
  - `llvm/exe` is product by docs/artifact/smoke ownership
  - `rust-vm` remains explicit engineering(stage0/bootstrap + tooling keep)
  - `vm-hako` remains reference
  - `wasm` remains experimental
- follow-up rule:
  - any raw change to `src/cli/args.rs` or `src/runner/dispatch.rs` is a later gate, not part of this phase
- active next is `30xG3`

## 30xG1 Result

- archived in `tools/archive/manual-smokes/`:
  - `ny_stage1_asi_smoke.sh`
  - `ny_stage3_bridge_accept_smoke.sh`
  - `async_smokes.sh`
  - `cross_backend_smoke.sh`
  - `selfhost_stage2_smoke.sh`
- explicit keep:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
- active next:
  - `30xG3`

## 30xG2 Result

- `docs/tools/nyash-help.md` is now a thin current stub
- historical capture moved to `docs/archive/tools/nyash-help.md`
- current CLI truth stays on:
  - `docs/tools/cli-options.md`
  - `src/cli/args.rs`
- active next:
  - `30xG3`

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
  - `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
  - `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
  - `tools/archive/manual-smokes/async_smokes.sh`

Plugin/macro/tooling archive/delete result:

- archived in `30xG1`:
  - `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
  - `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
  - `tools/archive/manual-smokes/async_smokes.sh`

### Smoke / test keep vs watch

- keep:
  - `tools/selfhost_smoke.sh`
  - `tools/selfhost_vm_smoke.sh`
  - `tools/selfhost_stage3_accept_smoke.sh`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
  - `tools/smoke_aot_vs_vm.sh`
- watch:
  - `tools/archive/manual-smokes/cross_backend_smoke.sh`
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
  - `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`

Smoke/test archive/delete result:

- archived in `30xG1`:
  - `tools/archive/manual-smokes/cross_backend_smoke.sh`
  - `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`
- explicit keep:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`

### Plugin / smoke orchestrator freeze

- explicit no-touch-first keep:
  - `tools/bootstrap_selfhost_smoke.sh`
  - `tools/plugin_v2_smoke.sh`
  - `tools/selfhost_smoke.sh`
  - `tools/selfhost_vm_smoke.sh`
  - `tools/selfhost_stage3_accept_smoke.sh`
  - `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
- keep until `30xE/G` clarifies wording:
  - `tools/smoke_aot_vs_vm.sh`
- archive-later queue:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
  - `tools/smoke_aot_vs_vm.sh`

Plugin/smoke orchestrator freeze result:

- `30xD3` is landed as docs-first only
- no-touch-first orchestrators stay live engineering keeps
- low-blast manual residues moved to `tools/archive/manual-smokes/` in `30xG1`
- remaining residue review is `run_spec_smoke.sh` keep plus `smoke_aot_vs_vm.sh`/`nyash-help.md` follow-up

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
- archive/stub:
  - `docs/tools/nyash-help.md`
  - `docs/archive/tools/nyash-help.md`

Docs/help archive/delete result:

- landed in `30xG2`:
  - `docs/tools/nyash-help.md` reduced to a thin current stub
  - `docs/archive/tools/nyash-help.md` now carries the historical capture

### Rewrite in `30xE`

- `README.md`
- `README.ja.md`
- `docs/development/selfhosting/quickstart.md`
- `docs/guides/selfhost-pilot.md`

### Stale wording archived or replaced

- `docs/tools/nyash-help.md`
  - now points to current CLI truth and archive path

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

### Selfhost / bootstrap freeze

- `src/runner/modes/common_util/selfhost/child.rs`
  - child capture route stays hard-wired to `--backend vm`
- `lang/src/runner/stage1_cli/core.hako`
  - raw stage1 compat lane still accepts `vm|pyvm`
- `tools/selfhost/run.sh`
  - unified selfhost wrapper still shells through `--backend vm`
- `tools/selfhost/selfhost_build.sh`
  - build wrapper still shells through `--backend vm`

Selfhost/bootstrap freeze result:

- `30xD2` is docs-first only
- no code edits in this slice
- selfhost/bootstrap wrapper changes are explicitly deferred until after `30xE` and `30xF`

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

## Legacy Disposition Queue

- archived in `30xG1`:
  - `tools/archive/manual-smokes/ny_stage1_asi_smoke.sh`
  - `tools/archive/manual-smokes/ny_stage3_bridge_accept_smoke.sh`
  - `tools/archive/manual-smokes/async_smokes.sh`
  - `tools/archive/manual-smokes/cross_backend_smoke.sh`
  - `tools/archive/manual-smokes/selfhost_stage2_smoke.sh`
- explicit keep:
  - `tests/nyash_syntax_torture_20250916/run_spec_smoke.sh`
- explicit recheck before archive/delete:
  - `tools/smoke_aot_vs_vm.sh`
- current archive blockers to clear in `30xE`:
  - `README.md` / `README.ja.md` still point at `tools/selfhost_vm_smoke.sh` and `tools/smoke_aot_vs_vm.sh`
  - `docs/development/selfhosting/quickstart.md` still points at `tools/selfhost_smoke.sh`
  - `docs/guides/selfhost-pilot.md` still points at `tools/bootstrap_selfhost_smoke.sh`
  - `docs/guides/exceptions-stage3.md` still points at `tools/selfhost_stage3_accept_smoke.sh`
  - `docs/releases/21.0-full-selfhosting.md` still points at `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`
  - `tools/smokes/jit-migration-plan.md` is historical and points at archived paths
- delete-ready now:
  - `none`

## Current Exact Next

1. `30xG3` compare/manual helper archive pass
2. `30xG4` post-switch docs cleanup

## Exit Condition For Phase Entry

- root docs read `phase-30x backend surface simplification` as the active lane
- role-first taxonomy is explicit in mirrors and child SSOTs
- `phase-29x` is read as landed precursor, not the current optimization front
