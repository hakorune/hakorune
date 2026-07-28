Status: Done
Date: 2026-06-18
Scope: decide whether hakorune-mir-plans Stage 1 can close after object-state split
Related:
  - docs/development/current/main/phases/phase-296x/296x-1093-BUILD-MIR-PLANS-OBJECT-STATE-PASSIVE-BUNDLE-SPLIT-001.md
  - src/mir/function/facts.rs
  - crates/hakorune_mir_plans

# BUILD-MIR-PLANS-STAGE1-CLOSEOUT-CANDIDATE-001

## Purpose

Audit the remaining MIR function metadata surface before closing Stage 1 of
`hakorune-mir-plans`.

## Decision

Do one more low-risk passive split before Stage 1 closeout.

```text
selected_next_family=function_fact_passive_bundle
selected_source=src/mir/function/facts.rs
selected_owner=crates/hakorune_mir_plans/src/function_fact_plan.rs
closeout_ready_before_split=0
closeout_ready_after_split=1
```

## Candidate Table

| Candidate | Decision | Reason |
|---|---|---|
| `src/mir/function/facts.rs` | split | Pure passive fact/plan vocabulary; depends only on MIR core IDs. |
| `src/mir/function/types.rs::StaticDataPlan` | defer | Tiny passive row, but producer is AST-backed and the win is too small for another Stage 1 row. |
| `src/mir/function/types.rs` declarations | keep | Declaration inventory is not plan vocabulary. |
| `src/mir/function/object_metadata.rs` declarations | keep | `UserBoxFieldDecl` / `RecordDecl` are source declaration inventory. |
| `src/mir/function/fastmem.rs` | keep | Depends on source span / fastmem-region boundary; not low-risk passive plan data. |
| `control_flow/plan/**` | keep | Builder-private lowering/planner subsystem; explicitly out of Stage 1 scope. |

## Stop Lines

```text
move_producer_logic=0
move_refresh_logic=0
move_builder_control_flow=0
move_mirfunction_or_metadata=0
move_ast_backed_declaration_inventory=0
add_main_crate_dependency_to_hakorune_mir_plans=0
behavior_change_allowed=0
```

## Next

```text
next_task=BUILD-MIR-PLANS-FUNCTION-FACT-PASSIVE-BUNDLE-SPLIT-001
```
