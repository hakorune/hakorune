---
Status: Landed
Date: 2026-06-14
Scope: CorePlan / JoinIR remaining family inventory after B1 boundary landing.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
  - src/mir/builder/control_flow/plan/normalizer/README.md
---

# COREPLAN-FOUND-002: Remaining Family Inventory

## Purpose

Convert the remaining CorePlan / JoinIR foundation gaps into ordered,
one-purpose task rows before code changes.

This is an inventory and task-order card. It does not add accepted source
shapes, remove legacy route boxes, or change release behavior.

## Decision

```text
coreplan_remaining_family_inventory_landed=1
boxcount_boxshape_mixed=0
release_default_changed=0
accepted_shape_added=0
next_implementation_family=C1_planner_required_ambiguity_failfast
summary=ok
```

## Family Order

### 1. C1 planner_required ambiguity / no silent Ok(None)

Reason:

```text
C1 protects every later refactor.
If a target-like shape is observed under strict/dev + planner_required,
route exhaustion must fail-fast instead of disappearing as Ok(None).
```

Owner seams:

```text
route entry router:
  src/mir/builder/control_flow/joinir/route_entry/router.rs

route registry / candidate collection:
  src/mir/builder/control_flow/joinir/route_entry/registry/

single planner:
  src/mir/builder/control_flow/plan/single_planner/
```

Next slice:

```text
COREPLAN-C1-001:
  Add a strict/dev inventory guard for target-like route exhaustion.
  Classify Ok(None) sites as non-candidate probe vs silent fallback.
  Add negative fixtures only; do not add accepted shapes.
```

Acceptance:

```text
planner_required_target_like_route_exhaustion_classified=1
planner_required_silent_ok_none_inventory=1
candidate_ambiguity_owner_documented=1
accepted_shape_added=0
release_default_changed=0
```

Stop lines:

```text
do not convert all optional facts Ok(None) into errors
do not hide ambiguity with route priority scoring
do not duplicate route truth between single_planner and route_entry/registry
```

### 2. D1 normalizer to composition-only

Reason:

```text
D1 removes normalizer ownership drift.
The normalizer should become an adapter/composition surface, not an AST
shape owner or route-specific acceptance owner.
```

Current inventory:

```text
plan/normalizer still has direct ASTNode:: usage
helpers_value.rs owns broad AST expression lowering
loop_body_lowering.rs and cond_lowering_* still classify AST shapes
recipe_tree composers still synthesize AST nodes for some routes
```

Next slice:

```text
COREPLAN-D1-001:
  Add a normalizer AST-boundary inventory guard.
  Report per-file ASTNode:: counts under plan/normalizer.
  Report synthetic ASTNode::Loop construction in recipe_tree composers.
  Start as report-only.
```

Acceptance:

```text
normalizer_ast_boundary_inventory=1
normalizer_ast_hit_counts_reported=1
synthetic_ast_composer_inventory=1
release_default_changed=0
accepted_shape_added=0
```

Stop lines:

```text
do not move AST matching to another folder without a named adapter boundary
do not merge D1 with C1 fail-fast work
do not add source shapes while reducing normalizer drift
```

### 3. E1 compatibility fallback zero closeout

Reason:

```text
E1 cannot close until active routed loop_*_v0 boxes are retired one at a time.
The legacy normalizer table being empty is not enough; active routed v0 boxes
are a separate closeout surface.
```

Current active v0 boxes:

```text
loop_scan_v0
loop_scan_methods_v0
loop_scan_methods_block_v0
loop_scan_phi_vars_v0
loop_collect_using_entries_v0
loop_bundle_resolver_v0
```

Next slice:

```text
COREPLAN-E1-001:
  Add an active-v0 inventory guard.
  Cross-check REGISTRY.md, LEGACY_V0_BOUNDARY.md, LoopFacts,
  route registry entries, predicates, handlers, composer, and matcher.
```

Follow-up:

```text
COREPLAN-E1-002:
  Retire exactly one v0 box with replacement proof.
  Candidate: loop_scan_methods_block_v0, if block-wrapper observation can fold
  into scan-methods facts without adding a new accepted shape.
```

Acceptance:

```text
active_v0_inventory_guard=1
active_v0_box_count_reported=1
legacy_normalizer_empty_and_active_v0_empty_are_separate=1
one_v0_box_per_retire_slice=1
```

Stop lines:

```text
do not remove active v0 route wiring without fixture/gate proof
do not mix planner_compat facade retirement with v0 route removal
do not add new loop_*_v0 boxes
```

### 4. loop-if-loop / loop-loop-if wiring

Reason:

```text
This is not selected as the next implementation family.
The fast gate already contains multiple nested loop and loop-if-loop fixtures.
Only open a BoxCount row when a concrete failing fixture proves a missing
accepted shape.
```

Known guards:

```text
tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
tools/smokes/v2/profiles/integration/joinir/phase29bs_loopframe_v1_nested_loop_strict_gate_vm.sh
```

Next slice:

```text
COREPLAN-LOOP-WIRING-001:
  Evidence-only failing-fixture selection.
  No implementation unless a first failing case proves the missing box.
```

Stop lines:

```text
do not add a broad nested-loop route from inventory alone
do not reopen old loop_*_v0 growth
do not change release default behavior
```

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/coreplan_compat_normalizer_legoization_guard.sh
```

## Closeout

This card makes the remaining CorePlan work executable in small slices:

```text
1. C1 fail-fast boundary guard
2. D1 normalizer AST-boundary inventory
3. E1 active-v0 inventory guard
4. one-v0 retire pilot
5. loop wiring only after failing-fixture evidence
```
