---
Status: Landed
Date: 2026-06-15
Task: COREPLAN-PHI-TXN-001
Scope: Define a PHI lifecycle transaction wrapper before migrating broader PHI construction paths.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-1036-COREPLAN-VARMAP-RESEAL-002-GENERIC-LOOP-BODY.md
  - docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md
  - src/mir/builder/emission/phi_lifecycle.rs
  - tools/checks/coreplan_phi_binding_boundary_guard.sh
---

# COREPLAN-PHI-TXN-001: PHI Transaction Boundary

## Decision

This is a BoxShape-only row. It adds a transaction wrapper over the existing
PHI lifecycle helpers without migrating broad PHI construction paths yet.

```text
selected_row=phi_lifecycle_transaction_boundary
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

`phi_lifecycle` remains the only owner allowed to call low-level PHI insert /
patch operations. `PhiTxn` centralizes provisional PHI ordering:

```text
begin
  -> define_provisional_phi
  -> patch_phi_inputs or abort_on_err
  -> commit
```

## Implementation

```text
PhiToken:
  block + dst token for a provisional PHI owned by the transaction

PhiTxn::begin:
  creates an empty transaction with a stable tag

PhiTxn::define_provisional_phi:
  calls phi_lifecycle::define_provisional_phi and records the token

PhiTxn::patch_phi_inputs:
  calls phi_lifecycle::patch_phi_inputs and removes the token from pending

PhiTxn::commit:
  fail-fast if any provisional PHI remains unpatched

PhiTxn::abort_on_err:
  rolls back pending provisional PHIs and returns a tagged fail-fast error
```

## Acceptance

```text
phi_transaction_boundary_defined=1
phi_txn_commit_failfast_on_unpatched=1
phi_txn_abort_rollback_defined=1
phi_low_level_callsite_owner=phi_lifecycle
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
summary=ok
```

## Proof Commands

```bash
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
cargo check --bin hakorune
```

## Stop Line

```text
do not migrate JoinIR merge PHI builders in this row
do not change accepted source shapes
do not add fallback for missing current block
do not make variable_map PHI truth
do not expose PhiTxn outside mir::builder
```

## Next

```text
COREPLAN-JOINIR-MERGE-PHI-001:
  migrate exactly one JoinIR merge PHI construction path to the PHI lifecycle
  transaction/API.
```
