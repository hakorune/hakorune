Status: accepted execution task; implementation not landed
Date: 2026-08-09
Row: PARSER-PUBLIC-AST-POSTPASS-FINAL-NOELSE-RECEIPT-I0
Parent: `parser-public-ast-postpass-final-no-else-receipt-d0-design-task-2026-08-09.md`

# FINAL-NOELSE-RECEIPT-I0

## Scope

Make the parser semantic BuildGate selection outcome reusable by the source
selection receipt without changing source path semantics.

```text
BuildGateSelectionOutcomeV1::{Then, Else, NoElse}
  = one parser-private semantic decision/receipt owner

SourceBuildGateBranchV1::{Then, Else}
  = path segment owner, unchanged
```

The receipt must emit one row for every top-level source record, including a
`NoElse` row. A no-else outcome creates no child path and cannot authorize a
descendant source seal.

## Required changes

1. Move or expose the semantic outcome through one parser-private shared
   module; do not define a second enum in the receipt or path module.
2. Change `BuildGateSelectionReceiptV1.selected_branch` to the semantic
   outcome and project `NoElse` directly.
3. Keep `SourceBuildGatePathV1` and `SourceBoxDeclarationPathV1` Then/Else-only.
4. Make source-seal survival match only `(Then, Then)` and `(Else, Else)`;
   `NoElse` returns non-surviving for any path segment.
5. Preserve decision-set evaluation, explain counters, body-gate scope, and
   grammar-evidence behavior.

## Acceptance tests

```text
top-level no-else source gate -> exactly one NoElse receipt
no-else projected AST has no child declaration/path
Then/Else receipt/path behavior unchanged
missing/duplicate/foreign/shape-mismatch receipts reject
NoElse never appears in a path segment
records.len() == receipts.len()
```

Run focused projection/source-seal/BuildCfg tests, parser FINAL and pointer
guards, `cargo fmt --all -- --check`, and `git diff --check`.

## Nonclaims

```text
no grammar-evidence redesign
no compatibility-arm replacement
no resolver/Builder/MIR/runtime activation
no public production switch
no retry/reparse/fallback
```

Implementation and the parser README, source-handoff SSOT, BuildGate
reference, task map, guard, and CURRENT_STATE receipt close in one commit.
