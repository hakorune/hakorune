---
Status: parked design stop — required before Hako inventory issuer
Date: 2026-08-08
Decision: no standalone source scanner; one typed ordinary Box parser branch
Parent: `frontend-ordered-box-method-inventory-d0-design-task-2026-08-08.md`
---

# HAKO-PARSER-BOX-DECLARATION-CARRIER-D0

## Problem

The `.hako` compiler has no canonical ordinary Box declaration product today.
Existing surfaces are not promotable:

```text
FuncScannerBox / StageBRuneBox:
  compatibility rescans and source rewriting

tools/hako_parser:
  tool-only line scanner and name-sorted output

ParserDeclarationBox:
  narrow declaration evidence string; semantic publication disabled

source_carrier_v1:
  typed parser-private substrate; declaration vocabulary/connection absent
```

Creating `HakoBoxMethodInventory.scan(source)` would introduce a second parser
authority and is rejected.

## Required architecture

```text
ParserProgramBox
  -> ParserDeclarationBox typed product entry
  -> one ordinary Box parser branch
  -> declaration children built once
  -> sealed Box declaration + ordered method inventory
  -> ProgramJSON/string only as one-way compatibility projection
```

The D0 must decide the typed declaration carrier vocabulary, source refs,
builder/sealer owner, build-gate selection transaction, duplicate/site errors,
and compatibility projection. It must preserve the source-carrier rule that
semantic nodes are not raw MapBox or JSON.

## Candidate physical layout

```text
lang/src/compiler/parser/declaration_carrier_v1/
  README.md
  declaration_refs_v1.hako
  declaration_records_v1.hako
  declaration_builder_v1.hako
  declaration_sealer_v1.hako
  declaration_outcome_v1.hako

lang/src/compiler/parser/decl/
  parser_box_declaration_product_box.hako
  parser_box_method_inventory_box.hako
```

Names remain provisional until D0 code/owner census confirms they do not
duplicate the existing source-carrier vocabulary.

## Mandatory stop lines

```text
no FuncScanner/StageBRune/tools parser import
no source/body slice rescan
no ProgramJSON/MapBox semantic carrier
no name sorting as source order
no parser_box.hako growth (currently 787 lines)
no semantic publication before typed product seal
no CallableContract contract issuance before Rust/.hako inventory parity
```

If `ParserBox` facade wiring is unavoidable, split it below the hard limit
before adding a call. Every new `.hako` file stays below 800 lines.

## Planned rows

```text
H0 D0 owner/vocabulary/API decision
H1 disconnected typed declaration substrate + guard
H2 canonical ordinary Box parser product branch
H3 ordered method inventory/duplicate/site issuer
H4 selected build-gate transaction/rebase
H5 test-only normalized Rust/.hako parity
H6 CallableContract(query) carriage + same-slice reference update
```

The current source-carrier guard is updated only in the explicit connection
row. H1 cannot remain a long-lived caller-zero asset.

## Parity evidence

The normalized parity report is test-only and ordered:

```text
inventory_v1
box=<box-statement-ordinal>:<name>:<instance|static>
method=<selected-ordinal>:<name>:<arity>:<result-token>:<provenance>:<runes>
```

Rust and `.hako` consume the same source and selected build profile. The gate
compares ordered rows and duplicate disposition. Spans/brands are not compared
as strings; their structural presence is asserted separately.
