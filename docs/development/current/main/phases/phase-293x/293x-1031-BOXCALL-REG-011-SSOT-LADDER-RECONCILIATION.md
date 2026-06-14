---
Status: Landed
Date: 2026-06-14
Task: BOXCALL-REG-011
Scope: Reconcile landed BoxCallable rows with older TypeAbi BoxDomain rows and name narrow proof commands.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
  - docs/development/current/main/design/type-abi-box-domain-ssot.md
  - docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md
  - docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md
  - src/box_callable/
  - src/type_abi/
  - src/runtime/plugin_loader_v2/enabled/box_callable_registry.rs
  - tools/hako_check/boxcall_contract.py
---

# BOXCALL-REG-011: SSOT Ladder Reconciliation

## Decision

The BoxCallable / TypeAbiCatalog foundation is no longer a first-implementation
gap. The code already contains the registry model, provider adapters,
projection helpers, route-plan vocabulary, report rows, and hako_check
contract surface.

`TypeAbi BoxDomain` remains a design/report umbrella for descriptor projection,
but callable truth and route-plan generation are reconciled under
`BoxCallableRegistry`.

```text
BoxCallableRegistry:
  callable truth

PluginLoader:
  plugin callable provider / snapshot input

type_registry:
  builtin callable provider / MethodEntry input

TypeAbiCatalog / BoxDescriptor:
  read-only projection and tooling surface
```

## Reconciliation

```text
BOXCALL-REG-001..005:
  landed as code/report vocabulary.

BOXCALL-REG-006..010:
  landed as route-plan, duplicate-truth, registry-snapshot, invoke-boundary,
  and plugin-catalog projection report rows/tests.

TYPEABI-BOXDOMAIN implementation rows:
  superseded for callable truth by BOXCALL-REG rows.

TYPEABI-BOXDOMAIN SSOT:
  retained as Box Domain / Type ABI projection umbrella.
```

## Inventory

```text
boxcallable_model_present=1
boxcallable_registry_present=1
builtin_type_registry_provider_present=1
plugin_loader_provider_present=1
plugin_loader_registry_snapshot_entrypoint_present=1
boxcallable_to_typeabi_projection_present=1
boxdescriptor_aliases_present=1
route_plan_vocabulary_present=1
hako_check_boxcall_contract_present=1
```

## Proof Commands

Broad `cargo test -q --lib plugin_loader_v2` is not the proof command for this
row because that filter also runs unrelated Future/Ring0 tests. Use narrow
callable/provider/projection proofs instead.

```bash
cargo test -q --lib box_callable
cargo test -q --lib type_abi
cargo test -q --lib export_box_callable_contracts_from_specs
cargo test -q --lib empty_loader_snapshot_is_empty_registry
cargo test -q --lib seeds_plugin_method_export_as_plugin_method_target
cargo test -q --lib seeds_plugin_lifecycle_export_as_birth_and_fini_targets
cargo test -q --lib plugin_callable_exports_project_to_catalog_through_registry
cargo test -q --lib plugin_snapshot_registry_projects_to_empty_catalog
cargo test -q --lib descriptor_aliases_cover_box_callable_projection
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
```

## Acceptance

```text
boxcallable_landed_rows_reconciled=1
typeabi_boxdomain_superseded_rows_marked=1
boxcallable_proof_commands_named=1
typeabi_catalog_execution_route_count=0
typeabi_pack_execution_route_count=0
plugin_loader_to_typeabi_direct_truth_count=0
type_abi_catalog_as_plugin_route_truth_count=0
id_space_mixed_count=0
summary=ok
```

## Stop Line

```text
do not add registry cache without measurement
do not route execution through TypeAbiCatalog or TypeAbiPack
do not expose PluginLoader internals as a broad public API
do not use broad plugin_loader_v2 test filters as this row's proof command
```
