---
Status: NoSafeSlice at reopen audit; the parser-only product WIP reaches the
existing GenericLoop representation blocker before fixture execution
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

The reopen audit was attempted with the existing parser-only expression product
WIP and a fixture that contains no `loop` statement. The focused guard still
reaches the existing GenericLoop representation failure while compiling the
imported parser surface:

```text
[plan/freeze:contract] generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(113) }
```

This is a predecessor/compiler capability blocker, not evidence that the
expression product is accepted. GenericLoop repair, a fixture workaround that
changes the accepted shape, and compatibility fallback are explicitly out of
scope. The product WIP, guard, and fixture remain parked as recoverable WIP;
the row must not advance to `H2-S2-S1-I0` until the blocker is resolved by its
own owner and the reopen audit is rerun with predecessor/parity gates green.

## Dependency-owner audit (2026-08-11)

The blocker is the existing transient-result publication family, not a new
GenericLoop semantic rule and not an R1 parser defect. The canonical owner
chain is already documented by:

```text
exact source call/result contract
  + successful CompletedUnifiedValueCallEmissionV1
  -> one non-Clone lowering-time result-publication receipt
  -> type_ctx[final destination]
  -> existing GenericLoop verifier
```

The GenericLoop carrier consumer remains verifier-only. The next design work
must reopen or select the existing owners, in this order:

1. `generic-raw-structured-generic-loop-carrier-representation-d0-task-2026-08-07.md`
   confirms the consumer boundary and exact missing transient-type contract.
2. `generic-raw-structured-static-call-result-publication-d0-task-2026-08-07.md`
   owns the current-owner static-call source contract and success-only
   publication bridge.
3. The existing I1/source-bound handoff row must prove the exact parser call
   site before any R1 reopen. It may not use method names, inferred Box types,
   GenericLoop backfill, retry, or a second publication owner.

This is a prerequisite consultation boundary, not an authorization to create
a generic Hako result-type framework. Once the exact owner closes, rerun the
same R1 guard and fixture from the parked WIP; do not change the accepted
expression-product shape merely to avoid the compiler boundary.
