# Phase 29ap P11: Pattern6_NestedLoopMinimal release adopt (CorePlan default)

## Goal

- Route nested minimal loops to CorePlan composer v2 in non-strict/release.
- Keep strict/dev tag and fail-fast behavior unchanged.
- Preserve legacy fallback when facts are missing.

## Steps

1. Release adopt path
   - Use the same nested facts + composer v2 as strict/dev.
   - Only adopt when facts are present; otherwise fall back to legacy.
   - No new tags/logs in release.

2. Regression gate
   - Add a release adopt smoke for `phase1883_nested_minimal` (exit code 9).
   - Ensure no shadow-adopt tag is printed.

3. Docs
   - Mark P11 done in `phase-29ap/README.md`.
   - Advance Now/Backlog/roadmap to next planned step.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

