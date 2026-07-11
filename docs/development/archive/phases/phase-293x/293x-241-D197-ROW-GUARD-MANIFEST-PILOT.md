# 293x-241 D197 Row Guard Manifest Pilot

Status: Complete

## Purpose

D197 starts reducing row-guard sprawl without changing existing guard
semantics. It adds a manifest-backed runner that can execute a small set of
existing guard scripts by row id or profile.

The old guard scripts remain authoritative. This row does not delete scripts,
does not move MIR JSON / pure-first EXE assertions into TOML, and does not wire
the manifest runner into `dev_gate.sh`.

## Decision

Decision: accepted.

Add:

```text
tools/checks/guard_rows.toml
tools/checks/run_row_guard.sh
```

`guard_rows.toml` stores only command-level row entries for the first pilot.
Each row has:

```toml
[[rows]]
id = "current-state-pointer"
label = "current state pointer guard"
profiles = ["pilot", "quick-static"]
cmd = ["bash", "tools/checks/current_state_pointer_guard.sh"]
```

`run_row_guard.sh`:

- parses the manifest with Python stdlib `tomllib`
- runs commands as argv arrays, never through shell eval
- supports `--list`, `--profile <name>`, and `--only <id,id,...>`
- runs from the repository root
- stops on first failed row and returns the child exit code

## Stop Lines

- Do not delete or rewrite existing row guard scripts in this row.
- Do not wire the runner into `dev_gate.sh` or `k2_wide_allocator_gate.sh`.
- Do not manifest pure-first EXE / MIR JSON Python assertion bodies yet.
- Do not replace app-local `test.sh` files.
- Do not create one TOML file per row in this pilot.

## Acceptance

- `tools/checks/run_row_guard.sh --list` prints manifest rows.
- `tools/checks/run_row_guard.sh --profile pilot` executes the static pilot
  guard set.
- `tools/checks/run_row_guard.sh --only current-state-pointer` executes one row.
- Existing guard scripts still run directly.
- `docs/tools/check-scripts-index.md` documents the new runner.

## Verification

```bash
tools/checks/run_row_guard.sh --list
tools/checks/run_row_guard.sh --profile pilot
tools/checks/run_row_guard.sh --only current-state-pointer
bash tools/checks/current_state_pointer_guard.sh
```
