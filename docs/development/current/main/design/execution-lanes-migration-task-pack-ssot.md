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

## Non-Goals

- raw CLI default flip in this task pack
- changing runtime implementation because of the docs lock alone
- moving Rune lane ownership into general stage/distribution policy
