# 293x-648 GUARD-MANIFEST-013 Declarative Guard Spec Pilot

Status: landed
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
bash tools/checks/guard_spec_pilot_guard.sh
bash tools/checks/run_row_guard.sh --only guard-spec-pilot
bash tools/checks/run_row_guard.sh --only hako-alloc-osvm-fast-path-route-closeout
bash tools/checks/guard_manifest_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
python3 -m py_compile tools/checks/guard_spec_runner.py
git diff --check
```

## Landed Result

`GUARD-MANIFEST-013` landed the first declarative guard spec pilot:

```text
tools/checks/guard_spec_runner.py
tools/checks/specs/hako_alloc_osvm_fast_path_route_closeout.toml
tools/checks/guard_spec_pilot_guard.sh
```

The selected pilot family is
`hako-alloc-osvm-fast-path-route-closeout`. Its public wrapper remains stable and
the manifest still owns the public-to-impl routing. Only the implementation body
became a thin `guard_spec_runner.py` call.

The row selects `MIMAP-141A`.
