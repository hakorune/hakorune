# 296x-1635 MIRBUILDER-MULTI-CARRIER-EXIT-PHI-001

Status: landed

## Decision

`MULTI-CARRIER-BREAK-CONTINUE-EARLY-RETURN-PHI-001` is closed by a
MirBuilder converter direct-shape pilot with explicit exit facts.

## Scope

```text
control.multi_carrier_exit_phi
  -> typed ExplicitMultiExitPhiI64Array operation
  -> break / continue / early_return exits all present
  -> two i64 carriers
  -> generated runnable Hako artifact
  -> MIR / EXE guard
```

```text
missing exit       -> Deny(UnstructuredControlFlow)
carrier arity drift -> Deny(PhiJoinRequired)
carrier escape      -> Deny(CarrierSensitiveAlias)
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_multi_carrier_exit_phi_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_negative_converter_fixtures_guard.sh
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family multi-carrier-exit-phi --check
```
