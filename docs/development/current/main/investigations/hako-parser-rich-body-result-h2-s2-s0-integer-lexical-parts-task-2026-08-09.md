---
Status: parked — blocked by exact carrier-source census
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-D0`
Mode: BoxShape / disconnected typed lexical substrate
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0

## Goal

Give the existing numeric scanner one private typed lexical result so the
later same-pass expression product does not reconstruct integer meaning or the
next parser position from the compatibility `JSON@pos` string.

This row changes no accepted grammar. `ParserNumberScanBox.scan_int` remains
the compatibility projection used by the live parser.

## Authority boundary

```text
existing ParserNumberScanBox numeric traversal
  -> ParserNumericLexicalPartsV1
       exact start/next position
       exact numeric spelling
       digit count
       kind = Integer | Float | Missing
       exact suffix spelling or absence
  -> exact integer typed admission
  -> compatibility JSON@pos projection
```

The scanner traverses the source once. The typed parts and compatibility text
must be projections of that same traversal; neither may parse the other.

The lexical parts are parser-private data, not a semantic numeric type, Recipe
value, MIR value, or public language capability. Float, suffix, and missing
rows remain representable so the compatibility projection never requires a
second scan.

## Exact first cohort

```text
accepted exact-integer typed admission:
  one or more ASCII decimal digits
  kind = Integer
  no suffix

parts retained for compatibility-only projection:
  existing float spelling
  existing suffixed integer spelling
  zero-digit/missing spelling

typed admission declines:
  digit count = 0 or kind = Missing
  kind = Float
  suffix is present

scanner issue:
  invalid start range
```

The current compatibility scanner's empty-input `0` default must be projected
only from `Missing`; synthetic zero must never enter exact-integer evidence.
`1.` remains the existing integer row ending before `.` because a float is
recognized only when at least one digit follows the decimal point.

## Structure

Prefer a small parser scan/result owner near
`lang/src/compiler/parser/scan/parser_number_scan_box.hako`. Keep constructors
private to that owner. Do not add public selection/filter helpers or a second
numeric scanner.

The live `parse_number2` path may continue consuming `scan_int` in this row.
Connecting typed parts to `ParserNodeProductV1` belongs to the later same-pass
expression/Return row, not S0.

## Acceptance tests

```text
positive:
  0
  42
  exact start/next position after digits
  compatibility JSON remains byte-for-byte equivalent

parts + typed-admission matrix:
  123}
  123usize
  1.5
  1.
  empty source
  null source
  non-digit start
  invalid start range

structural:
  one numeric traversal owner
  no JSON decode
  no source rescan
  no ParserNodeProductV1 connection
  no Return/SourceBody/method transaction
  no Take/Home/resolver/Recipe/MIR authority
  every touched/new Hako source below 800 lines
```

## Verification

Add a focused `hako_parser_rich_body_h2_s2_s0_guard.sh`, register it in
`docs/tools/check-scripts-index.md`, and run at least:

```bash
bash tools/checks/hako_parser_rich_body_h2_s2_s0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_r0_guard.sh
bash tools/checks/hako_parser_box_declaration_h1_guard.sh
bash tools/checks/hako_parser_parameter_list_h2_s1_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/naming_charter_guard.sh
```

## Nonclaims

```text
typed expression product
Return(Present, LiteralInt)
SourceBody coverage
ordinary Box method connection
unpublished method transaction
H3 seal or inventory publication
Take/share/release syntax
Home capability or Flow
resolver target, Recipe, Builder, MIR, runtime
retry or fallback
```

## Closeout

Implementation, focused fixture/guard, scan-owner README/reference receipt,
current pointers, commit, and push close together. The next row is
`HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1`, the sole same-pass Return product.

## Implementation probe receipt

The first implementation probe was intentionally not committed. A direct VM
fixture calling the existing loop-bearing numeric scanner froze before the new
lexical product could be exercised:

```text
[plan/freeze:contract]
generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(3) }
```

The same failure reproduces on clean HEAD with a direct call to the existing
`ParserNumberScanBox.scan_int("42}", 0)`. It is therefore a pre-existing
carrier type-publication gap, not evidence against the lexical-parts design.

The selected carrier reaches GenericLoop as `ValueId(3)`, but the exact
initializer producer is not yet proven. A formal parameter alias, a local
copy/concatenation result, and other initializer producers must be
distinguished before choosing the publication owner. `GenericLoop` correctly
refuses to invent the missing transient type.

The failed implementation is preserved only as:

```text
stash@{0}: wip/h2-s2-s0 numeric lexical parts
           (fails generic loop carrier type gate)
```

Do not resume it until
`HAKO-PARSER-NUMERIC-SCAN-CARRIER-SOURCE-D0` identifies the exact producer,
and the resulting producer-specific executable prerequisite is green.
