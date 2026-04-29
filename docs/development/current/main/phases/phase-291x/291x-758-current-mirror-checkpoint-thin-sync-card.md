# 291x-758 Current Mirror Checkpoint Thin Sync Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `CURRENT_TASK.md`
- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/phases/phase-291x/README.md`
- `CURRENT_STATE.toml`

## Why

The current mirrors already point to `CURRENT_STATE.toml`, but several
checkpoint sentences still named `291x-691` as the latest known cleanup
checkpoint. That made the thin mirrors look stale after the 291x-747..757
cleanliness burst.

## Decision

Keep the mirrors thin. Do not paste landed history into them.

Replace fixed latest-checkpoint wording with `CURRENT_STATE.toml` pointers and
keep `291x-691` only as the historical warning-backlog inventory baseline.

## Landed

- Replaced stale `291x-691` latest-checkpoint wording in current mirrors.
- Updated phase-291x README checkpoint text to use `CURRENT_STATE.toml`.
- Updated the warning baseline from the old 48-warning inventory to the current
  zero release lib-warning backlog note.
- Advanced `CURRENT_STATE.toml` to this card.

## Proof

- `rg -n 'latest known checkpoint: \`291x-691\`|cleanup checkpoint: latest known card \`291x-691\`|Current cleanup checkpoint:.*291x-691' CURRENT_TASK.md docs/development/current/main/05-Restart-Quick-Resume.md docs/development/current/main/10-Now.md docs/development/current/main/phases/phase-291x/README.md -g '*.md'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
