# 296x-1633 MIRBUILDER-SINGLE-SCALAR-LOOP-CARRIER-001

Status: landed

## Decision

`SINGLE-SCALAR-LOOP-CARRIER-001` is closed by a MirBuilder converter
direct-shape pilot for exactly one local `i64` loop carrier.

## Scope

```text
control.single_scalar_loop_carrier
  -> typed StructuredLoop operation
  -> exactly one i64 carrier
  -> generated runnable Hako artifact
  -> MIR / EXE guard
```

```text
PHI required   -> Deny(PhiJoinRequired)
carrier escape -> Deny(CarrierSensitiveAlias)
multi-carrier  -> not claimed
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_single_scalar_loop_carrier_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_negative_converter_fixtures_guard.sh
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family single-scalar-loop-carrier --check
```

## Next

```text
CANONICAL-EXPLICIT-PHI-001
```
