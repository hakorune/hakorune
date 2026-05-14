# 293x-291 LOOPCLEAN-003 while variant quarantine

Status: complete

## Decision

Decision: accepted.

`ASTNode::While` is retained as a legacy compatibility node, not a canonical
new-source parser output. New Stage-3 `while` source must enter the compiler as
`ASTNode::Loop`; legacy or hand-built `ASTNode::While` must still lower to
Program(JSON v0) `"type": "Loop"` until a future removal card can prove no
compat input depends on it.

## Scope

- Keep the `ASTNode::While` variant for compatibility.
- Keep traversal / MIR / Program(JSON) consumers able to handle legacy `While`.
- Add an explicit Program(JSON v0) regression for legacy `While -> Loop` lowering.
- Guard that parser-output tests no longer expect `ASTNode::While`.
- Guard passed locally.

## Non-goals

- Do not delete `ASTNode::While`.
- Do not remove MIR / planner / facts `While` arms in this row.
- Do not change `ForRange` / `LoopRange` semantics.
- Do not add a JSON `"While"` output shape.

## Acceptance

```bash
bash tools/checks/k2_wide_loopclean_while_variant_quarantine_guard.sh
```

## Next

`LOOPCLEAN-004` can commonize range-header parsing between canonical
`loop i in start..end` and legacy `for i in start..end` without changing the
internal `ForRange` / `LoopRange` route.
