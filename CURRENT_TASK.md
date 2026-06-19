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
STRING-CORRIDOR-STABLE-LENGTH-HINT-FALLBACK-RETIRE-001
```

Purpose:

```text
Retire string-corridor planning/correctness dependence on diagnostic
optimization_hints string parsing where typed string-corridor relation / plan
evidence already covers the route. Keep diagnostic hints output-only.
```

Current evidence:

```text
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001 is closed by 296x-1305.
PHI-INPUT-REMAT-OPERAND-MEMO-001 is closed by 296x-1306.
The next cleanup is stable-length hint fallback retirement.
```

Acceptance for the current slice:

```bash
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
```

## Task Order

1. Inventory `optimization_hints` parsing used as planning/correctness
   evidence.
2. Identify typed relation / plan evidence that can replace the fallback.
3. Retire only the proven fallback path.
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
