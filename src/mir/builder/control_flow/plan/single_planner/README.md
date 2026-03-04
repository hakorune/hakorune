## SinglePlanner rule order (SSOT)

`PLAN_RULE_ORDER` in `rule_order.rs` is the SSOT for planner payload selection
inside `single_planner::rules`.

Current state (Phase 29bq+):
- Planner payload has one active shape: `LoopCondContinueWithReturn`.
- `PLAN_RULE_ORDER` therefore contains only `LoopCondContinueWithReturn`.
- Other `PlanRuleId` values are kept for router-side planner-first tags and
  compatibility labels, not for single_planner payload matching.

When changing planner payload selection:
- update `rule_order.rs`
- update `rules.rs` (planner kind mapping / recipe-only routing)
- add a one-line reason in this file
