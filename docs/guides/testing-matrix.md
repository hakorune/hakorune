# Testing Matrix — Mapping Specs to Tests

Purpose
- Map invariants/constraints to the concrete tests (smokes/goldens/unit) that verify them.

Categories
- PHI hygiene (LLVM)
  - ir_phi_empty_check.sh — no empty PHIs
  - ir_phi_hygiene_if_phi_ret.sh — PHIs at block head with if/ret pattern
- Match normalization (VM/goldens)
  - match_literal_basic / literal_three_arms output smokes
  - match_guard_literal_or / type_basic_min goldens
- Historical handler compatibility (VM; retirement witnesses, not target syntax)
  - expr_postfix_catch_cleanup_output_smoke.sh — current postfix parser carrier
  - loop_postfix_catch_cleanup_output_smoke.sh — current loop/handler bridge
- LoopForm break/continue (VM)
  - loopform_continue_break_output_smoke.sh — basic continue/break
  - loop_nested_if_ctrl_output_smoke.sh — nested if inside loop
  - loop_nested_block_break_output_smoke.sh — nested bare block with break

Maintenance
- When adding an invariant or lifting a constraint, update this matrix and link the tests.
