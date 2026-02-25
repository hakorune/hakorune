# Phase 224-B: MethodCallLowerer + CoreMethodId Extension

**Status**: ✅ Complete
**Date**: 2025-12-10
**Purpose**: Metadata-driven MethodCall lowering for JoinIR loop conditions

---

## 🎯 Overview

Phase 224-B implements generic MethodCall lowering infrastructure for JoinIR loop conditions.
This enables `s.length()`, `indexOf()`, etc. to be used in loop conditions with full type safety.

### Key Achievement

- **Metadata-Driven Design**: No method name hardcoding - all decisions based on CoreMethodId
- **Box-First Architecture**: MethodCallLowerer as independent box answering one question
- **Type-Safe**: Uses existing CoreMethodId infrastructure (box_id, arity, return_type)

---

## 📦 Implementation

### 224-B-1: CoreMethodId Extension

**File**: `src/runtime/core_box_ids.rs` (+114 lines)

Added three new methods to `CoreMethodId`:

```rust
impl CoreMethodId {
    /// Pure function (no side effects, deterministic)
    pub fn is_pure(&self) -> bool { ... }

    /// Allowed in loop condition expressions
    pub fn allowed_in_condition(&self) -> bool { ... }

    /// Allowed in loop body init expressions
    pub fn allowed_in_init(&self) -> bool { ... }
}
```

**Whitelist Design** (Conservative Fail-Fast):

- **Condition**: StringLength, ArrayLength, MapHas, ResultIsOk (4 methods)
- **Init**: More permissive - includes substring, indexOf, MapGet, etc. (13 methods)
- **Pure but not whitelisted**: Still rejected to avoid surprises

**Tests**: 3 new unit tests (16 total CoreMethodId tests, all pass)

---

### 224-B-2: MethodCallLowerer Box

**File**: `src/mir/join_ir/lowering/method_call_lowerer.rs` (+362 lines, new)

Single-responsibility box:

```rust
pub struct MethodCallLowerer;

impl MethodCallLowerer {
    /// Lower MethodCall for loop condition
    pub fn lower_for_condition<F>(
        recv_val: ValueId,
        method_name: &str,
        args: &[ASTNode],
        alloc_value: &mut F,
        instructions: &mut Vec<JoinInst>,
    ) -> Result<ValueId, String> { ... }

    /// Lower MethodCall for LoopBodyLocal init
    pub fn lower_for_init<F>(...) -> Result<ValueId, String> { ... }
}
```

**Phase 224-B P0 Restrictions**:

- ✅ Zero-argument methods only (`s.length()`)
- ❌ Methods with arguments (`s.substring(0, 1)`) - Phase 224-C

**JoinIR BoxCall Emission**:

```rust
// Input: s.length()
// Output:
JoinInst::Compute(MirLikeInst::BoxCall {
    dst: Some(ValueId(100)),
    box_name: "StringBox",  // From CoreMethodId.box_id().name()
    method: "length",
    args: vec![recv_val],   // Receiver is first arg
})
```

**Tests**: 6 unit tests covering:

- ✅ Resolve string.length → CoreMethodId::StringLength
- ✅ Lower for condition (allowed)
- ✅ Lower for init (more permissive)
- ✅ Not allowed in condition (Fail-Fast)
- ✅ Unknown method (Fail-Fast)
- ✅ P0 restriction (no arguments)

---

### 224-B-3: Integration with condition_lowerer

**File**: `src/mir/join_ir/lowering/condition_lowerer.rs` (+17 lines)

Added MethodCall case to `lower_value_expression`:

```rust
ASTNode::MethodCall { object, method, arguments, .. } => {
    // 1. Lower receiver to ValueId
    let recv_val = lower_value_expression(object, alloc_value, env, instructions)?;

    // 2. Lower method call using MethodCallLowerer
    MethodCallLowerer::lower_for_condition(recv_val, method, arguments, alloc_value, instructions)
}
```

Previously: `Err("Unsupported expression in value context: MethodCall")`
Now: Full MethodCall lowering with type safety!

---

### 224-B-4: Module Registration

**File**: `src/mir/join_ir/lowering/mod.rs` (+1 line)

```rust
pub mod method_call_lowerer; // Phase 224-B: MethodCall lowering (metadata-driven)
```

---

## ✅ Test Results

### Unit Tests

```bash
cargo test --release --lib method_call_lowerer
# test result: ok. 6 passed; 0 failed

cargo test --release --lib core_box_ids::tests
# test result: ok. 16 passed; 0 failed
```

### Build Status

```bash
cargo build --release
# Finished `release` profile [optimized] target(s) in 1m 13s
# 0 errors, 7 warnings (pre-existing)
```

---

## 🎯 Usage Patterns

### Pattern 2: Loop with MethodCall Condition

```nyash
// Phase 224-B: s.length() in condition now supported!
loop(i < s.length()) {
    // MethodCall lowered to BoxCall in JoinIR
    i = i + 1
}
```

**Before Phase 224-B**:
```
[ERROR] Unsupported expression in value context: MethodCall
```

**After Phase 224-B**:
```
✅ MethodCall lowered to BoxCall("StringBox", "length", [s_val])
```

---

## 🔍 Design Principles Demonstrated

### 1. Box-First Architecture

- **MethodCallLowerer**: Standalone box, no dependencies on specific patterns
- **Single Responsibility**: "Can this MethodCall be lowered?" - that's it
- **Composable**: Used by condition_lowerer, body_local_init, etc.

### 2. Metadata-Driven

**NO method name hardcoding**:

```rust
// ❌ Bad (hardcoded):
if method_name == "length" { ... }

// ✅ Good (metadata-driven):
let method_id = CoreMethodId::iter().find(|m| m.name() == method_name)?;
if method_id.allowed_in_condition() { ... }
```

### 3. Fail-Fast

- Unknown method → Immediate error
- Not whitelisted → Immediate error
- Arguments in P0 → Immediate error with clear message

---

## 📊 Code Metrics

| Component | Lines | Tests | Status |
|-----------|-------|-------|--------|
| CoreMethodId extension | +114 | 3 | ✅ |
| MethodCallLowerer box | +362 | 6 | ✅ |
| condition_lowerer integration | +17 | (covered) | ✅ |
| **Total** | **+493** | **9** | ✅ |

**Net Impact**: +493 lines, 9 new tests, 0 regressions

---

## 🚀 Next Steps

### Phase 224-C: MethodCall Arguments Support

```nyash
// Phase 224-C target:
local ch = s.substring(i, i+1)  // 2-argument MethodCall
local pos = digits.indexOf(ch)  // 1-argument MethodCall
```

**Requirements**:

1. Extend MethodCallLowerer to handle arguments
2. Lower argument AST nodes to ValueIds
3. Pass argument ValueIds in BoxCall.args (after receiver)

### Phase 224-D: Option B - Promoted Variable Tracking

Fix the remaining issue in phase2235_p2_digit_pos_min.hako:

```
[ERROR] Variable 'digit_pos' not bound in ConditionEnv
```

**Root Cause**: Promotion system promotes `digit_pos` → `is_digit_pos`,
but condition lowerer still expects `digit_pos`.

**Solution**: Track promoted variables in CarrierInfo, remap during lowering.

---

## 📚 References

- **CoreBoxId/CoreMethodId**: `src/runtime/core_box_ids.rs` (Phase 87)
- **JoinIR BoxCall**: `src/mir/join_ir/mod.rs` (MirLikeInst enum)
- **Condition Lowering**: `src/mir/join_ir/lowering/condition_lowerer.rs` (Phase 171)
- **Phase 224 Summary**: `docs/development/current/main/PHASE_224_SUMMARY.md`

---

## ✨ Key Takeaway

**Phase 224-B establishes the foundation for generic MethodCall lowering in JoinIR.**

- No more "Unsupported expression" errors for common methods
- Type-safe, metadata-driven, extensible architecture
- Ready for Phase 224-C (arguments) and Phase 224-D (variable remapping)

**Everything is a Box. Everything is metadata-driven. Everything Fail-Fast.**
Status: Active  
Scope: methodcall lowerer 設計（ExprLowerer ライン）
