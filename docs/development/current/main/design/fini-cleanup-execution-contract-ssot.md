Status: Transitional bridge SSOT; sunset owned by LANGUAGE-RESULT-EXIT-C-PRIME0-R0
Scope: Stage-B JSON v0 bridge execution contract for canonical `cleanup`,
legacy `fini {}` / `local ... fini {}`, and postfix `cleanup`.
Related:
- `docs/reference/language/scope-exit-semantics.md`
- `docs/reference/language/lifecycle.md`
- `docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md`
- `src/runner/json_v0_bridge/lowering/scope_exit.rs`
- `src/runner/json_v0_bridge/lowering/expr/block_expr.rs`

# Cleanup/Fini Execution Contract SSOT

Purpose: document the current bridge truth until the accepted C′ family
replaces it with `CleanupRegistrationV1` and `VerifiedExitTransactionV1`.
Nothing in this file selects new canonical surface.

Target naming decision:
- standalone `cleanup {}` is the only canonical scope/block-exit surface;
- local/postfix cleanup and scope-position `fini {}` are retirement input;
- Box-member `fini {}` is a non-callable terminal-Home hook and is never
  lowered as a DropScope handler;
- direct `obj.fini()` is rejected by the accepted C′ target.

The `FiniReg -> Try(finally)` facts below describe the current bridge only.
They must be removed, not rebranded as the final cleanup representation.

## 1) Boundary and ownership

- Language SSOT (surface semantics): `docs/reference/language/scope-exit-semantics.md`.
- Compiler SSOT (this file): how Stage-B JSON v0 bridge lowers and validates those semantics.
- Runtime/VM should not reinterpret ownership/DropScope policy; it executes normalized MIR.

## 2) Normalization contract (`FiniReg` -> `Try(finally)`)

Entry:
- `normalize_scope_exit_registrations(stmts)` in `src/runner/json_v0_bridge/lowering/scope_exit.rs`.

Rules:
- Parser emits `StmtV0::FiniReg { prelude, fini }` as marker shape.
- Bridge inserts all `prelude` statements in-order.
- Bridge folds registrations in reverse and wraps tail with `StmtV0::Try { catches: [], finally: fini_body }`.
- Multiple `fini` registrations are applied in reverse registration order (LIFO).

## 3) `fini` body fail-fast contract

Forbidden in `fini` body:
- `return`
- `throw`
- `break`
- `continue`
- nested `FiniReg`

Freeze tag:
- `[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]`

Requirement:
- Return `Result::Err` directly (no panic/catch-unwind detour).

## 4) BlockExpr contract

When `BlockExpr.prelude` contains `FiniReg`, bridge uses scope-exit path:
- `src/runner/json_v0_bridge/lowering/expr/block_expr.rs::lower_blockexpr_with_scope_exit`

Rules:
- Tail `Expr` is converted into synthetic local (`__blockexpr_tail_tmp_*`) before list lowering.
- Tail `Stmt` must pass blockexpr tail validator.
- If lowered tail block is terminated, bridge must not emit extra `Void` const.

## 5) Catch/Cleanup sequencing boundary

- Postfix `cleanup` is represented as `Try.finally` in JSON v0 shape.
- The current bridge preserves its historical `catch -> cleanup -> outer
  scope fini/drop` sequence until atomic retirement.
- This sequence is implementation evidence, not current language semantics.
  C′ has no catch and gives standalone cleanup plus terminal Home release to
  `VerifiedExitTransactionV1` and the lifecycle DropPlan.

## 6) Change policy

- No AST rewrite for scope-exit behavior.
- Any acceptance expansion requires:
  1. fixture,
  2. fast gate coverage,
  3. freeze-tag/contract update when needed.
- Behavior-neutral refactor is allowed only if this contract remains true.
