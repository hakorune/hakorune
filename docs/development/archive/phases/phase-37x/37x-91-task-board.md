---
Status: Active
Decision: provisional
Date: 2026-04-03
Scope: `phase-37x bootstrap owner split` の concrete queue と speed-first acceptance をまとめる。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-37x/README.md
  - docs/development/current/main/phases/phase-37x/37x-90-bootstrap-owner-split-ssot.md
---

# 37x-91 Task Board

## Current Queue

| Order | Task | Status | Read as |
| --- | --- | --- | --- |
| 1 | `37xA selfhost_build owner split` | landed | shell owner split を最優先で取る |
| 2 | `37xB build.rs owner split` | landed | source owner split を product/engineering に切る |
| 3 | `37xC explicit keep freeze + drain map` | landed | cleanup しない keep 面と次 drain を分ける |
| 4 | `37xD proof/closeout` | landed | canonical evidence を戻して handoff する |
| 5 | `phase-39x stage0 vm gate thinning` | current | stage0/bootstrap lane の `--backend vm` 残面を inventory して direct route と keep gate を分ける |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `37xA1` | landed | Stage-B producer isolation |
| 2 | `37xA2` | landed | direct MIR / core-direct split |
| 3 | `37xA3` | landed | `ny-llvmc` / exe artifact split |
| 4 | `37xA4` | landed | dispatcher slimming |
| 5 | `37xB1` | landed | `build.rs` shared prelude freeze |
| 6 | `37xB2` | landed | product build wrapper split |
| 7 | `37xB3` | landed | engineering build wrapper split |
| 8 | `37xC1` | landed | explicit keep freeze |
| 9 | `37xC2` | landed | child.rs caller drain map |
| 10 | `37xD1` | landed | proof/closeout |

## Speed-First Acceptance

- during `37xA` / `37xB` / `37xC`, temporary smoke red is acceptable
- required green checks during split:

```bash
cd /home/tomoaki/git/hakorune-selfhost
cargo check --bin hakorune
git diff --check
```

- focused smoke / proof restoration is owned by `37xD1`
- cleanup/archive sweep starts only after `37xD1` evidence is back in place

## Exact Keeps

- explicit engineering keep:
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
- no-touch-first:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - raw backend default/token rewrite

## Current Result

- current front:
  - `phase-39x stage0 vm gate thinning`
- exact next:
  - move drained shims, legacy embedded smoke, and stale compat wrappers out of the live surface
- explicit reading:
  - first speed gain comes from making mixed owner surfaces readable
  - not from deleting `vm.rs`
  - not from flipping raw backend defaults
  - `37xD1` proof is the focused set:
    - `cargo check --bin hakorune`
    - `git diff --check`
    - `bash tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`
    - `bash tools/selfhost/stage1_mainline_smoke.sh --bin target/selfhost/hakorune.stage1_cli.stage2 apps/tests/hello_simple_llvm.hako`
  - `bash tools/smokes/v2/profiles/integration/selfhost/selfhost_minimal.sh` is inherited Stage-B source-route red (`Undefined variable: StageBMod`) and is not the helper-local acceptance line for this phase
  - after `37xD1`, the next cleanup lane is archive/delete of drained shims, legacy embedded smoke, and stale compat wrappers
