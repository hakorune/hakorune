---
Status: Landed
Date: 2026-06-28
Card: HAKO-COMPILER-CANONICAL-JSON-VALUE-WRITER-GUARD-001
---

# HAKO-COMPILER-CANONICAL-JSON-VALUE-WRITER-GUARD-001

## Summary

Add an executable guard for the compiler-facing canonical JSON writer lane so
the ordinary Hako library surface under `lang/src/compiler/lib/` stays
machine-checked and ABI-free.

The guard verifies the landed library files and the placement README:

- `lang/src/compiler/lib/text_builder.hako`
- `lang/src/compiler/lib/projection_value.hako`
- `lang/src/compiler/lib/canonical_json.hako`
- `lang/src/compiler/lib/README.md`

The surface remains ordinary compiler-library Hako code. The guard does not
promote the lane to TypeBox ABI, host ABI, package ABI, or language syntax.

## Authority

Semantic source:

```text
HAKO-COMPILER-CANONICAL-JSON-VALUE-WRITER-001
  -> rust_lifecycle_hako_compiler_canonical_json_value_writer_guard.sh
  -> canonical JSON library placement evidence
```

Implemented surface:

```text
tools/checks/rust_lifecycle_hako_compiler_canonical_json_value_writer_guard.sh
lang/src/compiler/lib/canonical_json.hako
lang/src/compiler/lib/projection_value.hako
lang/src/compiler/lib/text_builder.hako
lang/src/compiler/lib/README.md
docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
```

The guard is a placement verifier, not a new semantic projector and not an
ABI surface.

## Acceptance

```text
bash tools/checks/rust_lifecycle_hako_compiler_canonical_json_value_writer_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
git diff --check = green
```

## Non-Claims

```text
new_abi = 0
host_abi_compiler_semantics = 0
source_selfhost_claim = 0
runtime_fallback = 0
PythonSemanticProjectorGrowth = 0
TypeBoxABI = 0
language_syntax_change = 0
```

## Next

```text
MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001
```
