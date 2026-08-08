---
Status: closed implementation
Date: 2026-08-09
Decision: accepted bounded metadata/parse projection; implementation may open in fast mode
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-I0-B

## Scope

Move the remaining `NyashParser::parse` and metadata projection through the
already-landed total postpass owner without re-tokenizing or re-parsing.

```text
existing parser invocation
  -> one OpenParserPostpassProductV1
  -> one PostpassDemandV1 projection
  -> AST/metadata result
```

The source-seal and compatibility cohort rules from S0/I0-A remain unchanged.
Explain/full BuildGate decision-set parity belongs to I0-C, not this row.

## Design stop before implementation

Before code, fix the exact metadata ownership and public error-preservation
matrix in the SSOT. `ParserMetadata` must be moved/borrowed exactly once from
the postpass product; the new edge must not call a second parser entry or
reconstruct metadata from AST nodes.

### Design acceptance — 2026-08-09

I0-B uses one parser-private finalization helper shared by the string entry,
`NyashParser::parse`, and the metadata entry:

```text
parser.parse_program()
  -> parser.open_postpass_product(ast)
  -> finish_total_s0(PostpassDemandV1::None)
  -> CompletedParserPostpassV1
```

The completed product has one consuming projection:

```text
into_ast_and_metadata()
  -> (ASTNode, ParserMetadata)
```

`into_ast()` remains for AST-only callers. The metadata is moved from the
postpass product, never reconstructed from AST nodes and never taken again
from `NyashParser`. Compatibility cohorts preserve their already-collected
metadata just like ordinary source-sealed cohorts. Fuel, tokenizer choice,
`self` diagnostics, and `ParseError` propagation remain owned by the caller's
single parser invocation.

The shared helper is a transport/coordinator owner only. It does not classify
source by name, issue resolver authority, or add fallback/retry. I0-B does
not open explain demand; that remains I0-C.

## Implementation receipt — 2026-08-09

I0-B now routes `NyashParser::parse` and
`parse_from_string_with_fuel_and_metadata` through the shared
`parse_postpass_s0`/`string_postpass_entry` owner. The parser tokenizes and
parses once, opens one postpass product, and completes it once. Metadata is
consumed from `CompletedParserPostpassV1::into_ast_and_metadata()`; AST nodes
are never rescanned and parser metadata is never taken a second time.

Focused evidence:

```text
string_postpass_entry: 7 passed
parser_opt_annotations: 33 passed
parser source-session: 6 passed
delegate/build-config/transition/check-block suites: green
```

The existing parent-baseline nested member-gate red remains excluded under
`PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0`. I0-B does not open explain/full
BuildGate decision-set parity, resolver, Recipe, Builder, MIR, runtime, or
fallback/retry.

## Non-claims

```text
no explain parity
no full BuildGate decision-set change
no resolver source publication
no Recipe/Builder/MIR/runtime work
no fallback/retry/reparse
```

## Closeout

Implementation, focused parity tests, parser README, language reference,
postpass SSOT, task map, CURRENT_STATE, and guards must close in one commit.
All touched source files remain below the 800-line boundary.
