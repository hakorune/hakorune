---
Status: Active
Date: 2026-04-03
Scope: stage0/bootstrap lane の remaining direct/core route ownership を harden し、proof-only VM gates と compat keep を固定する。
---

# 41x-90 Stage0 Direct/Core Route Hardening SSOT

## Plain Reading

- problem: `rust-vm` is still a feature tax whenever a live bootstrap route keeps owning `--backend vm`
- target: move stage0/bootstrap mainline ownership toward `hakorune` binary direct/core routes and keep vm routes proof-only
- result: vm routes become `proof-only keep`, `compat keep`, or `archive-later`, instead of staying broad execution owners
- `40xD1` landed the archive sweep closeout; `41x` starts the hardening wave after the obvious shims are gone

## Success / Failure Rails

### Success

- keep only a small proof-only VM gate set
- keep `selfhost_build.sh` / `run.sh` as direct/core facades, not feature owners
- stop new features from landing on `--backend vm`, stage1 compat, or raw routes
- shrink `vm.rs` only after caller drain proves it is no longer a broad owner

### Failure

- selfhost/bootstrap mainline still runs through `--backend vm`
- stage1 compat or raw routes absorb new capability work
- proof-only VM gates drift back into day-to-day mainline

## Macro Reading

| Wave | Status | Read as |
| --- | --- | --- |
| `41xA direct/core route inventory` | landed | remaining direct/core route facades and caller families are inventoried |
| `41xB route hardening` | active | `selfhost_build.sh` / `run.sh` are hardened into direct/core-first facades |
| `41xC vm keep shrink` | queued | `vm.rs` is shrunk toward proof/oracle keep after caller drain |
| `41xD closeout` | queued | `rust-vm` is handed off as proof/compat keep rather than mainline ownership |

## Candidate Reading

| Path | State | Reading |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh` | route facade | Stage-B producer / direct MIR / EXE artifact / dispatcher are already split out; harden the route, do not grow vm-only branches |
| `tools/selfhost/run.sh` | outer facade | direct/core entry, but still a live facade that should not absorb new feature work |
| `tools/selfhost/run_stageb_compiler_vm.sh` | proof-only keep | explicit Stage-B VM gate; freeze and do not grow |
| `tools/selfhost/selfhost_vm_smoke.sh` | proof-only keep | VM parity proof only |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | proof-only keep | stage3 acceptance proof only until direct coverage replaces it |
| `lang/src/runner/stage1_cli/core.hako` | compat keep | raw compat lane; no new capability work |
| `src/runner/modes/vm.rs` | engineering keep | stage0/oracle keep until route hardening proves otherwise |
| `src/runner/core_executor.rs` | direct owner | already-materialized MIR(JSON) execution owner |
| `src/runner/build.rs` | settled split | already split into product/engineering helpers; do not reopen unless a new caller demands it |
| `tools/selfhost/bootstrap_selfhost_smoke.sh` | canonical proof home | live caller target after the top-level shim deletion |
| `tools/plugins/plugin_v2_smoke.sh` | canonical plugin proof home | live caller target after the top-level shim deletion |
| `tools/stage1_smoke.sh` | archived | legacy embedded bridge smoke archived in phase-38x |

## State Reading

| State | Read as |
| --- | --- |
| `route facade` | thin owner that should not gain new features |
| `vm gate` | explicit keep candidate; do not grow it |
| `compat keep` | legacy/raw contract keep; do not attach new capabilities |
| `archive-later shim` | not a real owner; drain callers first and then archive |
| `proof-only keep` | proof/acceptance route that stays live for now |
| `direct owner` | where new capability work should converge |
| `settled split` | already separated in helper files; do not reopen without a new caller |

## Inventory Results (41xA1 landed)

| Surface | Read as |
| --- | --- |
| `tools/selfhost/selfhost_build.sh` | route facade; direct/core-first helpers keep vm-shaped fallback edges out of the top level |
| `tools/selfhost/run.sh` | outer facade; runtime mode still touches `--backend vm`, direct mode stays on `run_stageb_compiler_vm.sh` |
| `tools/selfhost/run_stageb_compiler_vm.sh` | proof-only keep; explicit Stage-B VM gate |
| `tools/selfhost/selfhost_vm_smoke.sh` | proof-only keep; VM parity proof only |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | proof-only keep; stage3 acceptance proof only |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper boundary; caller drain comes before shrink |
| `src/runner/modes/vm.rs` | engineering keep |
| `lang/src/runner/stage1_cli/core.hako` | compat keep |
| `src/runner/core_executor.rs` | direct owner |
| `src/runner/build.rs` | settled split |

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-41x stage0 direct/core route hardening` |
| Blocker | `none` |
| Next | `41xB2 run.sh facade trim` |

- `41xA2` landed: proof-only VM gate set is frozen and non-growing
- `41xB1` landed: selfhost_build.sh direct/core route hardening is fixed as a route facade

## Direct/Core Hardening Rules

- keep `run_stageb_compiler_vm.sh` as proof-only
- keep `selfhost_vm_smoke.sh` / `selfhost_stage3_accept_smoke.sh` as proof-only
- keep `core.hako` as compat-only
- keep `vm.rs` from receiving new capability work
- keep `selfhost_build.sh` and `run.sh` direct/core-first, not feature owners
