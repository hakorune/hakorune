# 3002 - MIR-JSON-DYNAMIC-TYPING-HINT-COVERAGE-INVENTORY-001

Status: held

## Scope

Track non-Tier-1 dynamic typing gaps discovered while reviewing 2996/2997.
This is not a blocker for the route/helper preflight queue unless a ProgramJSON
gate hits one of these paths.

## Inventory

```text
MIR-JSON-BINOP-DST-TYPE-COVERAGE-001
  Float Add/Sub/Mul/Div dst_type hints; avoid Float Add -> i64.

MIR-JSON-PHI-TYPE-ROUNDTRIP-001
  Rust MIR JSON loader must consume PHI dst_type and/or metadata.value_types.

MIR-CALL-DST-TYPE-PUBLICATION-001
  Unified mir_call may need instruction-level dst_type when metadata is
  unambiguous.

STRING-RELATIONAL-COMPARE-POLICY-001
  Design decision first: keep Lt/Gt/Le/Ge string compare unsupported or define
  lexical semantics before adding cmp_kind=string.
```

## Non-Claims

- no implementation selected here;
- no `.hako` syntax/API change;
- no backend lowering, ABI, route selection, ProgramJSON, or Source Selfhost
  claim.

