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

R4 now has an atomic AST reconstruction substrate and a strict recursive JSON
v2 codec. A complete set of `BoxMethodInventoryRoundtripRowV2` values is
preflighted for declaration/name identity, duplicate names, contiguous
selected ordinals, and non-empty selected-gate paths before an infallible
inventory construction. The public JSON root chooses v2 or legacy mode once;
malformed nested v2 values reject the whole root, while legacy JSON remains
`CompatibilityOnly(LegacyJsonV1)`. This is a descriptive transport boundary
only; it cannot seal resolver source truth or promote compatibility rows.

R5-S1 keeps the deferred non-Main static-Box Builder edge on the same
carrier: `PreparedProgramDeferredStaticBoxWorkV1` transfers
`BoxMethodInventoryV1` directly to `ProgramDeferredStaticBoxLifecycleV1`.
The historical name-order behavior is an explicit
`into_compatibility_name_order()` projection inside the compatibility batch;
it is not source order and does not promote resolver authority.

R5-S2 carries the same inventory directly through the connected static-`Main`
compatibility child-port family. The raw/normal forwarding ports and the
compatibility batch no longer accept or reconstruct `HashMap<String, ASTNode>`.
The compatibility leaf still calls `declaration_order::sorted_method_entries`
for the historical helper-before-main execution order; this remains a named
compatibility projection, not source-order authority.

R5-S3 closes the Builder migration: production Builder callers no longer
round-trip the inventory through `clone_compatibility_map()` or
`into_compatibility_map()`. Remaining map constructors are test/transport
compatibility, and explicit name-order views remain retained where they own
historical slot, lowering, or catalog ordering.

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
