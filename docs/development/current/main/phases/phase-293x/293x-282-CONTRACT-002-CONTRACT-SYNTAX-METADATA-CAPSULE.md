# 293x-282 CONTRACT-002 Contract Syntax Metadata Capsule

Status: complete
Date: 2026-05-14

## Scope

Parse `requires`, `ensures`, and `invariant` as read-only contract metadata.

## Landed changes

- Added contract clause metadata to function declarations.
- Added invariant metadata to box and record declarations.
- Parsed `requires` / `ensures` before function and method bodies.
- Parsed `invariant` in box and record member lists.
- Kept `requires`, `ensures`, and `invariant` contextual rather than global keywords.
- Preserved function bodies without injecting runtime checks.
- Transported contract/invariant metadata through AST JSON and Program JSON v0.
- Added parser and Program JSON tests plus a dedicated guard.

## Non-goals

- No `assert` sugar.
- No runtime precondition/postcondition insertion.
- No invariant boundary policy.
- No verifier discharge.
- No Stage0 type or lifecycle checking.

## Guard

```bash
bash tools/checks/k2_wide_contract_syntax_metadata_guard.sh
```

## Next selected row

`TRANS-001 transition metadata capsule`.

`CONTRACT-003 contract runtime-check insertion` remains Stage1-owned and should
not be pulled into Stage0.
