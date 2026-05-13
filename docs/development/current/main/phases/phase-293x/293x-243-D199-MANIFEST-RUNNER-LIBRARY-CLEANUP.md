# 293x-243 D199 Manifest Runner Library Cleanup

Status: Complete

## Purpose

D199 removes the duplicated Python runner bodies introduced by D197 and D198.
Both shell entrypoints now delegate to one shared implementation:

```text
tools/checks/lib/manifest_runner.py
```

This is a BoxShape cleanup only. It does not change manifest contents, existing
guard semantics, app proof assertions, `dev_gate.sh`, or allocator-wide gate
membership.

## Decision

Decision: accepted.

Keep:

```text
tools/checks/run_row_guard.sh
tools/checks/run_proof_app.sh
```

as the stable human-facing entrypoints. Move shared behavior into:

```text
tools/checks/lib/manifest_runner.py
```

Add:

```text
tools/checks/manifest_runner_pilot_guard.sh
```

to verify that the wrappers stay thin, the shared runner uses Python stdlib
`tomllib`, commands remain argv arrays, and no runner is wired into `dev_gate.sh`
or `k2_wide_allocator_gate.sh` yet.

## Stop Lines

- Do not delete existing row guard scripts.
- Do not delete app-local `test.sh` files.
- Do not move MIR JSON or pure-first EXE assertions into TOML.
- Do not wire manifest runners into `dev_gate.sh` or allocator-wide gate.
- Do not add shell `eval` or Python `shell=True`.

## Acceptance

- `run_row_guard.sh` and `run_proof_app.sh` no longer embed Python heredocs.
- `manifest_runner.py` owns manifest parsing, selection, validation, dry-run,
  list output, and subprocess execution.
- `manifest_runner_pilot_guard.sh` passes.
- D197/D198 runner behavior remains unchanged for list/dry-run/selected rows.

## Verification

```bash
bash tools/checks/manifest_runner_pilot_guard.sh
tools/checks/run_row_guard.sh --only current-state-pointer
tools/checks/run_proof_app.sh M200
```
