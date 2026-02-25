# nested_loop_depth1 (SSOT)

Scope: depth=1 nested `loop(cond)` acceptance for loop-cond/loop-true pipelines
in strict/dev (planner-required). This page is the SSOT for nested-loop boxes.

## Boxes (Facts → Recipe → Lower)

- `nested_loop_depth1_methodcall`
  - Condition: `<` or `<=`
  - Body: local/assign/if/program/scopebox
  - Break/continue allowed only inside `if` branches
  - Requires at least one call (method/function/call)
  - No return / nested loop / while / for-range

- `nested_loop_depth1_break_continue_pure`
  - Condition: `<` or `<=`
  - Body: local/assign/if/program/scopebox
  - Break/continue allowed only inside `if` branches
  - No calls/prints/returns
  - A single trailing top-level `continue` is treated as a view-only fallthrough

- `nested_loop_depth1_no_break_or_continue`
  - Condition: `<` or `<=`
  - Body: local/assign/if/program/scopebox + call statements
  - No break/continue/return/loop/while/for-range
  - Requires at least one call

- `nested_loop_depth1_no_break_or_continue_pure`
  - Condition: `<` or `<=`
  - Body: local/assign/if/program/scopebox only
  - No break/continue/return/loop/while/for-range
  - No calls

## Shared constraints

- Program/ScopeBox are treated as containers (analysis-only view).
- Facts that accept a shape must lower it without fallback.
- Entry points:
  - `features/loop_cond_break_continue_pipeline.rs`
  - `features/nested_loop_depth1.rs`
- Shared scan logic: `facts/nested_loop_profile.rs`.
