# 3324 - SOURCE-SELFHOST-EXPLICIT-AUTHORITY-REGISTRY-BASIS-001

## Token

```text
SOURCE-SELFHOST-EXPLICIT-AUTHORITY-REGISTRY-BASIS-001
```

## Purpose

Register the consultation-approved proof axis that can move the Source
Selfhost wider route-selection design stop without using manual family
selection or historical route membership.

This card selects **C first** from the consultation result. It authorizes a
registry-gated `HardAuthoritySeamProofAxis` as selector input only. It does
not select a concrete family, does not select a hard authority pilot, and does
not claim Source Selfhost progress.

## Output Contract

```text
rust-lifecycle-source-selfhost-explicit-authority-registry-basis-v0
```

## Registered Axis

```text
authority_source_kind:
  HardAuthoritySeamProofAxis

proof_type:
  RustOracleParityWithAotExeGuard

allowed_selection_use:
  HardAuthorityCandidateSelectorInputOnly

reentry_condition:
  ConsultationGatedWiderRouteSelection
```

## Decision

```text
decision:
  RegisterExplicitAuthorityProofAxis

reason_token:
  ConsultationApprovedHardAuthoritySeamProofAxis

selected_next_card:
  MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-explicit-authority-registry-basis-v0.json

guard:
  tools/checks/
    rust_lifecycle_source_selfhost_explicit_authority_registry_basis_guard.sh
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
manual_family_selection = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
```
