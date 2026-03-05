# Control Flow Plan — Architecture (SSOT)

This folder implements the JoinIR “planner-first” pipeline in layers:

- **Facts**: Conservative shape observation (no AST rewrite). Produces box-local “facts” and/or recipes.
- **Recipe Contract**: Facts-derived route contract (`RecipeMatcher`) that chooses a single lowering route.
- **CorePlan**: A normalized plan in CorePlan vocabulary, ready for lowering to MIR.
- **Lowering**: Emits MIR (or freezes in `planner_required` mode when a contract is violated).

## Entry SSOT

The authoritative “entry wiring” and ownership boundaries are documented in `src/mir/builder/control_flow/plan/REGISTRY.md`
under “Entry SSOT (router→planner→composer→lower)”.

Legacy compatibility boundary (`loop_*_v0` inventory and removal rules):
- `src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md`

## Notes

- `strict/dev + planner_required` must fail-fast (no silent fallback).
- New accepted shapes must preserve the Facts→Lower contract: if Facts returns `Some`, Lower must be able to lower it.
