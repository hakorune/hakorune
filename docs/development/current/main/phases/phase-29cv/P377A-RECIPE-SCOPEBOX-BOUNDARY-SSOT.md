---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: centralize RecipeTree/Parts ScopeBox lexical-boundary lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P376A-RETURN-PRELUDE-SCOPEBOX-LOCAL-SCOPE.md
  - src/mir/builder/control_flow/plan/parts/stmt.rs
  - src/mir/builder/control_flow/plan/parts/dispatch/block.rs
---

# P377A: Recipe ScopeBox Boundary SSOT

## Intent

Continue the direct Stage1 env MIR verify cleanup at the lowering-owner seam.

P376A restored lexical `ScopeBox` local scope in one return-prelude entry. The
next inventory shows the remaining dominance errors are still created by local
values introduced inside statement containers or loops and then carried into
later branch-entry materialization. Most residual uses are PHI-only, but
`ParserControlBox.parse_if/4` has a non-PHI propagation chain, so a dead-PHI
cleanup alone is not sufficient.

This card centralizes the `ScopeBox` boundary rule used by RecipeTree/Parts
lowering:

```text
ScopeBox locals do not escape the ScopeBox.
Assignments to preexisting outer bindings may escape.
```

## Boundary

This is a BoxShape cleanup in the MIR builder lowering owner.

Do:

- create a shared Parts helper for `ScopeBox` lexical-boundary restoration
- use it from return-prelude lowering instead of a private duplicate
- make the rule available to RecipeBlock dispatch paths that lower `Stmt`
  wrappers
- keep lowering based on existing AST/Recipe facts

Do not:

- edit `.hako` source to predeclare loop locals
- add a new `GlobalCallTargetShape`
- add Stage0/body-specific emitters
- widen generic string/i64 classifier semantics
- change dominance verification policy as a workaround

## Evidence

P377 inventory from the current skipverify MIR payload found 24 verifier errors.
Every residual use is a PHI incoming edge, but one cluster has non-PHI
propagation:

```text
ParserControlBox.parse_if/4 %1348:
  bad PHIs = 2
  closure = 49
  non-PHI uses = 28
```

The source shape is a scoped whitespace scan:

```hako
{
  local nsrc = src.length()
  loop(j < nsrc) {
    local ch = src.substring(j, j + 1)
    ...
  }
}
if j < src.length() && src.substring(j, j + 1) == "}" { ... }
```

The owner is lexical boundary preservation before later branch-entry PHIs, not
Stage0 classification.

## Acceptance

```bash
cargo test --release recipe_scopebox_stmt_boundary_keeps_locals_scoped
cargo test --release return_prelude_scopebox_keeps_locals_scoped_and_outer_assignments_visible
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
  -o /tmp/p377_stage1_cli_env.exe
```

## Result

Implemented:

- `with_scopebox_binding_boundary` in Parts var-map scope helpers
- return-prelude ScopeBox lowering now uses the shared boundary helper
- RecipeBlock stmt dispatch unwraps `ScopeBox` before invoking injected stmt
  lowerers, so ScopeBox handling is not duplicated in every lowerer
- `recipe_scopebox_stmt_boundary_keeps_locals_scoped` locks the dispatch
  contract

Verified:

- `cargo test --release recipe_scopebox_stmt_boundary_keeps_locals_scoped` passed
- `cargo test --release return_prelude_scopebox_keeps_locals_scoped_and_outer_assignments_visible` passed
- `cargo build --release --bin hakorune` passed

Remeasure:

- Direct Stage1 env MIR verify still fails with
  `[freeze:contract][emit-mir/direct-verify] route=mir errors=24`

## Follow-up

P377A removed one duplicate ScopeBox-boundary implementation, but it did not
reduce the direct-verify bucket. The remaining evidence points at loop-local
values being present when short-circuit branch-entry PHIs are materialized:

```text
ParserControlBox.parse_if/4:
  %564 defined by loop-body substring in bb1770
  later PHI incoming uses it from bb1772

ParserControlBox.parse_if/4:
  %1348 defined by loop-body substring in bb1832
  later PHI incoming uses it from bb1834
```

The next card should target the JoinIR/loop-to-short-circuit boundary that
feeds `logical_shortcircuit` / branch-entry materialization, not more ScopeBox
dispatch cleanup.
