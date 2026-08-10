---
Status: reopen audit pending; prior GenericLoop blocker is closed, but this
row still lacks its registered guard and green acceptance evidence
Date: 2026-08-09
Row: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1`
Parent: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1`
Predecessor: `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R0` closed
Mode: BoxShape / behavior-neutral in-place expression traversal
---

# HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-R1

## Goal

Make the existing expression precedence traversal return one parser-private
rich result while keeping every current string-returning API as a projection.
The traversal may retain one exact unsuffixed integer lexical witness; every
operator or non-integer shape deterministically reduces it to CompatOnly.

This is an in-place refactor of the existing grammar owner, not a parallel
expression parser.

## Product

```text
ParserExpressionParseProductV1
  branch = ExactInteger | CompatOnly | ParseError
  compatibility fragment
  exact next position
  ExactInteger -> ParserNumberLexicalPartsV1
  ParseError   -> parser-private issue
```

It owns lexical/parser shape only. It does not issue a semantic type,
`ParserNodeProductV1`, source-carrier builder, Return, SourceBody, method,
resolver, Home, Recipe, MIR, or runtime meaning.

## One traversal

```text
ParserExprBox.parse_number_product2
  -> ParserNumberScanBox.scan_parts once
  -> ParserNumberScanBox.project_compat(outcome)
  -> ExactInteger / CompatOnly / ParseError

ParserExprBox.parse_factor_product_in_context2
  numeric FIRST -> parse_number_product2
  other FIRST   -> existing factor owner once -> CompatOnly

ParserExprPrecedenceBox product traversal
  exact leaf and no operator -> preserve ExactInteger
  unary/infix/ternary/group/postfix/other -> CompatOnly
  child ParseError -> propagate ParseError

legacy parse_*2 API
  -> product traversal
  -> compatibility fragment only
```

Do not pre-scan and then call the old numeric parser. Do not add an ambient
`last_typed_expr`, JSON decoding, source substring rescan, or duplicated
precedence loops. The product-returning functions are the actual traversal;
legacy string methods are thin projections.

## Exact disposition

```text
ExactInteger:
  Ready Integer
  leading_digit_count > 0
  suffix absent
  no unary/infix/ternary/group/postfix wrapper

CompatOnly:
  Float
  any valid non-integer expression
  exact integer participating in any larger expression

ParseError:
  InvalidStart
  Ready Missing at a numeric parse position
  current-profile suffixed integer rejection
  existing malformed/freeze result
```

`scan_int` compatibility remains byte-for-byte unchanged. If a product carries
a ParseError, its compatibility fragment may preserve the legacy parser/freeze
surface, but that fragment is never typed evidence.

## Structure and line limits

- put the product model in a small expression-owned file;
- refactor `parser_expr_precedence_box.hako` in place;
- add only the one factor-product callback needed by the `ParserBox` facade;
- keep `ParserBox` below 760 lines and every other touched Hako source below
  800 lines;
- avoid a new generic Plan/Recipe or public selection/filter API.

## Acceptance matrix

```text
ExactInteger:
  0
  42
  offset x42

CompatOnly:
  1.5
  .5
  -1
  1 + 2
  1 * 2
  1 < 2
  1 && 2
  1 ? 2 : 3
  (1)
  variable/call

ParseError:
  invalid start
  missing numeric token
  1usize under current profile

regression:
  legacy expression JSON and gpos unchanged
  scan_parts called once on the numeric product route
  string APIs contain no independent precedence traversal
  Return/SourceBody/parser-node connection = 0
```

## Verification

Add/register `hako_parser_rich_body_h2_s2_s1_r1_guard.sh` and run:

```bash
bash tools/checks/hako_parser_rich_body_h2_s2_s1_r1_guard.sh
bash tools/checks/hako_parser_source_carrier_p0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_s0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_r0_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/naming_charter_guard.sh
```

## Nonclaims

```text
ParserNodeProduct or SourceCarrierBuilder issuance
Return statement product
SourceBody/list/root seal
method/H3 connection
grammar expansion
Home, resolver, Recipe, MIR, runtime
```

## Closeout

Implementation, focused fixture/guard, expression owner docs, current pointers,
commit, and push close together. First run the focused
`H2-S2-S1-R1-REOPEN-AUDIT`; only after its guard and predecessor gates are green
may `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S1-I0` open. This row remains
parser-only and does not connect a method/result seal.
