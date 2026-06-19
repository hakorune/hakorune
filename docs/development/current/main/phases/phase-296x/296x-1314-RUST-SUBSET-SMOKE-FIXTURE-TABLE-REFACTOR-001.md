# 296x-1314 RUST-SUBSET-SMOKE-FIXTURE-TABLE-REFACTOR-001

Status: closed
Date: 2026-06-19

## Purpose

Refactor the rust-subset-to-hako smoke script from repeated per-fixture command
blocks into table-driven fixture runners.

This row is behavior-preserving. It does not add RustSubset schema support,
converter semantics, input-route ownership, or compiler acceptance shapes.

## Implementation

Updated:

```text
apps/rust-subset-to-hako/smoke.sh
```

The script now has small runners for:

```text
emit_mir_json
run_exe_diff
run_adapter_json_diff
run_adapter_to_python_hako_diff
run_simple_semantic_parity
```

Fixture coverage is expressed through tables:

```text
ADAPTER_FIXTURES
CONVERTER_FIXTURES
```

This keeps future app-front fixture additions to one table row instead of
copying MIR emit, EXE emit, output filtering, and diff blocks.

## Evidence

```bash
python3 apps/rust-subset-to-hako/selftest.py
cargo check -q --lib
bash apps/rust-subset-to-hako/smoke.sh
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

Observed result:

```text
summary=ok
```

## Boundary

```text
behavior_changed=0
schema_node_added=0
converter_core_changed=0
input_route_changed=0
vm_product_route=retired
```

## Next

Continue app-front next-task selection. Design-heavy language semantics such as
Rust `match` or `for` remain out of scope until an explicit design row accepts
them.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
