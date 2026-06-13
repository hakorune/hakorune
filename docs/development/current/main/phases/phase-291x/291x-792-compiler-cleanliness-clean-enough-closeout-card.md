---
Status: Landed
Date: 2026-06-13
Scope: compiler cleanliness clean-enough closeout after the MIR / JoinIR
  BoxShape cleanup checkpoint.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - docs/development/current/main/design/joinir-target-lowerer-thinning-ssot.md
  - docs/development/current/main/design/loop-body-local-init-thinning-ssot.md
  - docs/development/current/main/design/inline-boundary-builder-thinning-ssot.md
  - docs/development/current/main/design/generic-case-a-trim-thinning-ssot.md
  - docs/development/current/main/design/user-method-policy-thinning-ssot.md
---

# 291x-792: Compiler Cleanliness Clean-Enough Closeout

## Goal

Stop the open-ended compiler cleanup lane at a clean checkpoint and make the
return-to-implementation rule explicit.

This is a docs-only closeout card. No compiler behavior changed.

## Clean-Enough Decision

The compiler cleanup lane is clean enough to pause when the following are true:

```text
truth_owner_added_count=0
accepted_shape_added_count=0
route_selection_changed=0
optimizer_behavior_changed=0
verifier_check_removed_count=0
cleanup_entrypoints_have_ssot=1
implementation_lane_can_resume=1
```

Current judgment: satisfied for this cleanup checkpoint.

The remaining large files and deep paths are not automatically blockers. They
should only reopen cleanup when a concrete owner/seam is selected.

## Evidence At This Checkpoint

The cleanup work now has durable policy and owner surfaces:

```text
MIR cleanup policy:
  BoxShape-only / one-purpose series / minimal gate rules

compiler pipeline thinning:
  semantic_refresh / optimizer / verifier / JoinIR thinning stay boundary
  cleanup unless a separate behavior card is accepted

JoinIR target lowerers:
  route facade / dispatch / route-local builder / common seam split

recent JoinIR BoxShape slices:
  loop_update_analyzer tests split
  loop_body_local_init tests split and method-call shelf split
  inline_boundary_builder tests split
  generic_case_a trim skip_leading split
  funcscanner_trim loop_step builder split
  user_method_policy tests split
```

The purpose of these slices was not to hit a line-count target. The purpose was
to reduce hidden ownership and make route/policy/builder responsibilities
traceable from SSOT to code.

## Stop-Line

Do not continue broad cleanup by default.

Stop and create a separate card if the next cleanup candidate wants to:

```text
add or remove accepted source/MIR shapes
change route selection or fallback order
change optimizer behavior
merge or remove verifier checks
move policy truth into helper internals
delete compat / legacy surfaces without a retire gate
expand a shared adapter with route-specific options
flatten multiple semantic families at once
```

Policy / route decision / pattern acceptance files may remain longer than a
mechanical line-count threshold when they are the visible truth owner.

## Optional Follow-Ups

These are not required before returning to the active implementation lane:

```text
MIR-CLEAN-FOLLOWUP-001:
  choose one JoinIR merge semantic family before any deep flatten

MIR-CLEAN-FOLLOWUP-002:
  split one additional large test file only if it is test-only

MIR-CLEAN-FOLLOWUP-003:
  collapse one pure re-export mod.rs only after classification
```

Explicitly avoid repo-wide flattening, large-file sweeps, optimizer pass
merges, verifier check grouping, and route-specific generic adapter expansion.

## Return To Implementation

`CURRENT_STATE.toml` remains the active lane SSOT. At this checkpoint it points
to the mimalloc / FastMemory implementation lane, not to compiler cleanup.

Therefore:

```text
compiler_cleanup_status=closeout_clean_enough
current_state_pointer_update_required=0
next_default_lane=CURRENT_STATE.toml active_lane
```

Future compiler cleanup should be reopened only by a focused card with:

```text
selected_owner_family
BoxShape-only statement
forbidden behavior changes
targeted proof commands
```

## Proof

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

