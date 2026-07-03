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

Scope: Gate-C/NyVM wrapper smoke binary naming cleanup.

- make Gate-C v1 file and NyVM wrapper smokes spell the Hakorune-first
  executable resolver explicitly
- keep legacy `nyash` only as a named compatibility fallback
- keep JSON fixtures and expected smoke behavior unchanged in this slice
- add naming guard coverage so these smokes do not regress to direct legacy
  binary naming

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
SMOKES_ENABLE_GATE_C_V1=1 bash tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh
SMOKES_ENABLE_NYVM_WRAPPER=1 bash tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh
git diff --check
tools/checks/dev_gate.sh quick
```
