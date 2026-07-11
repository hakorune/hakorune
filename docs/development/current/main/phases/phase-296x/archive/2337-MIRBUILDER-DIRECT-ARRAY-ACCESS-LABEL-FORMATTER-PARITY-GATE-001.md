# 2337 MIRBUILDER-DIRECT-ARRAY-ACCESS-LABEL-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add the DirectArray access label formatter Rust-oracle `.hako` EXE parity
gate.

## Acceptance

```bash
bash tools/checks/rust_lifecycle_mirbuilder_direct_array_access_label_formatter_parity_gate.sh
```

Expected:

```text
parity_rows=15
parity_status=green
source_selfhost_claim=0
```

## Next

`MIRBUILDER-DIRECT-ARRAY-ACCESS-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001`
