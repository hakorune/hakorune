Status: accepted design; implementation not opened
Date: 2026-08-09
Row: PARSER-PUBLIC-AST-POSTPASS-FINAL-CLOSEOUT-D0
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`

# FINAL-CLOSEOUT-D0

The parser postpass boundary is now structurally closed through the typed
NoElse receipt and caller-zero helper retirement. This row is a design audit,
not a production switch.

## Final authority checklist

```text
decision evaluation:
  BuildGateSelectionOutcomeV1, once

selection receipt:
  one per top-level source record, including NoElse

source path:
  SourceBuildGateBranchV1 Then|Else only

source-seal survival:
  exact Then/Then or Else/Else only

shared public postpass:
  one projection owner

separate demands:
  grammar evidence, resolver source seal, explicit compatibility
```

## Required audit before any further cleanup

1. Confirm caller-zero retirement remains zero and no deleted helper is
   reintroduced by a compatibility import.
2. Confirm every public AST route remains on the shared postpass owner and no
   retry/reparse/fallback or name-based identity reconstruction exists.
3. Confirm body/function gates remain decision-covered but do not claim
   top-level source-ledger identity.
4. Confirm the 12-case BuildCfg gate, focused postpass/source-seal tests, all
   relevant guards, and the 760-line trigger remain green.
5. Keep grammar evidence and the explicit compatibility arm as separate
   authorities until their own typed demands are redesigned.

## Nonclaims

```text
no broad production switch
no resolver/runtime/Builder/MIR activation
no grammar-evidence redesign
no compatibility replacement
no retry/reparse/fallback
no SourceBuildGateBranchV1::NoElse
```

Only after this audit may a later caller-zero cleanup or production selection
design be opened. Any new semantic source contract requires its own D0 before
implementation.
