# Hakorune Frontend AST

This crate owns parser-independent source structure only. It must not depend
on resolver, MIR, runtime, provider, backend, environment, or route policy.

## Ordered Box-method inventory

`BoxMethodInventoryV1` is the canonical ordered AST carrier for Box methods.
It owns selected-declaration order, descriptive provenance, Box-local ordinal,
diagnostic span, and exact-name lookup. It is not a resolver source authority.

```text
ordered entries
  = selected-declaration carrier order

private name index
  = derived lookup only

iter_compat_name_order
  = explicit legacy execution-order projection only
```

The model exposes no HashMap, unordered iterator, arbitrary mutable AST borrow,
`insert`, `extend`, or `Deref<HashMap>`. Selected ordinals are issued by the inventory.
Compatibility imports are explicitly `CompatibilityOnly` and can never back a
resolver source contract.

Raw `ExplicitSource` provenance is also descriptive. A later parser-owned seal
must prove complete parsing, duplicate freedom, selected Box membership, and
exact source identity before lending resolver-grade rows.

Current status: R3 is landed. `ASTNode::BoxDeclaration.methods` now stores
`BoxMethodInventoryV1`, and compatibility consumers use explicitly named
exact-lookup, selected-order, name-order, or `CompatibilityOnly` projections.
The old public HashMap field and arbitrary mutable method access are gone.

Interface and static parser branches now issue fresh ordered `ExplicitSource`
rows with duplicate first/second coordinates. An unpublished
`PendingExplicitMethodV1` owns postfix mutation before one inventory commit.
`build_cfg` consumes and transforms declarations without changing inventory
name, order, provenance, site, or diagnostic span; delegate-free lowering
passes the inventory through unchanged.

Ordinary, interface, and static source methods now enter as ordered
`ExplicitSource` rows. Selected build gates retain their exact outer-to-inner
gate path and syntactic branch-member ordinal. Property and delegate helpers
enter only through complete generated batches whose collisions and ordinal
capacity are checked before one commit. A parsed `DelegateDecl` carries its
exact source selection; legacy JSON delegates remain compatibility-only and
cannot manufacture `Direct` provenance.

Raw rows still do not issue a resolver-grade source capability. Compatibility
rows cannot be upgraded. JSON v2 preservation is the separate R4 row; JSON v1
decoding remains explicitly compatibility-only.

## Boundaries

- parser: may issue explicit/generated rows through typed methods;
- build-gate parser: may merge an unpublished selected inventory only through
  atomic preflight/commit;
- resolver: may later consume only rows lent by the parser-owned seal;
- Builder/JSON compatibility: may use exact lookup or explicitly named
  compatibility projections, never claim source order;
- tests: may construct compatibility rows, but may not forge explicit source
  ordinals or provenance.

See
`docs/development/current/main/investigations/frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
for the authority and retirement contract.
