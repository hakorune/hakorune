---
Status: Investigation
Decision: accepted
Date: 2026-06-13
Scope: MIR cleanup inventory and task order. This is BoxShape-only planning;
no semantic acceptance shape is added here.
Related:
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/agent-current-entry-contract-ssot.md
  - src/mir/builder/README.md
---

# MIR Cleanup Inventory 2026-06-13

## Decision

Clean up MIR structure in low-risk order.

```text
first:
  inventory / policy / test-file splits / thin mod.rs classification

later:
  compat quarantine / builder control_flow entry map

last:
  deep path flatten pilot
```

Do not start with path flattening. `src/mir/builder/control_flow` is the largest
and deepest region, but it is also where planner/facts/lower/verify contracts
are most sensitive.

## Scope

Counts below are Rust source files under `src/mir`.

```text
src/mir rust files: 1497
src/mir rust lines: 297755

src/mir/builder rust files: 707
src/mir/builder rust lines: 116325

src/mir/builder/control_flow rust files: 562
src/mir/builder/control_flow rust lines: 91158
```

`src/mir/builder` is about 39% of `src/mir` by lines.
`src/mir/builder/control_flow` is about 31% of `src/mir` by lines.

## Top MIR Areas

```text
lines   files  path
116325   707   src/mir/builder
 28271   130   src/mir/join_ir
 18718    84   src/mir/global_call_route_plan
 17503    76   src/mir/passes
  9243    49   src/mir/control_tree
  8088    31   src/mir/generic_method_route_plan
  7316    28   src/mir/loop_route_detection
  6147    27   src/mir/join_ir_vm_bridge
  4928    18   src/mir/verification
  4180    14   src/mir/user_box_method_route_plan
  3677    19   src/mir/phi_core
  3212    12   src/mir/typed_object_plan
  2910    13   src/mir/fastmem_access_plan
  2905    16   src/mir/loop_canonicalizer
```

## Deepest Paths

Max observed Rust file directory depth under `src/mir` is 9 slash-separated
directory components after the repo root.

Representative deepest files:

```text
src/mir/builder/control_flow/plan/canon/generic_loop/step/placement/decision.rs
src/mir/builder/control_flow/facts/canon/generic_loop/step/placement/matcher.rs
src/mir/builder/control_flow/facts/canon/generic_loop/step/extract/var_step.rs
src/mir/builder/control_flow/joinir/merge/rewriter/stages/plan/terminator_rewrite.rs
```

Read:

```text
facts/canon/generic_loop/step/...
plan/canon/generic_loop/step/...
joinir/merge/rewriter/stages/plan/...
```

These are flatten candidates, but not first cleanup targets.

## Large Files

There are 52 Rust files under `src/mir` with 600 or more lines.

Top examples:

```text
699 src/mir/passes/callsite_canonicalize/tests.rs
699 src/mir/builder/control_flow/edgecfg/api/compose/tests.rs
697 src/mir/generic_method_route_plan/mir_json_routes.rs
685 src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs
683 src/mir/sum_variant_project_seed_plan.rs
682 src/mir/contracts/backend_core_ops.rs
674 src/mir/user_box_method_route_plan/origin_inference.rs
673 src/mir/array_text_combined_region_plan.rs
671 src/mir/string_kernel_plan.rs
670 src/mir/control_tree/normalized_shadow/anf/execute_box.rs
669 src/mir/exact_seed_backend_route.rs
669 src/mir/array_text_loopcarry_plan.rs
668 src/mir/thin_entry.rs
667 src/mir/sum_variant_tag_seed_plan.rs
666 src/mir/global_call_route_plan/tests/runtime_methods/string_methods.rs
665 src/mir/verification/hako_alloc_metadata.rs
661 src/mir/builder/control_flow/plan/loop_cond/break_continue_facts.rs
660 src/mir/typed_object_plan/storage_inference/tests.rs
660 src/mir/phi_core/phi_builder_box.rs
660 src/mir/join_ir/lowering/loop_update_analyzer.rs
660 src/mir/compiler/tests.rs
```

Read:

```text
test files are the safest first split targets
production files require a local README / seam before split
```

## Thin mod.rs Inventory

There are 97 `mod.rs` files under `src/mir` with 30 lines or fewer.
There are 45 with 10 lines or fewer.

Representative entries:

```text
1 src/mir/builder/schedule/mod.rs
2 src/mir/builder/control_flow/plan/generic_loop/facts/mod.rs
2 src/mir/builder/ssa/mod.rs
2 src/mir/contracts/mod.rs
2 src/mir/control_tree/normalized_shadow/tests/mod.rs
2 src/mir/loop_route_detection/support/function_scope/analyzers/tests/mod.rs
2 src/mir/lowerers/mod.rs
3 src/mir/builder/control_flow/plan/emit/mod.rs
3 src/mir/builder/control_flow/verify/observability/mod.rs
3 src/mir/builder/router/mod.rs
3 src/mir/builder/vars/mod.rs
```

Do not delete these mechanically. Classify first:

```text
pure re-export:
  may be collapsed if imports stay local and readable

module boundary:
  keep if it documents a layer or hides submodules

test grouping:
  keep if it stabilizes cargo test filters or fixture taxonomy
```

## Compat / Legacy Inventory

Filename-based candidates are small:

```text
src/mir/builder/control_flow/lower/planner_compat.rs
src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
src/mir/passes/string_corridor_sink/tests/concat_and_return_parts/publication_helper_shape_reads_folded_route_proof_without_legacy_candidates.rs
src/mir/string_corridor_compat.rs
src/mir/verification/legacy.rs
```

Text hits for `compat|legacy` are much larger and include comments, report
vocabulary, and intentional compatibility policy. They must not be treated as
delete candidates without classification.

```text
filename candidates:
  quarantine / keep / retire classification required

text hits:
  policy inventory only
```

## Cleanup Task Ladder

### CURRENT-CLEAN-000

Add this inventory.

Status: landed 2026-06-13.

Acceptance:

```text
src/mir size is measured
builder/control_flow size is measured
large file count is measured
thin mod.rs count is measured
compat/legacy filename candidates are listed
next task order is fixed
```

### CURRENT-CLEAN-001

Add a cleanup policy card.

Acceptance:

```text
BoxShape-only cleanup
no new accepted source/MIR shape
no optimizer behavior change
no perf keeper claim
one purpose per cleanup series
minimum gate per series is named
```

### MIR-CLEAN-001

Split large test files first.

Candidate first targets:

```text
src/mir/passes/callsite_canonicalize/tests.rs
src/mir/builder/control_flow/edgecfg/api/compose/tests.rs
src/mir/compiler/tests.rs
```

Acceptance:

```text
test module split only
no production behavior change
cargo test target for moved tests stays green
```

### MIR-CLEAN-002

Classify thin `mod.rs` files.

Acceptance:

```text
thin_mod_pure_reexport_count recorded
thin_mod_boundary_keep_count recorded
thin_mod_test_group_keep_count recorded
collapse candidates are listed
no module deletion yet
```

### MIR-CLEAN-003

Classify compat / legacy candidates.

Acceptance:

```text
filename candidates classified as keep/quarantine/retire
text hits are not treated as deletion candidates
retire candidates have tests/gates named before deletion
```

### MIR-CLEAN-004

Create `builder/control_flow` entry map.

Acceptance:

```text
facts / plan / lower / verify / joinir responsibilities documented
deep flatten pilot seam selected
forbidden cross-layer dependencies listed
```

### MIR-CLEAN-005

Deep path flatten pilot.

Candidate:

```text
generic_loop step placement
```

Acceptance:

```text
one deep subtree only
imports remain compatibility-safe or facade-backed
no acceptance shape added
targeted planner/facts tests green
```

## Non-Goals

```text
do not change MIR semantics
do not add BoxCount acceptance rules
do not update fast gates by widening accepted shapes
do not perform repo-wide module flattening
do not delete compat/legacy files before classification
do not move active perf lanes into cleanup cards
```

## Recommended Next Task

Proceed with `CURRENT-CLEAN-001`.

This should be docs-only and should define the cleanup gate style before any
file movement.
