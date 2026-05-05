---
Status: Active
Decision: accepted
Date: 2026-05-05
Scope: verified Stage0 capsule-exit task map after the remaining-capsule audit
Related:
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - docs/development/current/main/phases/phase-29cv/P381AV-SELECTED-SET-FIRST-SLICE.md
  - docs/development/current/main/phases/phase-29cv/P381AW-SELECTED-DECLARATIONS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P381BA-REMAINING-CAPSULE-RETIRE-INVENTORY.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - src/mir/global_call_route_plan/model.rs
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BC: Stage0 Capsule Exit Task Map

## Problem

The Stage0 audit correctly shows that the C shim surface is large and that the
remaining temporary capsules should not become permanent backend semantics.

However, the optimistic reading "uniform emitter is already enough, delete the
shape branches" is stronger than the current contracts allow. Stage0 can emit a
common same-module `call i64 @"symbol"`, but several shape-specific contracts
still exist around that call:

- lowering-plan metadata predicates still check target shape/proof strings
- direct-call shell still sets result origin per shape
- selected-set planning still has shape-specific registries for parser/static
  array paths
- same-module body emission still has special cases such as parser Program(JSON)
  and static string array handling

So the next work must be task-sliced as capsule retirement, not as one broad
branch deletion.

## Verified Counts

Current worktree measurements:

| Surface | Count |
| --- | --- |
| `lang/c-abi/shims/*.inc` | 81 files / 22,706 lines |
| `tools/smokes/v2/**/*.sh` | 1,496 scripts |
| `tools/smokes/v2/profiles/integration` | 1,220 scripts |
| `tools/smokes/v2/profiles/quick` | 155 scripts |
| `tools/smokes/v2/profiles/archive` | 81 scripts |
| `tools/smokes/v2/profiles/integration/apps` | 359 scripts |

The previously quoted `src/mir` Stage0 count is not reproducible as a single
stable scope in the current tree. The closest owner-focused measurements are:

| Scope | Count |
| --- | --- |
| `src/mir/global_call_route_plan/` non-test files | 20 files / 6,828 lines |
| `src/mir/global_call_route_plan.rs` plus that non-test directory | 21 files / 7,173 lines |

Do not use `26 files / 6,969 lines` as an SSOT unless a future card defines the
exact file list.

## Capsule Reading

Current temporary capsules from the Stage0 shape inventory:

| Capsule | Reading |
| --- | --- |
| `GenericStringVoidLoggingBody` | candidate, but still has void-sentinel return-shape handling |
| `ParserProgramJsonBody` | candidate only after dedicated parser Program(JSON) body emission is replaced or made uniform |
| `StaticStringArrayBody` | candidate only after `array_handle` / `ORG_ARRAY_STRING_BIRTH` origin is expressed without a shape branch |
| `MirSchemaMapConstructorBody` | candidate only after `map_handle` / MIR schema facts are expressed without a shape branch |
| `GenericStringOrVoidSentinelBody` | source-owner cleanup first unless uniform body emission can carry the sentinel contract cleanly |
| `BoxTypeInspectorDescribeBody` | source-owner scalar predicate cleanup first |
| `PatternUtilLocalValueProbeBody` | source-owner text/scalar cleanup first |

Permanent or permanent-candidate shapes remain:

- `Unknown`: fail-fast diagnostic
- `NumericI64Leaf`: ABI primitive, possibly shrinkable later
- `GenericI64Body`: scalar/bool/i64 candidate
- `GenericPureStringBody`: string-handle candidate with shrink target

## Task Order

### T0: Stage0 Measurement Scope Lock

Docs-only task. Define the exact Stage0 Rust measurement set before using file
count/line count as a progress metric.

Status: landed in
`docs/development/current/main/phases/phase-29cv/P381BD-STAGE0-MEASUREMENT-SCOPE-LOCK.md`.

Acceptance:

```bash
find lang/c-abi/shims -maxdepth 1 -name '*.inc' -print | wc -l
wc -l lang/c-abi/shims/*.inc | tail -n 1
find tools/smokes/v2 -type f -name '*.sh' | wc -l
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

### T1: Uniform Body-Emitter Contract Inventory

Before deleting any shape branch, write the per-capsule proof table:

- target shape / proof string
- return shape
- value demand
- result origin / scan origin side effect
- selected-set planner requirement
- body-emitter special case
- required smoke or unit test

This task may update docs and tests, but must not delete a capsule.

Status: landed in
`docs/development/current/main/phases/phase-29cv/P381BE-UNIFORM-BODY-EMITTER-CONTRACT-INVENTORY.md`.

### T2: First Capsule Retirement Candidate

Pick one candidate capsule and retire only that one. The likely first probe is
`GenericStringVoidLoggingBody`, because it has no result-origin propagation, but
the void-sentinel contract still must be proven.

Status: first C-side contract shrink landed in
`docs/development/current/main/phases/phase-29cv/P381BF-VOID-LOGGING-DIRECT-CONTRACT-SHRINK.md`;
Rust-side downstream return-contract readers landed in
`docs/development/current/main/phases/phase-29cv/P381BG-GLOBAL-CALL-RETURN-CONTRACT-READERS.md`;
return-contract storage landed in
`docs/development/current/main/phases/phase-29cv/P381BH-GLOBAL-CALL-RETURN-CONTRACT-STORAGE.md`;
proof/shape JSON compatibility and final capsule removal remain separate.

Acceptance must include:

- no new `GlobalCallTargetShape`
- no new body-specific `.inc` emitter
- route proof still comes from MIR/lowering-plan facts
- selected-set call and body emission stay green
- `stage0_shape_inventory_guard.sh` remains green

### T3: Origin-Carrying Capsule Retirements

Handle origin-carrying capsules one at a time:

1. `StaticStringArrayBody`
2. `MirSchemaMapConstructorBody`
3. `ParserProgramJsonBody`

Do not remove a shape until the origin/return contract is represented without
teaching Stage0 the source-owner meaning. If that requires a MIR-owned fact, add
the fact first and keep the deletion in a later card.

### T4: Source-Owner Cleanup Capsules

Keep these as source-owner cleanup tasks before uniform emitter deletion:

1. `GenericStringOrVoidSentinelBody`
2. `BoxTypeInspectorDescribeBody`
3. `PatternUtilLocalValueProbeBody`

These are not "shape delete only" tasks. Each must remove or simplify the
source-owner plumbing that forced Stage0 to carry the temporary capsule.

### T5: `.inc` Consolidation

Only after capsule callsites are retired, reduce the large `.inc` surfaces.

Primary targets:

- `hako_llvmc_ffi_module_generic_string_function_emit.inc`
- `hako_llvmc_ffi_module_generic_string_plan.inc`
- `hako_llvmc_ffi_module_generic_string_method_views.inc`
- `hako_llvmc_ffi_mir_call_need_policy.inc`
- `hako_llvmc_ffi_mir_call_shell.inc`

This is a cleanup result, not the first move. Do not delete helper files while a
capsule still needs their contract.

### T6: Smoke Inventory Before Smoke Deletion

Smoke deletion is a separate BoxShape cleanup lane. First create an inventory
that proves suite/gate/doc reachability for:

- `profiles/archive`
- `profiles/integration/archive`
- `profiles/integration/apps/archive`
- legacy `tools/smokes` outside v2
- `tools/archive/manual-smokes`

The current `1,496` smoke count is real, but the proposed `-200` first wave is
not yet proven. Delete only after references are audited.

## Boundary

Allowed:

- one capsule per implementation card
- docs/test inventory before deletion
- MIR-owned facts that remove shape-specific backend knowledge
- selected-set uniform emitter work that stays symbol/ABI-based

Not allowed:

- broad deletion of shape predicates because call emission already has a common
  `call i64 @"symbol"` path
- adding new public route contracts just to retire a capsule count
- adding body-specific `.inc` emitters for remaining source-owner helpers
- mixing smoke deletion with uniform emitter implementation

## Acceptance

This card is task mapping only:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
