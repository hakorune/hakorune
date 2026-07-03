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

Scope: `me.method` static-box call target repair only.

- fix static-box `main` lowering so `me.helper(...)` resolves to the current
  static box owner, not the top-level `main` function name
- keep PHI lifecycle / type propagation SSOT untouched
- do not add new language syntax, fallbacks, or `.hako` workarounds in this
  slice

Acceptance:

```bash
cargo test -q --features vm-reference --test phase246_json_atoi
tools/checks/dev_gate.sh quick
```
