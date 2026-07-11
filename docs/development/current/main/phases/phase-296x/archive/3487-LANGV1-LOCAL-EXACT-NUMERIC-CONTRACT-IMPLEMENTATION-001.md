# 3487 - LANGV1-LOCAL-EXACT-NUMERIC-CONTRACT-IMPLEMENTATION-001

## Status

Complete. Decisions are inherited from 3485 and the identity prerequisite is
complete in 3486.

Decision: accepted.

## Objective

Activate one exact-numeric semantic contract for explicit local initialization
and reassignment. Preserve lexical identity through `LocalSlotId(BindingId)`,
make the check-before-publication boundary explicit in MIR, and reject every
unsupported backend before program effects.

## Structural Owners

```text
lexical identity:
  LocalSlotId(BindingId), allocated only by the existing BindingId owner

contract inventory:
  FunctionMetadata.local_slot_contracts[]

write boundary:
  LocalContractWrite { dst, src, local_slot_id, write_kind }

runtime value check:
  shared ExactNumericRuntimeValueChecker

runtime execution:
  VM LocalContractWrite consumer

backend support:
  central local_slot_exact_numeric capability preflight
```

Generic `Copy`, source names, `ValueId`, `MirType`, and exact-numeric facts are
not contract authority.

## Ordered Tasks

1. Add typed `LocalSlotContract` and `LocalWriteKind` vocabulary in a focused
   owner module; do not add a second identity allocator.
2. Add `FunctionMetadata.local_slot_contracts` and deterministic semantic
   refresh/drift validation from explicit exact-numeric local declarations.
3. Add canonical `MirInstruction::LocalContractWrite`; initializer and
   reassignment order is RHS once, check, then publish fresh destination.
4. Reject explicit exact-numeric `local x: T` without an initializer before
   effects. Unannotated locals retain current behavior in this slice.
5. Route ordinary and CorePlan accepted local writes through the same
   operation owner. No by-name or fixture-specific path.
6. Verify branch/PHI/loop reachable incoming writes are checked and preserve
   one LocalSlotId. PHI/loop publication does not duplicate runtime checks.
7. Export contract rows and LocalContractWrite through MIR JSON.
8. Execute LocalContractWrite in the Rust VM using the shared exact-numeric
   checker. Runtime-check elision remains forbidden.
9. Add central capability preflight. PyVM/LLVM/AOT/Wasm reject before effects
   until they implement a typed consumer; VM fallback is forbidden.
10. Add focused init/reassignment/Any/shadow/if-PHI/loop/U1/backend fixtures.

## Stable Fail-Fast Tags

```text
type/local_contract_carrier_missing
type/local_contract_carrier_drift
type/local_contract_duplicate_slot
type/local_contract_write_site_missing
type/local_contract_write_site_drift
type/local_contract_violation
type/local_contract_uninitialized_forbidden
type/local_contract_check_after_publication_forbidden
type/local_contract_phi_unchecked_incoming
type/local_contract_phi_binding_mismatch
type/local_contract_loop_carrier_binding_mismatch
type/backend_local_contract_capability_missing
type/backend_local_contract_silent_drop
type/local_contract_copy_metadata_authority_forbidden
type/local_contract_lazy_read_check_forbidden
```

## Explicit Non-Claims

```text
all_local_types_activated = 0
optional_uninitialized_state = 0
local_proof_elision = 0
closure_capture_contract = 0
ffi_contract_activation = 0
non_vm_local_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Acceptance

```text
local_exact_numeric_contract_activation = 1
local_slot_identity_owner_count = 1
local_write_operation_owner_count = 1
init_and_reassignment_coverage = complete_for_first_slice
rhs_evaluation_count = 1
check_before_publication = 1
uninitialized_exact_local_rejected = 1
shadow_new_identity = 1
phi_loop_checked_edge_completeness = 1
runtime_check_elision = 0
vm_local_contract_supported = 1
unsupported_backend_pre_effect_failfast = 1
mir_json_local_contract_carrier = 1
changed_production_source_over_800_lines = 0
```

## Stop Line

Do not widen the accepted type family, add an Uninitialized runtime state,
teach non-VM backends to lower the operation, or begin the representation-only
`:T` audit in this card.

## Closeout Evidence

```text
typed carrier:
  FunctionMetadata.local_slot_contracts[]
  FunctionMetadata.local_identity_evidence

canonical write boundary:
  LocalContractWrite { dst, src, local_slot_id, write_kind }
  effect class = CONTROL, so optimization cannot erase the runtime check

producer coverage:
  ordinary local init/reassignment
  CorePlan local init/reassignment
  exact-numeric uninitialized local rejected before lowering

identity proof:
  LocalSlotId wraps the existing BindingId
  shadowing allocates a distinct slot
  branch PHI and loop carry rebuild checked-write provenance

consumer boundary:
  Rust MIR interpreter checks before destination publication
  MIR JSON exports typed carrier, operation, and identity evidence
  non-VM backends reject through central capability preflight

instruction diet:
  canonical MIR operations = 36
  total vocabulary rows = 52
  LocalContractWrite is transport-visible but not LLVM-lowerable
```

Focused builder, VM, carrier-drift, PHI/loop, optimizer, JSON, backend-gate,
and parameter-regression tests are green. `cargo check --all-targets
--features vm-reference`, the release `hakorune` build, JSON schema parsing,
the local contract fixture, and the current-state pointer guard are green.
Changed and new Rust source files remain below 800 lines.

The repository-wide quick gate currently stops before this card's tests at an
unrelated existing Hako parser naming guard:
`parser_control_box.hako missing required naming token: syntax-3 path`. The
existing typed-return fixture also exposes a pre-existing direct-verifier
refresh-order gap: `count_to/1` has a declared `i64` return but reaches the
verifier without its return carrier. Neither issue is bypassed or repaired in
this local-contract card; the carrier ordering question moves to 3488.
