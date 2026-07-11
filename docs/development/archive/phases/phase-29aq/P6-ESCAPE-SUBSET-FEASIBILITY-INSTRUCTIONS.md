---
Status: Done
Scope: escape/unescape feasibility (EscapeUtils) — unsupported
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P6: escape/unescape feasibility (EscapeUtils)

Goal: document escape/unescape as unsupported due to `validate_string` loop and
dynamic step/advance; no fixtures added.

## Subset (SSOT)

- escape_string: Unsupported (module contains `validate_string` loop)
- unescape_string: Unsupported (variable advance, not ScanWithInit)

## Fixture / smoke

- None (unsupported in current JoinIR/Composer)

## Gate wiring

- Document in `docs/development/current/main/phases/phase-29aq/README.md`.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
