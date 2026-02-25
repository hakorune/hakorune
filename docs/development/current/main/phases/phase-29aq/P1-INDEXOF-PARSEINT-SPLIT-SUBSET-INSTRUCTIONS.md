---
Status: Done
Scope: stdlib subsets (index_of/last_index_of, parse_integer, split)
Related:
- docs/development/current/main/phases/phase-29aq/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29aq P1: stdlib subsets (index_of/last_index_of, parse_integer, split)

Goal: add minimal subsets for three stdlib loops, with fixtures and gate smokes.
Order is fixed: index_of/last_index_of → parse_integer → split.

## P1-1: index_of / last_index_of (ScanWithInit)

Target: `apps/lib/json_native/utils/string.hako`

- `index_of(s, ch)`:
  - loop(i < s.length())
  - if s.substring(i, i + 1) == ch { return i }
  - i = i + 1
  - return -1
- `last_index_of(s, ch)`:
  - loop(i >= 0)
  - if s.substring(i, i + 1) == ch { return i }
  - i = i - 1
  - return -1

Subset policy:

- ScanWithInit facts only; no new CorePlan vocabulary.
- Return-in-loop is allowed only for immediate `return i` with literal fallback at end.
- Facts normalize `>= 0` reverse step to existing scan shape if possible; otherwise Ok(None).

Fixtures/smokes:

- `apps/tests/phase29aq_string_index_of_min.hako`
- `apps/tests/phase29aq_string_last_index_of_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_index_of_min_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_last_index_of_min_vm.sh`
- Wire both into `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## P1-2: parse_integer (Pattern2Break)

Target: `apps/lib/json_native/utils/string.hako`

- loop(i < s.length())
- d = this.index_of(digits, ch)
- if d < 0 { break }
- acc = acc * 10 + d
- i = i + 1
- return acc / neg handling at end

Subset policy:

- Pattern2Break facts only; keep break-only (no continue).
- Keep existing “not is_digit” normalization rules; no new UnaryOp.
- If neg handling is present, allow a final `return 0 - acc` literal expression only.

Fixtures/smokes:

- `apps/tests/phase29aq_string_parse_integer_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_parse_integer_min_vm.sh`
- Wire into `phase29ae_regression_pack_vm.sh`

## P1-3: split (SplitScan)

Target: `apps/lib/json_native/utils/string.hako`

- loop(i <= s.length() - separator.length())
- if s.substring(i, i + separator.length()) == separator { push segment; i = start }
- else { i = i + 1 }
- final push after loop

Subset policy:

- SplitScan facts only; no value-join handling in this phase.
- Only allow `separator.length() > 0` path (empty separator returns early).
- If step join/value_join is required, return Ok(None) (do not Freeze).

Fixtures/smokes:

- `apps/tests/phase29aq_string_split_min.hako`
- `tools/smokes/v2/profiles/integration/joinir/phase29aq_string_split_min_vm.sh`
- Wire into `phase29ae_regression_pack_vm.sh`

## Verification (required when code lands)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
