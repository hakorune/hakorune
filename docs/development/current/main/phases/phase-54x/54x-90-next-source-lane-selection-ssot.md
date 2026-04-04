---
Status: SSOT
Date: 2026-04-04
Scope: choose the next source lane after `phase-53x` landed so the repo does not drift back into a stale default.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-53x/README.md
  - docs/development/current/main/phases/phase-53x/53x-91-task-board.md
---

# 54x-90 Next Source Lane Selection SSOT

## Intent

- keep the handoff from `phase-53x` explicit
- rank the remaining source lane candidates by leverage, not by cleanup fatigue
- decide the next active source lane before any new work starts

## Canonical Reading

- `phase-53x` is landed and handed off.
- `rust-vm` is historical / proof / compat keep, not day-to-day ownership.
- `vm-hako` stays reference/conformance and is not part of archive/delete wholesale.
- `phase-54x` exists only to select the next source lane cleanly.

## Inventory Findings

- route/default/help surfaces that still feel live:
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `tools/selfhost/run.sh`
  - `src/cli/args.rs` help/default wording
  - `tools/selfhost/lib/selfhost_run_routes.sh` `stage-a` branch
- proof / compat / reference keeps:
  - `src/runner/modes/vm.rs`
  - `src/runner/modes/vm_fallback.rs`
  - `src/runner/modes/vm_hako.rs`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
  - `tools/selfhost/run_stageb_compiler_vm.sh`
  - `lang/src/runner/stage1_cli/core.hako`
  - bootstrap/selfhost proof smokes
  - `tools/smokes/v2/profiles/integration/vm_hako_caps/**`
- delete-ready:
  - none

## Retirement Corridor

- `phase-54x` must leave behind a concrete corridor to rust-vm retirement from the live source surface.
- the corridor is:
  1. `phase-55x rust-vm route-surface retirement prep`
  2. `phase-56x proof/compat keep pruning`
  3. `phase-57x rust-vm delete-ready audit / removal wave`
- this corridor is about rust-vm only.
- `vm-hako` stays reference/conformance keep and is explicitly outside the retirement corridor.
- `kilo` optimization stays far-future and is also outside this corridor.

## Candidate Reading

- highest-leverage next lane: `phase-55x rust-vm route-surface retirement prep`
- second lane: `phase-56x proof/compat keep pruning`
- third lane: `phase-57x rust-vm delete-ready audit / removal wave`
- reason for the order:
  - route/default/help surfaces should stop exposing rust-vm before keep surfaces are pruned
  - proof/compat keeps should be minimized before any delete-ready removal wave starts
  - delete-ready audit should happen only after callers and explicit keeps are stable

## Boundaries

- do not reopen `--backend vm` as a daily default
- do not turn proof-only keeps into the next owner lane
- do not select a lane just to mirror previous cleanup sequences
- keep `cargo check --bin hakorune` and `git diff --check` green during the decision
- do not place `vm-hako` into the rust-vm retirement bucket

## Success Conditions

- candidate lanes are inventoried
- a successor lane is selected
- the post-selection retirement corridor is explicit
- current mirrors point at the active lane
- the handoff is concise and honest
