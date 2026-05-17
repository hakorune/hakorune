# 293x-652 GUARD-MANIFEST-014 Proof App Test Wrapper Backfill

Status: landed
Date: 2026-05-18

## Decision

`GUARD-MANIFEST-014` is a BoxShape cleanup sidecar before `MIMAP-143A`.

`proof_app_manifest_test_entry_guard.sh` found two existing proof apps whose
app-local `test.sh` still called public `k2_wide_*` guards directly:

```text
MIMAP-134A
MIMAP-138A
```

The fix is to make those wrappers delegate through:

```text
tools/checks/run_proof_app.sh --only <id>
```

## Scope

- Update only the stale app-local `test.sh` wrappers.
- Keep public guard entrypoints stable.
- Keep `proof_apps.toml` unchanged.

## Stop Lines

- No allocator behavior.
- No compiler route behavior.
- No source syntax change.
- No guard semantics change.
- No provider activation, backend matcher, or silent fallback.

## Required Evidence

```text
bash tools/checks/proof_app_manifest_test_entry_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`MIMAP-134A` and `MIMAP-138A` app-local test wrappers now use the manifest proof
app runner. Current allocator work remains on `MIMAP-143A`.
