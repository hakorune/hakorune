# 3489 - LANGV1-REPRESENTATION-HINT-AND-CONTRACT-REFRESH-OWNER-001

## Status

Active BoxShape implementation card. Decisions are inherited from 3488.

Decision: accepted.

## Objective

Make `semantic_refresh` the only refresh-and-validation owner for all four
active exact-numeric contract carrier families. Close public consumer bypasses
and separate normative type meaning from mutable implementation status without
changing source semantics or activating another type family.

## Structural Owner

```text
semantic_refresh::refresh_and_validate_for_boundary
  -> deterministic carrier rebuild
  -> completeness and drift validation
  -> RefreshedContractBundle / validated module access
```

Do not add another carrier allocator, test-only carrier constructor, consumer
fallback, or environment-selected activation.

## Boundary Kinds

```text
Verifier
MirJsonExport
VmExecution
BackendPreflight
ToolDirectVerify
```

Use one typed boundary vocabulary. Do not branch on executable names, fixture
paths, function names, or backend helper names.

## Ordered Tasks

1. Inventory every public verifier, MIR JSON, VM, backend-preflight, and direct
   tool/test entry. Record the owning facade call and remaining bypass count in
   a checked table; do not treat path/use counts as semantic proof.
2. Add one `semantic_refresh` facade that accepts a mutable module and typed
   boundary kind, rebuilds all active carriers, validates them, and returns a
   boundary-scoped validated view or token.
3. Rebuild in one deterministic owner:
   - exact-numeric Box-field write contracts;
   - parameter-entry contracts;
   - return-exit contracts;
   - local-slot contracts, write inventory, and PHI/loop identity evidence.
4. Validate source/carrier completeness and deterministic rebuilt equality.
   Reuse existing MIR/CFG/SSA/BindingId identities; do not create a parallel
   epoch namespace unless an existing rewrite owner already exposes one.
5. Route verifier, MIR JSON, VM, backend preflight, and direct tool entry points
   through the facade. Consumer-local rebuild is forbidden.
6. Reproduce and structurally fix the existing `while_expected.hako`
   `count_to/1` return-carrier failure through the shared facade.
7. Add negative tests for direct bypass, missing carrier after refresh,
   source/carrier drift, local identity/write drift, and backend execution
   before preflight.
8. Audit consumers of source annotations, `FunctionSignature`, `MirType`, exact
   numeric facts, storage/layout metadata, Plan, and Rune. Classify each as
   semantic carrier, derived representation, explicit plan input, or debt.
9. Add one mutable type-contract status ledger under development docs. Move
   implementation/backend status out of normative `types.md` while preserving
   normative language meaning and links.
10. Add guards/tests proving representation facts and hints cannot satisfy or
    synthesize semantic carriers.

## Initial Code Inventory

```text
existing owner:
  src/mir/semantic_refresh.rs

known consumers/entry families:
  src/mir/verification/**
  src/runner/mir_json_emit/**
  src/backend/mir_interpreter/**
  src/mir/backend_capability.rs
  src/runner/modes/**
  src/host_providers/mir_builder.rs

existing carrier refresh helpers:
  exact_numeric_field_contracts
  type_contracts::parameter_entry
  type_contracts::return_exit
  type_contracts::local_slot
```

This inventory is an implementation starting point, not authority. The first
task must identify public entrances structurally before editing consumers.

## Stable Fail-Fast Tags

```text
type/contract_refresh_owner_bypassed
type/contract_refresh_required
type/contract_refresh_stale
type/contract_refresh_rebuild_failed
type/contract_carrier_missing_after_refresh
type/contract_carrier_source_drift
type/contract_carrier_family_missing
type/contract_direct_verifier_bypass_forbidden
type/contract_mir_json_bypass_forbidden
type/contract_vm_bypass_forbidden
type/contract_backend_preflight_bypass_forbidden
type/contract_tool_fixture_carrier_synthesis_forbidden
type/representation_fact_as_contract_proof_forbidden
type/mir_type_as_contract_proof_forbidden
type/exact_numeric_fact_as_contract_proof_forbidden
type/storage_layout_as_contract_proof_forbidden
type/source_type_as_rune_authority_forbidden
type/backend_started_before_contract_preflight_forbidden
type/runtime_backend_fallback_forbidden
```

Keep site-specific missing/drift/silent-drop tags underneath these umbrella
boundaries. Define each string once in its selected owner.

## Fixture Matrix

```text
direct verifier without facade:
  rejected as bypass

direct verifier through facade:
  all active carriers rebuilt before verification

MIR JSON / VM / backend preflight without validated boundary:
  rejected before export or effects

declared exact-numeric return with missing carrier before refresh:
  rebuilt deterministically; present after refresh

active contract missing after refresh:
  fail-fast

source annotation/carrier drift:
  fail-fast

CFG/SSA/BindingId/local-write mutation followed by stale evidence:
  rebuild or fail-fast before consumer

MirType/exact fact/storage layout used as semantic proof:
  fail-fast

source :T used directly as Plan/Rune authority:
  fail-fast

unsupported backend:
  existing capability failure before effects; no VM fallback
```

## Acceptance

```text
contract_refresh_owner_count = 1
contract_refresh_owner = semantic_refresh
public_contract_consumer_bypass_count = 0
active_carrier_family_count = 4
direct_typed_return_refresh_reproduction = green
representation_consumer_inventory_checked = 1
types_status_ledger_split = 1
new_type_family_activation = 0
runtime_check_elision_widened = 0
backend_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
changed_production_source_over_800_lines = 0
```

## Stop Line

Do not activate records, arrays, FFI, optional locals, new proof elision, or
non-VM contract lowering. Do not change null/void/Option semantics. Finish the
single refresh owner and representation audit before opening the next type row.
