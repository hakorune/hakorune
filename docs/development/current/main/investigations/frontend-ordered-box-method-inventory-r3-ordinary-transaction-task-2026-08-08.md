---
Status: queued after R2; do not execute early
Date: 2026-08-08
Decision: ordinary Box sole-inventory and generated/selected atomic cutover
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
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
  prepared whole delegate batch
  -> full collision/provenance preflight
  -> one commit
```

Generated rows never expose `ExplicitSource`. Selected user-written methods
remain `ExplicitSource` with their outer-to-inner gate path. Ordinary postfix
handling uses the shared R2 pending method and never mutates a published
inventory entry.

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
