---
Status: Done
Scope: stdlib trim coverage (trim_start, trim_end)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P4: stdlib trim coverage (trim_start, trim_end)

Goal: add fixtures/smokes for trim_start and trim_end, wired into the stdlib
pack and the JoinIR regression gate. No behavior or logging changes.

## Subsets (SSOT)

- trim_start: leading whitespace trimmed, trailing preserved
- trim_end: trailing whitespace trimmed, leading preserved

Inputs exclude empty-only strings; no new Pattern2 logic is introduced.

## Fixtures / smokes

- `apps/tests/phase29aq_string_trim_start_min.hako`
- `apps/tests/phase29aq_string_trim_end_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_trim_start_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_trim_end_min_vm.sh`

## Gate wiring

- Add both smokes to `tools/smokes/v2/profiles/integration/joinir/stdlib_string_pack_vm.sh`.
- Ensure `phase29ae_regression_pack_vm.sh` runs the stdlib pack.
- Update `docs/development/current/main/phases/phase-29ae/README.md`.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
