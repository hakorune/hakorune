# 291x-757 SkeletonKind Region Variant Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `SkeletonKind::{If2,BranchN}`
- cfg-test region skeleton extractor shelf
- FlowBox facts-to-box-kind mapping
- CorePlan skeleton docs
- `CURRENT_STATE.toml`

## Why

`If2` and `BranchN` are real CorePlan / FlowBox vocabulary, but the
`SkeletonKind` enum used by `LoopFacts` no longer needs to mirror them. The only
producer was a cfg-test region extractor shelf inside `skeleton_facts.rs`.

Keeping those variants in `SkeletonKind` required dead-code allowances and made
the facts layer look like a second structural SSOT.

## Decision

Keep `If2` and `BranchN` as CorePlan / FlowBox vocabulary.

Remove them from the loop-facts `SkeletonKind` enum and delete the cfg-test
region skeleton extractor shelf. Loop facts now carry only loop-focused
`SkeletonKind` values.

## Landed

- Removed `SkeletonKind::If2`.
- Removed `SkeletonKind::BranchN`.
- Removed cfg-test `try_extract_skeleton_facts_from_stmt`.
- Removed cfg-test `infer_region_skeleton_facts`.
- Kept `FlowboxBoxKind::{If2,BranchN}` through `box_kind_from_core_plan`.
- Simplified `box_kind_from_facts` to map only loop-facts skeleton values.
- Updated docs to make CorePlan, not `SkeletonKind`, the If2/BranchN SSOT.

## Remaining Queue Impact

The `SkeletonKind::{If2,BranchN}` item is closed. Remaining structural cleanup is
now:

- bridge strict/env/LowerOnly semantics
- `condition_lowering_box` / `condition_to_joinir` / `update_env`
- current mirror checkpoint wording / retired source pointers

## Proof

- `rg -n "SkeletonKind::(If2|BranchN)|try_extract_skeleton_facts_from_stmt|infer_region_skeleton_facts" src/mir/builder/control_flow -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
