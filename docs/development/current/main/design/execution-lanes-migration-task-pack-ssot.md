---
Status: SSOT
Decision: provisional
Date: 2026-03-24
Scope: execution-lane policy を cross-phase で docs-first 固定し、後続の wrapper/CI/runtime/default alignment を decision-complete にする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md
  - docs/development/current/main/phases/archive/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/phases/archive/phase-29cu/README.md
  - docs/development/current/main/phases/phase-29y/README.md
---

# Execution Lanes Migration Task Pack (SSOT)

## Goal

- parent execution-lane policy を 1 本に固定したうえで、既存 phase owners を崩さずに docs -> operational guidance -> runtime lane enforcement の順で進める。
- runtime throughput/debug work と lane-policy lock を混ぜない。
- raw CLI backend token/default flip は separate future reopen に切り離す。

## Owner Mapping

| owner | responsibility in this task pack |
| --- | --- |
| `phase-29ct` | stage0/native substrate reading |
| `phase-29ci` | stage1 bridge/proof/snapshot boundary |
| `phase-29y` | runtime operation policy |
| `phase-29cu` | Rune close-synced keep only |

## Fixed Order

### W0. Docs lock

- land `execution-lanes-and-axis-separation-ssot.md`
- land `artifact-policy-ssot.md`
- sync `CURRENT_TASK.md`, `10-Now.md`, `design/README.md`
- add backlinks from child SSOTs and relevant phase READMEs
- no code or route behavior change

### W0.5. Legacy/delete inventory lock

- land `execution-lanes-legacy-retirement-inventory-ssot.md`
- any legacy/delete-ready item discovered during this migration must be added there first
- actual delete work stays lane-local and follows the existing retirement policy

### W1. Operational-default alignment

- make docs/wrappers/CI guidance consistently read:
  - `llvm-exe` = operational default daily lane
  - `vm-hako` = reference/debug/bootstrap-proof
  - `rust-vm` = bootstrap/recovery/compat
- keep raw CLI default unchanged in this wave

### W2. Stage1 proof/snapshot cleanup

- normalize stage1 artifact wording across selfhost/bootstrap docs
- keep `target/selfhost/hakorune`, `lang/bin/hakorune`, `stage1-cli`, and `launcher-exe` as stage1 proof/snapshot artifacts only
- do not widen distribution wording inside stage1 docs

### W3. Runtime lane enforcement

- under `phase-29y`, enforce the runtime lane split in current docs and operational entrypoints
- `vm-hako` remains failure-driven reopen only
- `rust-vm` remains recovery/compat only

### W3.5. Rust VM source-execution capsule naming lock

This is the first non-delete cleanup for the remaining VM-shaped routes. The accepted reading is:

- `vm-hako` is the reference/conformance capsule and is not a delete target in this wave.
- MIR interpreter surfaces are diagnostic/test-oracle substrate and are not the same target as source execution.
- `vm-compat-fallback` is an explicit compat capsule guarded by `NYASH_VM_USE_FALLBACK=1`.
- The long-term retire target is the `--backend vm` Rust source-execution keep: source prepare / parse / macro expansion / MIR compile / in-crate interpreter execution.

Task order:

1. Behavior-preserving naming lock:
   - `VmRouteAction::Vm` -> `BootstrapRustVmKeep`
   - `VM_LANE_RUST_KEEP` -> `LANE_BOOTSTRAP_RUST_VM_KEEP`
   - `execute_vm_mode` -> `execute_bootstrap_rust_vm_keep`
   - `execute_vm_fallback_interpreter` -> `execute_compat_vm_fallback_capsule`
2. Route trace vocabulary update:
   - `--backend vm` without compat/reference override reports `lane=bootstrap-rust-vm-keep`
   - selection reason says this is an explicit deprecated/bootstrap keep, not a daily route
3. Caller inventory:
   - enumerate active `--backend vm` proof/recovery callers before any behavior gate
   - keep `vm-hako` reference callers out of this delete-readiness count
4. Future gated alias:
   - only after caller inventory, consider requiring an explicit allow env for `--backend vm`
   - do not gate or rename `--backend vm-hako` as part of this cleanup
5. Follow-up naming only:
   - classify `stage_a_compat_bridge.rs` as Program(JSON v0)->MIR compat bridge work, not VM retirement work

Stop-lines:

- no deletion in the naming-lock card
- no raw CLI backend default flip
- no `vm-hako` deletion or demotion
- no widening of source execution fallback
- no Program(JSON v0) bridge behavior change while renaming VM capsules

### Future explicit reopen

- any raw CLI backend default flip
- any lane removal or hard delete
- any `vm-hako` promotion discussion
- any future interpreter artifact activation or promotion criteria

## Acceptance

### W0 / W0.5

- parent SSOT is linked from `CURRENT_TASK.md`, `10-Now.md`, and `design/README.md`
- legacy/delete inventory exists and has initial seeded rows
- no current doc says stage1 is final distribution truth

### W1

- restart docs and operational guidance consistently point to `llvm-exe` as the daily lane
- no current guidance silently promotes `vm-hako`

### W2

- stage1 artifacts are consistently described as proof/snapshot/bridge only
- distribution wording is stage2-mainline daily lane, with stage2+ reserved for umbrella/end-state reading

### W3

- runtime policy docs and gates use the same vocabulary
- `phase-29y` continues to own runtime reopen criteria

### W3.5

- `route_orchestrator` unit tests use the bootstrap/reference/compat capsule vocabulary
- `NYASH_VM_ROUTE_TRACE=1 --backend vm ...` without compat/reference override selects `bootstrap-rust-vm-keep`
- `NYASH_VM_ROUTE_TRACE=1 --backend vm-hako ...` still selects `vm-hako-reference`
- compat fallback still fails fast unless `NYASH_VM_USE_FALLBACK=1`
- active proof/recovery callers for `--backend vm` are inventoried before any gate/delete step

## Non-Goals

- raw CLI default flip in this task pack
- changing runtime implementation because of the docs lock alone
- moving Rune lane ownership into general stage/distribution policy
- deleting `vm-hako` as part of Rust VM source-execution cleanup
