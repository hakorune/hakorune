---
Status: Selected
Date: 2026-06-26
Card: MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001
---

# MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001

## Summary

The current semantic-closure report no longer points at
`MirModule::new`. `MirModuleMinimalShell`, `MirFunctionConstructorShell`,
`CurrentModuleTake`, and `ReturnEmission` already have executable
DerivedShadow artifacts. The first remaining source-order materialization gap
is:

```text
edge_id = finalize_module.region_stack_pop
callsite = MirBuilder::finalize_module -> region::observer::pop_function_region
required_capability = FunctionRegionStackPop
```

This card selects the next executable owner as the focused prepared-state Hako
artifact for `FunctionRegionStackPop`.

## Authority

Semantic source:

```text
src/mir/region/observer.rs::pop_function_region
MirBuilderFunctionRegionStackPopPlanV1
```

Source behavior:

```text
trace disabled:
  NoOp

trace enabled:
  pop one current_region_stack entry
  discard the returned Option
```

Projection boundary:

```text
FunctionRegionStackPopExecutionProjectionV1
  input = prepared trace flag + current_region_stack
  stack transport = ArrayBox of RegionIdAsI64
  lowering = SequencePopOption
  result = Unit / discarded pop result
```

The projection does not claim host environment lookup or full
`is_region_trace_on()` execution. The trace flag is an explicit prepared input.

## Acceptance

```text
generated Hako artifact checked in
artifact manifest checked in
behavior recipe checked in
verifier result checked in
VerifiedFamilyArtifactContractV1 present
deterministic regeneration = green
generated Hako parse/MIR = green
generated Hako EXE/AOT = green

trace_disabled_nonempty:
  enabled = 0
  before = [10, 20]
  after = [10, 20]

trace_enabled_nonempty:
  enabled = 1
  before = [10, 20]
  after = [10]
  discarded = 20

trace_enabled_empty:
  enabled = 1
  before = []
  after = []
  error = 0

may_mutate = current_region_stack only
must_not_mutate = current_module / current_function / slot registry / module metadata
```

After the artifact lands, regenerate the semantic-closure report and let the
analyzer derive the next materialization gap. Do not prewrite the next
`SlotRegistryRelease` result into guards or task-order.

## New Files

```text
tools/rust_lifecycle/mirbuilder_function_region_stack_pop_artifacts.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-execution-projection-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-oracle-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-recipe-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-function-region-stack-pop-derived-hako-verifier-result-v0.json
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_function_region_stack_pop.artifact.json
tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_derived_artifact_guard.sh
```

## Updates

```text
tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py
minimal-mirbuilder-execution-path-semantic-closure-report-v0.json
CURRENT_STATE.toml
mirbuilder-rust-to-hako-converter-task-order-ssot.md
```

## Non-Claims

```text
observe_function_region push = 0
observe_control_form = 0
Region object construction = 0
NEXT_REGION_ID atomic allocation = 0
region log emission = 0
host environment lookup = 0
full is_region_trace_on implementation = 0
full MetadataContext conversion = 0
slot registry release = 0
module metadata publication = 0
semantic refresh = 0
all-functions PHI materialization = 0
full finalize_module = 0
full MirBuilder object transport = 0
full build_module generated Hako execution = 0
full minimal-path mainline selection = 0
source selfhost claim = 0
new backend route = 0
new ABI = 0
new canonical MIR instruction = 0
runtime fallback = 0
coverage percentage as proof = 0
bundle size as proof = 0
```

## Gates

```text
python3 -m py_compile tools/rust_lifecycle/mirbuilder_function_region_stack_pop_artifacts.py
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --family mirbuilder-function-region-stack-pop --check
bash tools/checks/rust_lifecycle_mirbuilder_function_region_stack_pop_derived_artifact_guard.sh
python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py --check
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --all --check
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check --release
git diff --check
```
