---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: repair invocation contract for dev sugar pre-expand utilities
Related:
  - docs/guides/dev-local-alias.md
  - tools/dev/at_local_preexpand.sh
  - tools/dev/dev_sugar_preexpand.sh
  - docs/development/current/main/phases/phase-29cv/README.md
---

# P359A: Dev Sugar Pre-Expand Invoke Contract

## Intent

Fix a small utility drift found during the active `tools/dev` inventory.

`tools/dev/at_local_preexpand.sh` is tracked as a shell helper, but this repo
does not record file mode changes (`core.filemode=false`). The docs and the
composed `dev_sugar_preexpand.sh` path assumed direct execution, so the broader
pre-expander failed with `Permission denied` when it tried to run the helper.

## Boundary

Allowed:

- make the composed helper call `at_local_preexpand.sh` through `bash`
- document the supported `bash tools/dev/...` invocation
- verify the documented and composed helper paths

Not allowed:

- change language sugar semantics
- add new syntax
- move these helpers to archive
- rely on executable file-mode bits for correctness

## Acceptance

```bash
bash tools/dev/at_local_preexpand.sh apps/tests/dev_sugar/at_local_basic.hako >/tmp/p359a_at_local.hako
bash tools/dev/dev_sugar_preexpand.sh apps/tests/dev_sugar/compound_and_inc.hako >/tmp/p359a_dev_sugar.hako
bash tools/dev/dev_sugar_preexpand.sh apps/tests/dev_sugar/print_when_fn.hako >/tmp/p359a_print_when.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
