---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: JoinIR target-specific lowerer thinning after compiler pipeline thinning.
Related:
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - src/mir/join_ir/README.md
  - src/mir/join_ir/lowering/README.md
  - src/mir/join_ir/lowering/common/README.md
  - src/mir/join_ir_vm_bridge_dispatch/README.md
---

# JoinIR Target Lowerer Thinning SSOT

## Decision

Target-specific JoinIR lowerers are active behavior owners. Thin them by
separating seams, not by deleting route-specific code or merging behavior.

```text
thin target lowerers by:
  naming shared seams
  keeping route truth in active route owners
  keeping observation routes observation-only
  reducing duplicate dispatch/logging scaffolds

do not thin by:
  removing Stage1/StageB LowerOnly rows
  merging Exec and LowerOnly behavior
  moving accepted source-shape decisions into target lowerers
  making dry-run/observation surfaces mutate MIR
```

This is a BoxShape lane. It must not add accepted loop/if/source shapes.

## Current Target Lowerer Surfaces

Active target-specific lowerers:

```text
src/mir/join_ir/lowering/skip_ws.rs
src/mir/join_ir/lowering/funcscanner_trim.rs
src/mir/join_ir/lowering/funcscanner_trim/dispatch.rs
src/mir/join_ir/lowering/stage1_using_resolver.rs
src/mir/join_ir/lowering/stageb_body.rs
src/mir/join_ir/lowering/stageb_funcscanner.rs
src/mir/join_ir/lowering/generic_case_a/*
```

Current shared seams:

```text
common/cfg_shape.rs:
  CFG and instruction probes

common/dispatch.rs:
  MIR-based vs handwritten lowering dispatch

common/type_hint.rs:
  IfSelect / IfMerge type hint extraction

common/case_a.rs:
  minimal Case-A shape guard

join_ir_vm_bridge_dispatch/lower_only_routes.rs:
  LowerOnly structural-observation route helper
```

## Ownership Rules

### Route Truth

Route truth stays with active route owners:

```text
Loop target registration:
  JOINIR_TARGETS

If target registration:
  JOINIR_IF_TARGETS and prefix policy helpers

Condition expression lowering:
  if_lowering_router / condition_lowerer / ExprLowerer

Case-A lowering:
  generic_case_a/*
```

Target-specific files may orchestrate their route, but they must not become a
second policy source for accepted shapes.

### LowerOnly

`LowerOnly` rows are live observation rows:

```text
LowerOnly:
  may run structural lowering observation
  must return to normal VM Route A
  must not handle output / exit
  must not be interpreted as execution failure under strict mode
```

Do not delete Stage1/StageB LowerOnly rows while they are needed for structural
lowering probes.

### Dry-Run / Observation

Observation surfaces such as `if_dry_runner`:

```text
may:
  scan and report
  count coverage
  call active route lowerers read-only

must not:
  mutate MIR
  select routes
  become If lowering truth
```

## Implementation Order

### JOINIR-TARGET-THIN-000: SSOT

This document. Fix the order and non-goals before changing more behavior-near
target lowerers.

### JOINIR-TARGET-THIN-001: Common Seam Classification

Landed in the compiler-thin cleanup burst:

```text
common.rs -> stable facade
common/cfg_shape.rs
common/dispatch.rs
common/type_hint.rs
```

Next actions:

```text
keep common.rs facade-only
do not add new mixed helper logic to common.rs
```

### JOINIR-TARGET-THIN-002: LowerOnly Observation Helper

Landed:

```text
lower_only_routes.rs:
  Stage1/StageB routes share one observation helper
  all LowerOnly routes return false to normal VM Route A
```

Next actions:

```text
keep Exec success/output handling in exec_routes.rs
keep LowerOnly observation in lower_only_routes.rs
```

### JOINIR-TARGET-THIN-003: Target Lowerer Route Inventory

Before moving code between target lowerers, write a read-only inventory for each
target:

```text
skip_ws:
  Exec target
  generic_case_a first, then existing dispatcher

funcscanner_trim:
  Exec target
  MIR shape probe, generic_case_a, handwritten fallback

stage1_using_resolver:
  LowerOnly target
  structural observation, normal VM execution

stageb_body:
  LowerOnly target
  structural observation, normal VM execution

stageb_funcscanner:
  LowerOnly target
  structural observation, normal VM execution
```

Acceptance:

```text
inventory_only=1
behavior_changed=0
accepted_shape_added=0
```

### JOINIR-TARGET-THIN-004: Candidate Shared Target Adapter

Only after `JOINIR-TARGET-THIN-003`, consider a shared adapter for the repeated
target pattern:

```text
find target function
probe CFG shape
construct LoopForm
check Case-A guard
call route-specific generic_case_a lowerer
fallback to handwritten route
```

This adapter must not own route policy. It may only orchestrate already-decided
target-local steps.

### JOINIR-TARGET-THIN-005: Route-Specific File Size Pass

After adapter proof, reduce large files only where behavior remains readable.

```text
prefer:
  route/local helpers inside route modules
  small route-specific submodules

avoid:
  one giant generic target lowerer
  name-based target dispatch outside target registry
  helper internals that hide fallback policy
```

## Guard Vocabulary

Use report/check vocabulary when adding a guard:

```text
joinir_target_lowerer_thinning_mode=boxshape
joinir_target_common_facade_only=1
joinir_loweronly_observation_only=1
joinir_loweronly_exec_count=0
joinir_observation_mir_mutation_count=0
joinir_target_shape_added_count=0
joinir_target_behavior_changed=0
```

## Proof Commands

Targeted checks for this lane:

```bash
cargo test -q mir::join_ir::lowering --lib
cargo test -q mir::join_ir_vm_bridge_dispatch --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Run broader JoinIR tests only when a slice touches frontend/lowering behavior
beyond docs or common scaffolding.
