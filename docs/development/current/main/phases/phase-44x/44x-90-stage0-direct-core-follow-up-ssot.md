---
Status: Active
Date: 2026-04-03
Owner: Codex
Scope: `phase-43x` で選ばれた direct/core follow-up を source/shell route の owner migration として進める。
---

# 44x-90 Stage0 Direct/Core Follow-Up SSOT

## Goal

- keep `rust-vm` as proof/oracle/compat keep instead of day-to-day owner
- move remaining live stage0/selfhost route defaults toward direct/core owners
- make VM-backed routes explicit fallback/proof surfaces instead of hidden defaults

## Current Reading

- direct/core ingress already exists:
  - `src/runner/mod.rs --mir-json-file`
  - `src/runner/core_executor.rs`
  - `tools/selfhost/stage1_mainline_smoke.sh`
- live VM owner pressure still exists in:
  - `tools/selfhost/lib/selfhost_build_stageb.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `src/runner/modes/common_util/selfhost/stage0_capture.rs`
- keep narrow/non-growing:
  - `lang/src/runner/stage1_cli/core.hako`
  - proof-only VM gate scripts

## Hotspots

| Surface | Why it still matters |
| --- | --- |
| `tools/selfhost/lib/selfhost_build_stageb.sh` | default Stage-B producer still points day-to-day work at VM-backed routes |
| `tools/selfhost/lib/selfhost_run_routes.sh` | runtime/direct helper defaults still execute or delegate through VM routes |
| `src/runner/modes/common_util/selfhost/stage0_capture.rs` | generic stage0 capture still hardcodes `--backend vm` child spawn |
| `tools/selfhost/run_stageb_compiler_vm.sh` | explicit VM gate is still reachable as a practical producer, not just proof/fallback |

## Success Conditions

- Stage-B producer default is direct/core-first
- runtime/direct helper default is direct/core-first or explicit proof-only VM
- stage0 capture helper is split into route-neutral capture plumbing plus backend-specific route builders
- `run_stageb_compiler_vm.sh` is left as explicit proof/fallback keep only
- `cargo check --bin hakorune` stays green

## Big Tasks

1. lock the exact direct/core target for Stage-B producer defaults
2. cut over `selfhost_build_stageb.sh` to direct/core-first
3. cut over `selfhost_run_routes.sh` runtime/direct defaults
4. neutralize `stage0_capture.rs` as a backend-agnostic helper boundary
5. demote `run_stageb_compiler_vm.sh` to explicit proof-only keep
6. prove and close out

## Micro Tasks

| Task | Status | Read as |
| --- | --- | --- |
| `44xA1` | landed | stage-b direct/core target lock |
| `44xA2` | landed | `selfhost_build_stageb.sh` direct/core-first cutover |
| `44xB1` | landed | `selfhost_run_routes.sh` runtime default cutover |
| `44xB2` | landed | `run.sh` direct route fallback explicitization |
| `44xC1` | landed | `stage0_capture.rs` route-neutral builder split |
| `44xC2` | active | `stage_a_route.rs` / compat caller switch |
| `44xD1` | queued | `run_stageb_compiler_vm.sh` proof-only demotion |
| `44xE1` | queued | proof / closeout |

## Current Front

| Item | State |
| --- | --- |
| Now | `phase-44x stage0 direct/core follow-up` |
| Blocker | `none` |
| Next | `44xC2 stage_a_route.rs / compat caller switch` |
