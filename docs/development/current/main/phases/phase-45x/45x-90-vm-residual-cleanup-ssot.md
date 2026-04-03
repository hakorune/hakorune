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
  - `src/runner/modes/vm.rs` broad execution owner
  - `src/runner/modes/vm_fallback.rs` explicit fallback interpreter
  - `lang/src/runner/stage1_cli/core.hako` raw compat hold line
  - `tools/selfhost/run_stageb_compiler_vm.sh` proof-only VM gate
- helper-layer callers still feed the residual routes:
  - `tools/selfhost/lib/selfhost_build_stageb.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `src/runner/modes/common_util/selfhost/stage_a_route.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- already route-neutral / not the broad target:
  - `src/runner/modes/common_util/selfhost/stage0_capture.rs`
  - `src/runner/modes/common_util/selfhost/stage0_capture_route.rs`

## Proof-Only Keep Boundary

- proof-only VM gates:
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `tools/selfhost/selfhost_smoke.sh`
  - `tools/selfhost/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- proof-adjacent direct VM utilities that still stay outside the mainline route:
  - `tools/selfhost/program_analyze.sh`
  - `tools/selfhost/gen_v1_min.sh`
- boundary rule:
  - proof-only gates stay explicit and non-growing
  - day-to-day callers must not discover new VM-backed defaults through these scripts
  - anything that needs to grow capability should move to direct/core or a dedicated non-VM route instead of enlarging this boundary

## Inventory

| Surface | Current role | Inventory read |
| --- | --- | --- |
| `src/runner/modes/vm.rs` | broad execution owner | plugin host init, source prep handoff, safety gate, parse, MIR compile, interpreter execution, JoinIR VM bridge dispatch |
| `src/runner/modes/vm_fallback.rs` | explicit fallback interpreter | legacy compat fallback, using/prelude merge, preexpand, Hako normalize, parse, MIR compile, in-crate MIR execute |
| `lang/src/runner/stage1_cli/core.hako` | compat hold line | raw `run_program_json`, `_mode_run`, `_run_raw_request`, and legacy backend policy |
| `tools/selfhost/run_stageb_compiler_vm.sh` | proof-only VM gate | Stage-B compiler route with explicit proof-only guard and `--backend vm` child execution |
| `tools/selfhost/lib/selfhost_build_stageb.sh` | helper route owner | Stage-B producer helper; still has VM-backed BuildBox emission as explicit keep path |
| `tools/selfhost/lib/selfhost_run_routes.sh` | helper route owner | runtime helper still shells out to `--backend vm` for the legacy runtime route |
| `src/runner/modes/common_util/selfhost/stage0_capture.rs` | route-neutral | spawn / timeout / capture / JSON extraction only |
| `src/runner/modes/common_util/selfhost/stage0_capture_route.rs` | backend-specific builder | holds the VM capture command builder only |
| `src/runner/modes/common_util/selfhost/stage_a_route.rs` | compat caller | stage-A capture caller; still reaches vm capture via route builder |
| `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` | compat caller | stage-A payload resolver / compat bridge; still uses capture route builder |

## Caller Edges

- `tools/selfhost/selfhost_build.sh` -> `tools/selfhost/lib/selfhost_build_stageb.sh`
- `tools/selfhost/run.sh` -> `tools/selfhost/lib/selfhost_run_routes.sh`
- `src/runner/selfhost.rs` -> `src/runner/modes/common_util/selfhost/stage_a_route.rs`
- `src/runner/modes/common_util/selfhost/stage_a_route.rs` -> `stage0_capture_route::build_stage0_vm_capture_command(...)`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` -> `stage0_capture_route::build_stage0_vm_capture_command(...)`
- `tools/selfhost/selfhost_smoke.sh`, `tools/selfhost/bootstrap_selfhost_smoke.sh`, `tools/selfhost/selfhost_stage3_accept_smoke.sh`, `tools/selfhost/program_analyze.sh`, `tools/selfhost/gen_v1_min.sh` still call `--backend vm` directly

## Hotspots

| Surface | Why it still matters |
| --- | --- |
| `src/runner/modes/vm.rs` | broad execution owner still holds the main rust-vm behavior surface |
| `src/runner/modes/vm_fallback.rs` | fallback still carries residual compatibility semantics |
| `lang/src/runner/stage1_cli/core.hako` | raw compat hold line must stay narrow / no-widen |
| `tools/selfhost/run_stageb_compiler_vm.sh` | proof-only VM gate must not drift back into the day-to-day route |
| `tools/selfhost/lib/selfhost_build_stageb.sh` | Stage-B producer helper still carries an explicit VM keep path |
| `tools/selfhost/lib/selfhost_run_routes.sh` | runtime helper still has a VM-backed legacy route path |

## Success Conditions

- `vm.rs` is proof/oracle keep only
- `vm_fallback.rs` is explicit fallback keep only
- `core.hako` remains compat hold line, not a growth point
- `run_stageb_compiler_vm.sh` remains proof-only keep
- proof-only VM gates stay explicit and non-growing
- `cargo check --bin hakorune` stays green

## Big Tasks

1. inventory residual vm owner surfaces and caller edges
2. shrink `vm.rs` to proof/oracle keep
3. drain `vm_fallback.rs` and shared vm helpers
4. freeze `core.hako` compat hold line and proof-only VM gates
5. prove and close the lane
