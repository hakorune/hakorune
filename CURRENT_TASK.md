# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-13
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read its `latest_card_path`.
3. Read its `latest_workstream_card` when present.
4. Check `active_lane` and `current_blocker_token`; do not infer them here.
5. Run:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the active card requires them. Current scope,
acceptance, parked items, and non-claims belong in the active card and the
workstream SSOT, not this pointer.

When `current_blocker_token` contains `DESIGN-STOP`, do not invent a new executable owner from historical mirrors. To keep the goal open until the frontier names a concrete next owner, wait at the frontier.

## Handoff

Read `latest_card_path` in `CURRENT_STATE.toml` before editing. The active
slice is the Resolved Region Flow V1 R0 pre-plan identity design stop. R0 proves
canonical BindingId is singular but body-local IDs are allocated during Lower,
after AST-only generic Facts; no complete resolved binding/scope tree is
available pre-plan. The join_ir/ownership private BindingId is a duplicate
lexical authority (`SchemaMismatchStop`). Source exits also have no target and
production recounts Recipe/Lower depth; no resolved RegionId owner exists.
Consultation must choose pre-plan canonical BindingId resolution versus an
explicit structural BindingKey authority with checked Lower mapping. Planner
allocation, private IDs, name-based identity, and Lower rediscovery are
rejected. R1 remains blocked; product behavior is unchanged. R9 legacy
loop_var retirement remains Epic completion. Keep every source file below 800
lines.
