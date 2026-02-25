---
Status: Done
Scope: stdlib derived helpers (contains, starts_with, ends_with, trim)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P5: stdlib derived string helpers (coverage-only)

Goal: add fixtures/smokes for non-loop derived helpers and wire them into the
stdlib pack and JoinIR regression gate. No new subsets or logging.

## Fixtures

- `apps/tests/phase29aq_string_contains_min.hako`
- `apps/tests/phase29aq_string_starts_with_min.hako`
- `apps/tests/phase29aq_string_ends_with_min.hako`
- `apps/tests/phase29aq_string_trim_min.hako`

## Smokes

- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_contains_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_starts_with_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_ends_with_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_trim_min_vm.sh`

## Gate wiring

- Add all four smokes to `tools/smokes/v2/profiles/integration/joinir/phase29aq_stdlib_pack_vm.sh`.
- Ensure `phase29ae_regression_pack_vm.sh` runs the stdlib pack.
- Update `docs/development/current/main/phases/phase-29ae/README.md`.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
