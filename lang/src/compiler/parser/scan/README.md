# Parser scan helpers

This directory owns parser-private lexical traversal helpers. It does not own
resolved semantic types, source-body meaning, Recipe values, MIR values, or
runtime behavior.

## Numeric scan boundary

`ParserNumberScanBox.scan_parts` is the sole numeric-token traversal. It emits
`ParserNumberScanOutcomeV1`: either `Ready(ParserNumberLexicalPartsV1)` or an
`InvalidStart` scanner issue. Ready parts contain exact source positions,
spelling, leading digit count, lexical kind, and suffix. `scan_int` is the live
legacy `JSON@pos` projection of that same outcome; it must not scan or decode
the source again.

The exact unsuffixed-decimal predicate is lexical admission only. It does not
issue a semantic integer type. `Float`, suffixed integer, and `Missing` rows
remain available solely so compatibility output can be projected without a
second traversal. Null, empty, end-of-source, and non-digit inputs use a valid
`Missing` row. Negative or past-end start positions use `InvalidStart`, never
`Missing`. The compatibility projection preserves the legacy synthetic zero
for both cases, but synthetic zero is never exact integer evidence.

Hako does not currently enforce private constructors. Repository guards keep
all lexical-parts and outcome construction inside `ParserNumberScanBox`.

The live expression parser continues to call `scan_int` until the later
same-pass expression/body row. Do not connect these parts directly to Return,
Home, resolver, Recipe, Builder, or MIR authority.
