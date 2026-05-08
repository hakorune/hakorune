---
Status: done
Date: 2026-05-09
Scope: distinct Contract(...) parser metadata on one declaration
---

# 293x-057 M11c Contract Repeatable Parser

## Decision

M11c-contract-repeat is live as a parser metadata shape.

Distinct `Contract(...)` values may appear on the same declaration:

```hako
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
```

Exact duplicate contract values still fail-fast as duplicate runes.

## Why

`M11c-required-verify` needs both `Contract(no_alloc)` and
`Contract(no_safepoint)` available as source/MIR metadata before it can prove
required inline. Keeping this as a parser metadata row avoids `.hako` source
workarounds and avoids teaching the backend to infer contracts from names.

## Owned

- Rust parser duplicate-key policy for distinct `Contract(...)` values
- `.hako` parser duplicate-key policy for distinct `Contract(...)` values
- focused accept/reject parser tests
- guard that locks the repeatable family to `Contract`

## Not Owned

- required-inline verification
- backend use of contract facts
- repeated `Hint(...)` semantics
- repeated ABI/ownership runes

## Acceptance

```bash
bash tools/checks/k2_wide_rune_contract_repeat_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Files

```text
src/parser/runes.rs
lang/src/compiler/parser/rune/rune_contract_box.hako
src/tests/parser_opt_annotations_parts/metadata.rs
src/tests/parser_opt_annotations_parts/rejects.rs
tools/checks/k2_wide_rune_contract_repeat_guard.sh
```
