---
Status: Landed
Date: 2026-06-28
Card: MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-001
---

# MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-RECHECK-001

## Summary

Recheck the prepared-state allocation-policy kernel adoption decision after
the minimal-path composed prefix reached `Closed`.

The earlier card
`MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001` deferred adoption
because no native `.hako` source owner existed yet. This card consumes the
new composed-prefix evidence and makes a machine-checkable decision for the
narrow already-mainline family.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Input Evidence

```text
composed prefix advance:
  prefix_state = Green
  next_unconsumed_edge_classification = Closed
  same_state_handoff_observed = 1

target family:
  hakorune_mir_builder::next_value_id_prepared_state_kernel

current family state:
  DerivedMainline

route:
  selfhost_mainline = derived_hako
  rust_bootstrap = retained
  fallback_policy = Forbidden
```

## Output

```text
MirBuilderAllocationPolicyHakoAdoptionDecisionV1

decision:
  Adopt | Defer | Reject
```

If the decision is `Adopt`, the card must select or create the native `.hako`
source owner and forbid generator overwrites of that source. If the decision
is `Defer` or `Reject`, the result must include a stable reason token.

## Required Delta

At least one code-facing artifact must land:

```text
adoption decision fixture
adoption guard
native Hako adoption candidate source
generator overwrite guard for the adopted family
```

## Acceptance

```text
composed_prefix_result_consumed = 1
prefix_state = Green
next_unconsumed_edge_classification = Closed
same_state_handoff_observed = 1
target_family_is_derived_mainline = 1
target_scope_is_narrow = 1
route_selection_present = 1
rust_bootstrap_retained = 1
fallback_policy = Forbidden
exactly_one_of = Adopt | Defer | Reject
reason_token_required_if_not_adopt = 1
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
rust_source_delete = 0
full_minimal_path_mainline_selected = 0
manual_next_owner_selection = 0
```

## Non-Claims

```text
full MirBuilder object adoption = 0
all generated artifacts HakoAdopted = 0
Python converter retirement = 0
Rust bootstrap removal = 0
Source Selfhost = 0
```

## Next

Follow the decision result:

```text
Adopt:
  enforce native source authority and generator overwrite guard

Defer:
  park adoption on the named missing requirement

Reject:
  keep the family generated and record why it should remain derived
```

## Closeout

```text
output_contract=rust-lifecycle-mirbuilder-allocation-policy-hako-adoption-decision-recheck-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
composed_prefix_result_consumed=1
prefix_state=Green
next_unconsumed_edge_classification=Closed
target_family_is_derived_mainline=1
target_scope_is_narrow=1
route_selection_present=1
rust_bootstrap_retained=1
fallback_policy=Forbidden
decision=Defer
reason_token=NativeHakoSourceOwnerMissing
native_hako_source_owner_present=0
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
rust_source_delete=0
manual_next_owner_selection=0
summary=ok
```
