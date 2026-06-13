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

Initial representative deepest files before MIR-CLEAN-005..009:

```text
src/mir/builder/control_flow/plan/canon/generic_loop/step/placement/decision.rs
src/mir/builder/control_flow/facts/canon/generic_loop/step/placement/matcher.rs
src/mir/builder/control_flow/facts/canon/generic_loop/step/extract/var_step.rs
src/mir/builder/control_flow/joinir/merge/rewriter/stages/plan/terminator_rewrite.rs
```

Current generic-loop canon owner after cleanup:

```text
src/mir/builder/control_flow/generic_loop_canon/
```

Read:

```text
facts/canon/generic_loop/step/... and plan/canon/generic_loop/step/...
  are now compatibility facades for generic-loop canon helpers

joinir/merge/rewriter/stages/plan/...
  remains a near miss and should not be flattened without a separate JoinIR
  merge ownership cleanup
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

Status: landed 2026-06-13.

Candidate first targets:

```text
src/mir/builder/control_flow/edgecfg/api/compose/tests.rs
src/mir/passes/callsite_canonicalize/tests.rs
src/mir/compiler/tests.rs
```

Landed first split:

```text
src/mir/builder/control_flow/edgecfg/api/compose/tests.rs
  -> src/mir/builder/control_flow/edgecfg/api/compose/tests/mod.rs
  -> src/mir/builder/control_flow/edgecfg/api/compose/tests/loop_.rs
  -> src/mir/builder/control_flow/edgecfg/api/compose/tests/seq.rs
  -> src/mir/builder/control_flow/edgecfg/api/compose/tests/if_.rs
  -> src/mir/builder/control_flow/edgecfg/api/compose/tests/cleanup.rs
```

Acceptance:

```text
test module split only
no production behavior change
cargo test target for moved tests stays green
```

Verification:

```bash
cargo test --release --lib edgecfg::api::compose -- --nocapture
cargo fmt --check
```

### MIR-CLEAN-002

Classify thin `mod.rs` files.

Status: landed 2026-06-13.

Classification after MIR-CLEAN-001:

```text
thin_mod_total_count=98
thin_mod_pure_reexport_count=9
thin_mod_boundary_keep_count=73
thin_mod_test_group_keep_count=16
thin_mod_deleted_count=0
```

Pure re-export collapse candidates:

```text
src/mir/builder/schedule/mod.rs
src/mir/builder/ssa/mod.rs
src/mir/contracts/mod.rs
src/mir/lowerers/mod.rs
src/mir/builder/vars/mod.rs
src/mir/builder/control_flow/plan/generic_loop/body_check/mod.rs
src/mir/optimizer_passes/mod.rs
src/mir/ssot/mod.rs
src/mir/passes/mod.rs
```

Test grouping keep examples:

```text
src/mir/builder/control_flow/edgecfg/api/compose/tests/mod.rs
src/mir/generic_method_route_plan/tests/core_routes/mod.rs
src/mir/global_call_route_plan/tests/void_sentinel/mod.rs
src/mir/passes/dce/tests/mod.rs
src/mir/user_box_method_route_plan/tests/mod.rs
```

Boundary keep examples:

```text
src/mir/builder/control_flow/plan/generic_loop/facts/mod.rs
src/mir/builder/control_flow/plan/canon/mod.rs
src/mir/builder/control_flow/plan/planner/mod.rs
src/mir/builder/control_flow/verify/mod.rs
src/mir/control_tree/step_tree/mod.rs
```

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

Status: landed 2026-06-13.

Filename candidate classification:

```text
keep:
  src/mir/builder/control_flow/plan/LEGACY_V0_BOUNDARY.md
    active boundary SSOT for routed loop_*_v0 compatibility boxes

  src/mir/verification/legacy.rs
    active verifier guard for legacy instructions lowered away by Core-15

quarantine:
  src/mir/builder/control_flow/lower/planner_compat.rs
    live lower-side facade for planner/lowerer exports and router probes
    current owner docs already reference this as an intentional boundary

  src/mir/string_corridor_compat.rs
    active quarantine for string helper/runtime-name semantic recovery

not_a_cleanup_candidate:
  src/mir/passes/string_corridor_sink/tests/concat_and_return_parts/publication_helper_shape_reads_folded_route_proof_without_legacy_candidates.rs
    test name contains "legacy_candidates"; it is not a legacy module

retire:
  none
```

Retire gates if this changes later:

```text
planner_compat:
  prove direct consumers moved to a new lower facade
  cargo test --release --lib control_flow::lower -- --nocapture

string_corridor_compat:
  prove helper/runtime-name recovery is replaced by selected route metadata
  cargo test --release --lib string_corridor -- --nocapture

verification/legacy:
  prove Core-15 lowered-away verifier is replaced by a non-legacy name
  cargo test --release --lib verification -- --nocapture
```

Acceptance:

```text
filename candidates classified as keep/quarantine/retire
text hits are not treated as deletion candidates
retire candidates have tests/gates named before deletion
```

### MIR-CLEAN-004

Create `builder/control_flow` entry map.

Status: landed 2026-06-13.

Entry:

```text
src/mir/builder/control_flow/FOLDERIZATION_MAP.md
```

Summary:

```text
facts:
  conservative observation and analysis-only views

plan:
  temporary FlowPlanner implementation namespace

lower:
  lower-side compatibility facade and ownership seam

verify:
  fail-fast diagnostics and contract validation

joinir:
  route entry, merge, and JoinIR glue
```

Selected deep flatten pilot:

```text
generic_loop step placement
```

Acceptance:

```text
facts / plan / lower / verify / joinir responsibilities documented
deep flatten pilot seam selected
forbidden cross-layer dependencies listed
```

### MIR-CLEAN-005

Deep path flatten pilot.

Status: landed 2026-06-13.

Candidate:

```text
generic_loop step placement
```

Landed owner:

```text
initial:
  src/mir/builder/control_flow/step_placement/

current after MIR-CLEAN-007:
  src/mir/builder/control_flow/generic_loop_canon/step_placement/
  README.md
  facts.rs
  plan.rs
```

Compatibility facades:

```text
src/mir/builder/control_flow/facts/canon/generic_loop/step/placement/matcher.rs
src/mir/builder/control_flow/plan/canon/generic_loop/step/placement/decision.rs
```

Verification:

```bash
cargo test --release --lib generic_loop::facts::extract -- --nocapture
cargo fmt --check
```

Acceptance:

```text
one deep subtree only
imports remain compatibility-safe or facade-backed
no acceptance shape added
targeted planner/facts tests green
```

### MIR-CLEAN-006

Deep path flatten follow-up.

Status: landed 2026-06-13.

Candidate:

```text
generic_loop update canon
```

Landed owner:

```text
initial:
  src/mir/builder/control_flow/generic_loop_update_canon/

current after MIR-CLEAN-007:
  src/mir/builder/control_flow/generic_loop_canon/update/
  README.md
  literal_match.rs
  literal_step.rs
  mod.rs
```

Compatibility facade:

```text
src/mir/builder/control_flow/facts/canon/generic_loop/update.rs
```

Verification:

```bash
cargo test --release --lib generic_loop::facts::extract -- --nocapture
cargo fmt --check
```

Acceptance:

```text
facts-only update canon subtree
old facts path remains facade-backed
no acceptance shape added
targeted generic-loop facts tests green
```

### MIR-CLEAN-007

Group shallow generic-loop canon owners before adding more.

Status: landed 2026-06-13.

Reason:

```text
step_placement and generic_loop_update_canon proved the flatten seam.
Adding condition/step_extract as more control_flow/ siblings would make the
top-level layer too wide.
```

Target shape:

```text
src/mir/builder/control_flow/generic_loop_canon/
  README.md
  step_placement/
    facts.rs
    plan.rs
  update/
    literal_match.rs
    literal_step.rs
```

Compatibility facades remain:

```text
facts/canon/generic_loop/update.rs
facts/canon/generic_loop/step/placement/matcher.rs
plan/canon/generic_loop/step/placement/decision.rs
```

Acceptance:

```text
no behavior change
existing shallow pilots grouped under one semantic family owner
no additional generic-loop acceptance shape added
targeted generic-loop facts tests green
```

### MIR-CLEAN-008

Move generic-loop condition canon under grouped owner.

Status: landed 2026-06-13.

Candidate:

```text
generic_loop condition canon
```

Landed owner:

```text
src/mir/builder/control_flow/generic_loop_canon/condition/
  bound.rs
  candidates.rs
  mod.rs
```

Compatibility facade:

```text
src/mir/builder/control_flow/facts/canon/generic_loop/condition.rs
```

Verification:

```bash
cargo test --release --lib generic_loop::facts::extract -- --nocapture
cargo fmt --check
```

Acceptance:

```text
facts-only condition canon subtree
old facts path remains facade-backed
no acceptance shape added
targeted generic-loop facts tests green
```

### MIR-CLEAN-009

Move generic-loop step extract under grouped owner.

Status: landed 2026-06-13.

Candidate:

```text
generic_loop step extract
```

Landed owner:

```text
src/mir/builder/control_flow/generic_loop_canon/step_extract/
  complex_step.rs
  next_step.rs
  shared.rs
  var_step.rs
  mod.rs
```

Compatibility facade:

```text
src/mir/builder/control_flow/facts/canon/generic_loop/step/extract.rs
```

Verification:

```bash
cargo test --release --lib generic_loop::facts::extract -- --nocapture
cargo fmt --check
```

Acceptance:

```text
facts-only step extract subtree
step extraction order preserved
old facts path remains facade-backed
no acceptance shape added
targeted generic-loop facts tests green
```

### MIR-CLEAN-010

Migrate generic-loop canon consumers away from old facades.

Status: landed 2026-06-13.

Scope:

```text
src/mir/builder/control_flow/plan/generic_loop/**
src/mir/builder/control_flow/facts/extractors/**
src/mir/builder/control_flow/plan/canon/generic_loop**
src/mir/builder/control_flow/facts/canon/generic_loop**
```

Decision:

```text
generic_loop_canon is the direct owner for canon functions and canon types.
Old facts/plan canon paths remain compatibility facades only.
```

Verification:

```bash
rg -n "control_flow::facts::canon::generic_loop|control_flow::plan::canon::generic_loop" src/mir
cargo test --release --lib generic_loop::facts::extract -- --nocapture
cargo fmt --check
```

Acceptance:

```text
generic-loop planner consumers import generic_loop_canon directly
generic_loop_canon internals do not import old facts/plan facades
old facts/plan canon paths remain re-export-only facades
no acceptance shape added
targeted generic-loop facts tests green
```

### MIR-CLEAN-011

Refresh the MIR cleanup next-task pointer after CURRENT-CLEAN-001 and
MIR-CLEAN-010 landed.

Status: landed 2026-06-13.

Decision:

```text
CURRENT-CLEAN-001 is already landed.
Next cleanup work starts with generic-loop facade quarantine closeout and one
thin mod.rs collapse pilot.
```

Acceptance:

```text
recommended next task no longer points to a landed card
next cleanup sequence is explicit
docs-only stale pointer fix
```

Verification:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

### MIR-CLEAN-012

Document generic-loop old facade quarantine after consumer migration.

Status: landed 2026-06-13.

Scope:

```text
src/mir/builder/control_flow/facts/canon/generic_loop**
src/mir/builder/control_flow/plan/canon/generic_loop**
```

Decision:

```text
Old facts/plan generic-loop canon paths are compatibility facades.
They are allowed to re-export generic_loop_canon only.
New consumers must import generic_loop_canon directly.
Do not delete the facades until a separate retire card proves no external
or legacy internal callers need the old paths.
```

Verification:

```bash
rg -n "control_flow::facts::canon::generic_loop|control_flow::plan::canon::generic_loop" src/mir
rg -n "facts::canon::generic_loop|plan::canon::generic_loop" src/mir/builder/control_flow/generic_loop_canon
cargo test --release --lib generic_loop::facts::extract -- --nocapture
```

Acceptance:

```text
facade quarantine rule is documented
new-consumer import rule is documented
old paths remain compatibility-only
no acceptance shape added
```

### MIR-CLEAN-013

Collapse one pure re-export thin `mod.rs` as the pilot.

Status: landed 2026-06-13.

Candidate selected:

```text
src/mir/builder/schedule/mod.rs
  -> src/mir/builder/schedule.rs
```

Reason:

```text
single-child pure module declaration
no layer boundary documentation in the mod.rs itself
parent already owns the schedule boundary comment
```

Verification:

```bash
cargo check --release --lib
cargo fmt --check
```

Acceptance:

```text
one thin mod.rs collapsed
no production behavior change
module path stays crate::mir::builder::schedule::block
no additional thin mod.rs files collapsed in the same pilot
```

### MIR-CLEAN-014

Split the next large MIR test file.

Status: landed 2026-06-13.

Target:

```text
src/mir/passes/callsite_canonicalize/tests.rs
```

Landed split:

```text
src/mir/passes/callsite_canonicalize/tests/mod.rs
src/mir/passes/callsite_canonicalize/tests/mcl.rs
src/mir/passes/callsite_canonicalize/tests/ncl.rs
src/mir/passes/callsite_canonicalize/tests/ucm.rs
```

Decision:

```text
Keep common imports in tests/mod.rs.
Group tests by callsite canonicalization family:
  MCL / Stage1 global call compatibility
  NCL closure canonicalization
  UCM user-box method canonicalization
```

Verification:

```bash
cargo test --release --lib callsite_canonicalize -- --nocapture
cargo fmt --check
```

Acceptance:

```text
test module split only
no production behavior change
callsite canonicalization tests stay green
```

### MIR-CLEAN-015

Collapse one more pure re-export thin `mod.rs`.

Status: landed 2026-06-13.

Candidate selected:

```text
src/mir/builder/ssa/mod.rs
  -> src/mir/builder/ssa.rs
```

Reason:

```text
pure module declaration file
module path remains crate::mir::builder::ssa::{local, phi_input_contract}
no layer boundary documentation in the mod.rs itself
```

Verification:

```bash
cargo check --release --lib
cargo fmt --check
```

Acceptance:

```text
one thin mod.rs collapsed
no production behavior change
ssa module paths stay stable
no additional thin mod.rs files collapsed in the same pilot
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

Proceed with the next low-risk BoxShape cleanup in this order:

```text
1. MIR-CLEAN-016: prepare JoinIR merge ownership docs before any deep path flatten
2. MIR-CLEAN-017: split another production-adjacent large test file
3. MIR-CLEAN-018: collapse another pure re-export thin mod.rs only after classification
```

Do not start another deep flatten pilot until the next owner seam is documented.
