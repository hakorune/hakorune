# 291x-756 LoopFacts Condition Shape Slot Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `LoopFacts::condition_shape`
- verifier debug cond-profile umbrella probe
- CondProfile migration SSOT
- condition observation SSOT
- `CURRENT_STATE.toml`

## Why

`ConditionShape` is still an active extractor-local observation vocabulary, but
`LoopFacts::condition_shape` had become a dead side slot. The release route and
active recipe composers consume route-specific facts or CondProfile-bearing scan
observations instead.

The only remaining field read was an unused debug umbrella helper that rebuilt a
CondProfile from `LoopFacts`. Active debug call sites already observe concrete
CondProfile values directly.

## Decision

Remove `LoopFacts::condition_shape`.

Keep `ConditionShape` itself in `scan_shapes.rs` and condition extractors until
the CondProfile migration can shrink those APIs in their own cards.

## Landed

- Removed `LoopFacts::condition_shape`.
- Kept `condition_shape` as a local value in the loop facts builder for current
  extractor calls.
- Removed the unused verifier `debug_observe_cond_profile(&CanonicalLoopFacts)`
  wrapper.
- Updated tests that manually construct `LoopFacts`.
- Updated active docs to say `ConditionShape` remains extractor-local and is no
  longer stored on `LoopFacts`.

## Remaining Queue Impact

The LoopFacts condition-shape field item is closed. Remaining structural cleanup
is now:

- `SkeletonKind::{If2,BranchN}`
- bridge strict/env/LowerOnly semantics
- `condition_lowering_box` / `condition_to_joinir` / `update_env`

## Proof

- `rg -n "pub condition_shape:" src/mir/builder/control_flow/plan/facts/loop_types.rs`
- `rg -n "condition_shape: ConditionShape::Unknown|facts\\.facts\\.condition_shape|debug_observe_cond_profile\\(" src/mir/builder/control_flow -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
