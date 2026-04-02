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
| 2 | `30xB smoke taxonomy split` | active | role-first smoke buckets and suite reading |
| 3 | `30xC rust-vm dependency inventory` | queued | internal `--backend vm` pressure by category |
| 4 | `30xD dangerous-early-flip lock` | queued | launcher/default/orchestrator denylist |
| 5 | `30xE user-facing main switch prep` | queued | README/help/examples move to `llvm/exe` first |
| 6 | `30xF backend default decision gate` | queued | decide raw CLI default only after the above |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `30xA1` | landed | root mirrors use `product / engineering / reference / experimental` |
| 2 | `30xA2` | landed | design role SSOT alignment |
| 3 | `30xB1` | landed | `vm-hako` reference smoke lock |
| 4 | `30xB2` | active | `wasm` experimental smoke lock |
| 5 | `30xB3` | queued | `llvm/exe` product vs `llvmlite` probe boundary lock |
| 6 | `30xB4` | queued | smoke matrix/guide cleanup |
| 7 | `30xC1-30xC4` | queued | `rust-vm` pressure by category |
| 8 | `30xD1-30xD3` | queued | do-not-flip-early lock |
| 9 | `30xE1-30xE4` | queued | user-facing main switch prep |
| 10 | `30xF1-30xF2` | queued | backend default decision last |

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

### Reference

- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md`
- `docs/development/current/main/design/artifact-policy-ssot.md`

### Experimental

- `docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md`
- `tools/smokes/v2/profiles/integration/phase29cc_wsm/README.md`
- `tools/smokes/v2/lib/wasm_g3_contract.sh`
- `tools/smokes/v2/configs/matrix.conf`

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
