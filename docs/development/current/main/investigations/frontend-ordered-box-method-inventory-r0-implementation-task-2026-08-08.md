---
Status: active implementation row — first bounded Refactor Series cell
Date: 2026-08-08
Decision: model only; connection is mandatory in immediate R1
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R0

## Goal

Add the passive frontend AST model that will replace the Box method HashMap:

```text
BoxMethodInventoryV1
BoxMethodEntryV1
BoxMethodProvenanceV1
BoxMethodSourceSelectionV1
BoxMethodDeclarationSiteV1
BoxMethodInventoryErrorV1
```

This is the first commit of one Refactor Series. It does not change parser
acceptance or `ASTNode::BoxDeclaration.methods` yet, but R1 must connect it
immediately. No other caller-zero proof asset may be inserted between R0 and
R1.

## Location and structure

```text
crates/hakorune_frontend_ast/src/box_method_inventory/
  mod.rs          public model/API, kept small
  error.rs        typed duplicate/identity errors
  tests.rs        focused model tests
```

Export the model from `crates/hakorune_frontend_ast/src/lib.rs`. Do not add
parser, resolver, MIR, runtime, provider, JSON, or environment dependencies to
the frontend AST crate.

## Required API

```text
empty()
iter_source_order()
get(name)
get_mut_preserving_identity(name)
into_source_order()
try_push(entry)
try_merge_selected_gate(selected, gate_site)
iter_compat_name_order()
```

The internal lookup index is private. Do not implement `Deref<HashMap>`,
unordered iteration, generic `insert/extend`, or source-authority
`From<HashMap>`.

Entry construction must make provenance/site explicit. Direct source ordinal
assignment is owned by the inventory, not the caller. Selected-gate merge
keeps branch origin and rebases selected ordinals only after full preflight.

## Focused tests

```text
direct rows retain insertion order and receive 0..N selected ordinals
duplicate direct name rejects without mutation
get and get_mut preserve entry identity
compat iteration is deterministic name order but does not mutate source order
selected-gate merge retains branch origin and rebases once
selected-gate collision rejects with destination unchanged
generated provenance never exposes an ExplicitSource site
```

## Nonclaims

```text
ASTNode field replacement
ordinary/interface/static parser cutover
generated producer cutover
JSON codec
Builder consumer migration
.hako issuer/parity
CallableContract parser acceptance
resolver contract/target
Recipe/Builder/MIR/runtime/production activation
```

## Acceptance

```text
cargo test -p hakorune-frontend-ast box_method_inventory
cargo check -q
git diff --check
```

All new Rust files remain below 800 lines; split tests from the model before
the model reaches 650--700 lines. The same commit updates the frontend AST
owner documentation and this task receipt. After R0 lands, CURRENT_STATE moves
directly to R1 rather than leaving the model parked.
