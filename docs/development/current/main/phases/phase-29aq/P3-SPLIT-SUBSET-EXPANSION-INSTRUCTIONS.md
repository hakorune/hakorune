---
Status: Done
Scope: stdlib split subset expansion (char/string)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P3: split subset expansion (stdlib coverage)

Goal: expand stdlib split coverage with char and string delimiter fixtures and
wire them into the JoinIR regression gate (phase29ae pack). Behavior is
unchanged; inputs avoid empty segments.

## Subsets (SSOT)

- Char delimiter: `split("a b c", " ")` (single-character separator)
- String delimiter: `split("a--b--c", "--")` (multi-character separator)

Notes:
- Empty segments are excluded by input in fixtures (no leading/trailing or
  repeated delimiters).
- No new Plan/Composer vocabulary; reuse existing SplitScan subset.

## Fixtures / smokes

- `apps/tests/phase29aq_string_split_char_min.hako`
- `apps/tests/phase29aq_string_split_string_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_split_char_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_split_string_min_vm.sh`

## Gate wiring

- Add both smokes to `tools/smokes/v2/profiles/integration/joinir/phase29aq_stdlib_pack_vm.sh`.
- Ensure `phase29ae_regression_pack_vm.sh` runs the stdlib pack.
- Update `docs/development/current/main/phases/phase-29ae/README.md`.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
