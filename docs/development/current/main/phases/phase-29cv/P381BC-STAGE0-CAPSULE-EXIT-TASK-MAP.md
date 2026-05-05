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

Retired capsule contracts:

| Capsule | Result |
| --- | --- |
| `GenericStringVoidLoggingBody` | retired as a shape in P381BJ; direct ABI truth now lives in `proof=typed_global_call_generic_string_void_logging` plus `return_shape=void_sentinel_i64_zero` |
| `StaticStringArrayBody` | retired as a shape in P381BL; direct ABI truth now lives in `proof=typed_global_call_static_string_array` plus `return_shape=array_handle` |
| `MirSchemaMapConstructorBody` | retired as a shape in P381BM; direct ABI truth now lives in `proof=typed_global_call_mir_schema_map_constructor` plus `return_shape=map_handle` |
| `ParserProgramJsonBody` | retired as a shape in P381BN; direct ABI truth now lives in `proof=typed_global_call_parser_program_json` plus `return_shape=string_handle`; dedicated body emission remains a later cleanup |
| `BoxTypeInspectorDescribeBody` | retired as a shape in P381BO; direct ABI truth now lives in `proof=typed_global_call_box_type_inspector_describe` plus `return_shape=map_handle`; active source-owner callers already use scalar predicates |
| `PatternUtilLocalValueProbeBody` | retired as a shape in P381BP; direct ABI truth now lives in `proof=typed_global_call_pattern_util_local_value_probe` plus `return_shape=mixed_runtime_i64_or_handle`; child-probe recognition uses proof/return facts |
| `GenericStringOrVoidSentinelBody` | retired as a shape in P381BQ; direct ABI truth now lives in `proof=typed_global_call_generic_string_or_void_sentinel` plus `return_shape=string_handle_or_null`; generic-method string-origin consumers use proof/return facts |

No temporary capsule remains in the Stage0 shape inventory.

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

Pick one candidate capsule and retire only that one. The first retired capsule
is `GenericStringVoidLoggingBody`: it had no result-origin propagation, and the
void-sentinel contract is now represented as stored MIR proof/return facts.

Status: first C-side contract shrink landed in
`docs/development/current/main/phases/phase-29cv/P381BF-VOID-LOGGING-DIRECT-CONTRACT-SHRINK.md`;
Rust-side downstream return-contract readers landed in
`docs/development/current/main/phases/phase-29cv/P381BG-GLOBAL-CALL-RETURN-CONTRACT-READERS.md`;
return-contract storage landed in
`docs/development/current/main/phases/phase-29cv/P381BH-GLOBAL-CALL-RETURN-CONTRACT-STORAGE.md`;
proof storage landed in
`docs/development/current/main/phases/phase-29cv/P381BI-GLOBAL-CALL-PROOF-CONTRACT-STORAGE.md`;
target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BJ-VOID-LOGGING-TARGET-SHAPE-RETIRE.md`;
the first static-array Rust consumer decoupling landed in
`docs/development/current/main/phases/phase-29cv/P381BK-STATIC-ARRAY-CONSUMER-CONTRACT-READ.md`;
static-array target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BL-STATIC-ARRAY-TARGET-SHAPE-RETIRE.md`;
MIR-schema map target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BM-MIR-SCHEMA-MAP-TARGET-SHAPE-RETIRE.md`;
Parser Program(JSON) target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BN-PARSER-PROGRAM-JSON-TARGET-SHAPE-RETIRE.md`;
BoxTypeInspector describe target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BO-BOX-TYPE-INSPECTOR-DESCRIBE-TARGET-SHAPE-RETIRE.md`;
PatternUtil local-value probe target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BP-PATTERN-UTIL-LOCAL-VALUE-PROBE-TARGET-SHAPE-RETIRE.md`;
generic string-or-void sentinel target-shape retirement landed in
`docs/development/current/main/phases/phase-29cv/P381BQ-GENERIC-STRING-OR-VOID-SENTINEL-TARGET-SHAPE-RETIRE.md`.

Acceptance must include:

- no new `GlobalCallTargetShape`
- no new body-specific `.inc` emitter
- route proof still comes from MIR/lowering-plan facts
- selected-set call and body emission stay green
- `stage0_shape_inventory_guard.sh` remains green

### T3: Origin-Carrying Capsule Retirements

Status: target-shape retirements landed for `MirSchemaMapConstructorBody`,
`ParserProgramJsonBody`, `BoxTypeInspectorDescribeBody`, and
`PatternUtilLocalValueProbeBody`. Parser Program(JSON), BoxTypeInspector
describe, and PatternUtil local-value probe still have dedicated body handling
that belongs to T5/uniform-emitter cleanup, not another target-shape variant.


Do not remove a shape until the origin/return contract is represented without
teaching Stage0 the source-owner meaning. If that requires a MIR-owned fact, add
the fact first and keep the deletion in a later card.

### T4: Source-Owner / Body Cleanup Follow-Ups

No target-shape capsule remains. Remaining cleanup for generic string-or-void
sentinel plumbing and PatternUtil local-value probe body handling must proceed
as source-owner/body cleanup or uniform-emitter cleanup, not as another public
shape.

### T5: `.inc` Consolidation

Only after capsule callsites are retired, reduce the large `.inc` surfaces.

Status: first selected-set consolidation landed in
`docs/development/current/main/phases/phase-29cv/P381BR-MODULE-GENERIC-SELECTED-KIND-REGISTRY.md`.
Parser Program(JSON) and static string array no longer have parallel planned
symbol arrays; their direct-contract identity is a kind on the unified planned
generic symbol registry. Their specialized body handling still remains for a
later uniform-emitter cleanup.

Failure-first probe landed in
`docs/development/current/main/phases/phase-29cv/P381BS-PARSER-PROGRAM-JSON-BODY-EMITTER-BLOCKER.md`:
parser Program(JSON) dedicated body-emitter deletion is blocked by the live
two-argument `BuildBox._parse_program_json/2` owner body and must wait for
source-owner cleanup or a matching MIR-owned two-argument body contract.

Array-push body cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BT-MODULE-GENERIC-ARRAY-PUSH-HELPER-CLEANUP.md`:
the generic array-push route and static string array body path now share push
argument decoding and array-string origin promotion helpers.

Array append emit cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BU-MODULE-GENERIC-ARRAY-APPEND-EMIT-HELPER-CLEANUP.md`:
the direct array-push route and static string array body path now share the
LLVM `nyash.array.slot_append_hh` emission helper while keeping their acceptance
guards separate.

String needle call cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BV-MODULE-GENERIC-STRING-NEEDLE-CALL-HELPER-CLEANUP.md`:
`indexOf`, `lastIndexOf`, and `contains` now share one-argument string needle
call emission while each route keeps its own LoweringPlan predicate.

Get helper-symbol cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BW-MODULE-GENERIC-GET-HELPER-SYMBOL-CLEANUP.md`:
the get emitter now computes the array-load classification and helper symbol
once before the `dst`/non-`dst` emission split.

Len helper-symbol cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BX-MODULE-GENERIC-LEN-HELPER-SYMBOL-CLEANUP.md`:
the len emitter now computes the string/array helper symbol once after route
acceptance and reuses it across the `dst`/non-`dst` emission split.

I64 call emit helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BY-MODULE-GENERIC-I64-CALL-EMIT-HELPER-CLEANUP.md`:
array push, string needle calls, len, substring, get, keys, and map set now
share optional-`dst` i64 call emission while keeping route-specific metadata
publication in each emitter.

Birth call emit helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381BZ-MODULE-GENERIC-BIRTH-CALL-EMIT-HELPER-CLEANUP.md`:
ArrayBox and MapBox newbox emission now shares the no-argument i64 birth call
helper while keeping type acceptance in `module_generic_string_emit_newbox`.

MIR call extern-view cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CP-MIR-CALL-EXTERN-VIEW-HELPER.md`:
env, hostbridge, and Stage1 extern route validators now share one exact
LoweringPlan view helper, and the current Stage1 env MIR/OBJ/EXE contract probe
is green.

MIR call extern-need cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CQ-MIR-CALL-EXTERN-NEED-CONTRACT-CLEANUP.md`:
extern need-policy rows now carry only route-specific mapping facts while the
shared runtime-call extern registry contract is checked once in the matcher.

Generic-method route tuple cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CR-GENERIC-METHOD-ROUTE-TUPLE-HELPER.md`:
route, emit-kind, need, and set-route consumers now share the route/core/kind/tier
tuple comparison and keep only their extra local checks.

Module-generic ownerless view cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CS-MODULE-GENERIC-OWNERLESS-VIEW-HELPER.md`:
MIR JSON array item, map field, and flags keys method-view predicates now share
one ownerless-view helper while route-specific proof/key checks stay local.

Module-generic prepass view reuse landed in
`docs/development/current/main/phases/phase-29cv/P381CT-MODULE-GENERIC-PREPASS-VIEW-REUSE.md`:
the call prepass now reads each LoweringPlan view once per call instruction and
feeds the cached views into the existing predicate chain.

Module-generic method dispatch view reuse landed in
`docs/development/current/main/phases/phase-29cv/P381CU-MODULE-GENERIC-METHOD-DISPATCH-VIEW-REUSE.md`:
the body emitter now reads one generic-method LoweringPlan view per method call
site and passes it through the method-specific emitters.

Module-generic origin publish cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CV-MODULE-GENERIC-ORIGIN-PUBLISH-HELPER.md`:
paired origin/scan-origin result publication now goes through one local helper
while origin-only updates stay explicit.

Module-generic i64 origin publish cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CW-MODULE-GENERIC-I64-ORIGIN-PUBLISH-HELPER.md`:
i64 type plus origin publication now has one helper, leaving scalar-only and
origin-only updates visible.

Module-generic MIR JSON key origin cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CX-MODULE-GENERIC-MIR-JSON-KEY-ORIGIN-HELPERS.md`:
inst/function/module field-key origin publication now shares one helper surface
between prepass and the actual `get` emitter.

Module-generic get route view cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CY-MODULE-GENERIC-GET-ROUTE-VIEW.md`:
the actual `get` emitter now consumes one route view for acceptance, helper
selection, and result-origin follow-up.

Module-generic prepass get route view cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381CZ-MODULE-GENERIC-PREPASS-GET-ROUTE-VIEW.md`:
the call prepass now reuses the same `get` route view while keeping numeric
value-field origin behavior explicit.

Module-generic prepass method helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DA-MODULE-GENERIC-PREPASS-METHOD-HELPER.md`:
generic-method prepass facts now go through one helper with accepted/no-match/
malformed outcomes.

Module-generic method emit dispatch cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DB-MODULE-GENERIC-METHOD-EMIT-DISPATCH-HELPER.md`:
generic-method emission now goes through one helper after method birth and
LoweringPlan view loading.

Module-generic method birth helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DC-MODULE-GENERIC-METHOD-BIRTH-HELPER.md`:
ArrayBox/MapBox method-birth recognition now has one helper shared by prepass
and emit handling.

Module-generic global emit helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DD-MODULE-GENERIC-GLOBAL-EMIT-HELPER.md`:
global-call emission now goes through one helper while preserving the
LoweringPlan-first fallback order.

Module-generic method emit helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DE-MODULE-GENERIC-METHOD-EMIT-HELPER.md`:
Method branch internals now live behind one helper after the main call-family
dispatch.

Module-generic prepass global helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DF-MODULE-GENERIC-PREPASS-GLOBAL-HELPER.md`:
global-call prepass facts now go through one helper after method and extern
family checks.

Module-generic prepass call-instruction helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DG-MODULE-GENERIC-PREPASS-CALL-INSTRUCTION-HELPER.md`:
call-instruction payload/view reads and family helper ordering now have one
prepass entry.

Module-generic emit call-instruction helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DH-MODULE-GENERIC-EMIT-CALL-INSTRUCTION-HELPER.md`:
emit-side `mir_call`/`call`/`boxcall` handling now uses the same
call-instruction helper vocabulary as prepass.

Module-generic emit externcall helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DI-MODULE-GENERIC-EMIT-EXTERNCALL-HELPER.md`:
emit-side `externcall` lowering-plan dispatch now has one active-body helper.

Module-generic emit ret helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DJ-MODULE-GENERIC-EMIT-RET-HELPER.md`:
emit-side `ret` value-reference and zext handling now has one active-body
helper.

Module-generic emit const helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DK-MODULE-GENERIC-EMIT-CONST-HELPER.md`:
emit-side `const` classification, publication, and diagnostics now have one
active-body helper.

Module-generic emit control terminator helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DL-MODULE-GENERIC-EMIT-CONTROL-TERMINATOR-HELPERS.md`:
emit-side `branch` and `jump` terminators now have active-body helper entries.

Module-generic emit instruction dispatcher cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DM-MODULE-GENERIC-EMIT-INSTRUCTION-DISPATCH.md`:
active-body emission now delegates per-instruction behavior through one
dispatcher.

Module-generic prepass instruction dispatcher cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DN-MODULE-GENERIC-PREPASS-INSTRUCTION-DISPATCH.md`:
active-function prepass now delegates per-instruction fact collection through
one dispatcher.

Module-generic function signature helper cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DO-MODULE-GENERIC-FUNCTION-SIGNATURE-HELPER.md`:
function definition emission now delegates LLVM signature text through one
helper.

Module-generic function context snapshot cleanup landed in
`docs/development/current/main/phases/phase-29cv/P381DP-MODULE-GENERIC-FUNCTION-CONTEXT-SNAPSHOT.md`:
function context save and restore now use one snapshot contract.

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
