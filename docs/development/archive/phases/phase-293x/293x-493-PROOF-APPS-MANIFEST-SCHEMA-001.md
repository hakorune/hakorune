# 293x-493 PROOF-APPS-MANIFEST-SCHEMA-001

Status: landed
Date: 2026-05-16

## Decision

`PROOF-APPS-MANIFEST-SCHEMA-001` is the tooling cleanup selected by
`RECORD-VALUES-REG-002`.

`tools/checks/proof_apps.toml` mostly uses the active `[[proof_apps]]` manifest
schema. The trailing M214/M215 rows use the singular `[[proof_app]]` schema and
are therefore not listed by `tools/checks/run_proof_app.sh --list`. This row
normalizes only those rows.

## Scope

- Change the M214/M215 manifest rows from `[[proof_app]]` to `[[proof_apps]]`.
- Add `label` and `profiles` fields matching nearby hako-alloc proof rows.
- Keep the existing `cmd` entries and proof guard bodies unchanged.
- If needed, extend `manifest_runner_pilot_guard.sh` to assert that M214/M215
  are visible through the proof-app runner list.

## Stop Lines

- Do not change `tools/checks/lib/manifest_runner.py` selection semantics.
- Do not wire proof-app pilots into `dev_gate.sh`, allocator-wide gates, or any
  default suite.
- Do not alter proof guard bodies or proof app source files.
- Do not touch compiler, parser, kernel, allocator, provider, hooks, host
  allocator replacement, or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `PAM.1` | Normalize M214/M215 manifest rows. | `run_proof_app.sh --list` shows both rows. | no runner semantic changes |
| `PAM.2` | Pin visibility in pilot guard if needed. | guard catches future schema drift. | no default gate wiring |
| `PAM.3` | Verify manifest runner surface. | required evidence is green. | no proof body changes |
| `PAM.4` | Closeout docs and advance current. | current moves to next row selection. | no behavior row |

## Required Evidence

```text
tools/checks/run_proof_app.sh --list | rg 'M214|M215'
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

This row closes when M214/M215 are visible through the proof-app manifest runner
and no default gate wiring or proof behavior has changed.

## Result

Landed:

- M214/M215 manifest rows now use the active `[[proof_apps]]` schema.
- M214/M215 now carry the runner-required `label` and `profiles` fields.
- `manifest_runner_pilot_guard.sh` now asserts M214/M215 are visible through
  `run_proof_app.sh --list`.

No runner selection semantics, proof guard bodies, default gate wiring, compiler,
parser, kernel, allocator, provider, hooks, host allocator replacement, or
`#[global_allocator]` behavior changed.

## Evidence

```text
tools/checks/run_proof_app.sh --list | rg 'M214|M215'
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
