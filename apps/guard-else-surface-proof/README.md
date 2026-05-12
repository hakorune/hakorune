# guard-else-surface-proof

Proof app for C200 `guard ... else` surface.

This app demonstrates both branches:

- a passing guard continues execution
- a failing guard enters the `else` block and returns early

The surface is source sugar only. It must lower to the existing negative `if`
shape and must not introduce allocator-specific behavior or backend routing.
