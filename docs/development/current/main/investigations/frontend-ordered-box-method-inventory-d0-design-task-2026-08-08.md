---
Status: design stop — canonical frontend method source authority
Date: 2026-08-08
Decision: required before resolver declared instance-method contract I0
Parent: `language-typed-callable-profile-d0-design-task-2026-08-08.md`
---

# FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0

## Decision brief

```text
Decision:
  Replace HashMap-as-source-authority with one AST-owned ordered method
  inventory. A name lookup is a private derived view, never a second truth.

Source authority + canonical issuer:
  parser-owned exact Box declaration + ordered method entries, each carrying
  source/generated provenance and a structural declaration site.

Non-authority:
  HashMap iteration/name order, resolver reconstruction, Builder catalogs,
  FuncScannerBox, Program(JSON) order, method/Box names, or Span alone.

Fail-fast boundary:
  duplicate source method, ambiguous selected build-gate merge, lost
  provenance/site/order, or unsupported Hako inventory owner stops before
  resolver receipt issuance.

Smallest next slice:
  finish the producer/provenance/site/merge census and select one
  BoxMethodInventoryV1 API plus compatibility-map retirement boundary.

Non-claims:
  CallableContract parser activation, nominal type resolution, semantic
  contract issuer, resolver target, Recipe, Builder/MIR, or production route.
```

## Evidence for the stop

The current frontend cannot prove the required source facts:

```text
ASTNode::BoxDeclaration.methods:
  HashMap<String, ASTNode>

ordinary/interface/static method insertion:
  insert return value ignored; duplicate silently overwrites

selected member gate merge:
  HashMap::extend; collision silently overwrites

Box/method span:
  current constructors commonly use Span::unknown()

JSON compatibility output:
  iterates the HashMap; it is not source-order authority
```

Once the AST exists, the overwritten declaration and original order cannot be
recovered. Resolver-side sorting or `(Box name, method name, arity)` lookup is
therefore forbidden as a repair.

## Canonical product

Use one source authority, not public `Vec + HashMap` dual truth:

```text
BoxMethodInventoryV1
  ordered entries
  private derived name lookup

BoxMethodEntryV1
  method key
  exact FunctionDeclaration
  provenance
  diagnostic span
```

Provenance is mandatory:

```text
Source { selected_method_ordinal }
GeneratedProperty
GeneratedDelegate
GeneratedMacroOrImport
CompatibilityOnly
```

Only an as-written `FunctionDeclaration` with `Source` provenance may back the
first `CallableContract(query)` resolver receipt. Property getter/compute
methods, implicit constructors, macro/import rows, delegate forwarding, and
compatibility reconstruction never borrow a source-method ordinal.

Constructors remain a separate inventory. Source properties are Box members,
not method declarations. A future general `BoxMemberDeclV1` may unify the
surface, but it is not required for this bounded instance-method row.

## Structural source identity

The first exact identity is compilation-profile scoped:

```text
source/catalog brand
program Box statement ordinal
selected source-method ordinal
```

`Span` is diagnostic only. Because build gates select a compilation profile,
the flat method ordinal is explicitly the selected compilation's identity; it
does not claim cross-profile as-written identity. A future cross-profile
identity would need the gate path as part of the site and requires a separate
Decision.

## Build-gate transaction

Both branches may be parsed into inventories, but only the selected inventory
is merged. Merge is fallible and transactional:

```text
preflight all selected keys against destination
  -> reject any collision before mutation
  -> append selected rows in source order
  -> rebase selected source-method ordinals once
```

Pruning may transform method bodies, but it preserves entry provenance and
ordinal. Existing order-insensitive branch-signature comparison may remain a
derived check; it does not become source identity.

## `.hako` boundary

Current `.hako` surfaces are not the missing authority:

```text
FuncScannerBox:
  compatibility scanner; skips/rewrites method details and has no exact
  duplicate/site receipt. It must not be promoted or enlarged.

ParserDeclarationBox:
  declaration evidence only; semantic publication remains disabled.
```

Therefore the implementation series needs a dedicated `.hako` ordered method
inventory issuer. Its owner/location is selected in this D0 before code; no
guess is added to `FuncScannerBox`. Rune spelling parity remains separately
owned by `lang/src/compiler/parser/rune/rune_contract_box.hako`.

## Finite implementation series

This is one BoxShape objective and may use Refactor Series Mode. It does not
add a callable semantic shape until the final source issuer/parity cell.

```text
FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0
  A. producer census and API/retirement Decision
  B. AST BoxMethodInventoryV1 + derived compatibility lookup (behavior neutral)
  C. Rust explicit/generated provenance, duplicate reject, selected-gate merge
  D. prune/JSON roundtrip preservation and source-site tests
  E. dedicated `.hako` inventory issuer + normalized parity
  F. CallableContract(query) Rust/.hako parser spelling activation
  G. same-slice reference/README closeout
```

Cells B--G may be 2--5 commits only while they remain one BoxShape series.
Changing accepted method semantics, adding overloads, or opening resolver
targets is a separate row.

## Mandatory producer census

Before cell B, classify every producer/consumer as one of:

```text
ExplicitSource
SelectedBuildGate
GeneratedProperty
GeneratedDelegate
GeneratedMacroOrImport
CompatibilityOnly
```

At minimum include ordinary, interface, static, record, property, constructor,
build-gate prune/merge, macro JSON, and selected Builder compatibility readers.
The census chooses the one derived-lookup API and names its retirement
condition; it does not mechanically migrate unrelated consumers.

## Required tests

```text
positive:
  source order retained
  exact selected source ordinals
  selected-gate order/rebase
  prune/JSON preserves provenance and order
  generated property rows absent from source-method rows
  Rust/.hako normalized inventory and rune spelling parity

reject:
  direct duplicate method
  duplicate inside either gate branch
  duplicate between outer member and selected branch
  generated row forged as Source
  missing/duplicate ordinal
  name-lookup order used as source order
```

No overload authority exists, so same-Box duplicate as-written method names
are parser errors. Overload identity requires a separate language Decision.

## File/size guard

Known implementation fronts include:

```text
crates/hakorune_frontend_ast/src/ast_node.rs
src/parser/declarations/box_def/state.rs
src/parser/declarations/box_def/body.rs
src/parser/declarations/box_def/interface.rs
src/parser/declarations/static_def/members.rs
src/parser/build_cfg/prune.rs
src/macro/ast_json/joinir_compat.rs
src/macro/ast_json/roundtrip.rs
lang/src/compiler/parser/rune/rune_contract_box.hako
```

`roundtrip.rs` is already near the 760-line design trigger, so its Box codec is
split by responsibility before inventory serialization grows it.
`FuncScannerBox` is also near the trigger and receives no new authority.
Every source file stays below 800 lines.

## Exit criteria

1. One `BoxMethodInventoryV1` owns order/provenance/site and name lookup is
   derived/private.
2. Every producer class and generated/source boundary is explicit.
3. Build-gate collision/rebase behavior is deterministic and pre-mutation.
4. The dedicated `.hako` issuer boundary is named without using FuncScanner.
5. The finite series, focused tests, compatibility-map retirement, and exact
   same-slice README/reference updates are fixed.

Only then may cell B open. Resolver declared-contract I0 stays parked until
the full Rust/.hako source inventory and `CallableContract(query)` spelling
parity are closed.
