# Selfhost Example Payloads

This directory is now generators-first.

## General Generators

- `gen_v1_*.sh`
- small helper/example payloads used to generate or inspect MIR(JSON)
- not part of the legacy compat selfhost wrapper stack

## Note

- the historical compat payload has moved to `tools/archive/legacy-selfhost/compat-codegen/`
- this directory no longer carries an active `29x-98` cleanup surface
- cleanup sequencing for the compat wrapper lives in `CURRENT_TASK.md` and `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
- keep this directory focused on generators and small helper/example payloads
