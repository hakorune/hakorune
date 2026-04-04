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

