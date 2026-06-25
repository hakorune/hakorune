# 296x-1681: MirBuilder Derived Context Bundle v1

Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-DERIVED-CONTEXT-BUNDLE-V1-001

## Decision

Land the first MirBuilder derived context bundle v1 as a membership-only
bundle contract.

The bundle owns:

```text
family artifact membership
bundle-local member ids
member ordering
family contract references
composed smoke entry
```

The bundle does not own:

```text
family selected methods
family semantic transports
family stable denials
family behavior recipes
```

Family semantics remain owned by each family artifact contract. CoreContext is
referenced through `VerifiedFamilyArtifactContractV1`; its selected methods,
transports, and denials are not copied into the bundle manifest.

## Scope

Updated artifact:

```text
lang/generated/rust_derived/hakorune_mir_builder/ordered_map_crate_bundle.hako
lang/generated/rust_derived/hakorune_mir_builder/ordered_map_crate_bundle.artifact.json
```

The bundle manifest now declares:

```text
bundle_contract_model = membership_only_v1
bundle_members = bundle-local member ids
bundle_member_contracts = family manifest / contract references
```

The bundle smoke now exercises the accepted CoreContext ID generator surface in
addition to existing scalar counters:

```text
CoreContextApi.next_value
CoreContextApi.peek_next_value
CoreContextApi.next_block
CoreContextApi.peek_next_block
CoreContextApi.next_binding
CoreContextApi.next_temp_slot
CoreContextApi.next_debug_join
```

## Authority Chain

```text
CoreContext VerifiedHakoFamilyIR
  + stable Deny results
  + artifact identity
    -> VerifiedFamilyArtifactContractV1
      -> family artifact manifest
      -> ordered_map_crate_bundle member contract reference
      -> bundle smoke
```

The bundle is a packaging and execution-composition artifact. It is not a new
semantic authority for CoreContext or any other family.

## Acceptance

```text
bundle_contract_model=membership_only_v1
bundle member ids are distinct
CoreContext member references VerifiedFamilyArtifactContractV1
CoreContext ID generator methods are exercised in the composed smoke
generated bundle parse/MIR/EXE-AOT green
CoreContext contract drift probes green
full converter matrix green
current pointer guard green
no silent hardcode guard green
cargo check --release green
git diff --check green
```

## Validated

```text
python3 tools/rust_lifecycle/generate_mirbuilder_ordered_map_crate_bundle.py --check
bash tools/checks/rust_lifecycle_ordered_map_crate_bundle_guard.sh
python3 tools/rust_lifecycle/verify_core_context_artifact_contract.py --drift-probes
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
cargo check --release
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh
git diff --check
```

## Non-claims

```text
all-family contract migration = 0
mainline_selected = 0
source selfhost claim = 0
bundle as semantic authority = 0
family selected-method copying = 0
family transport copying = 0
family denial copying = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
```

## Next

The next selected owner is the MirBuilder allocation policy slice:

```text
MIRBUILDER-ALLOCATION-POLICY-SLICE-001
```

This is distinct from CoreContext generator scalarization. It must cover the
higher-level `MirBuilder::next_value_id` reserved/function-local allocation
policy without treating raw i64 generator state as the full allocation
semantics.
