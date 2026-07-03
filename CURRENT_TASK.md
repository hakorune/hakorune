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

Status: closed for the Hakorune naming cleanup guard follow-up.

The completed guard follow-up is tracked in:

```text
docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md
```

Current state:

- all `HAKORUNE-*` naming cleanup slices in the task-order SSOT are landed;
- the only `active in this slice` entry is the always-on
  `NAMING-CHARTER-STAGE-TERM-DISAMBIGUATION-001` guardrail;
- broad package/env/ABI renames remain out of scope for maintenance slices;
- PHI / LocalSSA / variable-map internals remain out of scope for naming
  cleanup.

Stage-term existing-name migration has a classification inventory at:

```text
docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md
```

The first classified stage-term migration slice is landed:

```text
STAGE-TERM-SYNTAX3-ALIAS-001
STAGE-TERM-SYNTAX3-DIAGNOSTIC-WORDING-001
STAGE-TERM-MODEB-COMPAT-ENV-WORDING-001
STAGE-TERM-MODEB-PROOF-ROUTE-WORDING-001
STAGE-TERM-MODEB-STAGE1-BRIDGE-WORDING-001
```

`--syntax-3` is now the frontend syntax-level spelling; `--stage3` remains a
compatibility alias. Live MIR builder hints now say `syntax-3` and `mode-B`
compatibility routes. Live env docs/comments now describe `STAGEB` names as
mode-B compatibility aliases without renaming those compatibility surfaces.
The explicit proof-only selfhost route docs/diagnostics now also say mode-B
compatibility while keeping `--stage-b`, `stageb-delegate`, and script names as
compatibility surfaces. Stage-1 bridge module payload comments/docs also say
mode-B compatibility while keeping `HAKO_STAGEB_*` env names and bridge file
names unchanged. Next safe naming work, if explicitly selected, must pick a
different classified layer from that inventory and keep compatibility aliases
or replacement routes in the same slice.

Acceptance:

```bash
bash tools/checks/naming_charter_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```
