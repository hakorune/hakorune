---
Status: Landed
Date: 2026-06-28
Card: MIRBUILDER-ALLOCATION-POLICY-HAKO-NATIVE-SOURCE-OWNER-001
---

# MIRBUILDER-ALLOCATION-POLICY-HAKO-NATIVE-SOURCE-OWNER-001

## Summary

Materialize the native `.hako` source owner for the prepared-state
allocation-policy kernel. The previous adoption recheck deferred only because
no native source owner existed; this card closes that gap with a machine-
checkable native owner and generator overwrite guard.

This is intentionally narrow. It does not claim full MirBuilder source
selfhost, does not remove Rust bootstrap, and does not widen the route beyond
the prepared-state allocation-policy family.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Selected By

```text
MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-001
```

## Required Delta

At least one code-facing artifact must land:

```text
native Hako source owner file
module export wiring
machine-checkable adoption guard
fixture-backed adoption result
```

## Acceptance

```text
native_hako_source_owner_present = 1
generator_overwrite_guard = 1
decision = Adopt
target_family_is_derived_mainline = 1
target_scope_is_narrow = 1
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
manual_next_owner_selection = 0
```

## Non-Claims

```text
full MirBuilder object adoption = 0
all generated artifacts HakoAdopted = 0
Rust bootstrap removal = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-mirbuilder-allocation-policy-native-source-owner-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
native_hako_source_owner_present=1
generator_overwrite_guard=1
decision=Adopt
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
manual_next_owner_selection=0
summary=ok
```
