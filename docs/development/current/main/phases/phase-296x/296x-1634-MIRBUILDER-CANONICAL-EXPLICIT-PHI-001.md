# 296x-1634 MIRBUILDER-CANONICAL-EXPLICIT-PHI-001

Status: landed

## Decision

`CANONICAL-EXPLICIT-PHI-001` is closed by a MirBuilder converter direct-shape
pilot for two explicit scalar predecessor values.

## Scope

```text
control.canonical_explicit_phi
  -> typed ExplicitPhiI64 operation
  -> exactly two explicit predecessors
  -> generated runnable Hako artifact
  -> MIR / EXE guard
```

```text
inferred PHI        -> Deny(PhiJoinRequired)
multi-predecessor   -> Deny(PhiJoinRequired)
non-i64 PHI value   -> not claimed
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_explicit_phi_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_negative_converter_fixtures_guard.sh
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family canonical-explicit-phi --check
```
