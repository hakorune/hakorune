---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-LITERAL-INTEGER-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-LITERAL-INTEGER-DERIVED-HAKO-ARTIFACT-001

## Summary

`MirBuilder::build_literal(LiteralValue::Integer)` now has a bounded
DerivedShadow Hako artifact for the prepared-state minimal path.

The artifact materializes only:

```text
allocate ValueId through prepared-state allocation policy
emit ConstValue::Integer shell
publish MirType::Integer shell
return the allocated ValueId
```

It uses the existing prepared-state allocation-policy contract as a dependency
and keeps return emission / finalize composition out of scope.

## Authority

Semantic source:

```text
MirBuilderLiteralIntegerLoweringPlanV1
  -> LiteralIntegerLowering DerivedShadow artifact
```

Generated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json
```

The artifact is an executable materialization of the existing source-derived
literal integer lowering plan. It is not a general expression-lowering
artifact and does not select mainline execution.

## Acceptance

```text
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green
ConstValue::Integer shell emitted = green
MirType::Integer publication shell emitted = green
reserved candidate consumed by allocator dependency = green
```

The semantic closure report now consumes this artifact and advances the first
executable materialization gap:

```text
first_executable_materialization_gap = finalize_module.composition
next_slice = MIRBUILDER-BOUNDED-FINALIZE-DERIVED-HAKO-ARTIFACT-001
```

## Artifacts

- `tools/rust_lifecycle/mirbuilder_literal_integer_artifacts.py`
- `tools/checks/rust_lifecycle_mirbuilder_literal_integer_derived_artifact_guard.sh`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-literal-integer-derived-hako-oracle-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-literal-integer-derived-hako-recipe-v0.json`
- `docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-literal-integer-derived-hako-verifier-result-v0.json`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.hako`
- `lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_literal_integer_lowering.artifact.json`

## Non-Claims

```text
typed_integer_literal = 0
float_literal = 0
bool_literal = 0
string_literal = 0
null_literal = 0
void_literal = 0
full_expression_lowering = 0
return_emission = 0
finalize_module = 0
mainline_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_literal_integer_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-literal-integer-lowering --check
bash tools/checks/rust_lifecycle_mirbuilder_literal_integer_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_semantic_closure_report_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
cargo check --release
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```
