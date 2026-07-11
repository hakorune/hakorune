---
Status: Done
Scope: parse_integer variants (sign/whitespace/leading zero)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P7: parse_integer variants (stdlib coverage)

Goal: add stdlib coverage for parse_integer variants (sign, leading whitespace,
leading zero) using fixtures + smokes; keep behavior unchanged (invalid returns
0).

## Subset (SSOT)

- sign: "-42" -> -42
- leading whitespace: " 42" -> 0
- leading zero: "0123" -> 0

## Fixtures / smokes

- apps/tests/phase29aq_string_parse_integer_sign_min.hako
- apps/tests/phase29aq_string_parse_integer_ws_min.hako
- apps/tests/phase29aq_string_parse_integer_leading_zero_min.hako
- tools/smokes/v2/profiles/integration/joinir/phase29aq_string_parse_integer_sign_min_vm.sh
- tools/smokes/v2/profiles/integration/joinir/phase29aq_string_parse_integer_ws_min_vm.sh
- tools/smokes/v2/profiles/integration/joinir/phase29aq_string_parse_integer_leading_zero_min_vm.sh

## Gate wiring

- tools/smokes/v2/profiles/integration/joinir/stdlib_string_pack_vm.sh
- docs/development/current/main/phases/phase-29ae/README.md

## Verification

- cargo build --release
- ./tools/smokes/v2/run.sh --profile quick
- ./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
