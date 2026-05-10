---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M56 allocator hook runtime owner row
---

# 293x-108 M56 Allocator Hook Runtime Owner

## Decision

`M56 allocator hook runtime owner row` is live-docs.

M56 names the future runtime owner for allocator hook dry-run validation. It does
not add runtime hook code or process allocator replacement.

Design owner:

```text
docs/development/current/main/design/allocator-hook-runtime-owner-ssot.md
```

## Owned

- Runtime owner SSOT.
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh`
- docs/taskboard/current pointers for M56.

## Not Owned

- Runtime dry-run implementation.
- Runtime hook install/uninstall implementation.
- Process allocator replacement.
- `#[global_allocator]`.
- Hook environment variables.
- `.inc` hook/facade/policy name matching.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- runtime owner SSOT/card/taskboard/current/docs index/dev_gate/group wiring is
  synchronized;
- future owner path is named but not implemented yet;
- M55 guard remains forward-compatible after latest-card movement;
- no runtime hook code, env toggle, process allocator replacement, or `.inc`
  name matching exists.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_runtime_owner_guard.sh` passes.
