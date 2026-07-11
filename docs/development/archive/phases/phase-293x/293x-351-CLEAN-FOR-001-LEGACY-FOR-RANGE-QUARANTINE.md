# 293x-351 CLEAN-FOR-001 legacy for-range quarantine

Status: landed
Date: 2026-05-15

## Decision

`for i in start..end { ... }` is legacy Stage-3 compatibility syntax, not a
canonical Hakorune repetition surface. The canonical source form remains:

```hako
loop i in start..end {
    ...
}
```

## Implementation

- Rename the parser helper from `parse_for_range_stage3` to
  `parse_legacy_for_range_stage3`.
- Keep the legacy `FOR` token route only under the existing Stage-3 gate.
- Keep both legacy `for` and canonical `loop i in` routed through the same
  range-header parser and `ASTNode::ForRange` metadata shape.

## Non-goals

- Do not remove legacy `for` syntax in this cleanup row.
- Do not introduce source-level desugaring to condition loops.
- Do not merge `ForRange` into `Loop`.
- Do not add independent lowering semantics for `for`.

## Retire condition

Legacy `for` can be removed only after compatibility users are audited and
current docs/reference explicitly mark the removal. Until then, it is
quarantined as compatibility input.

## Follow-up

Proceed to `CLEAN-DEAD-001` first `#[allow(dead_code)]` cluster audit.
