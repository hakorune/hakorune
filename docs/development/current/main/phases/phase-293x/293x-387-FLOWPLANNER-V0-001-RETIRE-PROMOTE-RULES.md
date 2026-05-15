# 293x-387 FLOWPLANNER-V0-001 Retire / Promote Rules

Status: landed
Date: 2026-05-15

## Decision

Active `loop_*_v0` boxes remain allowed only as audited compatibility or
one-shape proof boxes. This row tightens the rule for adding, retiring, or
promoting them.

## Scope

- Update `src/mir/builder/control_flow/plan/REGISTRY.md` and
  `LEGACY_V0_BOUNDARY.md`.
- Add `retire_when` / `promote_when` wording for active `loop_*_v0` rows.
- State that new `loop_*_v0` boxes are rejected by default unless a blocker
  explicitly selects a one-shape proof lane.
- Keep behavior unchanged.

## Stop Lines

- No new accepted control-flow shape.
- No code movement.
- No route order change.
- No fixture/gate expansion in this row.

## Required Evidence

```text
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Notes

- `LEGACY_V0_BOUNDARY.md` now rejects new `loop_*_v0` modules by default.
- `REGISTRY.md` now records hold reasons and retire/promote directions for
  active routed v0 boxes.
- New v0 exceptions now require `retire_when` / `promote_when` wording.
