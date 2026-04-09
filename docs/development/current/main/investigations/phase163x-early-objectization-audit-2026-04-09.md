---
Status: Active Investigation
Date: 2026-04-09
Scope: early objectization audit for known user-box values and enum payloads under the lifecycle-value parent SSOT.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - src/stage1/program_json_v0/record_payload.rs
  - src/stage1/program_json_v0/lowering.rs
  - src/runner/json_v0_bridge/lowering/expr/access_ops.rs
  - src/runner/json_v0_bridge/lowering/expr/call_ops.rs
  - src/backend/mir_interpreter/handlers/sum_ops.rs
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/sum_ops.py
  - src/llvm_py/instructions/sum_runtime.py
  - src/mir/storage_class.rs
---

# Phase 163x Early Objectization Audit

## Findings

### 1. known user-box locals still enter handle/object world too early

- `src/runner/json_v0_bridge/lowering/expr/call_ops.rs`
  - `lower_new_expr()` always emits `MirInstruction::NewBox`
  - generic `lower_method_expr()` still emits `runtime_method_call(..., "RuntimeDataBox", ..., TypeCertainty::Union)`
  - this means known local user-box method paths do not yet have a thin/internal entry split.
- `src/runner/json_v0_bridge/lowering/expr/access_ops.rs`
  - canonical `FieldGet` / `FieldSet` preserve declared field types
  - but canonical MIR still does not mark a separate "known user-box local route" beyond the type hint.
- `src/llvm_py/instructions/field_access.py`
  - typed field fast paths exist for `IntegerBox` / `BoolBox` / `FloatBox`
  - they still take a handle receiver and rediscover the route backend-side
  - unknown receivers fall back to boxed `RuntimeDataBox` get/set field calls.

Classification:

- acceptable fallback for reflection / unknown receiver / ABI lanes
- real cleanup target for local known-receiver paths
- this is exactly the slice that wants thin-entry inventory first

### 2. record enum payloads objectize during Stage1 transport

- `src/stage1/program_json_v0/record_payload.rs`
  - record payloads are renamed to hidden payload boxes `__NyEnumPayload_<Enum>_<Variant>`
- `src/stage1/program_json_v0/lowering.rs`
  - record enum constructors materialize that hidden payload box through `New`
  - the resulting box is then inserted into the single payload slot of `EnumCtor`

Classification:

- compat fallback in the wrong layer from the new SSOT point of view
- acceptable only while JSON v0 remains the bridge transport
- not the next direct cleanup target unless canonical sum payload shape changes

### 3. sum values objectize again in VM/runtime fallback, and still do so on LLVM entry

- `src/backend/mir_interpreter/handlers/sum_ops.rs`
  - `handle_sum_make()` always builds `InstanceBox("__NySum_<Enum>")`
  - payload is stored through `__sum_payload`
- `src/llvm_py/instructions/sum_runtime.py`
  - synthetic runtime box declarations are auto-merged for every enum
- `src/llvm_py/instructions/sum_ops.py`
  - payload storage already distinguishes primitive payload families from generic handles
  - but `lower_sum_make()` still starts by `lower_newbox(..., runtime_box_name(enum_name), ...)`
  - so LLVM keeps a forced outer sum object even when the payload is still local and scalar-friendly.

Classification:

- VM/runtime box is acceptable as compat fallback
- unconditional LLVM-side outer sum box is the next meaningful cleanup target once thin-entry inventory exists

### 4. storage inventory already shows the right direction, but it is only inventory

- `src/mir/storage_class.rs`
  - current inventory can classify primitive field gets as inline / borrowed
  - there is still no aggregate-local storage class or explicit objectization barrier fact for sums/user-box locals

Classification:

- inventory/documentation gap, not a semantic blocker by itself
- useful evidence that the next step is route selection, not more backend-only guessing

## Acceptable Today vs Next Cleanup

Acceptable today:

- hidden payload boxes as JSON v0 / VM / LLVM compat carriers
- generic field fallback for unknown receiver / reflection / ABI paths

Next cleanup targets:

1. thin-entry inventory for known user-box field/method routes
2. thin-entry inventory for local sum values so LLVM/native can avoid the forced outer `__NySum_*` box on non-escaping paths
3. only after that, revisit whether tuple multi-payload needs a canonical sum shape change or should remain on compat-only hidden payload boxes

## Decision Input

This audit says **thin-entry inventory should come before tuple multi-payload**.

Reason:

1. the immediate waste is not payload count first; it is early handle/object entry
2. known user-box field paths and local enum/sum values already have enough type/storage evidence to justify a route-selection inventory
3. tuple multi-payload still depends on a larger canonical-sum shape decision, while thin-entry inventory can be landed as the local decision layer first
