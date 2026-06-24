---
Status: SSOT
Decision: accepted
Date: 2026-06-18
Scope: Rust VM / .hako VM active-development boundary after the RustSubset converter investigation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1075-VM-ACTIVE-LANE-RETIRE-001.md
  - apps/rust-subset-to-hako/probes/README.md
---

# VM Active Lane Retirement SSOT

## Decision

The VM is no longer a product-level app execution target for current compiler
construction work.

```text
rust_vm_active_product_target=0
rust_vm_semantic_reference_subset=1
hako_vm_active_product_target=0
hako_vm_small_subset_experiment=1
primary_app_validation_route=exe_aot
```

The VM remains useful for small semantic smoke tests and focused MIR reference
checks. It is not the route where new product apps, JSON-heavy converter apps,
or selfhost compiler fronts must prove full runtime behavior.

## Rationale

The RustSubset JSON -> `.hako` converter reached runtime through the VM route:

```text
filebox_read_enabled=1
json_tokenizer_probe_green=1
joinir_acceptance_blocker_cleared=1
global_mir_call_payload_normalized=1
```

The blocker moved from compiler acceptance to runtime collection semantics:

```text
mapbox_primitive_roundtrip=1
mapbox_user_box_roundtrip=0
arraybox_user_box_roundtrip=0
json_tree_parse_result=null
```

Keeping the VM as a product-level route would require continuing to grow:

```text
rust_vm_collection_user_box_semantics
hako_vm_collection_user_box_semantics
json_native_tree_runtime_surface
product_app_runtime_parity
```

That would split effort across too many execution engines while the current
goal is compiler construction and selfhost progress.

## Target Execution Ownership

```text
EXE/AOT:
  primary product/app validation route
  primary selfhost app-front validation route
  primary performance route

Rust VM:
  small semantic reference subset
  focused MIR smoke tests
  payload normalization reference tests
  no broad runtime parity expansion

.hako VM:
  optional small-subset experiment
  not a blocker for compiler construction
```

## Allowed VM Work

VM work is allowed only when it is narrow and directly protects a semantic
reference contract:

```text
allowed=payload_normalization_unit_tests
allowed=small_mir_semantic_smoke
allowed=fail_fast_diagnostic_for_unsupported_vm_surface
allowed=regression_test_for_already_supported_subset
```

## Disallowed VM Work

```text
disallowed=product_json_app_runtime_parity
disallowed=full_user_box_collection_semantics_for_app_execution
disallowed=feature_work_required_only_by_vm_product_route
disallowed=simultaneous_rust_vm_and_hako_vm_product_development
disallowed=silent_vm_fallback_to_hide_aot_gap
```

## Interactive Interpreter / REPL Parking

Interactive interpreter and Python-like REPL product work is parked.

```text
repl_active_product_target=0
interactive_interpreter_active_product_target=0
rust_mir_interpreter_repl_extension_allowed=0
hako_mir_interpreter_required_before_python_like_repl=1
```

Existing REPL and `MirInterpreter` documentation is historical or
semantic-reference material unless a later accepted current-state card
explicitly reopens the lane.

Do not expand REPL, `MirInterpreter`, or VM-interpreter runtime behavior for
product/app execution until the Rust VM / interpreter execution owner is
migrated to `.hako`, or until this lane is explicitly reopened for a bounded
reference-only slice.

Allowed work remains limited to narrow semantic-reference smoke tests,
regression tests for already-supported behavior, and fail-fast diagnostics for
unsupported VM surfaces.

## Converter Implication

The RustSubset converter should not be blocked on the Rust VM route.

```text
rust_subset_converter_primary_route=exe_aot
rust_subset_converter_vm_route=diagnostic_only
json_native_vm_collection_gap_blocks_vm_only=1
```

## Stop Lines

```text
do not fix broad VM runtime parity unless a semantic reference smoke requires it
do not require JSON/native app converter to pass on Rust VM
do not treat VM failure as compiler-construction failure when EXE/AOT is the selected route
do not add VM-specific workarounds to .hako source
do not use silent fallback to mask unsupported VM runtime surfaces
```
