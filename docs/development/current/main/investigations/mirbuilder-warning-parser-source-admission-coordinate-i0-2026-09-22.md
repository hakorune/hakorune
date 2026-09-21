---
Status: closed__2026-09-22__WarningParserSourceAdmissionCoordinate__DeletedAndVerified
Task: MIRBUILDER-WARNING-PARSER-SOURCE-ADMISSION-COORDINATE-I0
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i126-2026-09-22.md
Implementation permission: true for deleting the unused accessor only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I128
---

# Warning cleanup: unused parser source-admission coordinate accessor

## Six-line brief

```text
Decision: delete ParserSourceAdmissionRowV1::coordinate(); no caller observes
  this row through the accessor.
Source authority + canonical issuer: the existing parser source-admission row
  issuer and its rows() consumer.
Non-authority: warning guesses, AST/source re-reading, parser admission
  semantics, guard suppression, or a replacement coordinate projection.
Fail-fast boundary: any caller, compile error, focused red, or new warning
  owner rejects the deletion.
Smallest next slice: remove exactly the one method body and run parser source
  admission tests plus the stable guards.
Non-claims: no source lineage, resolver, route, fallback, production switch,
  old-edge retirement, or LegacyCallV0 change.
```

## Census and delete set

The selected warning is the method at
`src/parser/normal_callable_program_source/source_admission.rs:20`:

```text
ParserSourceAdmissionRowV1::coordinate
```

Repository-wide Rust search found only its declaration. The containing row is
still issued and consumed through `ParserSourceAdmissionRowsV1::rows()`; only
this convenience accessor is production/test-zero. The delete set is exactly
the method body. No field, issuer, row collection, source identity, or guard
file is changed.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib parser::normal_callable_program_source
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

The focused filter must execute nonzero named tests and pass. Record the new
lib/lib-test counts without suppressions or replacement accessors.

## Closeout evidence

`ParserSourceAdmissionRowV1::coordinate` was deleted; the coordinate field,
issuer, row collection, and parser authority remain unchanged. `cargo fmt
--all -- --check` passed. The parser source-admission filter passed **44/44**.
`cargo check --profile quick --lib -j4` passed with lib **1,691** warnings;
the warning group still contains `canonical_segment` and `local_line`.
`cargo test --profile quick --lib --no-run -j4` passed with lib-test **546**
warnings, down one from 547. The pointer guard passed. No `#[allow]`, test
deletion, replacement accessor, or semantic route change was introduced.
