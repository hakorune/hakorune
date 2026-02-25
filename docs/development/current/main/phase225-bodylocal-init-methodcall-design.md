# Phase 225: LoopBodyLocalInit MethodCall Lowering - Meta-Driven Design

## Background

Phase 224-D completed `ConditionAlias` variable resolution, but `loop_body_local_init.rs` still has hardcoded method name whitelists and box name mappings in `emit_method_call_init`:

```rust
// Line 387: Method name whitelist (substring is missing!)
const SUPPORTED_INIT_METHODS: &[&str] = &["indexOf", "get", "toString"];

// Line 433-438: Box name hardcoding
let box_name = match method {
    "indexOf" => "StringBox".to_string(),
    "get" => "ArrayBox".to_string(),
    "toString" => "IntegerBox".to_string(),
    _ => unreachable!("Whitelist check should have caught this"),
};
```

This causes errors like:
```
Method 'substring' not supported in body-local init (Phase 193 limitation - only indexOf, get, toString supported)
```

## Problem Statement

Test case `apps/tests/phase2235_p2_digit_pos_min.hako` fails at:
```nyash
local ch = s.substring(p, p+1)  // ❌ substring not in whitelist
local digit_pos = digits.indexOf(ch)  // ✅ indexOf is in whitelist
```

The hardcoded whitelist prevents legitimate pure methods from being used in loop body-local initialization.

## Goal

**Eliminate ALL hardcoding** and make method call lowering **metadata-driven** using `CoreMethodId`:

1. **No method name hardcoding** - Use `CoreMethodId::iter()` to resolve methods
2. **No box name hardcoding** - Use `method_id.box_id().name()` to get box name
3. **Metadata-driven whitelist** - Use `method_id.allowed_in_init()` for permission check
4. **Delegation to MethodCallLowerer** - Single responsibility, reuse existing logic
5. **Fail-Fast** - Methods not in `CoreMethodId` immediately error

## Target Pattern

```nyash
loop(p < s.length()) {
    local ch = s.substring(p, p+1)        // ✅ Phase 225: substring allowed
    local digit_pos = digits.indexOf(ch)  // ✅ Already works

    if digit_pos < 0 {
        break
    }

    num_str = num_str + ch
    p = p + 1
}
```

## Architecture

### Before (Phase 193 - Hardcoded)

```
LoopBodyLocalInitLowerer
  └─ emit_method_call_init (static method)
      ├─ SUPPORTED_INIT_METHODS whitelist ❌
      ├─ match method { "indexOf" => "StringBox" } ❌
      └─ Emit BoxCall instruction
```

### After (Phase 225 - Meta-Driven)

```
LoopBodyLocalInitLowerer
  └─ emit_method_call_init (static method)
      └─ Delegates to MethodCallLowerer::lower_for_init
          ├─ Resolve method_name → CoreMethodId ✅
          ├─ Check allowed_in_init() ✅
          ├─ Get box_name from CoreMethodId ✅
          ├─ Check arity ✅
          └─ Emit BoxCall instruction
```

**Key Principle**: `MethodCallLowerer` is the **single source of truth** for all MethodCall → JoinIR lowering.

## Implementation Plan

### 225-2: Add `MethodCallLowerer::lower_for_init`

**Location**: `src/mir/join_ir/lowering/method_call_lowerer.rs`

**Note**: This method **already exists** (added in Phase 224-C). We just need to verify it works correctly:

```rust
/// Lower a MethodCall for use in LoopBodyLocal initialization
///
/// Similar to `lower_for_condition` but uses `allowed_in_init()` whitelist.
/// More permissive - allows methods like `substring`, `indexOf`, etc.
pub fn lower_for_init<F>(
    recv_val: ValueId,
    method_name: &str,
    args: &[ASTNode],
    alloc_value: &mut F,
    env: &ConditionEnv,
    instructions: &mut Vec<JoinInst>,
) -> Result<ValueId, String>
where
    F: FnMut() -> ValueId,
{
    // 1. Resolve method name to CoreMethodId
    let method_id = CoreMethodId::iter()
        .find(|m| m.name() == method_name)
        .ok_or_else(|| format!("MethodCall not recognized as CoreMethodId: {}", method_name))?;

    // 2. Check if allowed in init context
    if !method_id.allowed_in_init() {
        return Err(format!("MethodCall not allowed in LoopBodyLocal init: {}.{}() (not whitelisted)", recv_val.0, method_name));
    }

    // 3. Check arity
    let expected_arity = method_id.arity();
    if args.len() != expected_arity {
        return Err(format!("Arity mismatch: {}.{}() expects {} args, got {}", recv_val.0, method_name, expected_arity, args.len()));
    }

    // 4. Lower arguments
    let mut lowered_args = Vec::new();
    for arg_ast in args {
        let arg_val = super::condition_lowerer::lower_value_expression(
            arg_ast,
            alloc_value,
            env,
            instructions
        )?;
        lowered_args.push(arg_val);
    }

    // 5. Emit BoxCall instruction
    let dst = alloc_value();
    let box_name = method_id.box_id().name().to_string(); // ✅ No hardcoding!

    let mut full_args = vec![recv_val];
    full_args.extend(lowered_args);

    instructions.push(JoinInst::Compute(MirLikeInst::BoxCall {
        dst: Some(dst),
        box_name,
        method: method_name.to_string(),
        args: full_args,
    }));

    Ok(dst)
}
```

**Verification needed**: Check that `allowed_in_init()` returns `true` for `substring` and `indexOf`.

### 225-3: Refactor `emit_method_call_init` to Delegate

**Location**: `src/mir/join_ir/lowering/loop_body_local_init.rs`

**Changes**:
1. **Delete** `SUPPORTED_INIT_METHODS` whitelist (line 387)
2. **Delete** hardcoded box name match (lines 433-438)
3. **Delegate** to `MethodCallLowerer::lower_for_init`

```rust
// Before: 80 lines of hardcoded logic
fn emit_method_call_init(...) -> Result<ValueId, String> {
    const SUPPORTED_INIT_METHODS: &[&str] = &["indexOf", "get", "toString"]; // ❌ DELETE

    if !SUPPORTED_INIT_METHODS.contains(&method) { ... } // ❌ DELETE

    let receiver_id = ...; // ✅ Keep (resolve receiver)
    let arg_ids = ...; // ✅ Keep (lower arguments)

    let box_name = match method { ... }; // ❌ DELETE

    instructions.push(JoinInst::Compute(MirLikeInst::BoxCall { ... })); // ❌ DELETE
}

// After: 20 lines of delegation
fn emit_method_call_init(
    receiver: &ASTNode,
    method: &str,
    args: &[ASTNode],
    cond_env: &ConditionEnv,
    instructions: &mut Vec<JoinInst>,
    alloc: &mut dyn FnMut() -> ValueId,
) -> Result<ValueId, String> {
    // 1. Resolve receiver (existing logic)
    let receiver_id = match receiver {
        ASTNode::Variable { name, .. } => {
            cond_env.get(name).ok_or_else(|| {
                format!("Method receiver '{}' not found in ConditionEnv", name)
            })?
        }
        _ => {
            return Err("Complex receiver not supported in init method call".to_string());
        }
    };

    // 2. Delegate to MethodCallLowerer! ✅
    MethodCallLowerer::lower_for_init(
        receiver_id,
        method,
        args,
        alloc,
        cond_env,
        instructions,
    )
}
```

**Key Change**: Argument lowering is now handled by `MethodCallLowerer::lower_for_init` (via `condition_lowerer::lower_value_expression`), so we don't need to duplicate that logic.

### 225-4: Verify CoreMethodId Metadata

**Location**: `src/runtime/core_box_ids.rs`

Check `allowed_in_init()` implementation (lines 432-464):

```rust
pub fn allowed_in_init(&self) -> bool {
    use CoreMethodId::*;
    match self {
        // String operations - allowed
        StringLength | StringSubstring | StringIndexOf => true, // ✅ substring and indexOf!

        // String transformations - allowed for init
        StringUpper | StringLower | StringTrim => true,

        // Array operations - allowed
        ArrayLength | ArrayGet => true,

        // ...
    }
}
```

**Verification**: Confirm that:
- `StringSubstring.allowed_in_init() == true` ✅ (line 436)
- `StringIndexOf.allowed_in_init() == true` ✅ (line 436)

No changes needed - metadata is already correct!

## Testing Strategy

### Unit Tests

**Location**: `src/mir/join_ir/lowering/method_call_lowerer.rs`

Existing tests to verify:
- `test_lower_substring_for_init` - substring in init context (line 346)
- `test_lower_indexOf_with_arg` - indexOf with 1 argument (line 433)
- `test_phase224c_arity_mismatch` - arity checking (line 401)

### E2E Test

**Location**: `apps/tests/phase2235_p2_digit_pos_min.hako`

Expected behavior after Phase 225:
```bash
$ ./target/release/hakorune --backend vm apps/tests/phase2235_p2_digit_pos_min.hako

# Before Phase 225:
❌ Error: Method 'substring' not supported in body-local init

# After Phase 225:
✅ p = 3
✅ num_str = 123
```

### Regression Tests

Run existing tests to ensure no breakage:
```bash
cargo test --release --lib method_call_lowerer
cargo test --release --lib loop_body_local_init
```

## Success Criteria

1. ✅ `cargo build --release` succeeds
2. ✅ All unit tests in `method_call_lowerer.rs` pass
3. ✅ All unit tests in `loop_body_local_init.rs` pass
4. ✅ `phase2235_p2_digit_pos_min.hako` runs successfully (substring error disappears)
5. ✅ **Zero hardcoded method names or box names** in `emit_method_call_init`
6. ✅ No regressions in existing tests

## Hardcoding Inventory (To Be Deleted)

### In `loop_body_local_init.rs`:

1. **Line 387**: `const SUPPORTED_INIT_METHODS: &[&str] = &["indexOf", "get", "toString"];`
2. **Lines 389-394**: Method whitelist check
3. **Lines 433-438**: Box name match statement

**Total lines to delete**: ~20 lines
**Total lines to add**: ~5 lines (delegation call)

**Net change**: -15 lines (cleaner, simpler, more maintainable)

## Benefits

### 1. **Metadata-Driven Architecture**
- Single Source of Truth: `CoreMethodId` defines all method metadata
- No duplication: Method name, box name, arity, whitelist all in one place
- Easy to extend: Add new methods by updating `CoreMethodId` only

### 2. **Single Responsibility**
- `MethodCallLowerer`: "MethodCall → JoinIR" conversion (Phase 224-B)
- `LoopBodyLocalInitLowerer`: Loop body-local init coordination (Phase 186)
- Clear boundary: Init lowerer delegates, doesn't duplicate logic

### 3. **Fail-Fast**
- Unknown methods → immediate error (not silent fallback)
- Arity mismatch → immediate error
- Not whitelisted → immediate error with clear message

### 4. **Type Safety**
- No string matching → use enum (`CoreMethodId`)
- Compile-time checks → catch errors early
- Refactoring-safe → rename detection

### 5. **Maintainability**
- Add new method: Update `CoreMethodId` only (one place)
- Change whitelist: Update `allowed_in_init()` only
- No scattered hardcoding across files

## Future Work (Not in Phase 225)

### Phase 226+: Additional Improvements

1. **Type inference**: Use actual receiver type instead of heuristics
2. **Custom method support**: User-defined box methods
3. **Optimization**: Dead code elimination for unused method calls
4. **Error messages**: Better diagnostics with suggestions

## References

- **Phase 186**: Loop Body-Local Variable Initialization (initial implementation)
- **Phase 193**: MethodCall support in body-local init (hardcoded version)
- **Phase 224-B**: MethodCallLowerer Box creation (metadata-driven)
- **Phase 224-C**: MethodCallLowerer argument support
- **Phase 224-D**: ConditionAlias variable resolution

## Commit Message Template

```
refactor(joinir): Phase 225 - LoopBodyLocalInit MethodCall meta-driven

- Delete SUPPORTED_INIT_METHODS whitelist in loop_body_local_init.rs
- Delete hardcoded box name match (indexOf→StringBox, etc.)
- Delegate emit_method_call_init to MethodCallLowerer::lower_for_init
- Use CoreMethodId metadata for allowed_in_init() whitelist
- Fix: substring now works in body-local init (digit_pos test)

Hardcoding removed:
- SUPPORTED_INIT_METHODS constant (line 387)
- Box name match statement (lines 433-438)
- Whitelist check (lines 389-394)

Net change: -15 lines (cleaner, simpler, more maintainable)

Single Source of Truth: CoreMethodId metadata drives all decisions
Single Responsibility: MethodCallLowerer handles all MethodCall lowering

✅ All tests passing
✅ phase2235_p2_digit_pos_min.hako now works
✅ Zero hardcoded method/box names remaining

Phase 225 complete - meta-driven architecture achieved
```
Status: Active  
Scope: body-local init methodcall 設計（ExprLowerer ライン）
