---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: promote lang include guard from dev utility surface to checks surface
Related:
  - tools/checks/lang_include_surface_guard.sh
  - docs/tools/check-scripts-index.md
  - docs/development/current/main/phases/phase-29cv/README.md
---

# P361A: Lang Include Guard Promotion

## Intent

Move the `lang/src` include-ban check out of active `tools/dev`.

The old `tools/dev/check_lang_includes.sh` script had no active callers, but
the contract it checks is still current: `lang/src` should not regain
`include "..."` source directives. That makes it a guard, not a dev utility.

## Boundary

Allowed:

- move the check to `tools/checks/lang_include_surface_guard.sh`
- wire the check into quick gate
- fail if the old `tools/dev/check_lang_includes.sh` path returns

Not allowed:

- change Hako include/using semantics
- change `hako_preinclude.sh` behavior
- scan archived docs or C `#include` files

## Guard

`tools/checks/lang_include_surface_guard.sh` scans only `lang/src` for source
lines matching `include "..."`. It also fails if the old active dev path
returns.

## Acceptance

```bash
bash tools/checks/lang_include_surface_guard.sh
bash -n tools/checks/lang_include_surface_guard.sh tools/checks/dev_gate.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
