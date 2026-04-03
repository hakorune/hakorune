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
| 1 | `37xA selfhost_build owner split` | active | shell owner split を最優先で取る |
| 2 | `37xB build.rs owner split` | queued | source owner split を product/engineering に切る |
| 3 | `37xC explicit keep freeze + drain map` | queued | cleanup しない keep 面と次 drain を分ける |
| 4 | `37xD proof/closeout` | queued | canonical evidence を戻して handoff する |

## Ordered Slice Detail

| Order | Slice | Status | Read as |
| --- | --- | --- | --- |
| 1 | `37xA1` | active | Stage-B producer isolation |
| 2 | `37xA2` | queued | direct MIR / core-direct split |
| 3 | `37xA3` | queued | `ny-llvmc` / exe artifact split |
| 4 | `37xA4` | queued | dispatcher slimming |
| 5 | `37xB1` | queued | `build.rs` shared prelude freeze |
| 6 | `37xB2` | queued | product build wrapper split |
| 7 | `37xB3` | queued | engineering build wrapper split |
| 8 | `37xC1` | queued | explicit keep freeze |
| 9 | `37xC2` | queued | child.rs caller drain map |
| 10 | `37xD1` | queued | proof/closeout |

## Speed-First Acceptance

- during `37xA` / `37xB`, temporary smoke red is acceptable
- required green checks during split:

```bash
cd /home/tomoaki/git/hakorune-selfhost
cargo check --bin hakorune
git diff --check
```

- canonical smoke / proof restoration is owned by `37xD1`

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
  - `37xA1 Stage-B producer isolation`
- exact next:
  - split `selfhost_build.sh` around Stage-B producer ownership before touching direct-run and EXE lanes
- explicit reading:
  - first speed gain comes from making mixed owner surfaces readable
  - not from deleting `vm.rs`
  - not from flipping raw backend defaults
