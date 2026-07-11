# 293x-868 RECORD-LOCAL-SCALARIZATION-SSOT-001 Boundary and Stop Lines

Status: landed
Date: 2026-05-20

## Decision

Add a small compiler SSOT for record-local scalarization before migrating
another allocator `ReportFields` owner.

SSOT:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Define the conceptual owners:
  - `RecordValueScalarizationBox`
  - `RecordHelperArgumentScalarizationBox`
  - `RecordLocalFacts`
- Fix the helper body shape stop lines.
- Fix the exact PHI propagation rule.
- Fix the same-owner receiver rule.
- Keep this as docs-only; no new compiler acceptance shape opens in this row.

## Stop Lines

- No new `ReportFields` owner migration in this row.
- No runtime record object.
- No `NewBox` / `typed_object_plan` for record helper carriers.
- No backend lowering route or owner-name matcher.
- No cross-function record-local ABI.
- No runtime materialization fallback.

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Completion Criteria

- The SSOT names owner boundaries, allowed shapes, helper body shape, PHI
  propagation, receiver rule, stop lines, and guard expectations.
- The next allocator row can reference this SSOT instead of restating the full
  contract.

## Next

Select `HAKO-ALLOC-REPORT-RECORD-014` to choose the next single
`ReportFields` owner under the new SSOT.
