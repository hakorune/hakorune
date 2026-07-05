# 3002 - MIR-JSON-DYNAMIC-TYPING-HINT-COVERAGE-INVENTORY-001

Status: held

## Scope

Track non-Tier-1 dynamic typing gaps discovered while reviewing 2996/2997.
This is not a blocker for the route/helper preflight queue unless a ProgramJSON
gate hits one of these paths.

Follow-up queue:

```text
3022-MIR-JSON-DYNAMIC-TYPING-HINT-DEBT-TRIAGE-001
```

Rule: do not put this inventory ahead of a real ProgramJSON `.hako`
implementation card unless the active parity gate fails on one of these exact
routes.

## Inventory

```text
MIR-JSON-BINOP-DST-TYPE-COVERAGE-001
  C1 / Tier-2. Float Add/Sub/Mul/Div dst_type hints; avoid Float Add -> i64
  and avoid Float Sub/Mul/Div falling into integer execution. This requires
  both MIR JSON emit-side proof and the AOT/Python execution-side contract.

MIR-JSON-PHI-TYPE-ROUNDTRIP-001
  C3 / Tier-3. Rust MIR JSON loader must consume PHI dst_type and/or
  metadata.value_types so dynamic values do not lose type facts through PHI.

MIR-CALL-DST-TYPE-PUBLICATION-001
  C5 / Tier-3. Unified mir_call may need instruction-level dst_type when
  metadata is unambiguous.

STRING-RELATIONAL-COMPARE-POLICY-001
  C2 / Tier-2. Design decision first: keep Lt/Gt/Le/Ge string compare
  unsupported/fail-fast, or define lexical semantics before adding
  cmp_kind=string. Do not silently allow pointer comparison for StringBox.
```

Already covered by pre-2998 active/pending cards:

```text
B / Tier-3 user_box_method single-observation helper input protection:
  2999 HAKO-AOT-HELPER-PARAM-PUBLICATION-POLYMORPHIC-INPUT-CONTRACT-001

A#3 / Tier-3 scalar_i64_or_missing_zero:
  3000 MIR-ROUTE-GENERIC-METHOD-SCALAR-RETURN-VALUE-TYPE-PUBLICATION-001

B / Tier-3 receiver-call helper single-observation extension:
  Parked in 3022 unless an active ProgramJSON capability gate reaches a
  receiver-call helper input path.
```

## Non-Claims

- no implementation selected here;
- no `.hako` syntax/API change;
- no backend lowering, ABI, route selection, ProgramJSON, or Source Selfhost
  claim.
