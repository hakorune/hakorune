# 293x-283 TRANS-001 Transition Metadata Capsule

Status: complete
Date: 2026-05-14

## Scope

Parse `transition Enum.Value -> Enum.Value by method` as box-local lifecycle
relation metadata.

## Landed changes

- Added `TransitionDecl` metadata to box declarations.
- Parsed `transition ... -> ... by ...` in box member lists.
- Kept `transition` and `by` contextual rather than global keywords.
- Transported transition metadata through AST JSON and Program JSON v0.
- Left transition legality, enum lookup, method lookup, runtime lowering, and
  lifecycle verifier facts unimplemented for Stage1 rows.
- Added parser and Program JSON tests plus a dedicated guard.

## Non-goals

- No `state` keyword.
- No Stage0 transition checker.
- No method existence check.
- No enum/variant existence check.
- No MIR/runtime lowering.
- No lifecycle verifier integration.

## Guard

```bash
bash tools/checks/k2_wide_transition_metadata_capsule_guard.sh
```

## Next selected row

`USES-001 method-level uses metadata capsule`.

`TRANS-002 transition legality checker` remains Stage1-owned.
