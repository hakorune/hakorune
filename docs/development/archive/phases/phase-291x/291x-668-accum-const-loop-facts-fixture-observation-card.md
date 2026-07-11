---
Status: Landed
Date: 2026-04-28
Scope: refresh accum_const_loop facts unit fixture observation
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs
---

# 291x-668: AccumConstLoop Facts Fixture Observation

## Goal

Make the accum-const-loop facts unit fixtures exercise the current scan
observation contract.

This is test cleanup. It must not change facts extraction behavior or planner
acceptance.

## Evidence

`try_extract_accum_const_loop_facts` now requires a `VarLessLiteral`
condition shape and a loop variable carried by `CondProfile`.

The success unit fixture still built its `ScanConditionObservation` from
`ConditionShape::Unknown` and `StepShape::Unknown`, so the fixture expected
`Some` from an observation that can only produce `None`.

The reject fixtures used the same unknown observation, which made them reject
before reaching the intended body-shape checks.

## Decision

Add a small test helper that creates the canonical `i < 3; i += 1`
observation:

- `ConditionShape::VarLessLiteral { idx_var: "i", bound: 3 }`
- `StepShape::AssignAddConst { var: "i", k: 1 }`

Use it in the accum-const-loop facts tests.

## Boundaries

- Do not change `try_extract_accum_const_loop_facts`.
- Do not change scan shape production.
- Do not change planner routing, recipe matching, or lowering.

## Acceptance

```bash
cargo fmt
cargo test accum_const_loop --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `cargo test accum_const_loop --lib` exercises the current observation
  contract.
- Accum-const-loop facts behavior is unchanged.
