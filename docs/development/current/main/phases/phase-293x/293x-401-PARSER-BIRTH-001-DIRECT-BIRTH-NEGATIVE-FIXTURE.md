# 293x-401 PARSER-BIRTH-001 Direct Birth Negative Fixture

Status: landed
Date: 2026-05-15

## Decision

`PARSER-BIRTH-001` follows `LIFECYCLE-BIRTH-001` by turning the new-only birth
policy into a parser-level negative fixture.
`birth` remains a constructor hook fired by `new`, not a receiver-callable
method.

The parser should reject direct source receiver calls such as:

```hako
page.birth(PageId(0), Bytes(32), 2, 2)
```

Constructor declarations remain valid:

```hako
box Page {
  birth(id, bytes) { }
}
```

## Scope

- Add a focused parser/source fixture for direct receiver `birth(...)`.
- Keep existing constructor declarations accepted.
- Keep the improved user-facing hint for `PARSER-BIRTH-002`.

## Stop Lines

- Do not reject constructor declarations `birth(...) { ... }`.
- Do not change plugin/BID method-id semantics.
- Do not rewrite lower-case host/substrate facade names in this row unless the
  lifecycle SSOT explicitly reclassifies them.
- Do not mix named constructor arguments or lifecycle reuse semantics into this
  parser fixture row.

## Required Evidence

```text
bash tools/checks/k2_wide_parser_birth_direct_call_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Added a parser lifecycle policy helper for direct source `birth` receiver
  calls.
- Rejected `obj.birth(...)` in both the legacy expression parser and the
  TokenCursor expression parser.
- Added focused parser fixtures that reject direct receiver `birth(...)` while
  keeping constructor declarations and `from Parent.birth(...)` delegation
  accepted.
- Added `tools/checks/k2_wide_parser_birth_direct_call_guard.sh`.

## Evidence

```text
cargo test -q --lib parser_birth_
bash tools/checks/k2_wide_parser_birth_direct_call_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
