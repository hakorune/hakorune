---
Status: Landed
Date: 2026-06-15
Task: COREPLAN-JOINIR-MERGE-PHI-001
Scope: Migrate one JoinIR merge PHI construction path to phi_lifecycle / PhiTxn.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1037-COREPLAN-PHI-TXN-001.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - src/mir/builder/control_flow/joinir/merge/exit_phi_builder.rs
  - src/mir/builder/emission/phi_lifecycle.rs
  - tools/checks/coreplan_phi_binding_boundary_guard.sh
---

# COREPLAN-JOINIR-MERGE-PHI-001: Exit PHI Builder Migration

## Decision

This is a BoxShape-only PHI migration row. It migrates exactly one JoinIR merge
PHI construction path: `exit_phi_builder`.

```text
selected_row=joinir_exit_phi_builder_phi_lifecycle_migration
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

`phi_lifecycle` remains PHI construction truth. JoinIR merge remains a lowering
bridge, not a place to add new semantic truth.

## Implementation

```text
exit_phi_builder:
  before:
    created a BasicBlock locally and pushed MirInstruction::Phi directly

  after:
    creates the exit block first
    uses PhiTxn::define_provisional_phi
    uses PhiTxn::patch_phi_inputs
    commits or aborts through PhiTxn

guard:
  joinir_exit_phi_builder_direct_phi_construction=0
```

## Acceptance

```text
one_joinir_merge_phi_path_migrated=1
joinir_exit_phi_builder_direct_phi_construction=0
phi_lifecycle_owner_preserved=1
phi_transaction_boundary_used=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
summary=ok
```

## Proof Commands

```bash
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only selfhost_parse_loop_min
cargo check --bin hakorune
```

## Stop Line

```text
one merge path only
do not broaden into loop_header_phi_builder in this row
do not route PHI construction through TypeAbiCatalog or BoxCallableRegistry
do not expand variable_map truth
do not change accepted source shapes
```

## Next

```text
COREPLAN-NORMALIZER-COMPOSITION-001:
  move one normalizer AST-owned decision behind an adapter / composition
  boundary without changing accepted source shapes.
```
