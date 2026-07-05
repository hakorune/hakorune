# 2999 - HAKO-AOT-HELPER-PARAM-PUBLICATION-POLYMORPHIC-INPUT-CONTRACT-001

Status: active

## Scope

Close the confirmed helper-policy gap before returning to 2998.

2997 protected only `StringHelpers.to_i64/1 param0` from single-observation
publication. The same polymorphic-input contract must cover the known numeric
conversion helpers that accept scalar or numeric-like boxed values.

## Required Contract

Extend `HelperParamTypePublicationPolicyV1` only for known polymorphic helper
inputs:

```text
StringHelpers.to_i64/1 param0
StringHelpers.int_to_str/1 param0
BoxHelpers.value_i64/1 param0
BoxHelpers.expect_i64/2 param0
MirJsonEmitBox._expect_i64/2 param0
MirSchemaBox._expect_i64/2 param0
```

Wire the policy into user-box / receiver-style param publication where the
route target is a helper. Keep receiver publication for ordinary user boxes
unchanged.

## Acceptance

- fixture rows name every protected helper input above;
- global-call param publication and helper-like receiver publication both
  consult `HelperParamTypePublicationPolicyV1`;
- `StringHelpers.int_to_str/1 param0` is explicitly guarded;
- the 2997 AOT route value-type publication contract gate remains green.

## Forbidden

- route-family unification;
- backend lowering or ABI changes;
- new `.hako` syntax or library API;
- broad BoxHelpers semantic inference;
- ProgramJSON traversal, projector retirement, or Source Selfhost claims.

## Next

```text
MIR-ROUTE-GENERIC-METHOD-SCALAR-RETURN-VALUE-TYPE-PUBLICATION-001
```

