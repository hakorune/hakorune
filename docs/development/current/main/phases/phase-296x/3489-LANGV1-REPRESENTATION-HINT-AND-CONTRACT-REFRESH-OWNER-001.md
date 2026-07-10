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

## Execution Slices

Keep these slices in this card. Each slice must build and keep the previously
landed focused gates green; do not create inventory-only or rerun-only cards.

### S1 - Facade and validated boundary token

```text
scope:
  typed ContractRefreshBoundary vocabulary
  one refresh_and_validate_for_boundary facade
  deterministic rebuild and four-family validation
  validated boundary token/view that cannot be forged by ordinary consumers

tests:
  all four carrier families rebuild
  missing/drifted carrier is repaired only inside the owner
  active carrier missing after owner completion fails

non-scope:
  consumer rewiring
  representation policy changes
```

### S2 - Consumer convergence

```text
scope:
  verifier entry
  MIR JSON entry
  VM execution entry
  backend capability preflight
  direct tool/provider entry

rule:
  each public path obtains the validated boundary token once
  downstream helpers consume it or a facade-owned validated module
  no consumer-local refresh or fixture carrier synthesis

proof:
  while_expected.hako direct typed-return reproduction is green
  bypass negatives fail with stable owner tags
```

### S3 - Representation authority audit

```text
classify every relevant consumer as exactly one of:
  semantic_contract_carrier
  derived_representation_fact
  explicit_plan_input
  migration_debt

families:
  source declared type metadata
  FunctionSignature / MirType
  exact-numeric value/return facts
  storage and layout metadata
  Plan and Rune inputs

rule:
  classification is by owner/API responsibility, never by function name or
  current successful behavior
```

### S4 - Ledger split and closeout

```text
scope:
  add one mutable type-contract status ledger
  remove mutable backend/activation status from normative types.md
  retain normative meaning and links in types.md
  run focused contract suites and current pointer guard

closeout gate:
  consumer bypass count = 0
  representation debt queue is explicit
  exact-numeric island is ready for T4 audit
```

## Post-3489 Task Gates

Do not materialize these as numbered cards until the preceding gate closes.

```text
T4 exact-numeric island closeout:
  audit Box field + parameter + return + local owner completeness
  prove shared runtime checker semantics and per-site timing remain distinct
  prove unsupported backends reject every active carrier before effects
  activate no new type family

T5 next-family consultation:
  compare record construction/update and typed Array<T> element boundaries
  require a named producer, mutation/construction owner, runtime consumer,
  MIR JSON carrier, backend capability, and negative fixture set
  select exactly one family; do not infer selection from matrix row names

Failure/Outcome after Type Guarantee:
  decide null / void / Option::None terminal relation
  classify local default, WeakRef upgrade, and Missing/Void/Null boxes by meaning
  migrate source/API uses before changing literal_null profile

Coercion/Equality after Failure/Outcome:
  close truthiness, compatibility equality, String concatenation, and typed
  type-test surfaces under Canonical vs explicit Compat2025

Ownership/Identity after coercion:
  remove ordinary-field implicit cascade fini
  replace prose escape policy with one ownership-operation vocabulary

Capability/Effect then v1 closeout:
  verify uses / observed effects / Rune obligations independently
  run full parser + VM/EXE conformance before unparked selfhost migration
```

The nested-loop feature-product decomposition and route-family authority
promotion remain parked compiler BoxShape work. They do not preempt this
language lane without an explicit priority decision.

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
