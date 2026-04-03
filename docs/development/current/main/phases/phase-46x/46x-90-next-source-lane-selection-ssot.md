---
Status: Landed
Date: 2026-04-03
Owner: Codex
Scope: shortlist and select the next source lane after `phase-45x` landed.
---

# 46x-90 Next Source Lane Selection SSOT

## Goal

- keep `rust-vm` as proof/oracle/compat keep, not a re-growing day-to-day owner
- choose the next source lane that cuts the most live VM feature tax
- keep the selection explicit so the next implementation phase can start from a clear lane choice

## Current Reading

- `phase-45x` is landed:
  - `vm.rs` / `vm_fallback.rs` / `core.hako` / proof-only VM gate script are narrowed
- the remaining live VM pressure is concentrated in helper-route defaults:
  - `tools/selfhost/lib/selfhost_run_routes.sh` still routes `runtime` through `--backend vm`
  - `tools/selfhost/lib/selfhost_build_stageb.sh` still carries a VM-backed keep path for BuildBox emission
- the VM core tail still exists, but it is no longer the only or best next target:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/common_util/vm_execution.rs`
  - `src/runner/modes/common_util/vm_user_factory.rs`
- proof-only gate remains explicit:
  - `tools/selfhost/run_stageb_compiler_vm.sh`

## Candidate Lanes

| Candidate | Leverage | Read as |
| --- | --- | --- |
| `stage0/runtime direct-core finalization` | highest | remove the last live VM default from the helper-route layer; keep VM only as explicit proof/fallback |
| `vm core tail shrink` | medium | continue shrinking `vm.rs` / `vm_fallback.rs` / shared VM helpers, but after live defaults are already drained |
| `archive sweep 2` | lower | clean out drained shims / legacy proof surfaces; useful but not the main feature-tax reducer |

## Recommendation

- selected successor lane: `phase-47x stage0/runtime direct-core finalization`
- reason:
  - the remaining live default pressure still sits in `selfhost_run_routes.sh`
  - route defaults create feature tax faster than the already-shrunk VM core tail
  - proof-only VM gates should remain explicit keeps, not hidden day-to-day producers

## Decision Rule

- choose the lane that removes live VM defaults from route helpers first
- keep `run_stageb_compiler_vm.sh` as an explicit proof/fallback keep
- do not widen `core.hako` or the raw compat lane while selecting the next phase

## Decision

- `phase-46x` is closed as a selection lane
- next implementation lane is `phase-47x stage0/runtime direct-core finalization`
- implementation order is:
  - runtime default cutover first
  - Stage-A source->MIR first migration second
  - Stage-B default-caller drain third
  - VM core tail shrink stays after helper-route defaults are drained
