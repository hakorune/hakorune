# 3349 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-SHADOW-CONSUME-MISMATCH-GUARD-EXPANSION-001
```

## Purpose

Expand the `SetSurfacePolicy / MapStoreI64` `.hako` fast-path shadow-consume
guard with explicit mismatch rows.

This card keeps the 3348 connection model: Rust reads the `.hako` artifact as a
shadow artifact, compares it with the Rust-owned route tuple, and remains the
route authority.

## Result

```text
hako_shadow_mismatch_guard_expanded = 1
route_kind_mismatch_rejected = 1
core_op_mismatch_rejected = 1
role_mismatch_rejected = 1
rust_authority_retained = 1
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_hako_shadow_consume_mismatch_guard_expansion_guard.sh
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-BOOL-RECIPE-COMPARE-PUBLICATION-FIRST-PARITY-ROW-SELECTION-001
```

## Non-Claims

```text
hako_runtime_route_authority = 0
hako_backend_lowering_authority = 0
route_selection_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```
