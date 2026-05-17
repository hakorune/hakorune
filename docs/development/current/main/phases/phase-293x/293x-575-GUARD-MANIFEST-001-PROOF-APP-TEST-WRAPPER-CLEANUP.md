# 293x-575 GUARD-MANIFEST-001 Proof App Test Wrapper Cleanup

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-001` is a BoxShape cleanup sidecar selected before adding more
MIMAP rows. The immediate target is the repeated app-local `test.sh` routing
for proof apps that already exist in `tools/checks/proof_apps.toml`.

This row does not change allocator behavior or compiler acceptance. It makes
the existing manifest runner the routing SSOT for manifest-backed proof app
test entrypoints.

Next selected row:

```text
GUARD-MANIFEST-002
```

`GUARD-MANIFEST-002` should select one `k2_wide_*` guard family for manifest
thin-wrapper migration. It must not bundle allocator behavior or compiler
acceptance changes.

## Scope

- Add a durable guard manifest migration SSOT.
- Convert manifest-backed `apps/*/test.sh` files to call
  `tools/checks/run_proof_app.sh --only <id>`.
- Add a no-growth guard that prevents manifest-backed proof app tests from
  directly calling `k2_wide_*_guard.sh`.
- Wire the new no-growth guard into the manifest pilot and check-script index.

## Stop Lines

- No deletion or rename of existing `k2_wide_*_guard.sh` entrypoints.
- No broad generator rewrite.
- No `dev_gate.sh` / allocator-wide integration for manifest pilots.
- No allocator `.hako` behavior.
- No compiler acceptance change.
- No backend `.inc` route or matcher change.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM001.1` | Add migration SSOT. | `guard-manifest-migration-ssot.md` exists and names runner owners. | no behavior |
| `GM001.2` | Thin manifest-backed proof app tests. | listed proof app tests delegate to `run_proof_app.sh --only <id>`. | keep guard entrypoints |
| `GM001.3` | Add no-growth guard. | guard fails on direct `k2_wide_*` calls from manifest-backed tests. | no dev_gate wiring |
| `GM001.4` | Wire docs/index/manifests. | check index lists the guard and row runner can list it. | no allocator-wide wiring |

## Required Evidence

```text
bash tools/checks/proof_app_manifest_test_entry_guard.sh
tools/checks/run_row_guard.sh --only proof-app-manifest-test-entry
tools/checks/run_proof_app.sh --only MIMAP-085A --dry-run
bash apps/hako-alloc-segment-page-membership-scalar-proof/test.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added `docs/development/current/main/design/guard-manifest-migration-ssot.md`.
- Added `tools/checks/proof_app_manifest_test_entry_guard.sh`.
- Added `proof-app-manifest-test-entry` to `tools/checks/guard_rows.toml`.
- Converted manifest-backed proof app `test.sh` files to
  `run_proof_app.sh --only <id>`.
- Kept all existing `k2_wide_*_guard.sh` entrypoints intact.
