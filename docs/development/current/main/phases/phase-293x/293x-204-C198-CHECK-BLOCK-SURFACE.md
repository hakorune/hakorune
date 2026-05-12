# 293x-204: C198 Check Block Surface

Status: Complete

## Decision

Add `check "name" { "label": expr }` as a general proof-list expression.

This is separate from ordinary `&&` / `||`:

- `&&` / `||` remain short-circuit control-flow operators.
- `check` evaluates every item left-to-right, even after a failed item.
- The v0 result is scalar pass/fail: `1` when every item is truthy, otherwise
  `0`.
- Labels are source-level proof metadata in this row. They are not yet a proof
  report object and they do not trigger automatic printing.

## Scope

Implemented:

- AST node for labeled check items.
- Default parser route and TokenCursor route for `check` blocks.
- MIR lowering as eager item evaluation plus scalar `1` / `0` accumulation.
- A proof app that verifies eager evaluation after a failed item.
- A focused guard for the parser, docs, app, and stop-lines.

Not implemented:

- variadic `all(...)`
- macro expansion
- automatic check failure printing
- allocator-specific proof DSL
- backend route selection
- changing `&&` / `||` semantics

## Acceptance

```bash
bash tools/checks/k2_wide_check_block_surface_guard.sh
```

The guard runs parser coverage for both parser routes and the VM proof app.
Unsupported backend coverage remains explicit future work; this row must not
silently claim backend completion from VM success alone.
