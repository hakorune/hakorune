# CURRENT_TASK

Status: SSOT pointer
Date: 2026-06-19
Scope: root restart anchor only. Do not store landed history here.

## Quick Restart

1. Read `docs/development/current/main/CURRENT_STATE.toml`.
2. Read the `latest_card_path` named in `CURRENT_STATE.toml`.
3. Check the worktree:

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

4. Run heavier gates only when the current code slice is ready:

```bash
tools/checks/dev_gate.sh quick
```

## Current Task

Read these fields in `docs/development/current/main/CURRENT_STATE.toml`:

- `active_lane`
- `active_phase`
- `latest_card_path`
- `current_blocker_token`

Current blocker:

```text
COREPLAN-LOOP-ACTUAL-SELECTION-TRACE-001
```

Purpose:

```text
Record the actual legacy loop route whose handler succeeds, so B-lite resolver
shadow diagnostics can compare against the real selected route instead of raw
candidate lists.
```

Current evidence:

```text
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001 is closed by 296x-1305.
PHI-INPUT-REMAT-OPERAND-MEMO-001 is closed by 296x-1306.
STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001 is closed by 296x-1307.
The next cleanup is loop actual-selection trace.
```

Acceptance for the current slice:

```bash
cargo test -q loop_route
cargo check -q --lib
cargo test -q string_corridor_sink
```

## Task Order

1. Locate the legacy loop route registry / handler success seam.
2. Add typed actual-selected-route observation without changing selection.
3. Keep B-lite resolver shadow read-only and do not delete suppression
   branches in this row.
4. Re-run the focused checks above.
5. Commit separately from later suppression retirement or rust-subset-to-hako
   app-front work.

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
