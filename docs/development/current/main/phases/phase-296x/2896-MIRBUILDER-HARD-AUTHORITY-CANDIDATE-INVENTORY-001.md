---
Status: Landed
Date: 2026-07-05
Scope: MirBuilder hard-authority candidate inventory.
---

# MIRBUILDER-HARD-AUTHORITY-CANDIDATE-INVENTORY-001

## Decision

Select the next hard-authority slice from the Fact track.

The first concrete candidate is `try_extract_loop_feature_facts` in
`src/mir/builder/control_flow/plan/facts/feature_facts.rs`.

Why this candidate first:

- it reads AST/body only
- it returns a fact DTO (`LoopFeatureFacts`)
- it does not mutate MIR
- it does not allocate IDs
- it does not require new `.hako` backend capability
- it is smaller than the planner/recipe route owners

`build_plan_with_facts_ctx` and `try_build_outcome` remain runner-up plan
candidates. They are still part of the inventory, but they are not the first
slice.

## Worker Inventory Note

The current worker scan confirmed the Fact boundary and kept the next slice
small:

- `try_extract_loop_feature_facts` is read-only and returns `LoopFeatureFacts`
- the observable contract is `nested_loop`, `exit_usage`, `exit_map`,
  `value_join=None`, and `cleanup=None`
- `ScopeBox` flattening is caller-owned in `loop_builder.rs`
- `flatten_scope_boxes` is analysis-only
- wrapper nodes outside `If` are invisible to exit usage unless the caller has
  already flattened them
- `build_plan_with_facts_ctx` and `try_build_outcome` stay held until the Fact
  slice lands
- existing unit-test anchors are already in place:
  `exit_usage_tracks_break_continue_return_in_if`,
  `exit_usage_ignores_nested_loops`,
  `nested_loop_detects_if_branch_loop`
- `has_unwind` remains false in the current walker, so the first fixture should
  keep it frozen as `false`
- `value_join` and `cleanup` remain `None`; the first fixture should not try to
  infer extra lowerer-owned meaning here

## Fixture Contract

The first Rust oracle fixture for this slice should freeze the primary
observables only:

```text
primary facts:
  exit_usage
  nested_loop

derived fact:
  exit_map

fixed empty fields:
  value_join = None
  cleanup = None
```

Recommended seed rows:

```text
row 1:
  body shape = if-with-break/continue/return
  exit_usage = { break=true, continue=true, return=true, unwind=false }
  nested_loop = false
  exit_map = present with 3 kinds

row 2:
  body shape = nested loop hidden under if
  exit_usage = default(false/false/false/false)
  nested_loop = true
  exit_map = None
```

Exact AST sketches:

```text
row 1:
  [
    If {
      condition: true,
      then_body: [Break],
      else_body: Some([Continue]),
    },
    Return { value: None },
  ]

row 2:
  [
    If {
      condition: true,
      then_body: [Loop { condition: true, body: [] }],
      else_body: None,
    },
  ]
```

This keeps the first fixture narrow while still proving the read-only fact owner
distinguishes the direct exit summary from nested-loop observation.

Construction rule:

```text
Use direct AST nodes only.
Do not wrap the seed rows in ScopeBox, Program, TryCatch, or LoopRange.
The fixture should match the post-flatten analysis view that
try_extract_loop_feature_facts observes.
```

Suggested JSON row schema:

```text
rows[].body:
  AST sketch serialized as a small expression tree

rows[].expected_exit_usage:
  { break: bool, continue: bool, return: bool, unwind: bool }

rows[].expected_nested_loop:
  bool

rows[].expected_exit_map_kinds:
  ordered list using the enum order
  [Return, Break, Continue, Unwind] filtered to present kinds, or []

rows[].expected_value_join:
  null

rows[].expected_cleanup:
  null
```

For the two seed rows, the expected names are:

```text
row 1:
  expected_exit_usage = { break=true, continue=true, return=true, unwind=false }
  expected_nested_loop = false
  expected_exit_map_kinds = [Return, Break, Continue]

row 2:
  expected_exit_usage = { break=false, continue=false, return=false, unwind=false }
  expected_nested_loop = true
  expected_exit_map_kinds = []
```

Top-level fixture fields should follow the existing Rust-oracle pattern:

```text
schema_version = 0
kind = MirBuilderLoopFeatureFactsRustOracleV1
token = MIRBUILDER-LOOP-FEATURE-FACTS-RUST-ORACLE-FIXTURE-001
owner = loop_feature_facts
rust_source.path = src/mir/builder/control_flow/plan/facts/feature_facts.rs
rust_source.oracle_surface = LoopFeatureFacts owner
non_claims.source_selfhost_claim = 0
non_claims.hako_adopted_decision = 0
non_claims.runtime_fallback = 0
non_claims.new_backend_route = 0
non_claims.new_abi = 0
non_claims.mir_type_migration = 0
non_claims.backend_lowering_migration = 0
non_claims.mir_mutation_migration = 0
```

## Evidence

Current inventory facts:

```text
existing_owner_count=166
existing_by_status.adopted_complete=150
existing_by_status.adopted_incomplete_inventory=16
source_scan_surface_count=1143
source_scan_candidate_count=626
source_scan_rejected_count=516
```

Relevant code surfaces:

```text
src/mir/builder/control_flow/plan/facts/feature_facts.rs::try_extract_loop_feature_facts
src/mir/builder/control_flow/plan/facts/loop_builder.rs::try_build_loop_facts_with_ctx
src/mir/builder/control_flow/plan/planner/outcome.rs::build_plan_with_facts_ctx
src/mir/builder/control_flow/plan/single_planner/rules.rs::try_build_outcome
src/mir/builder/control_flow/plan/recipe_tree/contracts.rs::RecipeContract
```

Boundary blocker evidence remains unchanged:

- typed-object/static-helper `ControlFormBox` helpers are not a safe path
- backend lowering and MIR mutation remain Rust
- ID allocation remains Rust

## Result

```text
next_owner_candidate=try_extract_loop_feature_facts
implemented_task=MIRBUILDER-LOOP-FEATURE-FACTS-RUST-ORACLE-FIXTURE-001
parity_gate=tools/checks/rust_lifecycle_mirbuilder_loop_feature_facts_parity_gate.sh
parity_status=green
fact_track=selected
plan_track=held
boundary_track=blocker_only
source_selfhost_claim=0
new_backend_route=0
new_abi=0
runtime_fallback=0
```

## Stop Line

Do not implement the next slice yet.
Do not widen `.hako` backend capability to fit `ControlFormBox`.
Do not move MIR mutation, backend lowering, or ID allocation into `.hako`.
Do not re-enter leaf-only pilot selection as the default next step.

## Next

Stop for the HakoAdopted decision consultation. Do not widen the backend-safe
body snapshot or move plan-track owners until the adoption boundary is accepted.
