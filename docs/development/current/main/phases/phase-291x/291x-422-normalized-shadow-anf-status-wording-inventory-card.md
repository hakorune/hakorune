---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow ANF status wording inventory
Related:
  - src/mir/control_tree/normalized_shadow/anf/README.md
  - src/mir/control_tree/normalized_shadow/anf/mod.rs
  - src/mir/control_tree/normalized_shadow/anf/contract.rs
  - src/mir/control_tree/normalized_shadow/anf/plan_box.rs
  - src/mir/control_tree/normalized_shadow/anf/execute_box.rs
  - src/mir/control_tree/normalized_shadow/common/expr_lowerer_box/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-421-normalized-shadow-known-intrinsic-comment-cleanup-card.md
---

# 291x-422: Normalized-Shadow ANF Status Wording Inventory

## Goal

Pick the next small compiler-cleanliness seam after normalized-shadow
KnownIntrinsic comment cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

The normalized-shadow ANF module has live P1/P2 implementation paths, but
several docs and source comments still describe the executor as a P0-only stub.

Stale wording appears in:

- `anf/README.md`: status, phase scope, environment-variable description, phase
  summary, and test list still describe "P0 stub" behavior.
- `anf/mod.rs`: module-level contract says `execute_box` always returns
  `Ok(None)` and that strict mode is debug logging only.
- `anf/contract.rs`: contract wording still frames out-of-scope handling as a
  graceful fallback and says diagnostics are not yet used by `execute_box`.
- `anf/plan_box.rs`: inline comments still describe MethodCall detection as a
  P0-only detection path because the executor was a stub.
- `execute_box.rs`: top-level comments and one unit-test comment/name still
  describe stub-only behavior.
- `common/expr_lowerer_box/mod.rs`: debug message says an ANF plan returning
  `None` means a P0 stub.

The current runtime shape is already more precise:

```text
ANF plan available
  -> executor may lower active P1/P2 cases
  -> executor returns Ok(None) only as a route decline / out-of-scope result
```

## Decision

Clean only stale ANF status wording. Treat `Ok(None)` as a route-decline
contract rather than a fallback or stub marker.

Do not change:

- ANF route selection
- `HAKO_ANF_*` environment-variable behavior
- `[phase145/*]` debug tags
- hoist logic
- accepted expression shapes
- generated JoinIR instructions

## Next Cleanup

`291x-423`: normalized-shadow ANF status wording cleanup.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "P0 stub|Skeleton implemented|debug logging only|not yet used in execute_box" \
  src/mir/control_tree/normalized_shadow/anf \
  src/mir/control_tree/normalized_shadow/common/expr_lowerer_box
rg -n "graceful fallback|safe fallback|conservative fallback|execute_box is stub|always returns Ok\\(None\\)" \
  src/mir/control_tree/normalized_shadow/anf \
  src/mir/control_tree/normalized_shadow/common/expr_lowerer_box
```

The final `rg` commands should produce no output.
