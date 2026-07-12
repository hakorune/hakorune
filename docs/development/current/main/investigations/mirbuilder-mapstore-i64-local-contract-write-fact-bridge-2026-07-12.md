# MapStoreI64 LocalContractWrite Fact Bridge

Status: Active implementation task.
Date: 2026-07-12
Parent decision: accepted deferred-root consultation after the exact-i64
MapStoreI64 witness slice.

## Decision

Reuse the existing `ExactNumericValueFact` owner. Do not create a dynamic
integer owner.

```text
source local declaration
-> LocalSlotContract
-> checked LocalContractWrite
-> ExactNumericValueFactSource::LocalContractWrite
-> MapStoreI64KeyWitness
```

`LocalSlotContract` alone is not authority, and `LocalContractWrite` alone is
not authority. The root is their checked, identity-matched pair.

## Scope

Add exactly one source variant:

```rust
LocalContractWrite {
    contract_id: String,
    local_slot_id: LocalSlotId,
    write_kind: LocalContractWriteKind,
    src: ValueId,
    block: BasicBlockId,
    instruction_index: usize,
}
```

Attach the Fact to the post-check `dst` ValueId, never to `src`. Accept only:

```text
matching LocalSlotContract
declared_type_name == "i64"
runtime_check_required == true
proof_elision_allowed == false
matching LocalSlotId and contract_id
write_kind = Init or Reassign
fresh LocalIdentityEvidence
```

Keep call-return, provider/helper, cast, unannotated Integer, and non-i64
numeric roots deferred.

## Refresh order

```text
LocalSlotContract refresh/validation
-> LocalIdentityEvidence rebuild
-> route convergence
-> ExactNumericValueFact rebuild including LocalContractWrite
-> MapStoreI64KeyWitness projection/verifier
```

## Acceptance

Positive:

```text
mapstore_i64_key_from_i64_local_init
mapstore_i64_key_from_i64_local_reassign
mapstore_i64_key_from_dynamic_src_after_checked_local_write
mapstore_i64_key_from_local_write_through_copy
mapstore_i64_key_from_two_checked_writes_through_phi
mapstore_i64_local_root_attaches_after_local_identity_refresh
```

Negative:

```text
unannotated_local_has_no_exact_fact
integer_mirtype_local_has_no_exact_fact
u64_local_is_not_i64_key_fact
raw_store_does_not_create_local_exact_fact
local_contract_without_write_does_not_create_fact
write_without_contract_rejects_claim
wrong_local_slot_id_rejects
fact_attached_to_src_before_check_rejects
stale_contract_id_rejects
mixed_i64_dynamic_phi_has_no_hard_fact
```

The MapStore route matcher remains unchanged. The existing witness verifier is
the only consumer added by this task.

## Non-claims

```text
new_dynamic_integer_owner = 0
route_selection_authority_moved = 0
call_return_root = deferred
provider_helper_root = deferred
cast_root = deferred
plan_authority_selected = 0
boundary_authority_selected = 0
runtime_mutation_authority = 0
backend_lowering_authority = 0
source_selfhost_claim = 0
```

## Stop boundary

Stop if LocalContractWrite cannot be matched to a fresh LocalSlotContract and
LocalIdentityEvidence without adding a second source authority.
