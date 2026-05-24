---
Status: Landed
Date: 2026-05-24
Scope: normalize manifest-backed proof app-local `test.sh` entrypoints.
Blocker: PROOF-APP-ENTRYPOINT-CLEANUP-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-219-PROOF-APP-ENTRYPOINT-CLEANUP-PHASE-LOCK.md
  - docs/tools/check-scripts-index.md
  - tools/checks/lib/proof_app_test_entry.sh
  - tools/checks/proof_app_manifest_test_entry_guard.sh
  - tools/checks/run_proof_app.sh
  - tools/checks/proof_apps.toml
---

# 294x-220 Proof App Entrypoint Cleanup

## Decision

Normalize manifest-backed proof app-local `test.sh` files through one shared
entry helper:

```bash
tools/checks/lib/proof_app_test_entry.sh <proof-id> "$@"
```

The helper preserves each app-local `test.sh` public path while centralizing the
actual dispatch to:

```bash
tools/checks/run_proof_app.sh --only <proof-id>
```

## Scope

Converted only app-local `test.sh` files that have a manifest proof id. Most
already delegated to `run_proof_app.sh --only <id>`; the remaining
manifest-backed direct-guard delegates now go through the same helper so the
manifest runner remains the single app-local proof entry owner.

Inventory after conversion:

```text
proof_app_test_entry delegates: 203
direct guard / guard-like delegates: 79
other app tests: 9
```

## Stop Line

This row does not change:

- proof app `.hako` behavior;
- manifest ids or command selection;
- validation profile semantics;
- direct guard delegates;
- non-proof app tests;
- exact `usize` language/runtime semantics.

## Verification

```bash
find apps -path '*/test.sh' -type f -print0 | xargs -0 -n 50 bash -n
bash -n tools/checks/lib/proof_app_test_entry.sh
bash tools/checks/proof_app_manifest_test_entry_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
