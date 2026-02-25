# Phase 196: Loop Continue Multi-Carrier Support

**Phase**: 196
**Status**: Infrastructure Complete（multi-carrier実行は通るが、更新式は次フェーズで改善予定）
**Date**: 2025-12-06
**Prerequisite**: Phase 193 (CarrierInfo/ExitMeta/ExitBindingBuilder infrastructure)
**Goal**: Enable Pattern 4 (loop with continue) to correctly handle multiple carrier variables

---

## Executive Summary

Phase 196 fixes the multi-carrier support gap in Pattern 4 loop lowering. Currently, loops with a single carrier (like "sum") work correctly, but loops with multiple carriers (like "sum" + "count") crash due to hardcoded single-carrier assumptions in the JoinIR lowerer.

---

## Current Behavior

### Single Carrier: PASS

**Test**: `apps/tests/loop_continue_pattern4.hako`
```hako
loop(i < 10) {
  i = i + 1
  if (i % 2 == 0) { continue }
  sum = sum + i
}
// Result: sum = 25 (1+3+5+7+9)
```

**Status**: Works correctly with ExitMeta::single("sum", ...)

---

### Multiple Carriers: FAIL

**Test**: `apps/tests/loop_continue_multi_carrier.hako`
```hako
loop(i < 10) {
  i = i + 1
  if (i % 2 == 0) { continue }
  sum = sum + i
  count = count + 1
}
// Expected (ideal): sum = 25, count = 5
// Phase 196 完了時点: 実行はクラッシュせず、出力は 5, 5（インフラは動くが更新式は暫定）
```

**Error（Phase 196 着手前）**:
```
assertion `left == right` failed: join_inputs and host_inputs must have same length
  left: 2
 right: 3
```

Phase 196 完了後はこのアサートは発生せず、多キャリアでも実行自体は通るようになった（ただし sum の更新式はまだ暫定で、意味論は今後のフェーズで修正する予定）。

### Task 196-2: Extend JoinIR Lowerer ExitMeta

**Files to modify**:
- `src/mir/join_ir/lowering/loop_with_continue_minimal.rs`

**Changes**:
1. Accept carrier information (from CarrierInfo or extracted from scope)
2. Allocate ValueIds for ALL carriers (not just "sum")
3. Generate JoinIR Select logic for each carrier
4. Return `ExitMeta::multiple(...)` with all carrier exit values

**Example change**:
```rust
// Before:
let exit_meta = ExitMeta::single("sum".to_string(), sum_exit);

// After:
let mut exit_values = Vec::new();
for carrier in &carriers {
    let carrier_exit = ...; // compute exit ValueId for this carrier
    exit_values.push((carrier.name.clone(), carrier_exit));
}
let exit_meta = ExitMeta::multiple(exit_values);
```

### Task 196-3: Verify ExitBindingBuilder Multi-Carrier

**File**: `src/mir/builder/control_flow/joinir/patterns/exit_binding.rs`

**Verification**:
1. Confirm `build_loop_exit_bindings()` iterates ALL carriers
2. Confirm variable_map is updated for ALL carriers
3. Add unit test for 2-carrier case

**Test case**:
```rust
#[test]
fn test_two_carrier_binding() {
    let carrier_info = CarrierInfo::with_carriers(
        "i".to_string(),
        ValueId(5),
        vec![
            CarrierVar { name: "count".to_string(), host_id: ValueId(10) },
            CarrierVar { name: "sum".to_string(), host_id: ValueId(11) },
        ],
    );

    let exit_meta = ExitMeta::multiple(vec![
        ("count".to_string(), ValueId(14)),
        ("sum".to_string(), ValueId(15)),
    ]);

    // ... verify 2 bindings created, variable_map updated for both
}
```

### Task 196-4: Update Pattern4 Caller Boundary

**File**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**Changes**:
1. Remove hardcoded `vec![ValueId(0), ValueId(1)]`
2. Compute `join_inputs` dynamically from carrier count
3. Ensure `join_inputs.len() == host_inputs.len()`

**Example change**:
```rust
// Before:
vec![ValueId(0), ValueId(1)]  // Hardcoded

// After:
let mut join_inputs = vec![ValueId(0)];  // loop_var is always ValueId(0)
for idx in 1..=carrier_info.carriers.len() {
    join_inputs.push(ValueId(idx as u32));
}
// join_inputs: [0, 1, 2] for 2 carriers
```

### Task 196-5: Multi-Carrier Test & Documentation

**Test command**:
```bash
./target/release/hakorune apps/tests/loop_continue_multi_carrier.hako
```

**Expected output**:
```
25
5
```

**Documentation updates**:
- Update CURRENT_TASK.md with Phase 196 completion
- Update phase193_5_multi_carrier_testing.md to note Phase 196 implementation
- Mark this file as complete

---

## Files to Modify

| File | Task | Change |
|------|------|--------|
| `loop_with_continue_minimal.rs` | 196-2 | Generate ExitMeta for all carriers |
| `pattern4_with_continue.rs` | 196-4 | Compute join_inputs dynamically |
| `exit_binding.rs` | 196-3 | Add multi-carrier unit test |
| `CURRENT_TASK.md` | 196-5 | Document completion |

---

## Success Criteria

- [x] `loop_continue_pattern4.hako` (single carrier) still passes ✅
- [x] `loop_continue_multi_carrier.hako` (2 carriers) passes with output "25\n5" ✅ (Phase 197 Complete)
- [x] No hardcoded carrier names remain in Pattern 4 lowering path ✅
- [x] ExitBindingBuilder unit tests pass for 2-carrier case ✅ (Phase 193-4)
- [x] join_inputs computed dynamically, not hardcoded ✅

---

## Related Documentation

- **Phase 193 Completion**: [PHASE_193_COMPLETION.md](PHASE_193_COMPLETION.md)
- **Phase 193-4 ExitBindingBuilder**: [phase193_exit_binding_builder.md](phase193_exit_binding_builder.md)
- **Phase 193-5 Testing Plan**: [phase193_5_multi_carrier_testing.md](phase193_5_multi_carrier_testing.md)

---

## Progress Log

### 2025-12-06 (Session 3 - Phase 197 Complete)
- **Phase 197-B COMPLETE**: Multi-carrier exit mechanism fully fixed
  - `reconnect_boundary()` now uses `remapper` to get per-carrier exit values
  - ExitMeta uses `carrier_param_ids` (Jump arguments) instead of `carrier_exit_ids` (k_exit parameters)
  - Root cause: k_exit parameters aren't defined when JoinIR functions merge into host
- **Phase 197-C COMPLETE**: AST-based update expression
  - LoopUpdateAnalyzer extracts `sum = sum + i` / `count = count + 1` patterns
  - Pattern 4 lowerer uses UpdateExpr for semantically correct RHS
- **Test result**: `loop_continue_multi_carrier.hako` outputs `25, 5` ✅

### 2025-12-06 (Session 2 - Implementation)
- **Task 196-2 COMPLETE**: Extended JoinIR lowerer to accept CarrierInfo parameter
  - Modified `lower_loop_with_continue_minimal()` signature
  - Dynamic ValueId allocation for N carriers
  - ExitMeta::multiple() with all carriers
  - Select generation for each carrier
- **Task 196-3 COMPLETE (inherent)**: ExitBindingBuilder already supports multi-carrier (Phase 193-4)
- **Task 196-4 COMPLETE**: Fixed Pattern4 caller boundary construction
  - Dynamic join_inputs generation matching carrier count
  - Passes carrier_info to lowerer
- **Task 196-5 PARTIAL → FIXED in Phase 197**: Semantic issue resolved
  - **BEFORE**: Assertion crash (join_inputs.len()=2, host_inputs.len()=3)
  - **AFTER Phase 196**: No crash, outputs 5/5 (infrastructure fixed)
  - **AFTER Phase 197**: Outputs 25/5 (semantics fixed)

### 2025-12-06 (Session 1 - Documentation)
- Created Phase 196 documentation (Task 196-1)
- Identified root cause: hardcoded ExitMeta::single() and join_inputs
- Task investigation completed by Agent
Status: Historical
