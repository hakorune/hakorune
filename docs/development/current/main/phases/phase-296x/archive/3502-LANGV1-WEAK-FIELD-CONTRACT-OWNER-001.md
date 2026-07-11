# 3502 - LANGV1-WEAK-FIELD-CONTRACT-OWNER-001

## Status

Complete. Design authority is the accepted decision in 3500; 3501 closes the
required WeakRef value law.

## Goal

Replace builder-local Weak field acceptance and split runtime storage with one
source-owned contract, one explicit MIR write, one declaration-indexed slot,
and one backend-neutral runtime owner.

## Structure

```text
source UserBoxFieldDecl.is_weak
  -> WeakFieldContractSpec
  -> semantic_refresh
  -> WeakFieldWriteContract
  -> WeakFieldWrite
  -> WeakFieldRuntime
  -> InstanceBox declared weak slot
```

The Rust VM is a thin semantic-reference adapter. It must not own field policy,
storage policy, fallback, routing, or optimization.

## Implementation Slices

### S1 - Source spec and declaration layout

1. Add stable box-schema fingerprint and `WeakFieldId` from ordered source
   declarations.
2. Add module-owned `WeakFieldContractSpec` rows.
3. Carry typed declared-field layout into `InstanceBox` construction.
4. Add declaration-indexed `WeakSlotState::{Empty, Occupied}` storage.
5. Keep ordinary field storage behavior unchanged.

### S2 - MIR and semantic refresh

1. Add `WeakFieldWriteSiteId` and explicit `MirInstruction::WeakFieldWrite`.
2. Add function-owned `WeakFieldWriteContract` carrier.
3. Rebuild specs/carriers only through `semantic_refresh`.
4. Canonicalize exact known Weak `FieldSet` producers.
5. Reject residual known Weak `FieldSet`, stale fingerprint, missing carrier,
   and MirType-as-proof.

### S3 - Runtime owner and dynamic convergence

1. Add backend-neutral `WeakFieldRuntime` with validate-before-publication.
2. Route explicit writes and dynamic InstanceBox `setField` through it.
3. Route reads through declaration layout and the same weak slot.
4. Remove declared Weak fields from `obj_fields`, `fields_ng`, and
   `box_fields` authority.
5. Remove separate Weak write `Barrier`; any required hook is infallible and
   internal to the owner operation.
6. Reject FastMem Weak stores until a dedicated consumer exists.

### S4 - Transport and backend gate

1. Export specs, carriers, and `weak_field_write` through refreshed MIR JSON.
2. Add `weak_field_runtime_guard_v1` central capability.
3. Allow only the Rust VM semantic-reference adapter.
4. Reject PyVM, LLVM, AOT, and Wasm before effects.
5. Forbid `WeakFieldWrite -> FieldSet + Barrier` legacy projection.

### S5 - Closeout

1. Add read/clear/dead-target fixtures.
2. Add accepted/rejected write and no-mutation fixtures.
3. Add alias/parameter/PHI and dynamic-dispatch fixtures.
4. Add storage-convergence, refresh, JSON, and backend fixtures.
5. Run focused tests plus the full grammar gate required by 3500.
6. Change activation/status rows only after all boundaries are green.

## Stable Tags

```text
[type/weak_field_contract_duplicate_spec]
[type/weak_field_contract_carrier_missing]
[type/weak_field_contract_stale_carrier]
[type/weak_field_contract_source_drift]
[type/weak_field_contract_refresh_bypass]
[type/weak_field_contract_residual_fieldset]
[type/weak_field_contract_violation]
[type/weak_field_contract_base_not_instance]
[type/weak_field_contract_runtime_layout_missing]
[type/weak_field_contract_check_after_publication_forbidden]
[type/weak_field_contract_mirtype_as_proof_forbidden]
[type/weak_field_contract_storage_split]
[type/weak_field_contract_runtime_bypass]
[type/weak_field_contract_fastmem_unsupported]
[type/weak_field_contract_legacy_projection_forbidden]
[type/weak_field_contract_backend_unsupported]
[type/weak_field_contract_backend_silent_drop]
```

## Non-Claims

```text
ownership_kernel_activation = 0
strong_field_cascade_policy_change = 0
ordinary_strong_field_semantics_change = 0
new_absence_state = 0
empty_weakref_language_value = 0
automatic_box_to_weak_conversion = 0
typed_weak_target_contract = 0
runtime_check_elision = 0
fastmem_weak_field_lowering = 0
llvm_weak_field_contract_support = 0
backend_weak_field_projection = 0
runtime_backend_fallback = 0
grammar_profile_change = 0
broad_static_type_checker = 0
selfhost_claim = 0
```

## Next Stop

After closeout, stop for:

```text
3503 - LANGV1-WEAK-FIELD-PRODUCT-BACKEND-SELECTION-DESIGN-STOP-001
```

## Closeout Evidence

```text
source owner = UserBoxFieldDecl.is_weak -> WeakFieldContractSpec
write owner = WeakFieldWrite + WeakFieldWriteContract
runtime owner = WeakFieldRuntime -> declaration-indexed WeakSlotState
ordinary storage bypass = rejected
dynamic alias write = runtime layout enforced
MIR JSON = refreshed spec + carrier + explicit operation
backend = Rust reference VM only; all other targets reject before effects
runtime check elision = 0
legacy FieldSet + Barrier projection = 0
focused weak-field tests = 13/13 green
cargo check --features vm-reference = green
current_state_pointer_guard = green
LANGV1_GRAMMAR_FULL = 38/38 + 15/15 green
```
