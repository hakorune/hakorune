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
STRING-CORRIDOR-SINK-REGRESSION-CLEANUP-001
```

Purpose:

```text
Restore the touched-module `cargo test -q string_corridor_sink` benchmark
routes after the dense string-corridor use-count / consumer-analysis slice.
Do this without source-name, function-name, or benchmark-specific branches.
```

Known failing command:

```bash
cargo test -q string_corridor_sink
```

Known failures:

```text
benchmark_len_substring_views_compiles_without_loop_string_consumers
benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route
```

Acceptance for the current cleanup slice:

```bash
cargo test -q string_corridor_sink
cargo test -q string_kernel_plan
cargo check -q --lib
```

## Task Order

1. Fix the string-corridor regression locally.
2. Keep the fix inside the corridor/string-kernel ownership boundary.
3. Re-run the focused module checks above.
4. Commit the cleanup slice separately from future loop-route or app-front
   work.

## Pointers

- Current state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- Latest phase card: read `latest_card_path` in `CURRENT_STATE.toml`
- Current docs policy:
  `docs/development/current/main/design/current-docs-update-policy-ssot.md`
- Restart mirror: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Thin dashboard: `docs/development/current/main/10-Now.md`
