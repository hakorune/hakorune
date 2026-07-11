# 2828 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-148

Status: Completed
Date: 2026-07-05

## Decision

Select `agg_local_scalarization_kind_formatter` as the one-hundred-
forty-fifth narrow Rust-oracle parity pilot owner.

## Evidence

```text
selected_owner:
  agg_local_scalarization_kind_formatter
source_surface:
  src/mir/agg_local_scalarization.rs:27
  Display::fmt
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Agg-local scalarization route collection remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-AGG-LOCAL-SCALARIZATION-KIND-FORMATTER-RUST-ORACLE-FIXTURE-001`
