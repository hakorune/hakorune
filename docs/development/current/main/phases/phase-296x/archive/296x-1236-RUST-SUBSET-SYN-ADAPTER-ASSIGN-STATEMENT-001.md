---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Add RustSubset Assign statement support through schema, adapter, converters, and EXE/AOT fixture parity.
Related:
  - apps/rust-subset-to-hako/schema/RustSubset-v0.md
  - apps/rust-subset-to-hako/tools/syn_adapter
  - apps/rust-subset-to-hako/convert.py
  - apps/rust-subset-to-hako/converter_core.hako
  - apps/rust-subset-to-hako/convert_assign_fixture.hako
  - apps/rust-subset-to-hako/examples/assign_input.rs
  - apps/rust-subset-to-hako/examples/assign_subset.json
  - apps/rust-subset-to-hako/examples/assign_expected.hako
---

# RUST-SUBSET-SYN-ADAPTER-ASSIGN-STATEMENT-001

## Decision

Select assignment statement support as the next RustSubset source shape after
`If`.

The v0 shape is statement-level, not expression-level:

```json
{
  "kind": "Assign",
  "target": {"kind": "Name", "name": "value"},
  "value": {"kind": "Binary", "op": "+", "left": {"kind": "Name", "name": "value"}, "right": {"kind": "Literal", "type": "i64", "value": 1}}
}
```

## Result

```text
schema_assign_statement_defined=1
syn_adapter_assign_statement_lowering=1
python_reference_assign_emit=1
hako_converter_assign_emit=1
assign_fixture_exe_aot_parity=ok
```

Fixture:

```text
apps/rust-subset-to-hako/examples/assign_input.rs
apps/rust-subset-to-hako/examples/assign_subset.json
apps/rust-subset-to-hako/examples/assign_expected.hako
apps/rust-subset-to-hako/convert_assign_fixture.hako
```

## Reproduction

```bash
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
```

For the full app-front gate:

```bash
RUST_SUBSET_RUN_ADAPTER=1 RUST_SUBSET_RUN_REGRESSION=1 bash apps/rust-subset-to-hako/smoke.sh
```

## Stop Lines

```text
do not support compound assignment in this row
do not support complex assignment targets beyond expression emission
do not move Rust parsing into .hako
do not change converter_core.hako input ownership
do not re-enable VM product route
```

## Contract

```text
output_contract=rust-subset-syn-adapter-assign-statement-v0

assign_statement_schema=ok
assign_statement_syn_adapter=ok
assign_statement_python_reference=ok
assign_statement_hako_converter=ok
assign_fixture_exe_aot_parity=ok
converter_core_input_route_changed=0

summary=ok
```
