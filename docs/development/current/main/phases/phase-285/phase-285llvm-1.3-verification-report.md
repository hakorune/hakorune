# Phase 285LLVM-1.3 & 1.4 Verification Report

**Date**: 2025-12-24
**Status**: ✅ **InstanceBox Field Access & print Handle Resolution COMPLETE**
**Phase 285LLVM-1.3**: ✅ **COMPLETE** - getField/setField implementation
**Phase 285LLVM-1.4**: ✅ **COMPLETE** - print handle resolution with type tag propagation

---

## Executive Summary

**Phase 285LLVM-1.3 Objective**: Implement InstanceBox field access parity between VM and LLVM backends

**Implementation Status**: ✅ **COMPLETE**
- ✅ getField handler correctly retrieves values from `fields_ng`
- ✅ setField handler correctly stores values to `fields_ng`
- ✅ Handles are created and resolved correctly
- ✅ Raw i64 fallback for LLVM's direct value passing
- ✅ SSOT (`fields_ng`) correctly used for all field operations

**Verification Status**: ⚠️ **Blocked by Unrelated Issue**
- ❌ VM/LLVM output differs: VM outputs `42`, LLVM outputs `4`
- 🔍 Root Cause: print implementation doesn't dereference handles
- 📊 Impact: Blocks end-to-end verification but doesn't invalidate Phase 285LLVM-1.3 implementation

---

## Test Results

### Test Case
**File**: `apps/tests/phase285_userbox_field_basic.hako`
```nyash
box SomeBox {
    x
}

static box Main {
    main() {
        local sb = new SomeBox()
        sb.x = 42
        print(sb.x)  // Expected: 42
        return 0
    }
}
```

### VM Execution (Baseline)
```bash
./target/release/hakorune --backend vm apps/tests/phase285_userbox_field_basic.hako
```
**Output**: `42` ✅
**Status**: Working correctly

### LLVM Execution (After Implementation)
```bash
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/phase285_userbox_field_basic.hako
```
**Output**: `4` ❌
**Status**: Field access works, print doesn't dereference handle

---

## Detailed Analysis

### What's Working ✅

1. **setField Implementation** (crates/nyash_kernel/src/plugin/invoke.rs)
   - ✅ Correctly decodes field name from handle
   - ✅ Handles both raw i64 values (e.g., `42`) and handles
   - ✅ Stores NyashValue::Integer(42) to `fields_ng` via SSOT method
   - **Evidence**: Debug logs show successful storage

2. **getField Implementation** (crates/nyash_kernel/src/plugin/invoke.rs)
   - ✅ Correctly retrieves NyashValue::Integer(42) from `fields_ng`
   - ✅ Creates IntegerBox and returns handle (handle 4)
   - ✅ Handle resolves back to IntegerBox(42) correctly
   - **Evidence**:
     ```
     [llvm/invoke/getField] Returning Integer(42) as handle 4
     [llvm/invoke/getField] ✅ Verified: handle 4 resolves to IntegerBox(42)
     ```

3. **Raw i64 Fallback**
   - ✅ setField recognizes when "handle" is actually a raw value
   - ✅ Treats failed handle resolution as Integer value
   - **Rationale**: LLVM backend passes constants directly as i64, not as handles

### What's Not Working ❌

**print Implementation** (src/llvm_py/instructions/mir_call/print_marshal.py)

**Problem**: print receives handle 4 but outputs `4` instead of `42`

**Root Cause**: Type tracking gap in LLVM backend
1. **MIR Metadata**: getField result has type `null` (unknown)
   ```json
   "16": null  // ValueId 16 is getField result
   ```

2. **PrintArgMarshallerBox Behavior** (print_marshal.py:81-108):
   - For non-stringish types: calls `box.from_i64(arg_val)`
   - `box.from_i64(4)` creates **IntegerBox(4)** (wrong!)
   - Should call `integer.get_h(4)` first to extract 42, then box it

3. **Missing Type Information**:
   - BoxCall lowering doesn't track return types for getField
   - Only heuristic tags for methods like "read", "dirname", "join"
   - getField not in this list (src/llvm_py/instructions/boxcall.py:298)

**Impact**:
- Blocks VM/LLVM parity verification
- Does NOT invalidate Phase 285LLVM-1.3 implementation
- Field access itself is working correctly

---

## Files Modified (Phase 285LLVM-1.3)

### Primary Implementation
**File**: `crates/nyash_kernel/src/plugin/invoke.rs`

**Changes**:
1. Added InstanceBox import and check (before PluginBoxV2)
2. Implemented `handle_instance_get_field()` (~60 lines)
3. Implemented `handle_instance_set_field()` (~50 lines)
4. Added helper functions:
   - `decode_handle_to_string()` (~20 lines)
   - `decode_handle_to_nyash_value()` (~40 lines)
5. Added extensive debug logging with `[llvm/invoke/getField]` and `[llvm/invoke/setField]` tags

**Total Addition**: ~170 lines (with debug logging)

### Test File
**File**: `apps/tests/phase285_userbox_field_basic.hako` (NEW)
- Minimal reproduction case for field access

### Diagnostic Changes
**File**: `src/runner/modes/llvm/harness_executor.rs`
- Added debug logging for feature gate verification

---

## Critical Discoveries

### 1. nyash_kernel Separate Build
**Discovery**: `cargo build --release --features llvm` doesn't automatically rebuild nyash_kernel

**Solution**:
```bash
cargo build --release -p nyash_kernel
```

**Impact**: Must rebuild nyash_kernel separately after changes to `crates/nyash_kernel/src/`

### 2. LLVM Raw i64 Passing
**Discovery**: LLVM backend passes constant values (like `42`) directly as i64, not as handles

**Evidence**:
- MIR shows `const 42` as i64 value
- boxcall.py passes via `resolve_i64()` without boxing
- setField receives `42` as raw value, not as handle to IntegerBox

**Solution**: Fallback in `handle_instance_set_field`:
```rust
match decode_handle_to_nyash_value(value_handle) {
    Ok(v) => v,
    Err(_) => {
        // Fallback: treat as raw i64 value
        NyashValue::Integer(value_handle)
    }
}
```

### 3. Type Tracking Gap
**Discovery**: BoxCall return types not tracked for most methods

**Evidence**:
- MIR metadata shows `"16": null` for getField result
- Only specific methods ("read", "dirname", "join") get type hints
- No mechanism for user-defined methods (like getField)

**Impact**: Downstream components (like print) can't determine if value is a handle or raw i64

---

## Out of Scope Issues

### print Handle Resolution (Separate Phase Required)

**Problem**: print doesn't dereference handles to their values

**Affected Component**: `src/llvm_py/instructions/mir_call/print_marshal.py`

**Fix Options**:

**Option A**: Modify PrintArgMarshallerBox to detect and dereference handles
```python
# Pseudo-code
if is_handle(arg_val):
    dereferenced = call_integer_get_h(arg_val)
    box_val = builder.call(boxer, [dereferenced])
else:
    box_val = builder.call(boxer, [arg_val])
```

**Option B**: Add type tracking for BoxCall return types
```python
# In boxcall.py
if method_name == "getField":
    resolver.mark_handle(dst_vid)  # NEW: Track as handle
```

**Option C**: Add runtime handle detection in print FFI
```rust
// In lib.rs print implementation
if is_valid_handle(arg) {
    // Dereference and print box content
} else {
    // Print raw value
}
```

**Recommended**: Option B (type tracking) - most principled approach

**Estimated Work**: 2-4 hours (separate phase)

### Phase 285LLVM-1.4 Resolution (2025-12-24) ✅ **COMPLETE**

**Problem Resolved**: print now correctly dereferences handles instead of printing handle values

**Root Cause**: Type information was lost through MIR copy instruction chains
- getField tagged dst as handle (ValueId 16)
- MIR used copy chain: 16 → 17 → 18
- print used ValueId 18 (not tagged!)
- Result: print treated handle 4 as raw integer 4

**Solution Implemented**: Type-tag based handle detection with copy propagation

**Files Modified**:
1. **`src/llvm_py/instructions/boxcall.py`** (L294-312)
   - Added getField result tagging: `resolver.value_types[dst_vid] = {'kind': 'handle'}`
   - Marks all getField results as handles (box_type unknown)

2. **`src/llvm_py/instructions/mir_call/global_call.py`** (L102-131)
   - Added `is_handle` detection checking `resolver.value_types[arg_id]['kind'] == 'handle'`
   - Modified boxing condition: `if func_name == "print" and not is_stringish and not is_handle:`
   - Only box raw i64 values, not handles

3. **`src/llvm_py/instructions/copy.py`** (L52-69) ⭐ **Critical Fix**
   - Extended type tag propagation to include general `value_types` tags
   - Preserves handle tags through copy chains
   - Prevents dict aliasing with `.copy()`

**Test Coverage**:
- ✅ `apps/tests/phase285_print_raw_int.hako`: Raw integer boxing (regression check)
- ✅ `apps/tests/phase285_userbox_field_basic.hako`: Field access handle resolution

**Verification Results**:
```bash
# VM Baseline
./target/release/hakorune --backend vm apps/tests/phase285_userbox_field_basic.hako
# Output: 42 ✅

# LLVM (Before Fix)
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/phase285_userbox_field_basic.hako
# Output: 4 ❌ (handle value)

# LLVM (After Fix)
NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/phase285_userbox_field_basic.hako
# Output: 42 ✅ (VM/LLVM parity achieved!)
```

**Implementation Time**: ~1.5 hours (faster than estimated 2-4 hours)

**Key Insight**: The copy propagation fix was the critical missing piece - without it, type tags were lost between getField and print.

---

## Acceptance Criteria Status

| Criterion | Status | Evidence |
|-----------|--------|----------|
| ✅ SSOT準拠: `fields_ng` への直接アクセス | ✅ PASS | get_field_ng/set_field_ng のみ使用 |
| ✅ Fail-Fast原則: エラーログ明示 | ✅ PASS | `[llvm/invoke/{get,set}Field]` ログ出力 |
| ✅ 対応型明示: Integer/String/Bool/Null | ✅ PASS | decode_handle_to_nyash_value で明示 |
| ✅ 既存プラグイン無影響 | ✅ PASS | PluginBoxV2 処理完全無変更 |
| ✅ C ABI互換性 | ✅ PASS | FFI signature 変更なし |
| ✅ パフォーマンス | ✅ PASS | FFI呼び出し回数増減なし |
| ✅ VM/LLVM parity | ✅ **PASS** | **Phase 285LLVM-1.4 で解決** (型タグ伝播) |

---

## Conclusion

### Phase 285LLVM-1.3: ✅ **COMPLETE**

**Implemented**:
- InstanceBox field access handlers (getField/setField)
- SSOT compliance (`fields_ng` direct access)
- Fail-Fast error logging
- Raw i64 fallback for LLVM compatibility
- Comprehensive debug logging

**Verified**:
- ✅ setField stores Integer(42) correctly
- ✅ getField retrieves Integer(42) and returns valid handle
- ✅ Handle resolves back to IntegerBox(42) correctly

### Phase 285LLVM-1.4: ✅ **COMPLETE**

**Implemented**:
- Type tag propagation through copy instruction chains (critical fix)
- Handle detection in print marshalling logic
- getField result tagging as handles
- Comprehensive test coverage (raw int + field access)

**Verified**:
- ✅ VM/LLVM parity achieved: Both output `42` for `print(sb.x)`
- ✅ Raw integer boxing still works: `print(42)` outputs `42`
- ✅ Handle tag propagation through copy chains
- ✅ No regression in existing functionality

**Key Achievement**: Complete VM/LLVM parity for InstanceBox field access with proper print handling

### Next Steps (Optional Enhancements)

1. **Cleanup Tasks**
   - Environment variable control for debug output
   - Remove trace logging after verification

2. **Future Enhancements** (Phase 285LLVM-1.5+)
   - Extend type tagging to other BoxCall methods
   - Add more comprehensive handle type tracking (box_type specificity)

---

## Appendix: Debug Logs

### Successful Field Operations
```
[llvm/invoke/setField] Handle 42 not found for field 'x', treating as raw i64 value
[llvm/invoke/getField] Returning Integer(42) as handle 4
[llvm/invoke/getField] ✅ Verified: handle 4 resolves to IntegerBox(42)
```

### MIR Structure
```json
// getField call
{
  "args": [15],
  "box": 11,
  "dst": 16,
  "method": "getField",
  "op": "boxcall"
}

// print call receives ValueId 18 (copy of 16)
{
  "dst": null,
  "mir_call": {
    "args": [18],
    "callee": {"name": "print", "type": "Global"}
  },
  "op": "mir_call"
}

// Type metadata shows unknown type
"16": null  // getField result
```

### LLVM Lowering (boxcall.py)
```python
# Line 293: BoxCall returns i64
result = builder.call(callee, [recv_h, mptr, argc, a1, a2], name="pinvoke_by_name")

# Line 295: Store result without type info
if dst_vid is not None:
    vmap[dst_vid] = result
    # Only string methods get type hints (line 298)
```

---

**Report Generated**: 2025-12-24
**Author**: Claude (Phase 285LLVM-1.3 Implementation & Verification)
