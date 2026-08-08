# Hakorune Frontend AST

This crate owns parser-independent source structure only. It must not depend
on resolver, MIR, runtime, provider, backend, environment, or route policy.

## Ordered Box-method inventory

`BoxMethodInventoryV1` is the target sole authority for Box method order,
provenance, Box-local declaration site, diagnostic span, and exact-name
lookup.

```text
ordered entries
  = source/selected order authority

private name index
  = derived lookup only

iter_compat_name_order
  = explicit legacy execution-order projection only
```

The model exposes no HashMap, unordered iterator, `insert`, `extend`, or
`Deref<HashMap>`. Direct source ordinals are issued by the inventory.
Compatibility imports are explicitly `CompatibilityOnly` and can never back a
resolver source contract.

Current status: the passive model and focused tests are landed; connection to
`ASTNode::BoxDeclaration.methods` is the immediate next Refactor Series cell.
Until that cutover, the old HashMap remains production storage and this model
has production consumers zero.

## Boundaries

- parser: may issue explicit/generated rows through typed methods;
- build-gate parser: may merge an unpublished selected inventory only through
  atomic preflight/commit;
- resolver: may later consume only explicit source rows in source order;
- Builder/JSON compatibility: may use exact lookup or explicitly named
  compatibility projections, never claim source order;
- tests: may construct compatibility rows, but may not forge explicit source
  ordinals or provenance.

See
`docs/development/current/main/investigations/frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
for the authority and retirement contract.
