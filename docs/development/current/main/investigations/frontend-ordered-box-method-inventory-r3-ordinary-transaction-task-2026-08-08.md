---
Status: closed — sole ordinary inventory and atomic generated/selected transactions landed
Date: 2026-08-08
Decision: ordinary Box sole-inventory and generated/selected atomic cutover
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
Next: `frontend-ordered-box-method-inventory-r4-json-codec-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R3

## Goal

Replace the ordinary Box mixed `HashMap` owner exactly once. Direct source
methods, selected build-gate methods, generated property methods, and generated
delegate methods enter one sole `BoxMethodInventoryV1` through typed atomic
transactions. No dual ledger or compatibility promotion is permitted.

## Required transactions

```text
ordinary direct method:
  PendingExplicitMethodV1 -> try_push_explicit_source

selected build gate:
  unpublished branch inventory
  -> full collision/path/ordinal preflight
  -> one merge and selected-ordinal rebase

generated property:
  PreparedGeneratedPropertyMethodBatchV1
  -> full internal/existing collision preflight
  -> one commit

generated delegate:
  DelegateDecl (or an equivalent AST carrier) retains its exact
  BoxMethodSourceSelectionV1
  -> selected merge prepends the outer-to-inner gate path
  prepared whole delegate batch
  -> full collision/provenance preflight
  -> one commit
```

Generated rows never expose `ExplicitSource`. Selected user-written methods
remain `ExplicitSource` with their outer-to-inner gate path. Ordinary postfix
handling uses the shared R2 pending method and never mutates a published
inventory entry.

The delegate lowerer consumes only the selection carried from parsing. It may
not manufacture `Direct` when the source selection is unavailable. Legacy
transport rows remain non-authoritative rather than being promoted to a fresh
source provenance.

## Required owner cutover

```text
BoxMemberState.methods = BoxMethodInventoryV1
HashMap merge/extend/insert/get_mut method ownership = 0
ordinary final compatibility conversion = 0
delegate compatibility round trip = 0
read-only validators consume explicit inventory read APIs
```

Constructors remain a separate authority and are outside this row.

## Acceptance

```text
ordinary direct lexical order
direct/direct duplicate reports both sites
direct/property and property/property collision reject atomically
once/birth_once multi-method failure leaves inventory unchanged
selected then/else and nested outer-to-inner path
selected ordinal rebase
selected collision leaves the whole state unchanged
generated row cannot become ExplicitSource
delegate collision leaves the whole inventory unchanged
fresh ordinary/interface/static parser output has zero CompatibilityOnly rows
parser Box-method HashMap ownership = 0
all touched source files < 800 lines
```

## Stop lines

```text
no resolver declaration or CallableContract issuer
no target, source-bound call relation, Recipe, Builder, or runtime route
no rollback repair after partial insertion
no source-order reconstruction from names
```

The implementation commit updates the exact parser/AST owner README, the
landed-status paragraph in `docs/reference/language/callable-contracts.md`,
this card, and `CURRENT_STATE.toml`. Reference updates are part of the
implementation row, not a deferred cleanup task.

## Landed receipt

```text
BoxMemberState.methods:
  BoxMethodInventoryV1 only

ordinary explicit method:
  unpublished PendingExplicitMethodV1
  -> postfix mutation
  -> one fallible inventory commit

selected build gate:
  exact syntactic branch-member ordinals retained in parser-private staging
  -> full provenance/collision/ordinal preflight
  -> one inventory merge

property/delegate generation:
  complete PreparedGeneratedBoxMethodBatchV1
  -> full internal/existing collision preflight
  -> one commit

delegate source authority:
  DelegateDecl carries Direct/SelectedBuildGate selection from parsing
  legacy JSON remains CompatibilityOnly(LegacyJsonV1)
  missing source selection never fabricates Direct
```

The parser-private source-member-ordinal staging is not a second method
ledger. It contains no declaration or lookup authority and exists only until
the selected branch inventory is atomically merged.

Focused receipts:

```text
cargo test -q -p hakorune-frontend-ast box_method_inventory
cargo test -q --lib parser_box_method_inventory_r2
cargo test -q --lib parser_unified_members_property_emit
cargo test -q --lib parser_delegate_surface
cargo test -q --lib parser_birth_once
cargo check -q
```

All are green. Box-method mutation through parser `HashMap::insert`,
`HashMap::extend`, or published-entry `get_mut` is zero. All touched source
files remain below 800 lines.
