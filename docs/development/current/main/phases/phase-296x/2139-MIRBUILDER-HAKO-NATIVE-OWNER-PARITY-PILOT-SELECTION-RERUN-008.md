# 2139 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-008

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-008
```

## Purpose

Select the ninth small hand-authored `.hako` native owner parity pilot after
the `generic_method_route_fact_token_formatter` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  closure_call_shape_classifier

selected_rust_surface:
  src/mir/ssot/closure_call.rs closure call dst/arg shape classifier

selected_next_card:
  MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
dst_present + arg_count -> ClosureCallShape
ClosureCallShape -> reject code
```

## Excluded Surface

```text
callsite canonicalization
NewClosure rewrite
backend fail-fast boundary
MIR instruction mutation
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  ClosureCallShapeClassifierSelectedAsNinthParityPilot

selected_next_card:
  MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no callsite canonicalization migration
no NewClosure rewrite migration
no backend fail-fast boundary migration
no MIR instruction mutation migration
```
