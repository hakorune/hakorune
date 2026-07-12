# MapStoreI64 Source-Backed Key Fact Task

Status: Complete for the exact-i64 slice; next frontier is design consultation
for deferred local/call/provider roots.
Date: 2026-07-12
Active lane remains: 3457 MapStoreI64 authority inventory/design boundary.
Decision source: 3457 follow-up consultation, accepted.

## Decision

```text
current I64Value = DerivedProjection
MirType::Integer hard authority = forbidden
new dynamic-integer owner = rejected
existing ExactNumericValueFact owner = reused
MapStoreI64 exact-i64 witness projection = next candidate
```

The existing `ExactNumericValueFact` owner is the numeric provenance authority.
MapStore-specific logic is a consumer projection and verifier only. It must not
become a second dynamic-integer owner.

## Objective

Add one behavior-preserving `MapStoreI64` key witness for values whose existing
exact-numeric fact proves declared type `i64`. Keep the current dynamic route
classification unchanged and do not feed the witness into route selection.

## Authority map

```text
source declarations / exact numeric contracts
        -> ExactNumericValueFact owner
        -> MapStoreI64KeyWitness projection
        -> MapStoreI64 hard-fact verifier
```

Source authority remains explicit declarations and canonical typed operations.
`value-origin` and `ValueDefMap` are rebuild mechanisms only; they are not
source authority.

Existing owner and refresh entrypoint:

```text
src/mir/exact_numeric_value_facts.rs
refresh_module_exact_numeric_value_facts
src/mir/semantic_refresh.rs::refresh_module_contracts_and_exact_numeric
```

Existing route oracle remains:

```text
src/mir/generic_method_route_plan/write_routes.rs
classify_key_route
```

The route oracle must remain independent of the witness projection.

## Scope

Accept a hard witness only when the existing exact-numeric owner provides
`declared_type_name == "i64"` through one of these closed roots:

```text
declared i64 parameter
declared i64 Box field
exact i64 constant
Copy from an accepted root
same-type i64 PHI
same-type i64 Select
checked exact-i64 BinaryOp
```

The projection should contain only hard witnesses. Absence means “no
source-backed hard witness”; do not add a `DerivedMirIntegerOnly` witness
variant.

Explicitly defer:

```text
unannotated parameter
generic dynamic Integer
LocalContractWrite without ExactNumericValueFact connection
exact-returning call without caller-side provenance
provider/helper result without a unified source fact
casts without source contract
i8/i16/i32/u8/u16/u32/u64/usize/isize roots
```

## Required implementation shape

1. Add a typed `MapStoreI64KeyWitness` projection in a narrow MIR module or
   existing generic-route fact boundary; keep every new source file below 800
   lines.
2. Read `function.metadata.exact_numeric_value_facts` after the existing exact
   numeric refresh. Do not create a parallel provenance table.
3. Key the witness by the existing route site and/or key `ValueId` so stale
   route/value identity can be rejected.
4. Add one `MapStoreI64HardFactVerifier` consumer boundary. It checks exact
   declared type, source-backed fact presence, key identity, and route-site
   freshness.
5. Keep `GenericMethodKeyRoute::I64Value`, `MapStoreI64`/`MapStoreAny` route
   selection, caller orientation, runtime mutation, publication, and backend
   lowering unchanged.

## Refresh order

```text
route convergence
-> ExactNumericValueFact rebuild
-> MapStoreI64 witness projection
-> hard-fact verifier
```

The witness is observational metadata in this slice. It must not become an
input to `write_routes.rs` route selection.

## Fixtures and oracle

Positive fixtures:

```text
mapstore_i64_key_from_declared_i64_parameter
mapstore_i64_key_from_declared_i64_field
mapstore_i64_key_through_copy
mapstore_i64_key_through_same_i64_phi
mapstore_i64_key_through_same_i64_select
mapstore_i64_key_from_checked_i64_add
mapstore_i64_key_fact_attaches_after_route_convergence
```

Negative fixtures:

```text
mapstore_i64_key_mirtype_integer_only_has_no_hard_fact
mapstore_i64_key_unannotated_parameter_has_no_hard_fact
mapstore_i64_key_any_parameter_has_no_hard_fact
mapstore_i64_key_u64_not_exact_i64
mapstore_i64_key_usize_not_exact_i64
mapstore_i64_key_mixed_exact_dynamic_phi_rejects
mapstore_i64_key_mixed_i64_u64_phi_rejects
mapstore_i64_key_call_result_without_source_fact_rejects
mapstore_i64_key_stale_value_id_rejects
mapstore_i64_key_route_site_drift_rejects
stored_value_change_does_not_change_key_witness
```

The independent oracle is the source declaration plus canonical MIR definition
graph and the expected `ExactNumericValueFactSource` chain. `write_routes.rs`
is only a separate route-decision parity veto; it must not generate the
expected Fact or witness.

## Fail-fast rules

The verifier must reject a claimed hard witness for:

```text
missing ExactNumericValueFact
witness/value identity mismatch
declared type other than exact i64
MirType::Integer-only evidence
unsupported Call/local/provider root
mixed exact/dynamic PHI or Select
different exact types merged
stale route site or ValueId
representation fact used as source proof
```

Suggested stable tags:

```text
[mirbuilder/mapstore_i64_key_fact_missing]
[mirbuilder/mapstore_i64_key_fact_stale]
[mirbuilder/mapstore_i64_key_fact_not_exact_i64]
[mirbuilder/mapstore_i64_key_fact_mirtype_only]
[mirbuilder/mapstore_i64_key_fact_unsupported_root]
[mirbuilder/mapstore_i64_key_fact_merge_rejected]
[mirbuilder/mapstore_i64_key_fact_route_drift]
```

Use existing default-off debug logging rules. Do not add a log-only workaround.

## Acceptance

```text
existing_exact_numeric_fact_owner_reused = 1
mapstore_i64_source_backed_key_witness_candidate = 1
mapstore_i64_first_hard_scope = exact_i64_only
current_i64value_disposition = derived_projection
mirtype_integer_hard_authority = 0
new_dynamic_integer_owner = 0
route_selection_authority_moved = 0
write_routes_oracle_retained = 1
caller_contract_widened = 0
mapstore_any_opened = 0
array_append_any_opened = 0
delete_opened = 0
plan_authority_selected = 0
boundary_authority_selected = 0
runtime_mutation_authority = 0
backend_lowering_authority = 0
source_selfhost_claim = 0
```

Commands must include the focused exact-numeric tests, the new witness
fixture/guard, `git diff --check`, and `bash
tools/checks/current_state_pointer_guard.sh`. A filtered test command that
collects zero tests is not evidence.

## Implementation result

Implemented the typed `MapStoreI64KeyWitness` projection and its verifier.
The projection runs after `refresh_module_exact_numeric_value_facts`, stores
only exact `i64` witnesses, and records missing/dynamic/non-`i64` routes in a
report-only rejection ledger. Existing `GenericMethodKeyRoute::I64Value`
classification and `write_routes.rs` route selection remain unchanged.

The existing 3457 inventory fixture and guard now cover the witness contract,
so no second lane guard was added.

## Stop boundary

Stop and return to design consultation if the existing exact-numeric owner does
not cover an intended root, if a second provenance owner appears necessary, or
if the witness would need to change route selection or runtime behavior.
