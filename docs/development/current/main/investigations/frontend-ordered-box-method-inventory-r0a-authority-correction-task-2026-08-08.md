---
Status: closed — authority/API correction implemented and verified
Date: 2026-08-08
Decision: raw inventory is an ordered AST carrier, never a resolver source capability
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R0A

## Why this correction is mandatory

Independent review found three R0 APIs that are too strong:

```text
iter_source_order:
  CompatibilityOnly batches can call it and falsely promote legacy order.

get_mut_preserving_identity:
  &mut ASTNode can change method identity while the private index stays stale.

try_push_explicit_source:
  public descriptive provenance cannot prove parser issuance or completeness.
```

## Corrected boundary

```text
BoxMethodInventoryV1:
  ordered selected-declaration AST carrier
  duplicate-free private index
  descriptive provenance only

future parser-owned seal:
  complete parse + duplicate freedom + selected Box membership + exact sites
  resolver-grade explicit-source loan
```

## Required implementation

```text
rename iter_source_order -> iter_selected_declaration_order
remove get_mut_preserving_identity
keep immutable get and atomic merge/batch validation
document raw provenance as descriptive/non-authoritative
```

If postfix/macro mutation later needs write access, use a body-limited view or
clone -> full identity validation -> atomic replace. Arbitrary `&mut ASTNode`
must not return.

## Nonclaims

```text
parser seal or resolver capability
AST field/parser cutover
Hako parity
CallableContract issuer, target, Recipe, Builder, or runtime activation
```

## Acceptance

```text
cargo test -p hakorune-frontend-ast box_method_inventory
cargo check -q
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Update the frontend AST README, D0 task, and landed reference/task receipt in
the same commit. All files remain below 800 lines. R1 opens only after R0A.

## Implementation receipt

```text
iter_source_order: removed
iter_selected_declaration_order: landed
into_source_order: removed
into_selected_declaration_order: landed
arbitrary &mut ASTNode lookup: removed
raw ExplicitSource resolver authority: 0
focused tests: 7 passed
cargo check: passed with repository-baseline warnings
production AST field consumers: 0 (R1)
```
