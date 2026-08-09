---
Status: closed — implementation, fixture, guard, and owner docs landed
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
  -> ParserNumberScanOutcomeV1
       Ready(ParserNumberLexicalPartsV1)
         exact start/next position
         exact numeric spelling
         leading digit count
         kind = Integer | Float | Missing
         exact suffix spelling or absence
       InvalidStart(requested start)
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

Ready parts retained for compatibility-only projection:
  existing float spelling
  existing suffixed integer spelling
  zero-digit/missing spelling

typed admission declines:
  leading digit count = 0 or kind = Missing
  kind = Float
  suffix is present

scanner issue, never Missing:
  InvalidStart when start < 0 or start > source length
```

The current compatibility scanner's empty-input `0` default must be projected
only from `Missing`; synthetic zero must never enter exact-integer evidence.
`1.` remains the existing integer row ending before `.` because a float is
recognized only when at least one digit follows the decimal point.

`InvalidStart` is a typed scanner issue, not a zero-digit lexical row. The live
compatibility projection preserves the historical `Int 0@requested_start`
result for that issue. `start == source length` is a valid Ready/Missing row.
Null input remains an explicit Ready/Missing compatibility case because no
source length exists to validate.

## Structure

Use a small parser scan/result owner near
`lang/src/compiler/parser/scan/parser_number_scan_box.hako`. Keep constructors
confined to that scanner owner. Hako does not enforce constructor privacy, so
the repository guard enforces the issuance boundary. Do not add a public
factory, selection/filter helpers, or a second numeric scanner.

Do not add source annotations or materialization copies such as `src: String`,
`i: i64`, `"" + src`, or `0 + i`. Those were retired compiler-acceptance
workarounds; the compiler-side prerequisite is already fixed.

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
  .5
  1.
  empty source
  start exactly at source length
  null source
  non-digit start
  negative start
  start past source length

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
current pointers, commit, and push close together. This row changes no public
grammar, so `docs/reference/language/**` remains unchanged. The next row is
`HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1`, the sole same-pass Return product.

## Closeout receipt

The landed scanner now performs exactly one numeric traversal and returns a
parser-private `Ready(parts) | InvalidStart` outcome. `scan_int` consumes that
outcome once and is projection-only. Exact integer admission requires a Ready
Integer row, at least one leading digit, and no suffix. The live expression
parser still calls only `scan_int`; no expression, Return, body, method,
resolver, Home, Recipe, or MIR authority opened.

Focused coverage includes decimal integers, offset input, suffix, `1.5`, `.5`,
`1.`, empty/end/null/non-digit Missing rows, and both negative and past-end
InvalidStart rows with byte-for-byte legacy compatibility projection.

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

The same failure reproduced on the then-current clean HEAD with a direct call
to `ParserNumberScanBox.scan_int("42}", 0)`. Later import and compiler census
proved it was an upstream static-call result publication gap, not evidence
against the lexical-parts design or a scanner parameter-type gap.

The failed implementation is preserved only under the stash message:

```text
wip/h2-s2-s0 numeric lexical parts (fails generic loop carrier type gate)
```

Do not restore that stash: its source annotations and materialization copies
are retired compiler-acceptance workarounds.

## Resume receipt

`GENERAL-STATIC-CALL-RESULT-PUBLICATION-I0` closed the earlier imported
`StringHelpers` result gap. The unmodified direct
`ParserNumberScanBox.scan_int("42}", 0)` fixture now exits `0` and returns the
expected compatibility `JSON@pos` result. No scanner source annotation or
GenericLoop acceptance change was required. The failed probe above remains
historical evidence only; the landed S0 was rebuilt from the clean source with
its original lexical-parts scope and nonclaims.
