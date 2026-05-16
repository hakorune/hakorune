# 293x-513 CURRENT-DOCS-PHASE-SLIM-001

Status: landed
Date: 2026-05-17

## Decision

`CURRENT-DOCS-PHASE-SLIM-001` is a BoxShape cleanup for current phase docs. It
keeps `CURRENT_STATE.toml` as the live pointer SSOT and reduces duplicated
current-row wording in restart/phase/taskboard mirrors.

## Scope

- Keep `docs/development/current/main/CURRENT_STATE.toml` as the current lane,
  latest card, and blocker SSOT.
- Slim repeated "current row" prose in:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/phases/phase-293x/README.md`
  - `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`
- Keep durable policy links and active phase/taskboard links visible.
- Keep landed history in phase cards/taskboards, but avoid duplicating live
  status paragraphs across mirrors.

## Stop Lines

- Do not archive/delete active cards or taskboards.
- Do not rewrite landed card history except replacing duplicated live pointers
  with SSOT references.
- Do not change current lane policy, allocator/provider activation status,
  mimalloc port purpose, or task ordering except this selected cleanup row.
- Do not touch source code in this docs-slim row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `DOC.1` | Inventory duplicated current-row wording across current mirrors and phase taskboard. | list of touched files is explicit. | no history rewrite |
| `DOC.2` | Replace duplicated live status prose with `CURRENT_STATE.toml` / `phase_status` pointers. | restart path still has direct links. | no policy loss |
| `DOC.3` | Keep the phase taskboard's live row table single-sourced. | taskboard points to current card. | no card deletion |
| `DOC.4` | Verify and close out. | required evidence is green. | no code edits |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row slimmed live-current duplicates across the restart/current mirrors and
phase-293x taskboard:

```text
CURRENT_TASK.md
docs/development/current/main/05-Restart-Quick-Resume.md
docs/development/current/main/10-Now.md
docs/development/current/main/phases/phase-293x/README.md
docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
```

The live active row remains owned by `CURRENT_STATE.toml`. The mirrors now keep
only restart pointers, durable policy links, and the `current_blocker_token`
required by `current_state_pointer_guard.sh`. Landed history remains in phase
cards/taskboards instead of the root current pointer.

Evidence:

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
