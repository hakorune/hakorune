# 293x-402 PARSER-BIRTH-002 Direct Birth Diagnostic Hint

Status: ready
Date: 2026-05-15

## Decision

`PARSER-BIRTH-002` follows the parser-level negative fixture by improving the
user-facing diagnostic for direct receiver `birth(...)` calls.

The parser already rejects:

```hako
page.birth(PageId(0), Bytes(32), 2, 2)
```

This row should make the diagnostic point construction toward:

```hako
new Page(PageId(0), Bytes(32), 2, 2)
```

## Scope

- Improve the direct source receiver `birth(...)` parser error text with a
  `use new Box(...)` hint.
- Keep the hint owned by the parser lifecycle policy helper.
- Add a narrow test/guard that fixes the user-facing diagnostic text.

## Stop Lines

- Do not widen source syntax to accept `obj.birth(...)`.
- Do not reject constructor declarations `birth(...) { ... }`.
- Do not change `from Parent.birth(...)` constructor delegation.
- Do not add named constructor arguments or lifecycle reuse semantics in this
  row.

## Required Evidence

```text
bash tools/checks/k2_wide_parser_birth_diagnostic_hint_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
