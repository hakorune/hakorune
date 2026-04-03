---
Status: Active
Date: 2026-04-03
Owner: Codex
Scope: residual rust-vm owner cleanup after direct/core follow-up; keep VM live only as proof/oracle/compat, not day-to-day mainline.
---

# 45x-90 VM Residual Cleanup SSOT

## Goal

- keep `rust-vm` as proof/oracle/compat keep instead of broad execution owner
- shrink the remaining vm-facing surfaces without reopening direct/core mainline ownership
- keep proof-only VM gates explicit and non-growing

## Current Reading

- direct/core ingress is already in place:
  - `src/runner/mod.rs --mir-json-file`
  - `src/runner/core_executor.rs`
  - `tools/selfhost/stage1_mainline_smoke.sh`
- residual vm surfaces still matter:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `lang/src/runner/stage1_cli/core.hako`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
- already route-neutral / not the broad target:
  - `src/runner/modes/common_util/selfhost/stage0_capture.rs`
  - `src/runner/modes/common_util/selfhost/stage0_capture_route.rs`

## Hotspots

| Surface | Why it still matters |
| --- | --- |
| `src/runner/modes/vm.rs` | broad execution owner still holds the main rust-vm behavior surface |
| `src/runner/modes/vm_fallback.rs` | fallback still carries residual compatibility semantics |
| `lang/src/runner/stage1_cli/core.hako` | raw compat hold line must stay narrow / no-widen |
| `tools/selfhost/run_stageb_compiler_vm.sh` | proof-only VM gate must not drift back into the day-to-day route |

## Success Conditions

- `vm.rs` is proof/oracle keep only
- `vm_fallback.rs` is explicit fallback keep only
- `core.hako` remains compat hold line, not a growth point
- `run_stageb_compiler_vm.sh` remains proof-only keep
- `cargo check --bin hakorune` stays green

## Big Tasks

1. inventory residual vm owner surfaces and caller edges
2. shrink `vm.rs` to proof/oracle keep
3. drain `vm_fallback.rs` and shared vm helpers
4. freeze `core.hako` compat hold line and proof-only VM gates
5. prove and close the lane
