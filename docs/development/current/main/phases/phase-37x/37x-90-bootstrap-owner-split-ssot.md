---
Status: SSOT
Decision: provisional
Date: 2026-04-03
Scope: bootstrap/product owner split を speed-first で進める順番と explicit keep を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-37x/README.md
  - docs/development/current/main/phases/phase-37x/37x-91-task-board.md
  - docs/development/current/main/phases/phase-36x/README.md
---

# 37x-90 Bootstrap Owner Split

## Goal

- bootstrap lane の「`hakorune` binary は使っているが execution owner がまだ `vm`」という混線を、まず `selfhost_build.sh` と `build.rs` から切る。
- `rust-vm` を別 runtime として育てるのではなく、`engineering(stage0/bootstrap + oracle)` 側に閉じる準備を進める。

## Fast Reading

- first target:
  - `tools/selfhost/selfhost_build.sh`
  - `src/runner/build.rs`
- explicit keep first:
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `tools/selfhost/selfhost_vm_smoke.sh`
- defer:
  - `src/runner/modes/common_util/selfhost/child.rs` caller drain
  - raw backend default/token rewrite

## Fixed Rules

- split owner first, then drain callers
- explicit engineering keep stays explicit; do not prematurely “clean” it away
- direct MIR / core-direct / `ny-llvmc` artifact lanes are the preferred successor lanes
- stage1 direct is a successor lane
- Stage-A / raw compat / shell residue must not gain new runtime capability
- post-`37xD1`, cleanup/archive sweep comes before any raw default/token rewrite

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `37xA selfhost_build owner split` | active | `selfhost_build.sh` を producer / direct-run / exe-artifact / dispatcher に切る | shell script が lane owner ごとに読める |
| `37xB build.rs owner split` | landed | `build.rs` を product build / engineering build に切る | source owner が path/function で読める |
| `37xC explicit keep freeze + drain map` | active | explicit engineering keep と next drain を固定 | `vm必須 keep` と `next caller drain` が混ざらない |
| `37xD proof/closeout` | queued | speed-first split を canonical proof に戻す | next phase が `child.rs` drain に集中できる |
| `post-37x cleanup/archive sweep` | queued-next | drained shim / legacy embedded smoke / stale compat wrapper を live surface から外す | archive/delete 対象が proof 後の state で読める |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `37xA1` | landed | Stage-B producer isolation | `emit_stageb_program_json_raw` family が一つの owner に見える |
| `37xA2` | landed | direct MIR / core-direct split | `emit_mir_json_from_source` と `run_program_json_v0_via_core_direct` が thin selector の下に分かれる |
| `37xA3` | landed | `ny-llvmc` / exe artifact split | EXE path が MIR->EXE artifact owner として読める |
| `37xA4` | landed | dispatcher slimming | primary/downstream dispatcher が lane router に縮む |
| `37xB1` | landed | `build.rs` shared prelude freeze | shared config/env/app/link prelude を no-touch-first で固定 |
| `37xB2` | landed | product build wrapper split | `build_core(..., llvm)` + `emit_llvm_object(...)` が product owner に寄る |
| `37xB3` | landed | engineering build wrapper split | `build_core(..., cranelift)` + `emit_engineering_object(...)` が engineering owner に寄る |
| `37xC1` | landed | explicit keep freeze | bootstrap vm keep scripts を “残すもの” として先に固定する |
| `37xC2` | active | child.rs caller drain map | owner split 後に減らす caller を exact にする |
| `37xD1` | queued | proof/closeout | canonical smoke / evidence command を戻して handoff |

## Current Focus

- active macro wave: `37xA selfhost_build owner split`
- active micro task: `37xC2 child.rs caller drain map`
- next queued micro task: `37xD1 proof/closeout`
- current blocker: `none`
- exact reading:
  - `selfhost_build.sh` is the biggest mixed-owner shell surface
  - `build.rs` is the biggest mixed-owner source surface
  - explicit engineering keep is frozen as:
    - `bootstrap_selfhost_smoke.sh`
    - `run_stageb_compiler_vm.sh`
    - `selfhost_vm_smoke.sh`
  - these keeps are not first-cut cleanup targets
  - temporary smoke red is acceptable during `37xA` / `37xB` / `37xC` if owner split moves forward and compile/diff checks stay green
  - after `37xD1`, cleanup/archive sweep targets drained shims, legacy embedded Stage1 smoke, and stale compat wrappers first
