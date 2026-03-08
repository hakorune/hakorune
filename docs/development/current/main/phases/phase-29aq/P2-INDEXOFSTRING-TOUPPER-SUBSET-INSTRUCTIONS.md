---
Status: Done
Scope: stdlib subsets (index_of_string, to_upper)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P2: stdlib subsets (index_of_string, to_upper)

Goal: add two stdlib subsets with fixtures and integration smokes, wired to the
JoinIR regression gate (phase29ae pack).

## P2-1: to_upper (Pattern1CharMap)

Target: `apps/lib/json_native/utils/string.hako`

- loop(i < s.length())
- local ch = s.substring(i, i + 1)
- result = result + this.char_to_upper(ch)
- i = i + 1

Notes:
- Use existing Pattern1CharMap facts/planner/normalizer path.
- No new CorePlan vocabulary or logs.

Fixtures/smokes:
- `apps/tests/phase29aq_string_to_upper_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_to_upper_min_vm.sh`

## P2-2: index_of_string (ScanWithInit dynamic needle)

Target: `apps/lib/json_native/utils/string.hako`

- loop(i <= s.length() - substr.length())
- if s.substring(i, i + substr.length()) == substr { return i }
- i = i + 1
- return -1

Notes:
- Treat as ScanWithInit with dynamic needle length.
- Facts must detect the dynamic needle length and forward scan shape.

Fixtures/smokes:
- `apps/tests/phase29aq_string_index_of_string_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_index_of_string_min_vm.sh`

## Gate wiring (SSOT)

- Add both smokes to `tools/smokes/v2/profiles/integration/joinir/stdlib_string_pack_vm.sh`.
- Ensure `phase29ae_regression_pack_vm.sh` runs the stdlib pack.
- Update `docs/development/current/main/phases/phase-29ae/README.md`.

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
