# 3493 - LANGV1-RECORD-VALUE-CONTRACT-OWNER-001

## Status

Active implementation card. The 3492 prerequisite and FULL grammar gate are
green.

Decision: accepted implementation scope from 3491.

## Objective

Activate record construction and record with-update under one
`RecordValueContractOwner`, one typed semantic carrier, one VM consumer, one
MIR JSON transport, and one central unsupported-backend gate.

## Prerequisite

```text
3492 parameter BindingId corrective = green
LANGV1_GRAMMAR_FULL differential cases = 12/12
Hako adapter process count = 1
sensitive-path FULL gate coupling = active
```

## Structural Boundary

```text
RecordDecl declaration inventory
  -> semantic_refresh record projection
  -> RecordValueContractOwner / RecordValueContract
  -> verifier or VM consumer
  -> derived MIR JSON / backend capability transport
```

Keep source declaration, semantic carrier, runtime consumer, and representation
transport in separate modules. Update the nearest owner README. Do not grow
`record_values.rs` into policy, runtime, JSON, and declaration ownership; it
remains the focused builder evaluation/carrier-publication owner.

## Ordered Tasks

1. Project parser-independent record/field identity and a schema fingerprint
   from the existing `RecordDecl` owner. Do not add a second allocator or
   name-hash authority.
2. Define source-owned record/field contract specs. Fingerprints contain only
   declaration identity, field identity/order, required/default disposition,
   and source type-contract spec.
3. Add `RecordValueContractOwner` and typed carriers for `Construct` and
   `WithUpdate` publication boundaries.
4. Add declaration-ordered field rows with final `ValueId`, source contract,
   and explicit disposition. Active first-slice fields use runtime checks
   unless `semantic_refresh` supplies a fresh existing proof.
5. Route rebuild/validation through
   `semantic_refresh::refresh_and_validate_for_boundary`. Forbid direct
   builder, verifier, JSON, VM, or backend carrier synthesis.
6. Structure construction as preflight, source-order explicit evaluation and
   immediate checks, declaration-order default evaluation and immediate
   checks, assembly, then publication. Evaluate every expression once.
7. Structure with-update as preflight, one base evaluation, schema validation,
   unchanged-field validation, source-order update evaluation/check, assembly,
   then replacement publication. Never mutate the base.
8. Keep structural rejects under one validation owner. Do not duplicate
   unknown/duplicate/missing-field decisions across carrier, verifier, and VM.
9. Add one VM consumer. Reuse the shared exact-numeric runtime value checker;
   do not duplicate type/range policy.
10. Add MIR JSON transport only after refresh validation.
11. Add one central `RecordValueContracts` backend capability. VM supports it;
    unsupported PyVM/LLVM/AOT/Wasm reject before effects without fallback.
12. Add focused fixtures and run contract, JSON, VM, backend-preflight, and
    sensitive-path FULL grammar gates.

## Carrier Shape

```text
RecordValueContract:
  contract_id
  boundary = Construct | WithUpdate
  record declaration identity
  schema fingerprint
  dst ValueId
  optional base ValueId
  declaration-ordered field rows

RecordFieldValueContract:
  field declaration identity
  diagnostic field name
  final ValueId
  source TypeContractSpec
  disposition = AnyDefault
              | RuntimeCheckedContract
              | VerifierProvenContract(fresh proof)
              | UnsupportedFailFast
```

Names are diagnostics only. `MirType`, `value_types`, storage plans, backend
routes, and successful VM execution are not identity or proof.

## Fixture Matrix

| Fixture | Expected |
| --- | --- |
| exact-numeric construction | checks active fields, then publishes |
| explicit field effects | exactly once in source order |
| missing default effects | exactly once in declaration order |
| wrong runtime field | reject before publication |
| unknown/duplicate/missing field | preflight reject where knowable |
| with-update | new value; base unchanged |
| wrong update base schema | reject before update effects |
| unchanged active field | checked in first slice |
| stale/source-drift/bypass | stable fail-fast |
| MirType/storage used as proof | stable fail-fast |
| MIR JSON | fresh carrier after refresh |
| unsupported backend | reject before effects |
| typed Array contract | remains inactive |

## Stable Fail-Fast Tags

```text
type/record_contract_unknown_record
type/record_contract_generic_unsupported
type/record_contract_constructor_arity_mismatch
type/record_contract_duplicate_field
type/record_contract_unknown_field
type/record_contract_missing_required_field
type/record_contract_default_unsupported
type/record_contract_field_contract_unsupported
type/record_contract_field_runtime_mismatch
type/record_contract_update_base_mismatch
type/record_contract_stale_carrier
type/record_contract_source_drift
type/record_contract_refresh_bypass
type/record_contract_representation_as_proof
type/record_contract_backend_unsupported
```

## Acceptance

```text
record_value_contract_owner_count = 1
record_contract_declaration_identity_owner_count = 1
record_contract_second_identity_allocator = 0
record_construction_contract_activation = 1
record_with_update_contract_activation = 1
record_field_evaluation_exactly_once = 1
record_default_evaluation_declaration_order = 1
record_with_update_mutates_base = 0
record_contract_refresh_owner = semantic_refresh
record_contract_vm_consumer_count = 1
record_contract_mir_json_transport = 1
record_contract_unsupported_backend_pre_effect_reject = 1
record_contract_representation_fact_authority = 0
typed_array_contract_activation = 0
changed_production_source_over_800_lines = 0
```

## Explicit Non-Claims

```text
typed_array_contract_activation = 0
record_generic_contract_activation = 0
ffi_contract_activation = 0
runtime_check_elision_widened = 0
new_backend_contract_lowering = 0
ownership_model_changed = 0
failure_outcome_migration = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Next

After closeout, re-audit the remaining type-family queue from current source
evidence. Typed `Array<T>` still requires a source-owned element contract and
one element-write owner; `MirType`, route, or storage evidence is insufficient.
