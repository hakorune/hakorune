---
Status: Active implementation task
Date: 2026-07-12
Owner: 3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001
Decision: accepted first semantic activation slice
---

# 3511 - LANGV1-FAILURE-OUTCOME-UNIT-NORESULT-HAKO-MEM-FREE-001

## Objective

Activate exactly one semantic no-result boundary:

```text
hako_mem_free.success -> Normal(Unit) -> NoPayload
```

The native C ABI is void. `void_sentinel_i64_zero` may remain only as a
backend-private discard-only encoding. It is not a semantic class, target
carrier, MIR integer result, source authority, or success proof.

## Accepted Owner Split

```text
normative source authority:
  HakoMemFreeApiContract

semantic owner:
  ExternCallOutcomeContractOwner

mechanical projection owner:
  ExternCallProjectionOwner

separate existing scalar API:
  MemCoreBox.free_i64 -> explicit Integer(0)
```

`MemCoreBox.free_i64` is not absorbed into this activation. Its explicit
integer result remains a separate API contract.

## Canonical Contract Row

```text
contract_id = extern-outcome:hako-mem-free:v1
route_id = extern.hako_mem.free
source_site = runtime_backend.extern.hako_mem_free.success
success_outcome = Unit
result_policy = NoPayload
value_use_policy = StatementOnly
abi_return_shape = CVoid
bridge_encoding = VoidSentinelI64Zero (backend-private, optional)
observability = DiscardOnly
mir_result_policy = NoResultValue
value_type_publication = PublishNothing
```

## Ordered Implementation Slices

## Progress

```text
S0_contract_row = complete
S1_route_separation = complete
S2_result_use_convergence = complete
S3_one_supported_backend = complete
S4_preflight_and_fixtures = pending
S5_activation = 0
```

S0/S1 provide the machine-readable outcome contract, expose
`CVoid`/`NoPayload` separately from the optional sentinel encoding, and stop
the route from publishing `MirType::Integer`. S2 canonicalizes an unused
temporary destination to `dst=None` and rejects a genuinely used result at the
contract boundary before effects. S3 now has one LLVM/object consumer that
emits only the native C-void call; VM/Wasm/legacy consumers remain unsupported.

### S0 — Contract row

- Add one machine-readable `ExternOutcomeSpec` for `hako_mem_free`.
- Bind it to the classified operation site and explicit API evidence.
- Keep activation disabled until S1-S4 are green.

### S1 — Route separation

- Separate semantic `NoPayload` from ABI `CVoid` and optional sentinel encoding.
- Remove scalar semantic demand for this route.
- Reject `MirType::Integer` publication for this contract.

### S2 — Result-use convergence

- Canonicalize an unused call result to `dst=None`.
- Reject assignment, return, comparison, truthiness, or any direct result use.
- Reject a valid `result_value`, scalar demand, or integer publication.
- Preserve `MemCoreBox.free_i64` unchanged as a separate scalar API.

### S3 — One supported backend

- Support exactly one MIR -> LLVM/object `hako_mem_free` consumer.
- Emit one native C-void call and no result store.
- Keep `void_sentinel_i64_zero` out of the route contract and remove the old
  zero-materializing emitter branch.
- Rust reference VM, Wasm, legacy LLVM harness, PyVM, and unknown backends
  fail before effects; no fallback is permitted.

### S4 — Preflight and fixtures

All contract, source freshness, result-use, capability, and projection
observability checks must run before argument evaluation and before the native
free call. Add positive and negative fixtures from the consultation matrix.

### S5 — Activation

Only after every guard is green, set the narrow activation flag for this one
contract. Do not activate Unit globally or change any other Void/Null site.

## Effect Order

```text
semantic_refresh
-> exact route resolve
-> ExternOutcomeSpec resolve
-> source-site freshness check
-> result-use check
-> backend capability check
-> projection observability check
-> argument evaluation
-> one native hako_mem_free call
-> Normal(Unit) with no result register publication
```

Any failed check must occur before the free call. No zero result is synthesized
and no VM fallback is attempted.

## Required Fixtures

Positive:

```text
hako_mem_free_statement_success_is_unit
hako_mem_free_null_pointer_success_is_unit
hako_mem_free_call_has_no_result_value
hako_mem_free_publishes_no_mir_type
hako_mem_free_llvm_emits_one_native_call
hako_mem_free_llvm_writes_no_result_register
mem_core_free_i64_remains_explicit_integer_api
```

Negative:

```text
hako_mem_free_assignment_rejects_before_free
hako_mem_free_return_value_use_rejects_before_free
hako_mem_free_compare_use_rejects_before_free
hako_mem_free_truthiness_use_rejects_before_free
hako_mem_free_valid_result_value_rejects
hako_mem_free_scalar_value_demand_rejects
hako_mem_free_integer_type_publication_rejects
hako_mem_free_projection_missing_or_observable_rejects
hako_mem_free_vm_wasm_and_unknown_backend_reject_before_free
```

## Stable Diagnostic Tags

```text
[failure/outcome_unit_result_use_forbidden]
[failure/outcome_unit_result_value_present]
[failure/outcome_unit_integer_publication_forbidden]
[failure/outcome_unit_projection_source_missing]
[failure/outcome_unit_projection_source_drift]
[failure/outcome_unit_projection_observable]
[failure/outcome_unit_projection_encoding_drift]
[failure/outcome_unit_backend_unsupported]
[failure/outcome_unit_refresh_bypass]
[failure/outcome_unit_vm_fallback_forbidden]
```

Diagnostics must include `route_id`, `source_site`, `result_value` or
`use_site`, `backend`, and projection details where applicable. They are
default-off diagnostics and must follow the existing log contract.

## Acceptance

```text
native/API contract and semantic owner are explicit
canonical MIR has no result value for hako_mem_free
void_sentinel_i64_zero is discard-only and non-authoritative
direct result use rejects before the free effect
MemCoreBox.free_i64 remains a separate Integer(0) API
exactly one LLVM/object consumer is supported
VM/Wasm/other consumers fail before effects without fallback
positive/negative fixtures and preflight guards are green
only hako_mem_free.success activation may become 1
all other Unit/absence/Err/Fault/Weak/FFI sites remain inactive
```

## Non-Claims

```text
global_Unit_runtime_carrier = 0
void_literal_runtime_migration = 0
VMValue_or_ConstValue_global_migration = 0
provider_missing_fallback_correction = 0
Weak_upgrade_Option_activation = 0
ForeignNull_adapter_activation = 0
hako_mem_alloc_Result_activation = 0
canonical_null_migration = 0
catch_behavior_change = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Commands

```bash
python3 tools/docs/failure_outcome_exhaustiveness.py --check
python3 tools/docs/failure_outcome_conflict_ledger.py --check
bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh
# S4 adds the remaining positive/negative matrix before S5 closeout.
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```

## Stop Boundary

If the route cannot prove no-result observability, if a second semantic owner
appears, or if an unsupported consumer would need fallback, stop and return to
design consultation. Do not broaden this card to provider missing-result,
Weak upgrade, ForeignNull, or global Void migration.
