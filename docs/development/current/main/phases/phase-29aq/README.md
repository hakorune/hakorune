---
Status: Complete
Scope: stdlib subset expansion (closeout)
Related:
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/phases/phase-29ap/README.md
---

# Phase 29aq: stdlib subset expansion (closeout)

Goal: expand stdlib coverage with conservative Plan/Composer subsets and
document unsupported cases for handoff.

## Ok(None) / Freeze boundary (SSOT)

- Ok(None): non-matching shapes (default).
- Freeze: only when a shape is explicitly gated and contradictory; do not add
  new Freeze cases in P0.

## Fixture / smoke naming (SSOT)

- Fixtures: `apps/tests/phase29aq_<area>_<func>_<shape>_min.hako`
- Smokes: `tools/smokes/v2/profiles/integration/joinir/phase29aq_<area>_<func>_<shape>_vm.sh`
- Gate: wire into `phase29ae_regression_pack_vm.sh` when a subset is added.

## Added subsets (P1–P7)

- LoopBreak (historical label `2`): trim_start/trim_end, parse_integer (basic + sign/whitespace/leading zero)
- ScanWithInit: index_of, last_index_of, to_upper
- SplitScan: split (min/char/string), index_of_string
- Derived helpers: contains/starts_with/ends_with/trim (fixture coverage)

## Inventory (stdlib json_native)

### StringUtils (`apps/lib/json_native/utils/string.hako`)

| Function | Loop summary | Provisional bucket | Notes |
| --- | --- | --- | --- |
| `trim_start` | scan until non-whitespace, `break` | LoopBreak | Subset already exists (Phase 29ap P6) |
| `trim_end` | reverse scan until non-whitespace, `break` | LoopBreak | Subset already exists (Phase 29ap P6) |
| `index_of` | scan, early `return` on match | ScanWithInit | Needs return-in-loop handling |
| `last_index_of` | reverse scan, early `return` | ScanWithInit | Needs return-in-loop handling |
| `index_of_string` | scan substrings, early `return` | SplitScan | Candidate subset |
| `to_upper` | scan + accumulate | ScanWithInit | Candidate subset |
| `to_lower` | scan + accumulate | LoopSimpleWhile subset | Already migrated (Phase 29ap P2) |
| `join` | array join with `if i > 0` separator | LoopSimpleWhile subset | Already migrated (Phase 29ap P3) |
| `split` | scan + push segments | SplitScan | Candidate subset |
| `is_integer` | scan digits, early `return false` | Unsupported | Return-in-loop needs CorePlan vocab |
| `parse_integer` | scan digits, `break` on invalid | LoopBreak | Subset variants (sign/whitespace/leading zero) |

### EscapeUtils (`apps/lib/json_native/utils/escape.hako`)

| Function | Loop summary | Provisional bucket | Notes |
| --- | --- | --- | --- |
| `escape_string` | scan + append `escape_char` | Unsupported | `validate_string` loop unsupported (module compile) |
| `unescape_string` | scan + variable advance | Unsupported | dynamic step (`advance`) |

### Lexer / Parser (out of scope for P0)

| File | Loop summary | Provisional bucket | Notes |
| --- | --- | --- | --- |
| `lexer/scanner.hako` | scan loops with nested loops (string/number) | Unsupported | Multiple nested scans |
| `lexer/tokenizer.hako` | tokenize loop with `break`/`continue` | Unsupported | Control-flow heavy |
| `parser/parser.hako` | `loop(true)` object/array parse | Unsupported | Structured parse loop |
| `utils/escape.hako` | escape/unescape scans | Unsupported | Nested scan/branching |
| `core/node.hako` | collection iteration loops | Unsupported | Not a parsing scan |

## Gate / Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29aq_stdlib_pack_vm.sh`

## Completed steps

- P1: Add stdlib subsets in priority order (index_of/last_index_of → parse_integer → split).
- P2: Add stdlib subsets (index_of_string → to_upper).
- P3: Expand split subsets (char/string delimiters; empty segments excluded by input).
- P4: Expand stdlib trim coverage (trim_start/trim_end fixtures).
- P5: Fix derived stdlib helpers (contains/starts_with/ends_with/trim) with fixtures.
- P6: Document escape/unescape unsupported due to `validate_string` loop.
- P7: Add parse_integer variants (sign/whitespace/leading zero fixtures).
- P8: Keep is_integer unsupported; handoff to return-in-loop minimal.

## Unsupported / Deferred

- StringUtils.is_integer (return-in-loop)
- EscapeUtils.escape_string / unescape_string

## Next phase (TBD)

- return-in-loop minimal (CorePlan vocabulary)
