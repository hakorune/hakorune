## SinglePlanner rule order (SSOT)

The ordering of rules is defined in `rule_order.rs` and is the SSOT for
"which planner runs first". `rules.rs` must stay aligned with that order.

Recent change:
- `LoopTrueBreak` is inserted immediately after `Pattern5` to keep
  loop(true) break/continue coverage close to other exit-focused rules.
- `LoopCondBreak` follows `LoopTrueBreak` to keep loop(cond) exit coverage nearby.

When adding a rule:
- update `rule_order.rs`
- update `rules.rs` (mapping + tag)
- add a one-line reason in this file
