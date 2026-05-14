# 293x-290 LOOPCLEAN-002 while parser normalization

Status: complete

## Decision

Decision: accepted.

`while` remains a Stage-3 compatibility surface, but parser output normalizes it to
canonical `ASTNode::Loop`. `ASTNode::While` stays in the model for legacy JSON /
roundtrip compatibility until `LOOPCLEAN-003` quarantines old decode-only paths.

## Scope

- Change `parse_while_stage3()` to return `ASTNode::Loop`.
- Add a parser regression proving new Stage-3 `while` output is `Loop`, not `While`.
- Add a lightweight row guard entry for the normalization contract.
- Guard passed locally.

## Non-goals

- Do not remove the `while` token.
- Do not delete `ASTNode::While` yet.
- Do not change `ForRange` / `LoopRange` semantics.
- Do not route `for` through `loop` in this row.

## Acceptance

```bash
bash tools/checks/k2_wide_loopclean_while_parser_normalization_guard.sh
```

The guard checks the docs/code anchors and runs the focused parser regression.

## Next

`LOOPCLEAN-003` quarantines `ASTNode::While` as legacy decode-only compatibility
so new source has a single repetition AST for condition loops.
