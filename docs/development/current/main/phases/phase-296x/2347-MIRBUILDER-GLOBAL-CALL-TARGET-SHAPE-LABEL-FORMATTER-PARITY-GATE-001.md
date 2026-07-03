# 2347 MIRBUILDER-GLOBAL-CALL-TARGET-SHAPE-LABEL-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add the GlobalCall target shape label formatter Rust-oracle `.hako` EXE parity
gate.

## Acceptance

```bash
bash tools/checks/rust_lifecycle_mirbuilder_global_call_target_shape_label_formatter_parity_gate.sh
```

Expected:

```text
parity_rows=4
parity_status=green
source_selfhost_claim=0
```

## Next

`MIRBUILDER-GLOBAL-CALL-TARGET-SHAPE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001`
