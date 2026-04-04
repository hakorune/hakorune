---
Status: Active
Date: 2026-04-04
Scope: inventory residual rust-vm / vm-hako source surfaces, classify keep-now / archive-later / delete-ready, and peel only drained leftovers while keeping vm-hako reference/conformance live.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-52x/README.md
  - docs/development/current/main/phases/phase-52x/52x-91-task-board.md
  - src/runner/modes/vm.rs
  - src/runner/modes/vm_fallback.rs
  - src/runner/modes/vm_hako.rs
  - src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs
  - lang/src/runner/stage1_cli/core.hako
  - tools/selfhost/run_stageb_compiler_vm.sh
  - tools/smokes/v2/profiles/integration/vm_hako_caps/README.md
---

# Phase 53x: Residual VM Source Audit

## Goal

- inventory the remaining rust-vm / vm-hako source surfaces
- keep vm-hako reference/conformance explicit and live
- classify keep-now / archive-later / delete-ready
- archive only drained wrappers/docs; delete only after caller drain

## Plain Reading

- phase-52x finished archive historical labeling polish
- phase-53x audits actual remaining source surfaces
- rust-vm is no longer day-to-day ownership
- vm-hako remains a live reference/conformance lane
- the job is to peel delete-ready rust-vm residues without touching live vm-hako reference work

## Inventory Snapshot

### keep-now

- `src/runner/dispatch.rs`
- `src/runner/route_orchestrator.rs`
- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`
- `src/runner/modes/vm_hako.rs`
- `src/macro/macro_box_ny.rs` deprecated compat runner branch
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- `tools/selfhost/emit_vm_hako_checkpoint_snippet.sh`
- `tools/selfhost/bootstrap_selfhost_smoke.sh`
- `tools/selfhost/selfhost_smoke.sh`
- `tools/selfhost/selfhost_stage3_accept_smoke.sh`
- `tools/plugins/plugin_v2_smoke.sh`
- `tools/selfhost/lib/selfhost_run_routes.sh` compatibility branch
- `tools/smokes/v2/profiles/integration/vm_hako_caps/**`
- `tools/smokes/v2/suites/integration/vm-hako-caps.txt`
- `tools/smokes/v2/suites/integration/vm-hako-core.txt`
- `tools/smokes/v2/suites/integration/phase29x-vm-hako.txt`
- `tools/checks/phase29x_vm_hako_*`
- `tools/checks/vm_*`
- `lang/src/vm/**`
- `src/config/env/vm_backend_flags.rs`

### archive-later

- none in the second pass; residual candidates stayed explicit keep once manual/helper use was confirmed

### delete-ready

- none in this first pass; delete-ready peel stays blocked on caller drain or on turning live proof/compat keeps into archive-only evidence

## Success Conditions

- residual source surfaces are inventoried
- delete-ready rust-vm residues are peeled only after caller drain
- vm-hako reference/conformance remains explicit and non-growing
- archive-ready docs/wrappers move cleanly without new live callers
- `cargo check --bin hakorune` and `git diff --check` stay green

## Failure Patterns

- treating vm-hako reference lane as archive/delete candidate
- deleting proof/compat keeps before replacement/classification
- reintroducing daily callers into vm-gated routes
- archive wording drifting back toward live-owner language

## Big Tasks

1. `53xA residual VM inventory`
   - `53xA1` residual VM caller inventory lock
   - `53xA2` proof-only / compat keep classification
2. `53xB delete-ready peel`
   - `53xB1` rust-vm delete-ready source peel
   - `53xB2` vm-hako reference keep freeze
3. `53xC archive/historical cleanup`
   - `53xC1` archive-ready docs/examples / wrapper cleanup
4. `53xD proof / closeout`
   - `53xD1` proof / closeout

## Boundaries

- vm-hako stays reference/conformance and is not archived wholesale
- proof-only gates stay explicit and non-growing
- compat keeps remain explicit and non-growing
- delete only after caller drain
- inventory first pass did not produce delete-ready source candidates yet
- second-pass worker audit confirmed `emit_vm_hako_checkpoint_snippet.sh` is still a live manual helper, so it stays keep-now
- `macro_box_ny.rs` stays compat keep because deprecated env wiring can still force the VM-backed runner branch
