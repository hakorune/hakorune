---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M54 allocator hook runtime dry-run boundary
---

# 293x-106 M54 Allocator Hook Runtime Dry-Run Boundary

## Decision

`M54 allocator hook runtime dry-run boundary` is live-docs.

M54 adds the runtime dry-run boundary SSOT and guard. It does not add runtime
hook code, process allocator replacement, or hook environment toggles.

Design owner:

```text
docs/development/current/main/design/allocator-hook-runtime-dry-run-ssot.md
```

## Owned

- Runtime dry-run boundary SSOT.
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh`
- docs/taskboard/current pointers for M54.

## Not Owned

- Runtime hook install/uninstall implementation.
- Process allocator replacement.
- `#[global_allocator]`.
- Hook environment variables.
- `.inc` hook/facade/policy name matching.
- Pointer `fetch_add`.
- OSVM unreserve/release.
- Native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh
bash tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- dry-run SSOT/card/taskboard/current/docs index/dev_gate/group wiring is
  synchronized;
- M53 guard remains forward-compatible after latest-card movement;
- no runtime hook code, env toggle, process allocator replacement, or `.inc`
  name matching exists.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_runtime_dry_run_guard.sh` passes.
