---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: preserve lexical ScopeBox locals in return-prelude lowering
Related:
  - docs/development/current/main/phases/phase-29cv/P375A-PHASE29CG-MIR-FIRST-DIRECT-VERIFY-REMEASURE.md
  - src/mir/builder/control_flow/plan/facts/return_prelude.rs
  - src/mir/builder/control_flow/plan/parts/stmt.rs
---

# P376A: Return-Prelude ScopeBox Local Scope

## Intent

Fix the next P375A dominance cluster at the lexical-scope owner, not by adding
Stage0 body knowledge.

The failing `ParserControlBox.parse_if/4` shape contains scoped blocks like:

```hako
{
  local nsrc = src.length()
  loop(j < nsrc) {
    local ch = src.substring(j, j + 1)
    ...
  }
}
```

`return_prelude` recipe facts flatten `ScopeBox` containers as an analysis view.
That is acceptable for shape detection, but the lowering side still needs to
restore lexical scope after the container has been lowered. Otherwise locals
such as `nsrc` and `ch` can stay in `branch_bindings` / `variable_map` and later
short-circuit entry PHIs try to materialize non-dominating values.

## Boundary

This is a BoxShape cleanup in the `.hako` compiler lowering owner.

Do:

- treat `ASTNode::ScopeBox` as a lexical boundary in `lower_return_prelude_stmt`
- let assignments to preexisting outer bindings escape the scope
- prevent locals introduced inside the scope from escaping
- keep recipe flattening as analysis-only; do not rewrite the AST

Do not:

- rewrite `.hako` source conditions
- add a new Stage0 body shape
- add a C shim body emitter
- widen generic string/i64 classifiers

## Rejected Probe

Branch-entry PHI dominance filtering was tested first. It avoided some stale
PHIs but made the Stage1 env direct verify worse by keeping more stale map state
alive. The owner is therefore lexical scope restoration before branch-entry PHI
materialization, not a PHI-entry filter.

## Acceptance

```bash
cargo test --release return_prelude_scopebox_keeps_locals_scoped_and_outer_assignments_visible
cargo build --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence

- `cargo test --release return_prelude_scopebox_keeps_locals_scoped_and_outer_assignments_visible` passed
- `cargo build --release --bin hakorune` passed
- `bash tools/checks/current_state_pointer_guard.sh` passed
- `git diff --check` passed
- Direct Stage1 env MIR verify still fails with
  `[freeze:contract][emit-mir/direct-verify] route=mir errors=24`

## Follow-up

This cleanup prevents one class of scoped-local leakage, but it is not the final
direct-verify unblocker. The remaining verifier details are still dominated by
non-dominating value uses in branch/loop materialization. The next card should
inventory those uses from a MIR payload and choose the exact owner before adding
more PHI filtering or broad scope cleanup.
