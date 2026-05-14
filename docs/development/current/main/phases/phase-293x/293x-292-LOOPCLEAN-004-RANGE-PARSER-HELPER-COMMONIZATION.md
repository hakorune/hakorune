# 293x-292 LOOPCLEAN-004 range parser helper commonization

Status: complete

## Decision

Decision: accepted.

Canonical `loop i in start..end` and legacy Stage-3 `for i in start..end`
continue to produce the same `ASTNode::ForRange` metadata shape, but their
range-header parsing is owned by one helper. This is a BoxShape cleanup only:
it does not change `LoopRange` / `ForRange` lowering semantics.

## Scope

- Share identifier / `in` / `..` header parsing through `parse_range_header`.
- Keep `for` as legacy compatibility surface.
- Keep canonical docs on `loop i in start..end`.
- Add a parser regression for legacy `for` producing the same range metadata shape.
- Guard passed locally.

## Non-goals

- Do not rename `ASTNode::ForRange`.
- Do not merge `ForRange` into `Loop`.
- Do not implement Stage1 `LoopRange` lowering.
- Do not add array iteration or step syntax.

## Acceptance

```bash
bash tools/checks/k2_wide_loopclean_range_parser_helper_guard.sh
```

## Next

Return to `PACKED-001 PackedArray eligibility gate` unless a separate
LoopRange rename decision is explicitly selected.
