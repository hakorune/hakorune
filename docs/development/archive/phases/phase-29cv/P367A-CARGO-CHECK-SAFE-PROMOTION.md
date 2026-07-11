---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: promote EXDEV-safe cargo check wrapper from tools/dev to tools/checks
Related:
  - tools/checks/cargo_check_safe.sh
  - tools/checks/exdev_rename_copy_fallback.c
  - tools/dev/README.md
---

# P367A: Cargo Check Safe Promotion

## Intent

Move the EXDEV-safe cargo check wrapper out of `tools/dev`.

This wrapper is a check entry for constrained filesystems, not an interactive
developer helper. The paired C preload helper moves with it so the owner stays
local.

## Boundary

Allowed:

- move `cargo_check_safe.sh` and its paired C helper to `tools/checks`
- update the wrapper's helper path
- update docs that name the active command
- shrink the `tools/dev` inventory

Not allowed:

- change cargo check defaults
- change LD_PRELOAD fallback semantics
- wire this wrapper into quick gate

## Acceptance

```bash
bash -n tools/checks/cargo_check_safe.sh
gcc -shared -fPIC -O2 -o /tmp/hako_exdev_rename_copy_fallback.so tools/checks/exdev_rename_copy_fallback.c -ldl
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
