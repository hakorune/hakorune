---
Status: SSOT
Date: 2026-04-04
Scope: retire route/default/help surfaces that still expose rust-vm as a selectable live code path after phase-54x selected the successor lane.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-54x/README.md
---

# 55x-90 Rust-VM Route-Surface Retirement Prep SSOT

## Intent

- remove the last route/default/help surfaces that widen rust-vm visibility
- keep proof/compat/reference payloads explicit while route surfaces are being narrowed
- hand off to `phase-56x` only after the route/default/help exposure is frozen

## Canonical Reading

- `phase-55x` is the first concrete retirement wave after the selection lane.
- the target is route/default/help exposure, not proof payload deletion.
- `vm-hako` remains reference/conformance and is explicitly out of scope here.

## Target Surfaces

- `src/cli/args.rs`
- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`
- `tools/selfhost/lib/selfhost_run_routes.sh`
- `tools/selfhost/run.sh`

## Inventory Findings

- strongest stale exposure: `src/cli/args.rs` raw backend default text/behavior, after help wording has already been narrowed
- strongest behaviorally live compat surface: `tools/selfhost/lib/selfhost_run_routes.sh` `stage-a` branch
- explicit router seams to keep but narrow: `src/runner/dispatch.rs`, `src/runner/route_orchestrator.rs`
- explicit narrow facade already in good shape: `tools/selfhost/run.sh`

## Boundaries

- do not prune `vm.rs`, `vm_fallback.rs`, `core.hako`, or `run_stageb_compiler_vm.sh` in this lane
- do not reopen `vm` / `vm-hako` as day-to-day defaults
- keep `stage-a` explicit compat-only
- keep `cargo check --bin hakorune` and `git diff --check` green

## Success Conditions

- backend/default/help exposure is frozen to the direct/core-first reading
- route comments and help output match the current owner model
- selfhost route surfaces stop implying hidden vm defaults
- the lane hands off cleanly to `phase-56x proof/compat keep pruning`
