---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: compiler pipeline thinning tasks after MIR-CLEAN-018.
Related:
  - docs/development/current/main/design/compiler-pipeline-ssot.md
  - docs/development/current/main/design/mir-cleanup-policy-ssot.md
  - docs/development/current/main/design/current-optimization-mechanisms-ssot.md
  - docs/development/current/main/design/joinir-observation-layer-ssot.md
  - docs/development/current/main/design/recipe-first-entry-contract-ssot.md
  - docs/reference/mir/metadata-facts-ssot.md
  - src/mir/semantic_refresh.rs
  - src/mir/optimizer/core.rs
  - src/mir/verification.rs
---

# Compiler Pipeline Thinning SSOT

## Decision

The compiler should become thinner by reducing visible ownership surfaces, not
by deleting semantic stages blindly.

```text
thin means:
  fewer public entry points
  named schedule stages
  one owner for each ordering contract
  no duplicated truth

thin does not mean:
  remove repeated passes without proof
  merge verifier checks and lose error precision
  move semantic truth into JoinIR
  change accepted source/MIR shapes
```

This is a BoxShape cleanup lane. It must not change route selection, optimizer
behavior, accepted CorePlan shapes, backend lowering, or perf keeper claims.

## Current Shape

The current effective pipeline is:

```text
.hako / AST
  -> MirBuilder
  -> early rune plan refresh
  -> MirOptimizer schedule
  -> pre-verification exact numeric / FastMemory count refresh
  -> MirVerifier
  -> RC insertion scaffold
  -> semantic metadata refresh
  -> callsite canonicalize
  -> semantic metadata refresh only if callsite canonicalize changed MIR
  -> backend / runner consumers
```

The cleanup target is not a single giant pass. The target is a smaller set of
named owner surfaces:

```text
MirBuilder
  -> MirOptPipeline
  -> MirVerifier
  -> SemanticRefreshPipeline
  -> Route/Plan consumers
```

## Non-Goals

```text
do not:
  remove JoinIR wholesale
  move existing refresh truth into TypeAbiCatalog
  treat hako_check as optimizer or verifier truth
  collapse DCE / memory-effect / placement-effect into one physical pass
  collapse CorePlan verifier into MIR verifier
  use this cleanup to add new accepted loop/source shapes
```

## Refresh Thinning

`src/mir/semantic_refresh.rs` is already the semantic metadata refresh SSOT.
The problem is not that refresh is completely scattered. The problem is that
the SSOT body is a long ordered list.

Read the current count as:

```text
refresh_function_semantic_metadata:
  about 50 direct helper calls

refresh_module_semantic_metadata:
  module pre-pass
  function pass
  route fixpoint
  post-fixpoint replay
  module post-pass
```

The first thinning step is private stage helpers inside
`src/mir/semantic_refresh.rs`, preserving exact order.

Recommended private stages:

```text
refresh_module_layout_and_decl_plans
refresh_function_source_and_fact_metadata
refresh_function_placement_metadata
refresh_function_pre_fixpoint_routes
refresh_module_route_convergence
refresh_function_post_fixpoint_consumers
refresh_function_experimental_seed_routes
refresh_module_contracts_and_exact_numeric
```

Order contracts:

```text
route_fixpoint:
  remains exactly once after function-local refresh

post_fixpoint_replay:
  remains after route_fixpoint so consumers observe final routes

fastmem_table_length:
  remains before fastmem access plans

module_metadata_snapshot:
  module metadata clone remains an ordering boundary

pre_verification_contracts:
  compiler-side exact numeric and FastMemory count refresh stay before verifier
  until a separate verifier-timing card proves they can move
```

## Remaining Refresh Entry Inventory

The remaining non-SSOT refresh entry points are not all bugs. Most are timing
seams where a caller mutates a small metadata subset before the full module
semantic refresh is legal or desirable.

Inventory contract:

```text
output_contract=hako-check-semantic-refresh-inventory-v0
tool_surface=hako_check_semantic_refresh_inventory
semantic_refresh_truth_source=src/mir/semantic_refresh.rs
semantic_refresh_remaining_duplicate_candidate_count=0
semantic_refresh_resolved_helper_count=1
semantic_refresh_behavior_changed=0
semantic_refresh_order_changed=0
```

Current inventory:

| id | kind | owner | status | next |
| --- | --- | --- | --- | --- |
| `compiler_pre_verification_contracts` | intentional timing seam | `src/mir/compiler/mod.rs` | keep | move only under a verifier-timing card |
| `compiler_post_rc_semantic_refresh` | canonical entry | `src/mir/compiler/mod.rs` | keep | canonical final module semantic refresh |
| `compiler_post_callsite_canonicalize_refresh` | conditional canonical entry | `src/mir/compiler/mod.rs` | keep | required after MIR mutation by callsite canonicalize |
| `builder_decl_layout_timing` | intentional timing seam | `src/mir/builder/module_lifecycle.rs` | keep | declaration-derived subset before function-local metadata |
| `json_v0_decl_layout_timing` | intentional timing seam | `src/runner/json_v0_bridge/lowering.rs` | keep | already uses `refresh_module_record_and_packed_layout_plans` |
| `json_v0_post_canonicalize_metadata_subset` | resolved helper | `src/runner/json_v0_bridge/core.rs` | resolved | owned by `refresh_module_json_v0_post_canonicalize_metadata` |
| `rune_immediate_attr_refresh` | intentional timing seam | builder / JSON v0 bridge / optimizer | keep | rune attrs mutate before full module refresh; inline consumes fresh plans |
| `string_corridor_local_mutation_refresh` | intentional local mutation seam | `src/mir/passes/string_corridor_sink/*` | keep | pass mutates one function and refreshes only affected metadata |

Next implementation order:

```text
SEMREFRESH-THIN-003:
  add read-only semantic-refresh-inventory report and guard

SEMREFRESH-THIN-004:
  landed: extract refresh_module_json_v0_post_canonicalize_metadata
  keep the bridge subset exact; full semantic refresh remains a separate
  bridge timing proof if ever needed

SEMREFRESH-THIN-005:
  revisit rune_immediate_attr_refresh only after inline/rune plan consumers have
  a narrower invalidation contract
```

## Optimizer Thinning

`src/mir/optimizer/core.rs` has more schedule entries than the visible compiler
should expose, but many entries are facades, no-op scaffolds, optional gates, or
diagnostics.

The safe target is a facade schedule, not physical pass deletion:

```text
normalize_frontend_surface:
  legacy normalization
  optional ref-field normalization
  Python helper normalization

placement_effect_pre:
  pre-DCE placement/effect transform

canonical_simplification:
  simplify_cfg
  DCE
  CSE

memory_cleanup_wave:
  memory_effect
  pure DCE cleanup rerun

placement_effect_post:
  post-DCE placement/effect transform

late_call_and_inline:
  boxfield optimization
  callsite canonicalization
  rune plan refresh
  inline soft leaf

optional_and_diagnostics:
  optional concat3
  optional Core-13 pure normalization
  diagnostics
```

Do not merge these without a separate optimizer semantics card:

```text
DCE with memory-effect
pre/post placement-effect
string_corridor_sink internal phases
Core-13 pure normalization into early normalization
CSE with DCE as if CSE were full SSA rewrite
```

## Verification Thinning

`src/mir/verification.rs` is already the MIR verifier entry. The next thinning
step is a named dashboard / grouped helper surface, not fewer checks.

Recommended visible groups:

```text
module_contracts:
  exact numeric field assignments
  module metadata invariants
  hako_alloc metadata
  hako_alloc page lifecycle

core_graph:
  SSA
  dominance
  CFG
  PHI predecessor coverage
  merge-block value use

runtime_safety:
  WeakRef / Barrier
  await checkpoints
  legacy-op rejection

semantic_contracts:
  string kernel plans
  rune contracts
  required inline plans
  FastMemory regions

optional_dev:
  PHI-off edge-copy strict
  return-block purity
```

Keep error precision. Do not combine unrelated checks if that hides the
diagnostic owner.

## JoinIR Boundary

JoinIR is not removed in this lane.

Current rule:

```text
JoinIR:
  observation / structure support

Facts -> Recipe -> Verifier -> Lower:
  semantic compiler path
```

Safe thinning:

```text
allowed:
  document which JoinIR paths are observation-only
  fence new Recipe/Verifier/Lower rules away from src/mir/join_ir/**
  inventory test-only or legacy JoinIR lowering facades
  retire proven-unused experiment hooks with gates

forbidden:
  delete JoinIR core loop route support
  make JoinIR the acceptance truth
  add new lowering heuristics under JoinIR observation modules
```

## Task Ladder

### COMPILER-THIN-000

Add this SSOT and link it from current docs.

Acceptance:

```text
docs-only
BoxShape-only
refresh / optimizer / verifier / JoinIR thinning boundaries documented
no code behavior change
```

### SEMREFRESH-THIN-001

Split `src/mir/semantic_refresh.rs` into private stage helpers.

Acceptance:

```text
public entry points unchanged
helper call order unchanged
route_fixpoint still runs once
post-fixpoint replay preserved
cargo test -q mir::semantic_refresh:: --lib
```

### SEMREFRESH-THIN-002

Redirect duplicated builder / bridge layout refresh seams to a shared refresh
helper only after SEMREFRESH-THIN-001 lands.

Acceptance:

```text
no verifier timing change
bridge refresh still calls full semantic refresh
JSON v0 bridge behavior unchanged
targeted bridge tests green
```

### SEMREFRESH-THIN-003

Inventory remaining semantic refresh duplicate entry seams.

Acceptance:

```text
output_contract=hako-check-semantic-refresh-inventory-v0
json_v0_post_canonicalize_metadata_subset is classified
intentional timing seams are documented
behavior/order change counters remain 0
```

### SEMREFRESH-THIN-004

Move the JSON v0 post-canonicalize metadata subset behind a semantic refresh
helper.

Acceptance:

```text
refresh_module_json_v0_post_canonicalize_metadata exists
src/runner/json_v0_bridge/core.rs does not directly call the subset helpers
semantic_refresh_remaining_duplicate_candidate_count=0
json_v0_bridge targeted tests green
```

### OPT-THIN-001

Introduce a `MirOptPipeline` facade schedule while preserving the exact current
subpass order.

Acceptance:

```text
optimizer behavior unchanged
seven visible schedule groups exist
pass-order test locks critical ordering
no DCE / placement / memory-effect physical merge
```

### OPT-THIN-002

Classify optimizer no-op scaffolds and reserved hooks.

Acceptance:

```text
reorder / intrinsics / type_hints / ref-field normalization classified
default visible schedule no longer reads like active behavior where it is not
no scaffold deletion without usage gate
```

### OPT-THIN-003

Expose the visible optimizer schedule through hako_check/report as a read-only
explanation surface.

Acceptance:

```text
output_contract=hako-check-optimizer-schedule-v0
hako_check optimizer-schedule reports seven visible groups
truth source is src/mir/optimizer/core.rs::MIR_OPT_PIPELINE_GROUPS
hako_check_optimizer_truth_count=0
optimizer_behavior_changed=0
optimizer_physical_pass_merge_count=0
keeper_selection=0
```

### VERIFY-THIN-001

Add a verifier boundary table and optionally split `verify_function` into
private grouped helpers while preserving checks.

Acceptance:

```text
check list unchanged
error types unchanged
optional/dev gates unchanged
diagnostic precision preserved
```

### JOINIR-THIN-001

Inventory JoinIR observation vs legacy lowering surfaces.

Acceptance:

```text
JoinIR observation role documented
Recipe/Verifier/Lower ownership remains outside JoinIR observation modules
test-only / legacy facades have retire conditions
no JoinIR core route removal
```

### JOINIR-THIN-002

Retire one proven-unused JoinIR experiment hook.

Acceptance:

```text
usage inventory names the hook
targeted joinir / recipe-first gate remains green
no source acceptance change
```

## Reading Rule

When a future cleanup proposal says "reduce pass count", first classify it as
one of:

```text
facade:
  visible schedule cleanup, behavior unchanged

duplicate owner:
  same responsibility exposed from multiple entry points

behavior merge:
  semantic optimizer/verifier behavior changes
```

Only `facade` and proven `duplicate owner` work belong in this lane. `behavior
merge` requires a separate accepted card with fixtures and gates.
