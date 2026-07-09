# 3422 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001
```

## Purpose

Materialize a StringScalarI64Routes caller-orientation contract as a
hand-authored `.hako` source and a checked-in generated typed Rust artifact.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Selection Authority

The selected surface is the one remaining homogeneous read-only candidate after
MapLoad:

```text
selection rule:
  ReadSurfaceGeneratedArtifactMinimalityAfterMapLoadV1

surface:
  StringScalarI64Routes

proof axis:
  PriorScopedReadAuthorityContinuation
  + HomogeneousScalarI64NoPublicationReadSurface
  + RustOracleCompatFailFastRetained
```

String is selected before Collection because all three rows share one receiver
domain policy, `ScalarI64`, `NoPublication`, and read effect. Collection keeps
mixed receiver domains and `AnyLength` Box metadata, so it is out of scope.

## Ownership

```text
caller-orientation source authority:
  lang/src/compiler/lib/string_scalar_i64_caller_orientation_contract.hako

route semantics authority:
  lang/src/compiler/lib/string_search_scalar_i64_policy_classifier.hako

generated typed artifact:
  src/mir/generic_method_route_plan/generated/
    string_scalar_i64_caller_orientation_contract.rs
```

The contract must reference exactly these existing policy row IDs and must not
copy route kind, core operation, lowering tier, result shape, or proof source:

```text
string_indexof_scalar_i64_routes
string_lastindexof_scalar_i64_routes
string_contains_scalar_i64_routes
```

## Required Delta

1. Add one `.hako` metadata-only contract row for each existing String policy
   row ID.
2. Add a deterministic generator and checked-in typed Rust artifact.
3. Register the generated module without adding a live route/runtime/backend
   consumer.
4. Add a guard for exact row-set parity, source/artifact freshness, existing
   Rust oracle retention, and no live consumer registration.

## Contract Vocabulary

```text
policy_row_id = existing String policy row ID
orientation_kind = CallerOrientationContractMetadataOnly
scope = SingleSurface
runtime_consumer = Forbidden
backend_lowering_consumer = Forbidden
mutation_consumer = Forbidden
publication_consumer = Forbidden
mismatch_policy = FailFast
```

## Non-Claims

```text
caller_orientation_runtime_path = 0
caller_runtime_dispatch_authority = 0
caller_selected_route_authority = 0
route_selection_authority_switch = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
string_to_scalar_known_wide_authority = 0
collection_caller_orientation_authority = 0
delete_hako_route_decision_authority_pilot = 0
source_selfhost_claim = 0
```

## Out Of Scope

- any live consumer of the generated contract
- Collection or Write caller-orientation contracts
- route selection, MIR emission, backend lowering, mutation, publication
- ScalarKnown-wide authority and Delete revival
