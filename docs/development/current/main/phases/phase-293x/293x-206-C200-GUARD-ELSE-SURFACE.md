# 293x-206: C200 Guard Else Surface

Status: Complete

## Purpose

Add a small early-exit source surface for proof/application code:

```hako
guard condition else {
    return 0
}
```

The surface is syntax only. It lowers to the existing negative `if` shape:

```hako
if !(condition) {
    return 0
}
```

## Implementation

- `guard` is a tokenizer keyword.
- The parser accepts `guard expr else block` as a control-flow statement.
- The canonical AST remains `ASTNode::If`.
- The generated condition is `UnaryOp::Not` over the original guard
  condition.

## Stop Line

C200 does not add:

- a `Guard` AST node
- exception or fallback semantics
- allocator-specific behavior
- a backend route selector
- `.inc` provider/hook/native allocator matching

## Acceptance

- `docs/reference/language/EBNF.md` records `Decision: accepted`.
- Parser regression proves `guard` lowers to negative `If`.
- Missing `else` is rejected.
- Proof app demonstrates pass-through and early-exit branches.
- Focused guard confirms no shim leakage.
