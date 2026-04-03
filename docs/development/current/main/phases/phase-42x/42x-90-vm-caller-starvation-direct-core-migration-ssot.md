---
Status: Active
Date: 2026-04-03
Owner: Codex
Scope: `rust-vm` を即削除せず、live caller を starvation して direct/core owner へ寄せるための route contract を固定する。
---

# 42x-90 VM Caller Starvation / Direct-Core Migration SSOT

## Goal

- reduce future feature tax by starving day-to-day callers away from vm-gated routes
- keep explicit VM proof gates as `do-not-grow`
- migrate mainline owner pressure toward direct/core routes already present in the `hakorune` binary

## Classification

| Surface | Read as | Phase-42x action |
| --- | --- | --- |
| `tools/selfhost/selfhost_build.sh` | route facade with downstream mixed pressure | active migration target |
| `tools/selfhost/lib/selfhost_build_direct.sh` | direct/core helper owner | active migration target |
| `tools/selfhost/lib/selfhost_build_dispatch.sh` | route dispatcher | active migration target |
| `tools/selfhost/run.sh` | outer facade | active migration target |
| `tools/selfhost/lib/selfhost_run_routes.sh` | route bodies | active migration target |
| `src/runner/modes/common_util/selfhost/child.rs` | thin helper boundary under vm child capture | caller-drain target |
| `src/runner/modes/vm.rs` | engineering keep / proof-oracle owner | drain then shrink |
| `src/runner/modes/vm_fallback.rs` | engineering fallback keep | drain then shrink |
| `lang/src/runner/stage1_cli/core.hako` | compat keep | do-not-grow keep |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit VM gate | do-not-grow keep |
| `tools/selfhost/selfhost_vm_smoke.sh` | VM proof gate | do-not-grow keep |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | acceptance proof gate | do-not-grow keep |
| `src/runner/core_executor.rs` | direct owner | canonical migration target |
| `tools/selfhost/run_stage1_cli.sh` | direct Stage1 route | canonical migration target |
| `tools/selfhost/stage1_mainline_smoke.sh` | direct proof home | canonical migration target |

## Migration Rule

- new capability work goes to direct/core owners only
- proof-only VM gates do not grow, even if they remain executable
- compat keep does not absorb new backend/capability work
- caller drain happens before any attempt to archive `vm.rs`

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-42x vm caller starvation / direct-core owner migration` |
| Blocker | `none` |
| Next | `42xA1 caller starvation target lock` |

- `phase-41x` landed: direct/core route hardening and `vm.rs` proof/oracle shrink are complete enough for handoff
- `42xA1` will lock the exact active migration surfaces and the exact proof-only keep set
- `42xA2` will freeze `run_stageb_compiler_vm.sh` / `selfhost_vm_smoke.sh` / `selfhost_stage3_accept_smoke.sh` / `core.hako` as explicit `do-not-grow`

## Big Tasks

1. lock caller starvation targets and route owners
2. move selfhost/build facades toward direct/core ownership
3. drain vm-facing helper/broad-owner callers
4. return proof to a focused direct/core acceptance line

## Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `42xA1` | active | lock caller starvation targets and active migration surfaces |
| `42xA2` | queued | freeze proof-only VM keep set as explicit `do-not-grow` |
| `42xB1` | queued | starve `selfhost_build.sh` downstream callers toward direct/core helper owners |
| `42xB2` | queued | trim `run.sh` day-to-day route pressure so it stays route-only facade |
| `42xC1` | queued | drain `child.rs` until it owns spawn/capture/timeout/JSON selection only |
| `42xC2` | queued | split `vm.rs` preflight/source-prepare ownership out of the broad execution path |
| `42xC3` | queued | move shared vm user-factory ownership out of `vm.rs` / `vm_fallback.rs` and drain fallback callers |
| `42xC4` | queued | hold `core.hako` compat lane as explicit no-widen while direct/core routes take new work |
| `42xD1` | queued | proof / closeout |
