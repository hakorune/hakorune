---
Status: ready
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-D0`
Predecessor: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0` closed
Mode: BoxShape / disconnected same-pass statement product
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1

## Goal

Extend the existing live Return statement arm with one parser-private product
for the exact `return <unsuffixed decimal integer>` cohort. Compatibility JSON
and typed `Return(Present, LiteralInt)` must come from the same parse decisions
and the same numeric lexical outcome.

This row does not seal a SourceBody, attach a method, or activate new grammar.

## Exact owner path

```text
ParserStmtCoreBox existing Return arm
  -> existing expression/numeric grammar owner
  -> ParserNumberScanBox.scan_parts once
  -> exact Ready Integer lexical admission
  -> one LiteralInt node
  -> one Return(Present) node
  -> ParserNodeProductV1::Typed
  + unchanged compatibility Return JSON
```

Do not add a sibling Return parser, scan source text again, decode JSON, or
store a typed result in ambient parser state. If a small expression product is
needed, it is returned privately from the existing numeric/expression owner
and consumed by the existing Return arm.

## Exact first cohort

```text
Typed:
  `return 0`
  `return 42;`
  exact Ready Integer parts
  leading_digit_count > 0
  suffix absent
  next token is the existing Return terminator

CompatOnly:
  bare return
  return variable/call/binary/group/unary
  float or suffixed integer
  any other syntactically valid Return expression

ParseError:
  InvalidStart scanner outcome
  malformed Return/expression
  no parser progress
```

`CompatOnly` retains the existing compatibility fragment but publishes no
typed node. `ParseError` publishes neither a typed node nor a repaired default.
The lexical compatibility projection may preserve legacy output; it is not
semantic evidence.

## Product boundary

Use the existing `ParserNodeProductV1` branch vocabulary and parser-private
source-carrier builder. The exact Typed row owns:

```text
LiteralInt scalar spelling/value
Return presence = Present
exact next parser position
compatibility Return fragment as a sibling projection
```

It does not own a statement list, SourceBody root, method site, parameter
product, parser transaction, resolver identity, Home, Recipe, MIR, or runtime
meaning. Bare `return` remains distinct from `return 0` before compatibility
projection.

## Acceptance matrix

```text
positive Typed:
  return 0
  return 42;
  offset source position

CompatOnly with typed publication 0:
  return
  return x
  return f()
  return 1 + 2
  return (1)
  return -1
  return 1.5
  return 1usize

ParseError with typed publication 0:
  invalid numeric start
  malformed expression
  zero progress

structural:
  existing Return arm is the sole statement owner
  one numeric traversal for the exact row
  no JSON decode or source rescan
  no ambient typed side channel
  live compatibility output unchanged
  no SourceBody/method/H3 connection
  all touched/new Hako files below 800 lines
```

## Guard and docs

Add `hako_parser_rich_body_h2_s2_s1_guard.sh`, register it in
`docs/tools/check-scripts-index.md`, update the statement parser owner README
or nearest owner reference, and run:

```bash
bash tools/checks/hako_parser_rich_body_h2_s2_s1_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_s0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_r0_guard.sh
bash tools/checks/hako_parser_parameter_list_h2_s1_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/naming_charter_guard.sh
```

## Nonclaims

```text
SourceBody/list/root seal
multiple statements
method-bound body result
unpublished method transaction
ordinary Box production connection
H3 declaration seal
Take/share/release activation
Home capability or Flow
resolver target, Recipe, Builder, MIR, runtime
retry or fallback
```

## Closeout

Implementation, focused tests/guard, owner docs, current pointers, commit, and
push close together. The next row remains `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-I0`,
which owns the exact one-statement SourceBody seal and compatibility projection.
