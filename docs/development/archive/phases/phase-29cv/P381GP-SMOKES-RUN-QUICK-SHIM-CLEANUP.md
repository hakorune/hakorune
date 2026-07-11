# P381GP Smokes Run Quick Shim Cleanup

Date: 2026-05-06
Scope: remove a false-green legacy quick-smoke wrapper path.

## Context

`tools/smokes/v2/run_quick.sh` was still a hand-written wrapper that attempted
to run two archived Phase 2160 quick canaries:

```text
profiles/quick/core/phase2160/loop_scan_ne_else_break_canary_vm.sh
profiles/quick/core/phase2160/loop_scan_ne_else_continue_canary_vm.sh
```

Those paths no longer exist. The wrapper also used `|| true`, so it printed a
successful-looking completion line even when the targeted smoke files were
missing.

## Change

- Replaced `run_quick.sh` with a compatibility shim to:

```bash
tools/smokes/v2/run.sh --profile quick "$@"
```

- Documented the shim in `tools/smokes/v2/README.md`.

## Result

The legacy entry no longer carries stale smoke paths or swallows failures. Users
who still call `run_quick.sh` now get the same quick profile as the canonical
smokes v2 runner.

This is smoke-entry cleanup only. It does not delete smoke cases and does not
change compiler behavior.

## Validation

```bash
bash -n tools/smokes/v2/run_quick.sh tools/smokes/v2/run.sh
bash tools/smokes/v2/run_quick.sh --dry-run --skip-preflight
bash tools/smokes/v2/run.sh --profile quick --dry-run --skip-preflight
rg -n 'profiles/quick/core/phase2160|\|\| true' tools/smokes/v2/run_quick.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
