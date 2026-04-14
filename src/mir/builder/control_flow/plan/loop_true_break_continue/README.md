# LoopTrueBreakContinue (Phase 29bq)

Scope: planner-required only, strict/dev gate for loop(true) bodies with
multiple `if(cond) break/continue` and effect statements.

- Facts: conservative extraction, no AST rewrite
  - `return(value)` is allowed as:
    - `ExitIf` tail exit (no prelude, no else), or
    - unconditional tail `return(value)` as the final stmt in the loop body (`TailReturn` recipe item)
  - `return` (without value) is out-of-scope for this box
  - `Program { ... }` statements are allowed only as "general-if body" (no loops, no break/continue/return)
- Nested loops: one nested loop allowed (cond may be non-true if body is loop_true subset)
- Normalizer: CorePlan::Loop + CorePlan::If + CorePlan::Exit
- Carrier merge: per-continue carrier values are joined in `step_bb` via `ContinueWithPhiArgs` (no single "next_val" assumption)
- Not for release default; used only when planner_required is enabled
- Facts are implemented in `src/mir/builder/control_flow/plan/loop_cond/true_break_continue.rs`
- Nested-loop condition helpers live in `src/mir/builder/control_flow/plan/loop_cond/true_break_continue_helpers.rs`

## Vocabulary

### GeneralIfElseExit (Phase 29bq selfhost unblock)

Pattern: `if { general-if-body } else { exit-if + preludes }`

- **Then-side**: general-if body (no exit, local/assign/nested-if only)
- **Else-side**: exit-mixed pattern (exit-if + prelude statements)
  - Exit-if can be else-less: `if { continue }` is accepted
  - Preludes: Assignment/Local/MethodCall/FunctionCall/Print
- **Recipe**: `GeneralIfElseExit { if_ref, else_recipe: ElseExitMixedRecipe }`
  - `ElseExitMixedRecipe { else_body: RecipeBody, items: Vec<ElseItem> }`
  - `ElseItem`: ExitIf(StmtRef) | PreludeStmt(StmtRef)
  - StmtRef indices are RELATIVE to else_body (0..else_body.len())
- **Lowering**: Pipeline processes `else_recipe.items` sequentially (no re-judgment)
  - Then-side: `lower_general_if_body_block()`
  - Else-side ExitIf: `exit_if_map::lower_if_exit_stmt()`
  - Else-side PreludeStmt: `lower_simple_stmt()`
  - PHI merge: `build_join_payload()` (existing SSOT, returns Result<Vec<CoreIfJoin>, String>)
- **Selfhost blocker**: `RewriteKnown.try_apply/1`
- **Fixture**: `phase29bq_selfhost_blocker_rewriteknown_try_apply_loop_true_else_exit_min.hako`
