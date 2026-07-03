# CURRENT_TASK

Status: SSOT pointer
Date: 2026-07-03
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read the `latest_card_path` named in `CURRENT_STATE.toml`.
3. Read the workstream/task-order doc named by `latest_workstream_card`, when present.
4. If `current_blocker_token` names an explicit design-stop frontier, also
   read `docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md`
   before selecting any new family-specific task.
5. Check the worktree:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the current code slice is ready.

## Current Fields

Read these fields in `docs/development/current/main/CURRENT_STATE.toml`:

- `active_lane`
- `active_phase`
- `latest_workstream_card`
- `latest_card_path`
- `current_blocker_token`

If `current_blocker_token` names the explicit design-stop frontier, do not
invent a new executable owner from historical mirrors. Review the frontier
card first and keep the goal open until the frontier names a concrete next
owner.

Current implementation details, acceptance, parked items, and non-claims live in
the active card and task-order SSOT. Do not duplicate them here.

## Immediate Maintenance Slice

Scope: Hakorune naming cleanup guard follow-up.

- use `docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md`
  as the task-order SSOT;
- keep broad package/env/ABI renames out of small cleanup slices;
- continue only with thin, behavior-preserving naming cleanups that have an
  SSOT task token and guard coverage;
- preserve compatibility route names such as Stage1/Stage-B unless the selected
  slice explicitly narrows the layer and acceptance gate;
- do not touch PHI / LocalSSA / variable-map internals for naming cleanup.

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
