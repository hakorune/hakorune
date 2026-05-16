# userbox-nullable-loop-return-proof

Status: Active
Scope: allocator-neutral proof for `MIR-ROW-C` nullable user-box object return.

This app proves that a same-module method returning a loop-selected
`null | object` value publishes enough route metadata for pure-first EXE to use
the returned object in both a field read and a same-module method call.

Stop lines:

- no hako_alloc dependency
- no backend matcher shortcut
- no provider / hook / host allocator replacement behavior
