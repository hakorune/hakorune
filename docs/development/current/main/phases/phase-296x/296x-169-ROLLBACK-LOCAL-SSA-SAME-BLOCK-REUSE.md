---
Status: Current
Date: 2026-05-28
Scope: roll back the LocalSSA same-block field_get reuse non-keeper.
Blocker: ROLLBACK-LOCAL-SSA-SAME-BLOCK-REUSE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-168-POST-LOCAL-SSA-SAME-BLOCK-REUSE-MEASUREMENT.md
  - src/mir/builder/ssa/local.rs
---

# 296x-169 Rollback LocalSSA Same-Block Reuse

## Purpose

Remove the row167 field_get-only LocalSSA same-block reuse rule after row168
measured it as a performance non-keeper. Keep the observation/probe trail, but
restore the structural baseline from after row162/row163 before selecting a new
owner.

## Evidence

```text
rollback_contract=rollback-local-ssa-same-block-reuse-v0
removed_rule=local_ssa_same_block_field_get_reuse
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
restored_instruction_count=180
restored_copy_count=88
restored_local_ssa_copy_count=38
restored_field_get_result_chain_copy_count=23
exact_exe_smoke=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The same-block field_get reuse was a structural win but not a timing keeper in
the exact-EXE parity lane. Do not continue optimizing LocalSSA same-block reuse
without a more direct body-timing or backend-shape explanation.
```

## Next

```text
row170:
  post-rollback-owner-refresh

Goal:
  refresh owner selection from the restored baseline and avoid returning to the
  same LocalSSA same-block reuse owner without new evidence.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_rollback_local_ssa_same_block_reuse_guard.sh
```
