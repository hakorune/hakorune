---
Status: closed__2026-09-22__WarningBaselineRefreshI128__SelectedI129
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I128
Date: 2026-09-22
Parent: mirbuilder-warning-parser-source-admission-coordinate-i0-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-DYNAMIC-V2-REJECT-DEAD-VARIANTS-I129
---

# MirBuilder warning baseline refresh I128

## Six-line brief

```text
Decision: refresh the warning surface and select exactly one bounded row;
  keep the old-edge lane parked unless a new caller-zero proof exists.
Source authority + canonical issuer: quick-profile Cargo diagnostics, the
  warning classification policy, and the existing LoopBreak retirement card.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  or a new semantic receipt without a named owner.
Fail-fast boundary: a new warning, unclassified red, nonzero old-edge caller,
  or missing absence guard stops selection and records NoSafeSlice.
Smallest next slice: census one caller-zero old-edge candidate, or one
  production-zero warning item only when that deletion is proven finite.
Non-claims: no broad warning cleanup, parser semantic change, VM repair, or
  LegacyCallV0 retirement beyond one explicitly selected slice.
```

## Baseline and old-edge boundary

I127 deleted the production/test-zero `ParserSourceAdmissionRowV1::coordinate`
accessor. The warning group remains grouped in lib at **1,691** because
`canonical_segment` and `local_line` remain unused; lib-test is **546**. The
parser source-admission filter passed **44/44**, quick check and no-run passed,
and no guard or semantic source row changed.

The I120 caller census remains binding: compatibility registry, ledger-free
legacy, shared `lower_loop_or_freeze_v1`, and parser composite successor
evidence still prevent old-edge deletion. Re-evaluate the owner-requested
`MIR-RETIRE-FIRST-OLD-EDGE-R0` first; if caller-zero is still absent, choose
one production-zero warning item with a finite delete set. Do not delete the
remaining parser row accessors as a group without a separate caller census.

The I120 old-edge caller-zero proof did not change, so the requested
`MIR-RETIRE-FIRST-OLD-EDGE-R0` remains `NoSafeSlice`. The selected finite
warning cohort is the four unconstructed variants in
`DynamicV2AotCallMetadataRejectV1`: `MissingCallRole`, `DuplicateCallRole`,
`MissingNormalResult`, and `DuplicateNormalResult`. Repository-wide Rust,
test, tool, and tracked-document search found only their enum declarations.
The similarly named `MissingCallRole` in
`DynamicV2RecipeOperationCursorRejectV1` is a separate live variant and is
excluded. I129 deletes exactly these four variants.

## Acceptance

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Record both warning counts and the complete selected owner/caller inventory.
