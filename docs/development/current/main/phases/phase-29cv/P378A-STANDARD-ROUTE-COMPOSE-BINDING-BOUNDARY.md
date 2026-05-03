---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: keep StandardEntry route composition scratch bindings out of PlanLowerer pre-loop snapshots
Related:
  - docs/development/current/main/phases/phase-29cv/P377A-RECIPE-SCOPEBOX-BOUNDARY-SSOT.md
  - src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc.rs
  - src/mir/builder/control_flow/plan/lowerer/loop_lowering.rs
---

# P378A: Standard Route Compose Binding Boundary

## Intent

Continue the direct Stage1 env MIR verify cleanup at the next ownership seam.

P377A proved the residual 24 dominance errors are not caused by duplicate
`ScopeBox` dispatch handling. The next inventory points at loop-body locals
being present when a following short-circuit expression materializes branch
entry PHIs:

```text
ParserControlBox.parse_if/4:
  %564  defined by loop-body substring in bb1770, later used from bb1772
  %1348 defined by loop-body substring in bb1832, later used from bb1834
  %1832 defined by loop-body substring in bb1865, later used from bb1867
```

The active loop route is `LoopCondBreak` / `loop_cond_break_continue`. Its
composer builds a `CorePlan`, but it also uses lowerer helpers that mutate
`builder.variable_ctx.variable_map` while materializing plan-local values.
If those scratch bindings survive composition, `PlanLowerer::lower_loop`
captures them as `pre_loop_map` and restores them after the loop. A later
short-circuit branch then sees loop-body locals as outer bindings and can
materialize invalid PHI inputs.

This card locks the StandardEntry route boundary:

```text
Compose may allocate ValueIds and build CorePlan scratch state.
Compose must not publish builder.variable_map bindings to the outer route.
PlanLowerer final_values is the only StandardEntry loop route that publishes
post-loop outer bindings.
```

## Boundary

This is a BoxShape cleanup in the JoinIR route-entry owner.

Do:

- snapshot `builder.variable_ctx.variable_map` before StandardEntry compose
- restore that snapshot before `lower_verified_core_plan`
- keep final outer binding publication in `PlanLowerer` / `CoreLoopPlan.final_values`
- add a canary that a compose-time scratch binding is not visible after route
  composition

Do not:

- edit `.hako` source to predeclare loop locals
- widen `logical_shortcircuit` PHI materialization as a workaround
- change dominance verifier semantics
- add a new loop/body route shape
- touch Stage0, C shim, or global-call classifiers

## Acceptance

```bash
cargo test --release standard_route_compose_restores_variable_map_before_lower
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Remeasure:

```bash
timeout --preserve-status 240s env \
  NYASH_LLVM_SKIP_BUILD=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  bash tools/selfhost_exe_stageb.sh \
  lang/src/runner/stage1_cli_env.hako \
  -o /tmp/p378_stage1_cli_env.exe
```

## Result

Implemented:

- `with_standard_compose_binding_boundary` at the StandardEntry route owner
- `route_standard` now restores compose scratch `variable_map` before
  `lower_verified_core_plan`
- `standard_route_compose_restores_variable_map_before_lower` locks the
  compose-before-lower binding boundary

Verified:

- `cargo test --release standard_route_compose_restores_variable_map_before_lower` passed
- `cargo build --release --bin hakorune` passed
- `bash tools/checks/current_state_pointer_guard.sh` passed
- `git diff --check` passed

Remeasure:

- Direct Stage1 env MIR verify improved from 24 errors to 10 errors:

```text
[freeze:contract][emit-mir/direct-verify] route=mir errors=10
```

The `ParserControlBox.parse_if/4` loop-local bucket from P377A disappeared.
Remaining errors are a different dominance class and should be handled by the
next card, not by widening this StandardEntry boundary.
