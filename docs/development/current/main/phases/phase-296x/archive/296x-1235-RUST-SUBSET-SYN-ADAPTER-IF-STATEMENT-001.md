---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Add RustSubset If statement support through schema, adapter, converters, and EXE/AOT fixture parity.
Related:
  - apps/rust-subset-to-hako/schema/RustSubset-v0.md
  - apps/rust-subset-to-hako/tools/syn_adapter
  - apps/rust-subset-to-hako/convert.py
  - apps/rust-subset-to-hako/converter_core.hako
  - apps/rust-subset-to-hako/convert_if_fixture.hako
  - apps/rust-subset-to-hako/examples/if_input.rs
  - apps/rust-subset-to-hako/examples/if_subset.json
  - apps/rust-subset-to-hako/examples/if_expected.hako
---

# RUST-SUBSET-SYN-ADAPTER-IF-STATEMENT-001

## Decision

Select `If` statement support as the next RustSubset source shape after
struct/impl/function/let/return/tail-expression/unsupported handoff.

This shape is useful for selfhost-style code and small enough to keep the app
front readable.

## JSON Shape

```json
{
  "kind": "If",
  "cond": {"kind": "Binary", "op": "==", "left": {"kind": "Name", "name": "x"}, "right": {"kind": "Literal", "type": "i64", "value": 0}},
  "then": [{"kind": "Return", "value": {"kind": "Literal", "type": "i64", "value": 1}}],
  "else": [{"kind": "Return", "value": {"kind": "Literal", "type": "i64", "value": 2}}]
}
```

## Result

```text
schema_if_statement_defined=1
syn_adapter_if_statement_lowering=1
python_reference_if_emit=1
hako_converter_if_emit=1
if_fixture_exe_aot_parity=ok
```

Fixture:

```text
apps/rust-subset-to-hako/examples/if_input.rs
apps/rust-subset-to-hako/examples/if_subset.json
apps/rust-subset-to-hako/examples/if_expected.hako
apps/rust-subset-to-hako/convert_if_fixture.hako
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
do not add full Rust control-flow semantics in this row
do not support else-if as if without an explicit later shape
do not move Rust parsing into .hako
do not change converter_core.hako input ownership
do not re-enable VM product route
```

## Contract

```text
output_contract=rust-subset-syn-adapter-if-statement-v0

if_statement_schema=ok
if_statement_syn_adapter=ok
if_statement_python_reference=ok
if_statement_hako_converter=ok
if_fixture_exe_aot_parity=ok
converter_core_input_route_changed=0

summary=ok
```
