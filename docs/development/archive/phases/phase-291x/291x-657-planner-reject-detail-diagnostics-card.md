---
Status: Landed
Date: 2026-04-28
Scope: move planner reject-detail state to diagnostics ownership
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/planner-entry-guards-ssot.md
  - src/mir/builder/control_flow/verify/diagnostics/planner_reject_detail.rs
  - src/mir/builder/control_flow/plan/facts/reject_reason.rs
---

# 291x-657: Planner Reject Detail Diagnostics

## Goal

Separate cross-router planner reject-detail state from the plan/facts reject
vocabulary.

This is BoxShape cleanup. It must not change reject reasons, handoff targets,
log tags, planner acceptance, or lowering behavior.

## Evidence

Worker inventory split `plan/facts/reject_reason.rs` into five live surfaces:

- `RejectReason` stable vocabulary;
- `RejectReason::as_freeze_message()` strict/dev message mapping;
- `HandoffTarget` and per-box handoff tables;
- `[plan/reject]` / `[plan/accept]` log formatting;
- `LAST_PLAN_REJECT_DETAIL` thread-local state consumed by router/freeze paths.

Only the last surface is not facts-owned. It is diagnostic state shared by
facts, route exhaustion, whitelist misses, planner-required freezes, and final
JoinIR loop freezes.

## Decision

Move only the `LAST_PLAN_REJECT_DETAIL` storage and accessors into
`verify/diagnostics`.

Keep these in `plan/facts/reject_reason.rs` for this slice:

- `RejectReason`;
- `RejectReason::as_freeze_message()`;
- `HandoffTarget`;
- `handoff_tables`;
- `log_reject` and `log_accept`.

## Boundaries

- Do not move `RejectReason` or handoff tables.
- Do not change first-writer preservation for route-level fallback details.
- Do not change `[plan/reject]` / `[plan/accept]` output.
- Do not touch `lower::planner_compat` in this card.

## Acceptance

```bash
cargo fmt
cargo test planner_reject_detail --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added `verify::diagnostics::planner_reject_detail` as the owner for
  last-reject detail state.
- Updated final loop freeze, planner-required freeze, route exhaustion, and
  whitelist-miss callers to use the diagnostics owner directly.
- Kept `RejectReason`, handoff tables, and `[plan/reject]` / `[plan/accept]`
  formatting in `plan/facts/reject_reason.rs`.
- Updated the planner entry guard SSOT with the split ownership contract.
