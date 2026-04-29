---
Status: Landed
Date: 2026-04-29
Scope: loop facts structural vocabulary cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/facts/loop_types.rs
  - src/mir/builder/control_flow/plan/facts/feature_facts.rs
  - src/mir/builder/control_flow/plan/facts/skeleton_facts.rs
---

# 291x-710: Structural Vocabulary Hold

## Why

After `291x-709`, the remaining release warnings were no longer duplicate
helpers or dead shelves. They were structural vocabulary that current release
routing does not yet construct or read directly:

- `LoopFacts::condition_shape`
- `SplitScanFacts::shape`
- `CleanupKindFacts::{Return,Break,Continue}`
- `SkeletonKind::{If2,BranchN}`

Removing them would collapse the facts vocabulary that FlowBox/CorePlan adoption
and test/dev observation still use as the migration shape.

## Decision

Keep the vocabulary, but mark only the exact held fields/variants with
`#[allow(dead_code)]` and local comments explaining the hold reason.

This keeps the cleanup lane warning-free without broad module-level suppression
and without changing planner or lower behavior.

## Changes

- marked `LoopFacts::condition_shape` as a structural condition vocabulary hold
- marked `SplitScanFacts::shape` as a structural split-scan vocabulary hold
- marked `CleanupKindFacts::{Return,Break,Continue}` as cleanup vocabulary holds
- marked `SkeletonKind::{If2,BranchN}` as region skeleton vocabulary holds

## Result

- `cargo build --release --bin hakorune` warning count moved from **4** to **0**
- no recipe, routing, or lower semantics changed

## Proof

```bash
cargo build --release --bin hakorune
```
