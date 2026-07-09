# 3421 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001
```

## Purpose

Materialize the 3420 MapLoad-only caller-orientation basis as a hand-authored
`.hako` contract plus a checked-in generated typed Rust artifact.

This is an implementation card. A docs-only closeout is forbidden.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Ownership

```text
caller-orientation source authority:
  lang/src/compiler/lib/map_load_scalar_i64_caller_orientation_contract.hako

route semantics authority:
  lang/src/compiler/lib/map_load_scalar_i64_policy_classifier.hako

generated typed artifact:
  src/mir/generic_method_route_plan/generated/
    mapload_scalar_i64_caller_orientation_contract.rs

generated artifact role:
  derived transport only; not semantic/edit authority
```

The caller contract references the existing MapLoad policy row identity. It
must not duplicate route kind, core operation, lowering tier, return shape,
proof family, or proof set as a second authority source.

## Required Delta

1. Add the hand-authored `.hako` caller-orientation contract.
2. Add a deterministic generator for the typed Rust artifact.
3. Add the generated module to `generated/mod.rs` without registering a live
   runtime or backend consumer.
4. Add a guard that verifies:
   - the referenced policy row exists and is MapLoad-only;
   - the generated artifact is current;
   - orientation remains metadata-only and single-surface;
   - runtime/backend/mutation/publication consumers remain absent.
5. Keep the existing MapLoad `.hako` decision versus Rust oracle fail-fast
   comparison unchanged.

## Contract Vocabulary

The new contract may carry only orientation-specific metadata and the existing
policy row reference:

```text
policy_row_id = map_load_scalar_i64_routes
orientation_kind = CallerOrientationContractMetadataOnly
scope = SingleSurface
runtime_consumer = Forbidden
backend_lowering_consumer = Forbidden
mutation_consumer = Forbidden
publication_consumer = Forbidden
mismatch_policy = FailFast
```

## Fail-Fast Boundary

```text
unknown policy row ID                         -> fail-fast
policy row is not MapLoadScalarI64Routes      -> fail-fast
generated artifact is stale                   -> fail-fast
runtime/backend consumer registration appears -> fail-fast
mutation/publication is enabled               -> fail-fast
```

No fallback, warn-only mismatch, runtime `.hako` source parsing, or backend
compensation is allowed.

## Acceptance

```text
mapload_caller_orientation_hako_contract_materialized = 1
mapload_caller_orientation_generated_typed_artifact = 1
mapload_caller_orientation_policy_row_reference_verified = 1
mapload_caller_orientation_artifact_current = 1
mapload_caller_orientation_no_live_consumer_guard = 1
mapload_hako_route_decision_authority_retained = 1
mapload_rust_oracle_compat_checker_retained = 1
mapload_mismatch_fail_fast = 1
```

## Non-Claims

```text
caller_orientation_runtime_path = 0
caller_runtime_dispatch_authority = 0
caller_selected_route_authority = 0
caller_orientation_result_consumed_by_runtime = 0
caller_orientation_result_consumed_by_backend = 0
route_selection_authority_switch = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
mapload_to_scalar_known_wide_authority = 0
delete_hako_route_decision_authority_pilot = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Acceptance Commands

```bash
python3 tools/rust_lifecycle/generate_mapload_scalar_i64_caller_orientation_contract.py \
  > /tmp/mapload_scalar_i64_caller_orientation_contract.rs
cmp /tmp/mapload_scalar_i64_caller_orientation_contract.rs \
  src/mir/generic_method_route_plan/generated/mapload_scalar_i64_caller_orientation_contract.rs
bash tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_mapload_caller_orientation_contract_artifact_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
cargo check -q
```

## Out Of Scope

- caller runtime dispatch
- route selection changes
- MIR instruction emission or ValueId allocation
- backend lowering
- mutation or publication execution
- String / Collection caller-orientation expansion
- ScalarKnown-wide authority
- Delete revival
