# LoopCondBreakContinue SSOT

## Goal

Provide a strict/dev-only planner path for loop(cond) bodies that contain
multiple exit-if break/continue plus simple effect statements. This is a
structural plan line and must not rewrite the AST.

## Scope

- Enabled only when strict/dev + HAKO_JOINIR_PLANNER_REQUIRED=1.
- Release default remains unchanged.
- Analysis-only view (no AST rewrite or expression reordering).

## Facts (acceptance rules)

- condition is not `true` literal and must be a supported bool expression.
- no nested loops in the body.
- `return` only via exit-if (no return elsewhere in the body).
- body must include at least one break/continue.
- continue-only body is rejected (break is required).
- Program/ScopeBox statements are treated as containers and validated/lowered as loop body statements.
- each statement is one of:
  - assignment/local/methodcall/functioncall
  - if-statement that is either:
    - exit-if (then/else is break or continue only), or
    - conditional update (pure assignments, optional tail break/continue)
    - else-only return (then=non-exit, else=return) or then-only break (then=break, else=non-exit)
  - if-stmt kind is encoded in the recipe; Lower must not re-derive it.

## Normalizer contract

- Lower into `CorePlan::Loop` with `ExitIf` for break/continue.
- Conditional updates are expressed with `Select` (no AST rewrite).
- Else-only return / then-only break are lowered via recipe-first branch lowering (no AST rewrite).
- Carriers are detected from assignments in the loop body.
- Header PHIs merge `preheader` and `step_bb` values.
- `step_bb` is a **join** block for per-iteration carrier values.
- Multi-continue is supported without AST rewrite by using `CoreExitPlan::ContinueWithPhiArgs`:
  - Each continue edge provides `(step_phi_dst, value_on_edge)` pairs.
  - The lowerer records these per-edge values and populates `step_bb` PHIs before insertion.

## Per-edge carrier merge (SSOT)

Because loop bodies may contain multiple early-continue edges that skip later updates, the
carrier value at the backedge is **path-dependent**. This SSOT uses a structural plan
mechanism instead of rewriting/hoisting code:

- `step_bb` has PHIs for each carrier (`loop_cond_step_join_*`), with inputs collected from:
  - each `ExitIf(ContinueWithPhiArgs)` edge, and
  - the fallthrough backedge (`CorePlan::Exit(ContinueWithPhiArgs)` at end-of-body).
- `header_bb` PHIs consume the `step_bb` join values.

This keeps:

- **no AST rewrite** (evaluation order unchanged),
- strict/dev-only enablement,
- and local verifier checks (missing PHI inputs fail-fast in the lowerer).

## Gates (SSOT)

- `tools/smokes/v2/profiles/integration/joinir/phase29bq_loop_cond_multi_exit_planner_required_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
