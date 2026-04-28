---
Status: Landed
Date: 2026-04-28
Scope: prune plan facts IfPhiJoinFacts compatibility alias
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/facts/mod.rs
  - src/mir/builder/control_flow/plan/facts/loop_builder.rs
  - src/mir/builder/control_flow/plan/facts/loop_types.rs
  - src/mir/builder/control_flow/plan/recipe_tree/if_phi_join_builder.rs
  - src/mir/builder/control_flow/plan/recipe_tree/matcher/patterns.rs
  - src/mir/builder/control_flow/plan/composer/coreloop_v1_tests/if_phi_join.rs
---

# 291x-663: IfPhiJoinFacts Alias Prune

## Goal

Remove the `plan::facts::IfPhiJoinFacts` compatibility alias for the
facts-owned if-phi-join shape.

This is BoxShape cleanup. It must not change if-phi-join extraction, recipe
construction, recipe verification, planner acceptance, or lowering behavior.

## Evidence

`IfPhiJoinFacts` is defined by the facts layer in
`control_flow::facts::if_phi_join_facts`. `plan::facts` still re-exported the
extractor and aliased the type back into plan facts. Current users are small:

- `plan/facts/loop_builder.rs`
- `plan/facts/loop_types.rs`
- `plan/recipe_tree/if_phi_join_builder.rs`
- `plan/recipe_tree/matcher/patterns.rs`
- `plan/composer/coreloop_v1_tests/if_phi_join.rs`

## Decision

Import `IfPhiJoinFacts` and `try_extract_if_phi_join_facts` from the facts
owner surface.

Remove the plan-facts compatibility alias and extractor re-export.

## Boundaries

- Do not move `IfPhiJoinFacts`.
- Do not change recipe builder fields or verification logic.
- Do not change planner-required behavior.
- Do not mix this with broader recipe-tree facade cleanup.

## Acceptance

```bash
cargo fmt
cargo test if_phi_join --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- If-phi-join callers now import facts-owned types/functions from
  `control_flow::facts`.
- `plan::facts` no longer aliases `IfPhiJoinFacts` or re-exports its extractor.
- If-phi-join behavior is unchanged.
