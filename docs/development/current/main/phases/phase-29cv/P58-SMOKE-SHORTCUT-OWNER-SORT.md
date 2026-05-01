---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: sort remaining root smoke shortcut hold queue items.
Related:
  - tools/ROOT_SURFACE.md
  - tools/archive/manual-smokes/README.md
  - docs/guides/operator-boxes.md
  - tools/checks/dev_gate.sh
  - docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md
---

# P58 Smoke Shortcut Owner Sort

## Goal

Close the remaining root smoke shortcut hold queue without deleting active user
or gate entrypoints.

This is a BoxShape cleanup slice. It separates archived manual smokes from
documented root shortcuts and protected gates.

## Owner Reading

- `llvm_smoke.sh` is a historical llvmlite harness compat/probe smoke, not
  ny-llvmc daily proof.
- `modules_smoke.sh` is an old manual modules JSON VM smoke with no current root
  gate owner.
- `opbox-json.sh` and `opbox-quick.sh` are still documented root dev shortcuts.
- `vm_plugin_smoke.sh` is called by `tools/checks/dev_gate.sh` and plugin guard
  checks, so it is a protected current smoke wrapper.

## Decision

- move `llvm_smoke.sh` and `modules_smoke.sh` to
  `tools/archive/manual-smokes/`
- keep `opbox-json.sh` and `opbox-quick.sh` in root as documented manual smoke
  shortcuts
- keep `vm_plugin_smoke.sh` in root as a protected current smoke wrapper
- update active docs that still pointed at the moved manual smokes
- close the root hold queue in `tools/ROOT_SURFACE.md`

## Non-goals

- do not run the archived LLVM harness smoke as part of this cleanup
- do not change OperatorBox smoke behavior
- do not change plugin pilot smoke coverage
- do not delete archived smoke files

## Acceptance

```bash
bash -n tools/archive/manual-smokes/llvm_smoke.sh
bash -n tools/archive/manual-smokes/modules_smoke.sh
bash -n tools/opbox-json.sh
bash -n tools/opbox-quick.sh
bash -n tools/vm_plugin_smoke.sh
test ! -e tools/llvm_smoke.sh
test ! -e tools/modules_smoke.sh
rg -n 'tools/archive/manual-smokes/(llvm_smoke.sh|modules_smoke.sh)' \
  tools/archive/manual-smokes/README.md docs/development/current docs/development/issues apps tools/smokes/jit-migration-plan.md
rg -n 'tools/opbox-(json|quick).sh' README.md README.ja.md docs/guides tools/ROOT_SURFACE.md
rg -n 'tools/vm_plugin_smoke.sh' tools/checks tools/ROOT_SURFACE.md
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/llvm_smoke.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main docs/development/current/llvm docs/development/issues apps
! rg -g '!docs/development/current/main/phases/phase-29cv/P5*.md' --fixed-strings \
  'tools/modules_smoke.sh' \
  tools src lang Makefile README.md README.ja.md docs/guides docs/development/current/main docs/development/current/llvm docs/development/issues apps
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
