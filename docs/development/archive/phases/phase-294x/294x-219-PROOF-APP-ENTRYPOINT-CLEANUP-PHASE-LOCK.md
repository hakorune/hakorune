---
Status: Landed
Date: 2026-05-24
Scope: cleanup slice selection for proof app / guard entrypoint boilerplate.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-217
Related:
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/tools/check-scripts-index.md
  - tools/checks/run_proof_app.sh
  - tools/checks/lib/manifest_runner.py
  - tools/checks/proof_apps.toml
---

# 294x-219 Proof App Entrypoint Cleanup Phase Lock

## Decision

Pause exact-`usize` field-group migration for a short cleanup slice and select
`PROOF-APP-ENTRYPOINT-CLEANUP-001` as the next blocker.

This is a BoxShape cleanup row, not a new language feature and not a
field-group migration. The active lane remains `phase-294x usize semantic
foundation`; this row only prevents proof/guard boilerplate from making later
allocator rows harder to review.

## Inventory

Current `apps/*/test.sh` entrypoints are not all identical:

```text
total test.sh files: 291
run_proof_app.sh delegates: 193
direct guard delegates: 89
other app tests: 9
```

Therefore, do not bulk-delete app-local `test.sh` files. First normalize only
the manifest-backed proof-app delegates that already call `run_proof_app.sh`.
Direct guard delegates and non-proof app tests stay unchanged until their own
row classifies them.

## Next Cleanup Row

`PROOF-APP-ENTRYPOINT-CLEANUP-001` should:

- create or reuse one stable app-local shim contract for manifest-backed proof
  app tests;
- convert only `run_proof_app.sh --only <id>` delegates that have a matching
  `proof_apps.toml` entry;
- preserve app-local `test.sh` paths for compatibility;
- keep direct guard delegates and non-proof app tests out of scope;
- add a lightweight inventory/check so future rows do not reintroduce thick
  app-local boilerplate.

## Stop Line

Do not change:

- proof app `.hako` behavior;
- guard semantics or validation level selection;
- `proof_apps.toml` item ids;
- direct guard wrappers;
- non-proof application tests;
- active exact-`usize` language/runtime semantics.

## Verification

Docs-only cleanup selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
