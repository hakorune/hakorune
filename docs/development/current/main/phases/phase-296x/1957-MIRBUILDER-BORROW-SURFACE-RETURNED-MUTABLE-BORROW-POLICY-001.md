# 1957 - MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001

## Token

```text
MIRBUILDER-BORROW-SURFACE-RETURNED-MUTABLE-BORROW-POLICY-001
```

## Purpose

Select the replacement policy for the machine-derived returned mutable borrow
cluster selected by the borrow-surface policy rerun.

Raw returned `&mut` transport remains denied. The selected replacement is a
bounded owner mutation frame over `LoopCondReturnInBodyPhiMaterializer`'
`current_bindings` map.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-borrow-surface-returned-mutable-borrow-policy-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_borrow_surface_returned_mutable_borrow_policy.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_borrow_surface_returned_mutable_borrow_policy_guard.sh
```

## Acceptance

```text
selected_cluster.borrow_kind = ReturnedMutableBorrow
selected_cluster.return_shape = mutable_ref
selected_cluster.receiver_axis = mutable_receiver
selected_cluster.source_module = loop_cond_return_in_body_phi_materializer
selected_cluster.repaired_owner_edge_confidence = FileScoped

source_surface_verified = 1
source_signature_verified = "&mut self -> &mut BTreeMap<String, ValueId>"
bounded_callsite_verified = lower_return_in_body_block
mutation_frame_consumers_verified = 1

strict_policy.raw_returned_mutable_borrow = Deny
deny_reason = ReturnedMutableBorrow
raw_mutable_alias_selected = 0
returned_mutable_borrow_allowed = 0
rust_lifetime_syntax_transport = 0

replacement_policy = BoundedWithMapOperation
bounded_mutation_frame_selected = 1
bounded_frame_owner = LoopCondReturnInBodyPhiMaterializer
bounded_field = current_bindings
alias_escape_allowed = 0
stored_borrow_allowed = 0
caller_owned_mutable_alias = 0

explicit_mutation_api_selected = 0
mut_lease_selected = 0
replace_owned_transfer_selected = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Result

```text
decision:
  SelectReplacementPolicy

reason_token:
  ReturnedMutableBorrowReplacedByBoundedWithMapOperation

selected_next_card:
  MIRBUILDER-LOOP-COND-RETURN-IN-BODY-PHI-MATERIALIZER-
  CURRENT-BINDINGS-MUTATION-FRAME-001
```

This card selects a policy only. It does not emit Hako, materialize a native
source seed, adopt a family, or claim Source Selfhost.

## Non-Claims

```text
no raw mutable alias transport
no returned &mut Hako surface
no Rust lifetime syntax transport
no MutLease
no ReplaceOwned whole-map transfer
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
