---
Status: closed — AST field and compatibility consumers cut over
Date: 2026-08-08
Decision: replace the AST field and compile consumers without claiming source authority
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
Prerequisite: `frontend-ordered-box-method-inventory-r0a-authority-correction-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-R1

## Goal

Replace `ASTNode::BoxDeclaration.methods: HashMap<String, ASTNode>` with
`BoxMethodInventoryV1` and migrate the minimum constructor/consumer surface so
the repository compiles. Inputs whose exact parser provenance is not yet wired
must enter through the atomic `CompatibilityOnly` batch constructor.

This is deliberate under-claiming. R1 removes the public HashMap ownership but
does not yet claim that ordinary parser rows are resolver-grade source rows.

## Required work

```text
AST field type replacement
manual/test/legacy constructors -> atomic CompatibilityOnly batch import
exact lookup consumers -> inventory.get
name-sorted compatibility consumers -> iter_compat_name_order
whole-field clone/move transports -> inventory clone/move
compile-time removal of direct HashMap insert/extend access through the AST field
```

If a consumer needs source order or typed provenance in this row, stop with
`NoSafeSlice`; do not promote compatibility rows.

## Nonclaims

```text
ordinary/interface/static ExplicitSource issuer
duplicate parser rejection
exact diagnostic Box/member sites
selected build-gate transaction cutover
property/delegate/macro provenance cutover
ordered JSON v2
Hako parser carrier/parity
CallableContract parser/resolver issuer
resolver target, CallSlot, Builder/MIR/provider activation
```

## Acceptance

```text
cargo test -p hakorune-frontend-ast box_method_inventory
cargo check -q
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Add focused compatibility tests where the type replacement changes behavior.
Keep every touched/new Rust file below 800 lines. The same implementation
commit updates the frontend AST README and any landed `docs/reference/**`
receipt affected by the actual field surface. R2 follows immediately with the
shared pending/direct substrate and interface/static source issuance. Ordinary
source issuance waits for the atomic mixed-owner R3 cutover; no unrelated
caller-zero product may be inserted between these rows.

## Implementation receipt

```text
ASTNode::BoxDeclaration.methods:
  BoxMethodInventoryV1

old public HashMap field:
  0

compatibility imports:
  explicit CompatibilityOnly only

generic map-like inventory APIs:
  IntoIterator/Deref/Index/insert/remove/get_mut/values = 0

parser-owned ExplicitSource seal:
  0; R2
```

Verification:

```text
cargo test -q -p hakorune-frontend-ast box_method_inventory  # 7 passed
cargo test -q --no-run                                      # green
cargo check -q                                              # green
git diff --check                                            # green
current_state_pointer_guard                                 # green
touched Rust files >= 800 lines                             # 0
```

Legacy consumers that still require a map receive one named compatibility
projection at their old boundary. No resolver, Recipe, Builder route, or
language acceptance was activated by this row.
