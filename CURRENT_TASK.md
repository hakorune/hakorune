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
slice is Resolved Region Flow V1 R0 binding/region seam inventory. Before code,
identify the sole canonical lexical BindingId producer, immutable binding_ctx
snapshot seam, scope/shadow owner, assignment Place seam, resolved
break/continue target owner, and classify the private join_ir/ownership
BindingId. RegionFlow must allocate no IDs, use no names as identity, simulate
no SSA values, and publish no cloned normalized AST. WIP `f74e5961e1` is green
acceptance-neutral evidence but must be restored selectively only after R0;
R1 first removes ProjectedStmt AST clone ownership. R1-R8 lead to generic
baseline cutover, and R9 caller-zero retirement of mandatory loop_var remains
Epic completion. Product/Planner/Lower behavior stays unchanged in R0. Keep
every source file below 800 lines.
