---
Status: parked design
Date: 2026-08-09
Decision: open only after I0-A closeout; no implementation in this row yet
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
