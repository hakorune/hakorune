Status: Done
Date: 2026-06-18
Scope: design the MIR interpreter feature boundary before code gating
Related:
  - docs/development/current/main/phases/phase-296x/296x-1124-BUILD-VM-MIR-INTERPRETER-COMPILE-AUDIT-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001

## Decision

```text
output_contract=build-vm-mir-interpreter-feature-gate-design-v0

feature_name=vm-reference
initial_feature_default=on
initial_behavior_changed=0
vm_types_feature_gated=0
mir_interpreter_feature_gated=planned
backend_vm_alias_feature_gated=planned

default_off_selected_now=0
reason=runner_and_tests_still_have_live_vm_callers

selected_next_task=BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001
summary=ok
```

The first implementation row should introduce a `vm-reference` feature boundary
without changing default behavior. This creates the structural seam needed to
retire default-compiled interpreter code later, while avoiding a large
simultaneous runner/test migration.

## Boundary Model

```text
always_available:
  src/backend/vm_types.rs
  backend::VMValue
  backend::VMError
  backend::vm::{VMValue,VMError}

feature_gated_by_vm_reference:
  src/backend/mir_interpreter/**
  backend::MirInterpreter
  backend::NyashVm
  backend::VM
  backend::vm::VM
  runner VM execution paths
  REPL VM execution paths
  JoinIR VM bridge execution paths
  VM reference tests
```

`VMValue` / `VMError` are runtime value/error vocabulary, not the interpreter
engine. They remain always available so host APIs, JoinIR conversion helpers,
ABI helpers, and diagnostics do not inherit the interpreter feature.

## Migration Ladder

```text
step_1=BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001
  Add the feature and cfg the interpreter exports while keeping it in default.
  behavior_changed=0

step_2=BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001
  Classify runner/REPL/JoinIR/JSON-v0 VM callers as exe-aot, vm-reference, or retire.
  code_behavior_changed=0

step_3=BUILD-VM-TEST-CALLER-CLASSIFICATION-001
  Classify src/tests and tests VM callers as semantic reference, archive, or migrate.
  code_behavior_changed=0

step_4=BUILD-VM-REFERENCE-DEFAULT-OFF-PREFLIGHT-001
  Verify default build can compile with vm-reference disabled.

step_5=BUILD-VM-REFERENCE-DEFAULT-OFF-IMPLEMENTATION-001
  Remove vm-reference from default only after runner/test callers are classified.
```

## Runner Policy

```text
product_app_route=exe_aot
vm_route=reference_only
vm_backend_without_feature=fail_fast
silent_fallback_to_vm=0
silent_fallback_from_vm_to_aot=0
```

When `vm-reference` is disabled, explicit VM runner modes should fail fast with
a clear diagnostic. They must not silently switch to AOT, because that hides
backend selection mistakes.

## Feature Naming

```text
selected_feature=vm-reference
rejected_feature=vm-legacy
reason=vm-legacy_is_historical_check_cfg_silencer_and_too_broad
```

`vm-legacy` already exists as a historical check-cfg silencer. Reusing it would
mix old compatibility vocabulary with the current semantic-reference boundary.

## Stop Lines

```text
do_not_gate_vm_types=1
do_not_remove_vm_from_default_in_scaffold_row=1
do_not_rewrite_runner_routes_in_feature_scaffold=1
do_not_delete_vm_tests_before_classification=1
do_not_use_vm_legacy_as_new_boundary=1
do_not_add_aot_fallback_when_vm_feature_missing=1
```
