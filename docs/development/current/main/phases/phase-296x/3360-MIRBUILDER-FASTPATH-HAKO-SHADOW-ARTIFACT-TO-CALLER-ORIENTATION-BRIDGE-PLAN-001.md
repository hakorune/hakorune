# 3360 - MIRBUILDER-FASTPATH-HAKO-SHADOW-ARTIFACT-TO-CALLER-ORIENTATION-BRIDGE-PLAN-001

## Token

```text
MIRBUILDER-FASTPATH-HAKO-SHADOW-ARTIFACT-TO-CALLER-ORIENTATION-BRIDGE-PLAN-001
```

## Purpose

Record the bridge plan for the fast-path `.hako` shadow-consume connection.

The current `SetSurfacePolicy / MapStoreI64` handoff is useful because the Rust
fast path now observes `.hako` policy evidence, but the connection currently
parses `.hako` source text through `include_str!`, string search, and
`split('|')`. That shape is transitional debt and must not be promoted into
live route authority.

## Decision

```text
selected_path:
  C_SHADOW_TYPED_ARTIFACT_FIRST_THEN_HAKO_CALLER_ORIENTATION

short_term:
  Rust fast path consumes a checked-in generated typed `.hako` artifact as
  shadow evidence. Rust route authority is retained.

long_term:
  `.hako` caller orientation becomes the target shape. Rust route logic moves
  down to host oracle / compat checker, not runtime truth.

bootstrap_policy:
  do not call hakorune from build.rs in the first step
```

## Current Debt

```text
current_source:
  src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs

debt:
  include_str!(write_set_mapstore_i64_policy_classifier.hako)
  quoted-row string search
  row.split('|')

allowed_status:
  shadow validation only

forbidden_status:
  route authority
  runtime authority
  backend lowering authority
```

## Selected Next

```text
selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-GENERATED-TYPED-ARTIFACT-SHADOW-CONSUME-SET-MAPSTORE-I64-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_fastpath_hako_shadow_artifact_to_caller_orientation_bridge_plan_guard.sh
```

## Claims

```text
fastpath_hako_shadow_artifact_to_caller_orientation_bridge_plan = 1
current_include_str_split_connection_debt_recorded = 1
selected_checked_in_generated_typed_artifact_shadow_consume = 1
selected_long_term_hako_caller_orientation = 1
build_rs_hako_compiler_invocation = 0
rust_authority_retained = 1
source_selfhost_claim = 0
```

## Non-Claims

```text
hako_runtime_route_authority = 0
rust_fastpath_rewired = 0
route_selection_authority_switch = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
live_hako_authority = 0
caller_orientation_runtime_path = 0
source_text_parsing_as_authority = 0
source_selfhost_claim = 0
```
