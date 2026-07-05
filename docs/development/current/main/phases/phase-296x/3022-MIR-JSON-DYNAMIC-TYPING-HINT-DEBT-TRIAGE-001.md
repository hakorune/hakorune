# 3022 - MIR-JSON-DYNAMIC-TYPING-HINT-DEBT-TRIAGE-001

Status: parked

## Scope

Keep the dynamic typing hint debt discovered around 2996/2997 visible without
turning ProgramJSON migration back into guard-only work.

This card is a triage queue, not an implementation card. Open one focused
implementation card only when a ProgramJSON capability gate hits the route, or
when 3020/3021 are closed and the next ProgramJSON capability would depend on
the same value-type publication path.

## Queue

```text
MIR-JSON-BINOP-DST-TYPE-COVERAGE-001
  Tier-2. Float Add/Sub/Mul/Div dst_type hints and AOT/Python execution
  contract. Prevent Float Sub/Mul/Div from falling into integer execution.

STRING-RELATIONAL-COMPARE-POLICY-001
  Tier-2. Design decision first: string Lt/Gt/Le/Ge must be fail-fast or have
  defined lexical semantics. Do not silently allow pointer comparison.

MIR-JSON-PHI-TYPE-ROUNDTRIP-001
  Tier-3. Loader must consume PHI dst_type and/or metadata.value_types before
  dynamic values through PHI can be trusted.

MIR-CALL-DST-TYPE-PUBLICATION-001
  Tier-3. Unified mir_call may need instruction-level dst_type when metadata is
  unambiguous.

USER-BOX-METHOD-SINGLE-OBSERVATION-PROTECTION-EXTENSION-001
  Tier-3. Extend the 2999 helper-param publication policy to receiver-call
  helper inputs if an active gate reaches those paths.
```

## Stop Conditions

- Do not insert this card ahead of 3020 unless the 3020 parity gate fails on one
  of these exact routes.
- Do not add a guard-only card when the active failure is a missing `.hako`
  ProgramJSON traversal implementation.
- Do not define string relational ordering without a reference-spec decision.

## Non-Claims

- no `.hako` syntax/API change;
- no ProgramJSON traversal capability;
- no backend lowering, ABI, route selection, or Source Selfhost claim.
