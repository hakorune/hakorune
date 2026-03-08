---
Status: Ready
Scope: Release adopt `match_return` subset (no tag, no behavior change)
Related:
- docs/development/current/main/phases/phase-29at/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/coreplan-purity-stage1-ssot.md
- docs/development/current/main/design/coreplan-shadow-adopt-tag-coverage-ssot.md
---

# Phase 29at P4: match_return release adopt (no tag)

## Objective

Use the CorePlan `BranchN` path for the `match_return` subset in **non-strict**
execution as well, while keeping behavior unchanged and emitting **no new tags**
in raw output.

Rationale:
- P3 proved strict/dev shadow adopt for `return (match ...)`.
- P4 makes the CorePlan path the default for that subset, matching the long-term
  “CorePlan is SSOT” goal.

## Non-goals

- No value-join `match` (still out of scope).
- No planner/normalizer match generation (this stays as the return fast-path).
- No new env vars.

## Implementation outline

1. In `src/mir/builder/stmts.rs`, reuse the P3 subset detection and CorePlan
   build path, but allow it in non-strict mode:
   - strict/dev: keep emitting `[coreplan/shadow_adopt:match_return]` (existing).
   - non-strict: adopt CorePlan **without** emitting any tag.

2. Add a non-strict smoke that asserts:
   - exit code is correct
   - output matches expected
   - raw output does **not** contain `[coreplan/shadow_adopt:match_return]`
   - raw output does **not** contain `[plan/fallback:`

   Suggested files:
   - `tools/smokes/v2/profiles/integration/joinir/match_return_release_adopt_vm.sh`
   - Wire into `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
   - Add to `docs/development/current/main/phases/phase-29ae/README.md` gate list.

## Acceptance (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

