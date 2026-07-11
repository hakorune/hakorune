# 296x-1700 MIRBUILDER-CURRENT-MODULE-TAKE-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-CURRENT-MODULE-TAKE-001

## Purpose

Close the current-module take frontier edge for the prepared-state minimal
MirBuilder path. This slice makes `finalize_module` consumption of the
prepared `self.current_module` shell an explicit source-derived capability
provider, without implementing typed-value verification, current-function
take, full finalize, generated Hako, backend routes, ABI changes, or runtime
behavior.

## Source Authority

```text
source:
  src/mir/builder/module_lifecycle.rs::MirBuilder::prepare_module
  src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module

predecessor:
  MirBuilderReturnTypePublicationPlanV1
    non_claims.module_take = 0

module transport:
  MirModuleMinimalShellTransportPlanV1
```

The selected edge is limited to:

```text
install:
  self.current_module = Some(module)

take:
  let mut module = self.current_module.take().unwrap()
```

## Capability

```text
provider:
  MirBuilderCurrentModuleTakePlanV1

capability:
  CurrentModuleTake

result contract:
  taken_value = MirModuleMinimalShell
  source_state = self.current_module
  post_take_state = None
```

## Derived Frontier Result

After registering `CurrentModuleTake` as a `PlanOnly` provider, the frontier
analyzer advances to the next live edge:

```text
edge:
  finalize_module.verify_typed_values

callsite:
  MirBuilder::finalize_module -> verify typed values are defined

detail:
  TypedValueVerificationRequired

next slice:
  MIRBUILDER-TYPED-VALUE-VERIFICATION-001
```

## Non-Claims

```text
verify_typed_values = 0
current_function_take = 0
full_finalize_module = 0
module_metadata_publication = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
mainline_selected = 0
source_selfhost_claim = 0
```

## Acceptance

```text
python3 -m py_compile \
  tools/rust_lifecycle/mirbuilder_current_module_take.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_current_module_take_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
