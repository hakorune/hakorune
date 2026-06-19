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
PHI-INPUT-REMAT-OPERAND-MEMO-001
```

Purpose:

```text
Add predecessor-local memoization to PHI input rematerialization so the same
predecessor plus the same original ValueId reuses the same materialized ValueId.
Keep cycle handling fail-closed and do not expand accepted rematerialization
shapes in this row.
```

Current evidence:

```text
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001 is closed by 296x-1305.
The next cleanup is upstream PHI rematerialization identity stabilization.
```

Acceptance for the current slice:

```bash
cargo test -q phi_input_materializer
cargo test -q string_corridor_sink
cargo check -q --lib
```

## Task Order

1. Locate the PHI input rematerialization owner.
2. Add predecessor-local memoization without broadening accepted shapes.
3. Add focused unit coverage for same-pred same-source reuse and cycle
   fail-closed behavior where the existing seam allows it.
4. Re-run the focused checks above.
5. Commit separately from later loop-route or rust-subset-to-hako app-front
   work.

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
