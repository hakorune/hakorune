# Phase 138: ReturnValueLowererBox - Return Lowering SSOT

**Date**: 2025-12-18
**Status**: DONE ✅
**Scope**: Extract return lowering to shared Box (behavior unchanged)

---

## Goal

Establish SSOT for return value lowering by extracting logic to ReturnValueLowererBox:
- Extract `lower_return_value_to_vid()` from `loop_true_break_once.rs`
- Create shared `ReturnValueLowererBox` for Normalized shadow paths
- Keep **挙動不変**: Phase 136/137 behavior unchanged
- Keep **Phase 139 準備**: Prepare for post_if_post_k.rs unification

## Scope

### ✅ In Scope (Phase 138 P0)

- **Boxification**: Extract return lowering to `common/return_value_lowerer_box.rs`
- **Migration**: `loop_true_break_once.rs` uses Box (2 call sites)
- **Tests**: 5 unit tests for all supported patterns
- **Regression**: All Phase 136/137 smokes unchanged

### ❌ Out of Scope (Phase 139)

- **post_if_post_k.rs**: Not modified (different responsibility)
- Unification planned for Phase 139 P0

## SSOT Location

**File**: `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`

**Function**: `ReturnValueLowererBox::lower_to_value_id()`

### Supported Patterns (Phase 136-137)

1. **Variable**: `return x` → env lookup → ValueId
2. **Integer literal**: `return 7` → Const generation → ValueId
3. **Add expression**: `return x + 2` or `return 5 + 3` → BinOp(Add, lhs, rhs) → ValueId

### Out-of-Scope Patterns

- Other operators: `return x - 2` → `Ok(None)` (fallback)
- Variable + variable: `return x + y` → `Ok(None)` (fallback)
- Nested expressions: `return (x + 2) + 3` → `Ok(None)` (fallback)

## Implementation

### New Files

1. **`src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`** (~300 lines)
   - Main function: `lower_to_value_id()`
   - Helper: `lower_binary_op()` for Add expressions
   - 5 comprehensive unit tests

2. **`src/mir/control_tree/normalized_shadow/common/mod.rs`**
   - Module export for common utilities

### Modified Files

1. **`src/mir/control_tree/normalized_shadow/mod.rs`**
   - Added `pub mod common;`

2. **`src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`**
   - Removed `lower_return_value_to_vid()` method (~115 lines)
   - Added import: `use super::common::return_value_lowerer_box::ReturnValueLowererBox;`
   - Updated 2 call sites:
     - Line 417: post_k return processing
     - Line 481: k_exit return processing
   - Updated SSOT documentation (lines 29-43)

### Call Sites (Phase 138 P0)

**File**: `loop_true_break_once.rs`

1. **post_k return processing** (line 417):
```rust
match ReturnValueLowererBox::lower_to_value_id(
    value_ast,
    &mut post_k_func.body,
    &mut next_value_id,
    &env_post_k
)? {
    Some(vid) => { post_k_func.body.push(JoinInst::Ret { value: Some(vid) }); }
    None => { return Ok(None); }
}
```

2. **k_exit return processing** (line 481):
```rust
match ReturnValueLowererBox::lower_to_value_id(
    value_ast,
    &mut k_exit_func.body,
    &mut next_value_id,
    &env_k_exit
)? {
    Some(vid) => { k_exit_func.body.push(JoinInst::Ret { value: Some(vid) }); }
    None => { return Ok(None); }
}
```

## Unit Tests (5 tests)

**File**: `src/mir/control_tree/normalized_shadow/common/return_value_lowerer_box.rs`

1. `test_lower_variable` - Variable lookup in env
2. `test_lower_integer_literal` - Const instruction generation
3. `test_lower_add_var_plus_int` - `x + 2` pattern
4. `test_lower_add_int_plus_int` - `5 + 3` constant folding
5. `test_out_of_scope_subtract` - Fallback for unsupported operators

## Verification

```bash
# Build
cargo build --release -p nyash-rust --features llvm

# Unit tests
cargo test --lib

# Phase 137 regression (6 tests)
bash tools/smokes/v2/profiles/integration/apps/phase137_loop_true_break_once_return_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase137_loop_true_break_once_return_add_const_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase137_loop_true_break_once_post_return_add_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase137_loop_true_break_once_return_add_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/phase137_loop_true_break_once_return_add_const_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/phase137_loop_true_break_once_post_return_add_llvm_exe.sh

# Phase 97 regression (2 tests)
bash tools/smokes/v2/profiles/integration/apps/phase97_next_non_ws_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/phase97_json_loader_escape_llvm_exe.sh

# Phase 131/135/136 regression
bash tools/smokes/v2/profiles/integration/apps/phase131_loop_true_break_once_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase135_loop_true_break_once_post_empty_return_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase136_loop_true_break_once_return_literal_vm.sh
```

## Acceptance Criteria

- ✅ cargo test --lib: 1194 tests PASS (+5 new unit tests)
- ✅ Phase 137 regression: 6/6 PASS
- ✅ Phase 97 regression: 2/2 PASS
- ✅ Phase 131/135/136 regression: 3/3 PASS
- ✅ Behavior unchanged: All Phase 136/137 fixtures/smokes PASS
- ✅ SSOT established: ReturnValueLowererBox is single source of truth
- ✅ post_if_post_k.rs unchanged: Phase 139 P0 scope

## Boxification Trigger

**Condition**: When 2+ files need identical return lowering logic

**Result**: Phase 138 P0 achieved this trigger with loop_true_break_once.rs migration

**Next Step**: Phase 139 P0 - Migrate post_if_post_k.rs to ReturnValueLowererBox

## Current Status

Phase 138 - DONE ✅ (2025-12-18)

SSOT established: `common/return_value_lowerer_box.rs`

Return lowering unified for loop paths, ready for if-with-post paths in Phase 139.

## Architecture Impact

- **Code Reduction**: ~115 lines removed from loop_true_break_once.rs
- **Maintainability**: Single location for return lowering improvements
- **Testability**: Isolated unit tests for return lowering logic
- **Extensibility**: Easy to add new return patterns in one location
