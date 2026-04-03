---
Status: Landed
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
| `tools/selfhost/selfhost_build.sh` | route facade with downstream mixed pressure | landed route-main split input |
| `tools/selfhost/lib/selfhost_build_direct.sh` | direct/core helper owner | active migration target |
| `tools/selfhost/lib/selfhost_build_dispatch.sh` | route dispatcher | active migration target |
| `tools/selfhost/run.sh` | route-only facade | landed route-only facade split input |
| `tools/selfhost/lib/selfhost_run_routes.sh` | route bodies | landed helper-owned route bodies |
| `src/runner/modes/common_util/selfhost/child.rs` | thin route-tag helper; stage0 child capture moved out | landed shell-only drain |
| `src/runner/modes/common_util/selfhost/stage0_capture.rs` | stage0 child capture helper | active migration target |
| `src/runner/modes/common_util/vm_source_prepare.rs` | vm source preflight helper | landed preflight/source-prepare split |
| `src/runner/modes/vm.rs` | engineering keep / proof-oracle owner | drain then shrink |
| `src/runner/modes/vm_fallback.rs` | engineering fallback keep | drain then shrink |
| `lang/src/runner/stage1_cli/core.hako` | compat keep | do-not-grow keep |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit VM gate | do-not-grow keep |
| `tools/selfhost/selfhost_vm_smoke.sh` | VM proof gate | do-not-grow keep |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | acceptance proof gate | do-not-grow keep |
| `src/runner/core_executor.rs` | direct owner | canonical migration target |
| `tools/selfhost/run_stage1_cli.sh` | direct Stage1 route | canonical migration target |
| `tools/selfhost/stage1_mainline_smoke.sh` | direct proof home | canonical migration target |

## Caller-Starvation Targets

| Surface | Why it stays in the active migration set |
| --- | --- |
| `tools/selfhost/selfhost_build.sh` | it still fans out into route helpers that can pull new work back toward vm-gated routes, even though the route-main glue is now helper-owned |
| `src/runner/modes/common_util/selfhost/stage0_capture.rs` | it now owns vm child capture and still needs preflight/source-prepare / vm fallback drain |
| `src/runner/modes/vm.rs` | it still owns proof/oracle execution pressure and can widen again if callers are not starved |
| `src/runner/modes/vm_fallback.rs` | it still mirrors the vm owner pressure and needs the same drain discipline |

## Do-Not-Grow Keeps

| Surface | Why it stays frozen |
| --- | --- |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit VM proof gate only; no new feature work |
| `tools/selfhost/selfhost_vm_smoke.sh` | proof gate only; keep tiny and stable |
| `tools/selfhost/selfhost_stage3_accept_smoke.sh` | acceptance proof gate only; no widening |
| `lang/src/runner/stage1_cli/core.hako` | raw compat keep only; no-widen |
| `src/runner/core_executor.rs` | direct owner only; no fallback growth |
| `tools/selfhost/run_stage1_cli.sh` | direct Stage1 proof home only |
| `tools/selfhost/stage1_mainline_smoke.sh` | direct proof smoke only |

## Migration Rule

- new capability work goes to direct/core owners only
- proof-only VM gates do not grow, even if they remain executable
- compat keep does not absorb new backend/capability work
- caller drain happens before any attempt to archive `vm.rs`

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-43x next source lane selection` |
| Blocker | `none` |
| Next | `43xA1 candidate lane shortlist` |

- `phase-41x` landed: direct/core route hardening and `vm.rs` proof/oracle shrink are complete enough for handoff
- `42xA1` locked the active migration surfaces and the exact proof-only keep set
- `42xA2` froze the proof-only VM keeps as explicit `do-not-grow`
- `42xB1` landed: `selfhost_build.sh` route-main orchestration is helper-owned now; downstream caller starvation continues from the new facade split

## Big Tasks

1. lock caller starvation targets and route owners
2. move selfhost/build facades toward direct/core ownership
3. drain vm-facing helper/broad-owner callers
4. return proof to a focused direct/core acceptance line

## Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `42xA1` | landed | lock caller starvation targets and active migration surfaces |
| `42xA2` | landed | freeze proof-only VM keep set as explicit `do-not-grow` |
| `42xB1` | landed | starve `selfhost_build.sh` downstream callers toward direct/core helper owners |
| `42xB2` | landed | trim `run.sh` day-to-day route pressure so it stays route-only facade |
| `42xC1` | landed | drain `child.rs` until it owns spawn/capture/timeout/JSON selection only |
| `42xC2` | landed | split `vm.rs` preflight/source-prepare ownership out of the broad execution path |
| `42xC3` | landed | move shared vm user-factory ownership out of `vm.rs` / `vm_fallback.rs` and drain fallback callers |
| `42xC4` | landed | hold `core.hako` compat lane as explicit no-widen while direct/core routes take new work |
| `42xD1` | landed | proof / closeout |
