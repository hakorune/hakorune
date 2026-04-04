---
Status: Active
Date: 2026-04-04
Scope: audit the remaining rust-vm surfaces for delete-ready removal now that route-surface retirement and keep-pruning are landed.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/README.md
  - docs/development/current/main/phases/phase-56x/README.md
  - docs/development/current/main/phases/phase-56x/56x-90-proof-compat-keep-pruning-ssot.md
  - docs/development/current/main/phases/phase-56x/56x-91-task-board.md
---

# Phase 57x: Rust-VM Delete-Ready Audit / Removal Wave

## Goal

- separate true keep-now rust-vm surfaces from delete-ready residue
- audit caller-zero, proof/compat replacement coverage, and removal risk before deleting anything broad
- prepare a small, explicit removal wave instead of another wording-only pass

## Plain Reading

- `phase-56x` finished pruning proof/compat keeps without deleting them.
- `phase-57x` is the first lane allowed to conclude delete-ready status for rust-vm surfaces.
- `vm-hako` remains reference/conformance and stays out of scope.

## Focus Surfaces

- `src/runner/modes/vm.rs`
- `src/runner/modes/vm_fallback.rs`
- `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs`
- `lang/src/runner/stage1_cli/core.hako`
- `tools/selfhost/run_stageb_compiler_vm.sh`
- residual wrapper/docs/examples that still shadow these surfaces

## Success Conditions

- every remaining rust-vm surface is classified as `keep-now`, `archive-later`, or `delete-ready`
- delete-ready candidates have caller-zero proof or equivalent replacement proof
- no `vm-hako` reference/conformance payload is mixed into rust-vm removal
- `cargo check --bin hakorune` and `git diff --check` stay green

## First-Pass Result

- `57xA/B1` locked the first-pass inventory and caller audit.
- no target rust-vm source surface is delete-ready yet.
- `57xB2` therefore narrows the wave toward archive/manual-smoke residue instead of broad source deletion.
- stale proof wrappers may still be aligned to current `hakorune` / exit-code contracts while staying explicit keeps.

## Failure Patterns

- deleting proof/compat keeps without a caller/replacement audit
- treating archive/historical evidence as live blockers
- mixing `vm-hako` reference lane cleanup into rust-vm removal

## Big Tasks

1. lock the exact removal inventory
   - `57xA1` residual rust-vm delete-ready inventory lock
   - `57xA2` keep/delete/archive classification freeze
2. audit and peel delete-ready residue
   - `57xB1` caller-zero audit
   - `57xB2` removal candidate prep
   - `57xC1` removal wave
3. prove and close
   - `57xD1` proof / closeout
