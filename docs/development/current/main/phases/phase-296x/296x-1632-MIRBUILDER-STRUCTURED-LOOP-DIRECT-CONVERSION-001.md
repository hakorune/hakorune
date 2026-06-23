# 296x-1632 MIRBUILDER-STRUCTURED-LOOP-DIRECT-CONVERSION-001

Status: landed

## Decision

`STRUCTURED-LOOP-WITHOUT-CARRIED-STATE-001` is closed by the MirBuilder
converter direct-shape pilot. It is not the app-front RustSubset while support
and not the compiler-core `loop_simple_while` route.

## Scope

```text
control.structured_loop_without_carried_state
  -> typed StructuredLoop operation
  -> generated runnable Hako artifact
  -> MIR / EXE guard
```

```text
break / continue / early return -> Deny(UnstructuredControlFlow)
loop-carried semantic state      -> Deny(LoopCarriedStateRequired)
PHI requirement                  -> Deny(PhiJoinRequired)
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_structured_loop_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_negative_converter_fixtures_guard.sh
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family structured-loop-without-carried-state --check
```

```text
full_mirbuilder_crate_claim=0
single_scalar_loop_carrier_claim=0
phi_claim=0
runtime_fallback=0
```
