# 293x-222: D196 Stop-The-Line Guard Refresh

Status: Complete

## Scope

D196 keeps allocator and record-lane guards focused on durable stop lines
instead of wiring every small cleanup/probe row into daily or allocator-wide
gate paths.

This row fixes the current rule:

- milestone/algorithm/provider guards may stay in `allocator-wide` when they
  protect production behavior, backend leakage, provider activation, hook
  installation, or allocator replacement stop lines.
- `C206+` cleanup/probe guards stay local-run and index-listed by default.
- A `C206+` guard may be promoted to a heavier gate only when the card says
  which production stop line it protects and why the existing shared guard is
  insufficient.

## Non-Goals

D196 does not:

- remove existing row guards.
- change allocator behavior.
- add packed ArrayBox compiler auto-use.
- reopen provider activation, hook installation, or process allocator
  replacement.
- make quick/dev gates slower.

## Acceptance

- `tools/checks/k2_wide_guard_refresh_policy_guard.sh` passes.
- `C206b-C206d` inline-record probe guards remain index-listed but are not
  wired into `tools/checks/dev_gate.sh` or
  `tools/checks/k2_wide_allocator_gate.sh`.
