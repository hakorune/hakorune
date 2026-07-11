---
Status: landed
Decision: accepted
Date: 2026-05-18
---

# 293x-675 ROW-VALIDATION-PROFILE-001 Manifest Schema Pilot

## Decision

Add row validation profile metadata to the manifest runner and seed the current
segment-map readiness family. This keeps small allocator rows proof-first while
making the intended validation weight explicit.

## Owner

```text
docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md
tools/checks/lib/manifest_runner.py
tools/checks/proof_apps.toml
docs/tools/check-scripts-index.md
```

## Scope

- Add optional manifest fields:
  - `row_kind`
  - `validation_profile`
  - `first_pattern`
  - `closeout_pack`
  - `exe`
  - `exe_skip_reason`
- Add manifest runner selectors:
  - `--validation-profile`
  - `--row-kind`
  - `--closeout-pack`
- Seed the current segment-map readiness family:
  - `MIMAP-149A`: `scalar-mir`
  - `MIMAP-151A`: `inventory`
  - `MIMAP-153A`: `scalar-mir`
- Keep existing public guard execution behavior unchanged.

## Stop Lines

- Do not weaken or skip any existing public row guard.
- Do not wire manifest profile selection into `dev_gate.sh` or allocator-wide.
- Do not bulk-delete `k2_wide_*` entrypoints.
- Do not implement VM/MIR/EXE subcommand splitting in this row.
- Do not change allocator behavior, compiler acceptance, route vocabulary, or
  backend lowering.

## Evidence

```text
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/run_proof_app.sh --validation-profile scalar-mir --dry-run
bash tools/checks/run_proof_app.sh --row-kind inventory --dry-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

`MIMAP-154A` remains the active allocator planning blocker. A future
`ROW-VALIDATION-PROFILE-002` may split selected public guard bodies into
manifest-selectable VM/MIR/EXE commands after the current segment-map closeout
shape is chosen.
