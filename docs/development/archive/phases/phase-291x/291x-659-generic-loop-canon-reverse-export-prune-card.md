---
Status: Landed
Date: 2026-04-28
Scope: prune generic-loop canon reverse re-export
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/canon/generic_loop.rs
  - src/mir/builder/control_flow/plan/generic_loop/facts/extract/v0.rs
  - src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs
  - src/mir/builder/control_flow/plan/generic_loop/facts/extract/tests.rs
---

# 291x-659: Generic-Loop Canon Reverse Export Prune

## Goal

Remove the facts-side reverse re-export of plan-owned generic-loop step
placement helpers.

This is BoxShape cleanup. It must not change generic-loop facts extraction,
step-placement classification, planner acceptance, or lowering behavior.

## Evidence

Worker inventory found that `facts::canon::generic_loop` forwards these names
from `plan::canon::generic_loop`:

- `classify_step_placement`
- `StepPlacement`
- `StepPlacementDecision`

The only external consumers of that facts-side forwarding are the generic-loop
v0/v1 extractors under `plan/generic_loop/facts/extract`. Those extractors can
import the plan-owned placement API directly and leave facts canon as the owner
only for facts-side condition/update canon helpers.

## Decision

Move v0/v1 extractor imports for `classify_step_placement` and
`StepPlacement` to `control_flow::plan::canon::generic_loop`.

Remove the reverse re-export from `facts::canon::generic_loop`.

While validating this slice, the generic-loop focused test set exposed leaked
`NYASH_JOINIR_DEV` / `HAKO_JOINIR_PLANNER_REQUIRED` process environment state.
Keep that fix local to the extractor tests by guarding and restoring those two
variables around each env-sensitive assertion.

## Boundaries

- Do not move `plan::canon::generic_loop` placement types or classifier.
- Do not change `ConditionCanon`, `UpdateCanon`, or loop increment canon
  ownership.
- Do not change reject reasons or planner-required behavior.
- Do not mix this with `BodyLocalRoute`, feature pipeline, or recipe-tree
  facade cleanup.

## Acceptance

```bash
cargo fmt
cargo test generic_loop --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Generic-loop v0/v1 extractors now import step placement classification from
  the plan canon owner directly.
- `facts::canon::generic_loop` no longer forwards plan-owned step placement
  names.
- Generic-loop extractor tests now restore their JoinIR environment variables
  and run deterministically as a focused group.
- Generic-loop facts extraction behavior is unchanged.
