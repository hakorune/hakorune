---
Status: SSOT
Decision: accepted
Date: 2026-07-30
Scope: JoinIR target-specific lowerer thinning after compiler pipeline thinning.
Related:
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - src/mir/join_ir/README.md
  - src/mir/join_ir/lowering/README.md
  - src/mir/join_ir/lowering/common/README.md
---

# JoinIR Target Lowerer Thinning SSOT

## Decision

Target-specific JoinIR lowerers are active behavior owners. Thin them by
separating seams, not by deleting route-specific code or merging behavior.

```text
thin target lowerers by:
  naming shared seams
  keeping route truth in active route owners
  reducing duplicate dispatch/logging scaffolds

do not thin by:
  deleting target-specific lowerer evidence
  moving loop-target classification into VM dispatch
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
src/mir/join_ir/lowering/generic_case_a/*
```

Current shared seams:

```text
common/cfg_shape.rs:
  CFG and instruction probes

common/dispatch.rs:
  MIR-based vs handwritten lowering dispatch

common/string_whitespace.rs:
  trim-style whitespace predicate instruction sequence

common/type_hint.rs:
  retained shared type-hint extraction

common/case_a.rs:
  minimal Case-A shape guard

loop_target_policy.rs:
  neutral five-name Loop/If/strict classification
```

## Ownership Rules

### Route Truth

Route truth stays with active route owners:

```text
Loop target registration:
  lowering/loop_target_policy.rs

VM execution registration:
  JOINIR_VM_EXEC_TARGETS

Condition expression lowering:
  native MirBuilder If/PHI owner

Case-A lowering:
  generic_case_a/*
```

Target-specific files may orchestrate their route, but they must not become a
second policy source for accepted shapes.

### Retired LowerOnly Observation

The former Stage1/StageB `LowerOnly` VM rows were observation-only:

```text
explicit VM bridge
-> target-specific lowering observation
-> unconditional ordinary VM continuation
```

`JOINMODULE-VM-LOWERONLY-OBSERVATION0-REOWN-RET0` retires that dispatch
surface. `JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0` then
retires the three caller-zero target lowerers and their exclusive evidence.
The neutral five-name Loop/If policy remains active; only skip/trim retain VM
Exec routes.

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

Retired:

```text
JOINMODULE-VM-LOWERONLY-OBSERVATION0-REOWN-RET0:
  five-name classification -> neutral loop_target_policy
  lower_only_routes.rs -> deleted
  VM target table -> two Exec rows only
```

Preserved:

```text
keep Exec success/output handling in exec_routes.rs
keep Stage1/StageB lowerers and direct tests
```

### JOINIR-TARGET-THIN-003: Target Lowerer Route Inventory

Status: landed as read-only inventory.

Before moving code between target lowerers, keep the route inventory explicit.
The inventory follows the neutral five-name loop-target policy; VM execution
registration is the separate two-row `JOINIR_VM_EXEC_TARGETS`.

#### `Main.skip/1`

```text
bridge_kind: Exec
lowerer_entry: lower_skip_ws_to_joinir
route_shape:
  if lower_generic_enabled:
    try try_lower_skip_ws_generic_case_a first
  else:
    dispatch through dispatch_lowering to MIR-probed or handwritten builder
fallback:
  generic failure -> existing dispatcher
  MIR-probe failure -> handwritten builder
execution:
  VM bridge may execute and produce output
```

#### `FuncScannerBox.trim/1`

```text
bridge_kind: Exec
lowerer_entry: lower_funcscanner_trim_to_joinir
route_shape:
  dispatch through dispatch_lowering
  MIR path checks entry CFG / string patterns
  optional generic Case-A through LoopToJoinLowerer
  otherwise shared handwritten builder
fallback:
  MIR-probe or generic failure -> handwritten builder
execution:
  VM bridge may execute and produce output
```

#### Generic append-defs ArrayAccumulation asset

```text
status: retired by GENERIC-CASE-A-APPEND-DEFS-RET0
reason:
  no shape issuer or source/MIR path selected its three-input synthetic recipe
non-claim:
  FuncScannerHelpersBox._append_defs/2 remains a normal-MIR source helper;
  future rehome requires a separately selected source-aware owner
```

#### `Stage1UsingResolverBox.resolve_for_source/5`

```text
status: retired by JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0
reason: no production or retained explicit-VM caller after LowerOnly observation retirement
```

#### `StageBBodyExtractorBox.build_body_src/2`

```text
status: retired by JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0
reason: no production or retained explicit-VM caller after LowerOnly observation retirement
```

#### `StageBFuncScannerBox.scan_all_boxes/1`

```text
status: retired by JOINMODULE-FORMER-LOWERONLY-TARGET-LOWERERS-RETIRE0-RET0
reason: no production or retained explicit-VM caller after LowerOnly observation retirement
```

Shared repetition visible after this inventory:

```text
find target function
probe entry CFG shape
optionally construct LoopForm
optionally try LoopToJoinLowerer / generic Case-A
fall back to route-local handwritten builder
```

This repeated shape is only a candidate seam. It does not justify merging route
truth or reviving the retired VM observation surface.

Acceptance:

```text
inventory_only=1
behavior_changed=0
accepted_shape_added=0
append_defs_loweronly_without_exec_route=0
```

### JOINIR-TARGET-THIN-004: Shared Target Adapter

Status: landed for the thin common subset.

Only after `JOINIR-TARGET-THIN-003`, a shared adapter may cover the repeated
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

Landed seam:

```text
common/target_adapter.rs:
  try_generic_case_a_route
  owns:
    lower_generic_enabled gate
    simple LoopForm construction
    Case-A guard call
    generic-hook debug lines
  does not own:
    target selection
    entry_is_preheader / has_break policy
    route-specific LoopToJoinLowerer entrypoint
    handwritten fallback
    VM execution policy
```

Current users:

```text
trim:
  entry_is_preheader=true
  has_break=true
  route entrypoint=lower_case_a_for_trim
  note: string-pattern CFG checks stay route-local before the adapter call

skip_ws:
  has a target-local minimal LoopForm construction path.
```

Closeout decision:

```text
JOINIR-TARGET-THIN-004 is complete when:
  trim and Stage-B target-local generic Case-A hooks share the adapter
  stage1_using_resolver remains route-local
  skip_ws remains route-local

do not extend the adapter with:
  params_len guard policy
  route-specific constructed-LoopForm dumps
  not-simple debug branch policy
  manually assembled LoopForm intake
```

Rationale:

```text
stage1_using_resolver:
  moving it would require adapter options for params_len and diagnostics.
  That makes the shared helper a policy shelf instead of a seam.

skip_ws:
  uses a target-local minimal LoopForm construction path.
  Moving it would require a second intake mode and hide the canary route shape.
```

Next cleanup must start from `JOINIR-TARGET-THIN-005`, not by widening
`target_adapter.rs`.

### JOINIR-TARGET-THIN-005: Route-Specific File Size Pass

Status: started with route-local facade split.

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

Landed split:

```text
funcscanner_trim.rs:
  route facade only
  owns public lower_funcscanner_trim_to_joinir entry

funcscanner_trim/builder.rs:
  route-local handwritten/shared JoinIR construction

funcscanner_trim/dispatch.rs:
  MIR-vs-handwritten dispatch and route-local sanity checks

funcscanner_trim/skip_leading.rs:
  route-local skip_leading JoinIR function construction

common/string_whitespace.rs:
  shared whitespace predicate instruction builder for trim-style lowerers
  used by handwritten trim and generic Case-A trim
  does not own route acceptance or fallback policy

skip_ws.rs:
  route facade only
  owns public lower_skip_ws_to_joinir entry
  preserves generic-first route order

skip_ws/builder.rs:
  route-local handwritten/shared JoinIR construction

skip_ws/dispatch.rs:
  MIR-vs-handwritten dispatch

skip_ws/generic_probe.rs:
  route-local minimal LoopForm canary
  not routed through common/target_adapter.rs

Former LowerOnly-only Stage1/mode-B target lowerers:
  retired atomically with their exclusive builders, dispatchers, fixtures,
  Case-A entrypoints, and ValueId ranges
```

The skip/trim split remains physical packaging. The former LowerOnly-only
Stage1/mode-B packages are retired assets: their Case-A entrypoints and route
selection no longer exist, while skip/trim Exec behavior is unchanged.

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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Run broader JoinIR tests only when a slice touches frontend/lowering behavior
beyond docs or common scaffolding.
