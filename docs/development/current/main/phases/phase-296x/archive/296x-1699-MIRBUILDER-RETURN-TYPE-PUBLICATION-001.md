# 296x-1699 MIRBUILDER-RETURN-TYPE-PUBLICATION-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-RETURN-TYPE-PUBLICATION-001

## Purpose

Close the return-type publication frontier edge for the prepared-state minimal
MirBuilder path. This slice makes `finalize_module` publication of
`type_ctx.value_types[result_value]` into `function.signature.return_type` an
explicit source-derived capability provider, without implementing module take,
full finalize, generated Hako, backend routes, ABI changes, or runtime
behavior.

## Source Authority

```text
source:
  src/mir/builder/module_lifecycle.rs::MirBuilder::finalize_module

input type owner:
  MirBuilderLiteralIntegerLoweringPlanV1
    result_contract.published_type = MirType::Integer

predecessor:
  MirBuilderReturnEmissionPlanV1
    non_claims.return_type_publication = 0
```

The selected edge is limited to:

```text
lookup:
  self.type_ctx.value_types.get(&result_value).cloned()

publish:
  function.signature.return_type = mt
```

## Capability

```text
provider:
  MirBuilderReturnTypePublicationPlanV1

capability:
  ReturnTypePublication

result contract:
  signature_return_type = MirType::Integer
  source_value_type = type_ctx.value_types[result_value]
  source_value_type_owner = LiteralIntegerLowering
```

## Derived Frontier Result

After registering `ReturnTypePublication` as a `PlanOnly` provider, the
frontier analyzer advances to the next live edge:

```text
edge:
  finalize_module.take_module

callsite:
  MirBuilder::finalize_module -> take current_module

detail:
  CurrentModuleTakeRequired

next slice:
  MIRBUILDER-CURRENT-MODULE-TAKE-001
```

## Non-Claims

```text
module_take = 0
verify_typed_values = 0
full_finalize_module = 0
phi_return_type_inference = 0
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
  tools/rust_lifecycle/mirbuilder_return_type_publication.py \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_return_type_publication_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
