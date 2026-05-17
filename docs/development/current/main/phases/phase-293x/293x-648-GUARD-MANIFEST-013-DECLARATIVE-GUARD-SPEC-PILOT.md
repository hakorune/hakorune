# 293x-648 GUARD-MANIFEST-013 Declarative Guard Spec Pilot

Status: selected current
Date: 2026-05-18

## Decision

`GUARD-MANIFEST-013` is the next BoxShape cleanup row after the batch migration
inventory. It should pilot a declarative guard spec for one small guard family
instead of continuing one-by-one hand migration.

## Scope

- Choose one low-risk guard family from the inventory output.
- Define the smallest declarative spec schema needed for that family.
- Generate or interpret the shared boilerplate without shell `eval` or
  `shell=True`.
- Keep the public `tools/checks/k2_wide_*.sh` entrypoint stable.
- Add a no-growth guard for the pilot schema.

## Stop Lines

- No allocator behavior.
- No compiler route behavior.
- No source syntax change.
- No all-at-once conversion of hundreds of guards.
- No deletion or rename of existing public guard entrypoints.
- No `dev_gate.sh` / allocator-wide policy change.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/guard_manifest_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
