---
Status: active implementation
Date: 2026-08-09
Decision: switch the string/build-config AST parser family through the S0 coordinator
Parent: `parser-public-ast-postpass-s0-implementation-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-I0-A

## Scope

Switch exactly one production edge: the string/build-config AST parser
family. The existing wrappers inherit this edge and must keep their public
signatures and diagnostics:

```text
parse_from_string_with_fuel_and_build_config
  -> parse_from_string
  -> parse_from_string_with_fuel
  -> parse_from_string_with_build_config
```

The implementation must call the S0 coordinator once per parser invocation:

```text
tokenize once
parse_program once
open one postpass product
finish_total_s0(PostpassDemandV1::None)
project AST once
```

`parse_from_string_with_fuel_and_metadata`, `NyashParser::parse`, and the
explain-report family remain explicitly parked for I0-B/I0-C.

## Acceptance matrix

Compare the old and new edge before deleting the selected old call. The
comparison is AST shape and public diagnostics, not source-seal internals.

```text
ordinary Box / multiple ordinary Boxes
static Box / interface Box / record Box
ordinary + compatibility mixed program
top-level BuildGate selected then/else
nested BuildGate inside a supported source body
fuel = Some(0), Some(exact), Some(exhausting), None
unknown feature / malformed predicate
delegate-free and delegate-bearing compatibility cohorts
sync Box parser parity
```

The same source must produce the same AST and error family under the old and
new edge. Fuel is assigned once on parser construction; no wrapper may reset
or drop it. A compatibility result is successful AST transport and must not
be retried through the ordinary rich arm.

## Required code boundary

- Add one private string-entry helper that opens the S0 postpass product.
- Replace only the selected `parse_from_string_with_fuel_and_build_config`
  old delegate call with the helper's AST projection.
- Keep grammar-evidence APIs separate; they intentionally stop before the
  delegate postpass and are not part of this row.
- Do not expose `CompletedParserPostpassV1` or raw source seals publicly.
- On any source/postpass error, return the existing `ParseError` family and
  drop the unpublished product; no catch-and-fallback or retry.

## Same-commit closeout

Update the parser README, language reference, postpass SSOT, task map,
CURRENT_STATE, consolidated guard, and focused parity tests in the same
commit. The next row is I0-B for `NyashParser::parse` and metadata projection.

## Non-claims

```text
metadata wrapper cutover
NyashParser::parse cutover
explain-report parity/full BuildGate decision set
resolver source publication
Recipe/CallSlot/Builder/MIR/runtime
legacy helper retirement for remaining callers
```
