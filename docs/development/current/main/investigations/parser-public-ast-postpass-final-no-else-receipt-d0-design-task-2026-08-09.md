Status: accepted design; implementation not opened
Date: 2026-08-09
Row: PARSER-PUBLIC-AST-POSTPASS-FINAL-NOELSE-RECEIPT-D0
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`

# FINAL-NOELSE-RECEIPT-D0

The receipt uses the existing semantic decision outcome:

```text
BuildGateSelectedBranchV1::{Then, Else, NoElse}
  = decision-set and receipt selection outcome

SourceBuildGateBranchV1::{Then, Else}
  = source/Box path segment only
```

Every top-level gate emits exactly one source record and exactly one receipt.
`NoElse` has no child path and cannot authorize a descendant. Source-seal
survival matches only Then/Then and Else/Else. Receipt omission, registration
delay, `Option<Then|Else>`, and `NoElse -> Else` conversion are forbidden.

## Implementation order

1. Move the semantic outcome to a parser-private shared owner (or raise the
   existing decision-set enum visibility without creating a second enum).
2. Change `BuildGateSelectionReceiptV1.selected_branch` to that outcome.
3. Project NoElse directly; leave path construction Then/Else-only.
4. Update `source_seal_survives` and finalizer checks so NoElse authorizes no
   descendant path.
5. Add positive top-level no-else coverage plus missing/duplicate/foreign,
   shape-mismatch, and descendant-under-NoElse negatives.
6. Keep decision evaluation, explain counters, body-gate scope, and grammar
   evidence unchanged.

## Acceptance

```text
records.len() == receipts.len()
one receipt per top-level gate
NoElse never enters a path segment
Then/Else path behavior unchanged
all receipts preserve brand/id/path/predicate/coordinate
no Builder/MIR/CFG/PHI or public caller switch
same-commit README/reference/task/guard/CURRENT_STATE update
all touched source files < 760 lines
```

This row opens only after FINAL-RETIRE-S0 closes. It is not a production
cutover and does not redesign grammar evaluation or compatibility lowering.
