# 3499 - LANGV1-TYPED-ARRAY-EXACT-NUMERIC-ELEMENT-CONTRACT-001

## Status

Active substantive implementation card. Decision: accepted by 3498.

## Objective

Activate one invariant exact-numeric element contract for canonical explicit
`Array<T>` annotations while preserving unannotated Arrays as `AnyDefault`.
Activation remains zero through S1-S4 and changes only after all source sites,
runtime aliases, carriers, consumers, backend vetoes, and fixtures close in S5.

```text
source Array<T> annotation
  -> ArrayElementContractSpec
  -> ArrayStateContractClaim / boundary adapter
  -> semantic_refresh TypedArrayElementContract
  -> TypedArrayElementContractRuntime
  -> one-lock claim/check/mutate
```

## Closed First Subset

```text
accepted element specs:
  i8 i16 i32 i64 u8 u16 u32

rejected/deferred:
  u64 isize usize bool f64 String Box Record Array<T> aliases
  new Array<T>() / new ArrayBox<T>() / T[]
```

Constructor generic syntax is not a second source owner. `u16[]` remains the
Static Const Table spelling. Homogeneous inference and `MirType::Array` remain
representation evidence only.

## Source Boundaries

All explicit `Array<T>` boundaries have the same contract meaning:

```text
LocalInit / LocalReassign
ParameterEntry
ReturnExit
BoxFieldWrite
RecordConstruct / RecordWithUpdate
```

Existing boundary owners retain evaluate/publication timing and delegate the
state claim to `TypedArrayElementContractOwner`. Source names are diagnostics,
never identity.

## State Law

`ArrayStateIdentity` is the runtime contract attachment. Refactor the current
cell to one lock:

```text
ArrayStateCell { identity, state: RwLock<ArrayStatePayload> }
ArrayStatePayload { storage, element_contract }
```

Claiming an uncontracted populated state audits existing elements in index
order and attaches only after all pass. A mismatch leaves storage and contract
unchanged. Different contracts on one state always reject, even when current
values fit both ranges.

Copy/call/field/return/PHI/loop/share preserve the selected state contract.
Deep clone and slice create fresh identity with the same spec. Promotion,
clear, sort, reverse, remove, and pop preserve the attached contract.

## Carrier Vocabulary

```text
ExactArrayElementType = I8 | I16 | I32 | I64 | U8 | U16 | U32

ArrayElementContractSpec { element }

TypedArrayContractBoundary =
  LocalInit | LocalReassign | ParameterEntry | ReturnExit |
  BoxFieldWrite | RecordConstruct | RecordWithUpdate

TypedArrayElementContract {
  contract_id
  boundary
  source_identity
  array_value
  state_term
  element_spec
  disposition = RuntimeCheckedContract
  runtime_check_required = true
  proof_elision_allowed = false
  backend_capability_required = typed_array_exact_numeric_state_guard_v1
}

ArrayStateContractClaim { contract_id, array }
```

Instruction-local sites use the claim operation. Parameter/return hooks may
call the same carrier/runtime owner without synthetic CFG insertion.
`ArrayElementWriteWitness` remains mutation evidence, not contract authority.

## Evaluation And Mutation Order

Typed literals attach the empty-state contract before evaluating elements,
then evaluate/check/append each element in source order and publish only after
completion. Existing dynamic Arrays evaluate once, audit/claim atomically,
then publish the boundary.

```text
Push/LiteralAppend:
  receiver -> value -> lock -> contract check -> mutate -> Void publication

Set/Insert:
  receiver -> index -> value -> lock -> receiver/index/bounds validation
  -> element check -> mutate -> result publication
```

Current index/bounds error precedence and `set(index == len)` behavior remain.
A failed element check never performs that mutation. RMW helpers check the
computed new value before commit.

## Runtime And Bypass Boundary

`TypedArrayElementContractRuntime` is the only claim/write guard. Extract the
existing exact-numeric value/range checker to a backend-neutral subordinate;
do not duplicate range policy.

Dynamic Array method aliases must hit the same state guard. Raw append/store/
insert and specialized RMW helpers become private checked mutation primitives
or require an unforgeable permit. Contracted storage cannot be mutated through
a legacy helper bypass.

Runtime-check elision is forbidden. An O(1) same-spec state claim is runtime
observation, not verifier proof elision.

## Backend And JSON

Rust MIR interpreter is the only first-slice consumer. MIR JSON transports
refreshed carriers without runtime identity/pointers. PyVM, ny-llvmc,
LLVM/AOT/Wasm reject before effects.

Modules with typed Array carriers may not use the 3497 validated legacy Call
projection because that projection cannot preserve the state guard.

## Stable Tags

```text
type/typed_array_contract_unsupported_spelling
type/typed_array_contract_unsupported_source_site
type/typed_array_contract_unsupported_element
type/typed_array_contract_uninitialized_forbidden
type/typed_array_contract_non_array_value
type/typed_array_contract_carrier_missing
type/typed_array_contract_stale_carrier
type/typed_array_contract_source_drift
type/typed_array_contract_refresh_bypass
type/typed_array_contract_representation_as_proof
type/typed_array_contract_state_conflict
type/typed_array_contract_existing_element_mismatch
type/typed_array_contract_element_runtime_mismatch
type/typed_array_contract_runtime_bypass
type/typed_array_contract_legacy_projection_forbidden
type/typed_array_contract_backend_unsupported
```

## Refactor Series

### S1 - Source Law And Spec

1. Centralize `Array<T>` annotation parsing into one closed spec owner.
2. Correct `docs/reference/language/EBNF.md`: unannotated literals are
   canonical `AnyDefault`; explicit typed context creates the contract.
3. Correct `guarantee_matrix.rs` and the mutable status ledger to
   `MetadataOnlyNonGuarantee` / `Transitional` until closeout.
4. Reject unsupported element/spelling cases without runtime activation.

### S2 - Carrier And Refresh

1. Add source spec, claim vocabulary, typed carrier, and boundary identity.
2. Add adapters for all seven boundary kinds.
3. Extend state-term collection to claims and parameter/return values.
4. Rebuild/validate carrier, conflict, freshness, and drift in
   `semantic_refresh`; add MIR JSON transport.

### S3 - Runtime State

1. Move storage and optional contract under one state lock.
2. Implement monotonic atomic claim/adoption and existing-element audit.
3. Extract/reuse the exact-numeric checker.
4. Prove share/clone/slice/promotion contract laws.

### S4 - Checked Mutation

1. Guard LiteralAppend/Push/Set/Insert and compound final stores.
2. Route dynamic Array dispatch and specialized helpers through the guard.
3. Close raw mutation bypasses structurally.
4. Wire every source boundary without partial activation.

### S5 - Activation Closeout

1. Enable one VM consumer and central capability gate.
2. Veto legacy projection for typed modules.
3. Add source/spec, claim/identity, mutation/order, freshness/bypass/backend
   positive and negative fixtures.
4. Run normal/FULL grammar and focused contract/runtime/backend gates.
5. Update matrix/status ledger and only then activate the first slice.

## Acceptance

```text
typed_array_contract_owner_count = 1
typed_array_supported_element_spec_count = 7
typed_array_contract_attaches_to_state_identity = 1
typed_array_mutability_invariant = 1
typed_array_any_to_t_runtime_audit = 1
typed_array_all_source_boundary_count = 7
typed_array_runtime_consumer_count = 1
typed_array_raw_mutation_bypass_count = 0
typed_array_runtime_check_elision = 0
typed_array_non_vm_backend_failfast = 1
unannotated_array_element_contract = AnyDefault
changed_production_source_over_800_lines = 0
```

## Non-Claims

```text
ordinary_collection_contract_activation = 0
constructor_generic_semantics = 0
typed_array_u64_activation = 0
typed_array_pointer_sized_activation = 0
typed_array_non_numeric_activation = 0
typed_array_nested_activation = 0
typed_array_type_alias_activation = 0
typed_array_read_exact_value_publication = 0
runtime_check_elision_widened = 0
backend_array_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Conditions

Return to design before activation if one-lock state cannot preserve current
clone/share semantics, any accepted annotation boundary cannot reach the same
owner, or a non-VM backend requires silent legacy projection/fallback.
