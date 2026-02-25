# Phase 186: Body-local Init Lowering (箱化モジュール化)

**Status**: In Progress
**Date**: 2025-12-09
**Dependencies**: Phase 184 (LoopBodyLocalEnv), Phase 185 (Pattern2 integration)

## Overview

Phase 186 introduces **LoopBodyLocalInitLowerer** - a dedicated box for lowering body-local variable initialization expressions to JoinIR. This completes the body-local variable support by handling initialization expressions like `local digit_pos = pos - start`.

## Motivation

Phase 184 introduced LoopBodyLocalEnv to track body-local variables, and Phase 185 integrated it into Pattern2 for update expressions. However, **initialization expressions** were not yet lowered to JoinIR:

```nyash
loop(pos < 10) {
    local digit_pos = pos - start  // ← Init expression NOT lowered yet!
    sum = sum + digit_pos           // ← Update expression (Phase 184)
    pos = pos + 1
}
```

**Problems without Phase 186**:
- `digit_pos` was declared in LoopBodyLocalEnv but had no JoinIR ValueId
- Using `digit_pos` in update expressions failed with "variable not found"
- Body-local calculations couldn't be performed in JoinIR

**Phase 186 Solution**:
- Lower init expressions (`pos - start`) to JoinIR instructions
- Assign JoinIR ValueId to body-local variable in env
- Enable body-local variables to be used in subsequent update expressions

## Scope Definition

### In Scope (Phase 186)

**Supported init expressions** (int/arithmetic only):
- Binary operations: `+`, `-`, `*`, `/`
- Constant literals: `42`, `0`, `1`
- Variable references: `pos`, `start`, `i`

**Examples**:
```nyash
local digit_pos = pos - start   // ✅ BinOp + Variables
local temp = i * 2              // ✅ BinOp + Variable + Const
local offset = base + 10        // ✅ BinOp + Variable + Const
local cnt = i + 1               // ✅ BinOp + Variable + Const
```

### Out of Scope (Phase 186)

**NOT supported** (Fail-Fast with explicit error):
- String operations: `s.substring(...)`, `s + "abc"`
- Method calls: `box.method(...)`
- Complex expressions: nested BinOps, function calls

**Examples**:
```nyash
local ch = s.substring(pos, 1)  // ❌ Method call → Fail-Fast error
local msg = "Error: " + text    // ❌ String concat → Fail-Fast error
local result = calc(a, b)       // ❌ Function call → Fail-Fast error
```

**Rationale**: Phase 178 established Fail-Fast principle for unsupported features. String/method call support requires additional infrastructure (BoxCall lowering, type tracking) - defer to future phases.

## Architecture

### Box Theory Design

Following 箱理論 (Box-First) principles:

```
┌─────────────────────────────────────────────────────────────┐
│ LoopBodyLocalInitLowerer (NEW)                             │
│ - Single responsibility: Lower init expressions to JoinIR   │
│ - Clear boundary: Only handles init, not updates            │
│ - Fail-Fast: Unsupported expressions → explicit error       │
└─────────────────────────────────────────────────────────────┘
           ↓ (uses)
┌─────────────────────────────────────────────────────────────┐
│ LoopBodyLocalEnv (Phase 184)                               │
│ - Storage box for body-local variable mappings             │
│ - name → JoinIR ValueId                                    │
└─────────────────────────────────────────────────────────────┘
           ↓ (used by)
┌─────────────────────────────────────────────────────────────┐
│ CarrierUpdateEmitter (Phase 184)                           │
│ - Emits update instructions using UpdateEnv                │
│ - Resolves variables from condition + body-local envs      │
└─────────────────────────────────────────────────────────────┘
```

### Pipeline Integration

**Pattern2 Pipeline** (Phase 179-B + Phase 186):
```
1. Build PatternPipelineContext (loop features, carriers)
2. LoopConditionScopeBox::analyze() → ConditionEnv
3. ⭐ LoopBodyLocalInitLowerer::lower_inits_for_loop() ← NEW (Phase 186)
   - Scans body AST for local declarations
   - Lowers init expressions to JoinIR
   - Updates LoopBodyLocalEnv with ValueIds
4. LoopUpdateAnalyzer::analyze_carrier_updates()
5. CarrierUpdateEmitter::emit_carrier_update_with_env()
6. JoinModule construction + MIR merge
```

**Pattern4 Pipeline** (similar integration):
```
1-2. (same as Pattern2)
3. ⭐ LoopBodyLocalInitLowerer::lower_inits_for_loop() ← NEW
4. ContinueBranchNormalizer (Pattern4-specific)
5-6. (same as Pattern2)
```

## Module Design

### File Structure

```
src/mir/join_ir/lowering/
├── loop_body_local_env.rs       (Phase 184 - Storage box)
├── loop_body_local_init.rs      (Phase 186 - NEW! Init lowerer)
├── update_env.rs                (Phase 184 - Resolution layer)
└── carrier_update_emitter.rs    (Phase 184 - Update emitter)
```

### LoopBodyLocalInitLowerer API

```rust
//! Phase 186: Loop Body-Local Variable Initialization Lowerer
//!
//! Lowers body-local variable initialization expressions to JoinIR.

use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::loop_body_local_env::LoopBodyLocalEnv;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;
use crate::mir::join_ir::{JoinInst, MirLikeInst, ConstValue, BinOpKind};
use crate::mir::ValueId;

pub struct LoopBodyLocalInitLowerer<'a> {
    /// Reference to ConditionEnv for variable resolution
    cond_env: &'a ConditionEnv,

    /// Output buffer for JoinIR instructions
    instructions: &'a mut Vec<JoinInst>,

    /// ValueId allocator
    alloc_value: Box<dyn FnMut() -> ValueId + 'a>,
}

impl<'a> LoopBodyLocalInitLowerer<'a> {
    /// Create a new init lowerer
    pub fn new(
        cond_env: &'a ConditionEnv,
        instructions: &'a mut Vec<JoinInst>,
        alloc_value: Box<dyn FnMut() -> ValueId + 'a>,
    ) -> Self {
        Self {
            cond_env,
            instructions,
            alloc_value,
        }
    }

    /// Lower all body-local initializations in loop body
    ///
    /// Scans body AST for local declarations, lowers init expressions,
    /// and updates LoopBodyLocalEnv with computed ValueIds.
    ///
    /// # Arguments
    ///
    /// * `body_ast` - Loop body AST nodes
    /// * `env` - LoopBodyLocalEnv to update with ValueIds
    ///
    /// # Returns
    ///
    /// Ok(()) on success, Err(msg) if unsupported expression found
    pub fn lower_inits_for_loop(
        &mut self,
        body_ast: &[ASTNode],
        env: &mut LoopBodyLocalEnv,
    ) -> Result<(), String> {
        for node in body_ast {
            if let ASTNode::LocalAssign { variables, values, .. } = node {
                self.lower_single_init(variables, values, env)?;
            }
        }
        Ok(())
    }

    /// Lower a single local assignment
    fn lower_single_init(
        &mut self,
        variables: &[String],
        values: &[ASTNode],
        env: &mut LoopBodyLocalEnv,
    ) -> Result<(), String> {
        // Handle each variable-value pair
        for (var_name, init_expr) in variables.iter().zip(values.iter()) {
            // Skip if already has JoinIR ValueId (avoid duplicate lowering)
            if env.get(var_name).is_some() {
                continue;
            }

            // Lower init expression to JoinIR
            let value_id = self.lower_init_expr(init_expr)?;

            // Store in env
            env.insert(var_name.clone(), value_id);
        }
        Ok(())
    }

    /// Lower an initialization expression to JoinIR
    ///
    /// Supported:
    /// - BinOp(+, -, *, /) with Variable/Const operands
    /// - Const (integer literal)
    /// - Variable (condition variable reference)
    ///
    /// Unsupported (Fail-Fast):
    /// - String operations, method calls, complex expressions
    fn lower_init_expr(&mut self, expr: &ASTNode) -> Result<ValueId, String> {
        match expr {
            // Constant integer
            ASTNode::Integer { value, .. } => {
                let vid = (self.alloc_value)();
                self.instructions.push(JoinInst::Compute(MirLikeInst::Const {
                    dst: vid,
                    value: ConstValue::Integer(*value),
                }));
                Ok(vid)
            }

            // Variable reference (from ConditionEnv)
            ASTNode::Variable { name, .. } => {
                self.cond_env
                    .get(name)
                    .ok_or_else(|| format!("Init variable '{}' not found in ConditionEnv", name))
            }

            // Binary operation
            ASTNode::BinOp { op, left, right, .. } => {
                let lhs = self.lower_init_expr(left)?;
                let rhs = self.lower_init_expr(right)?;

                let op_kind = self.convert_binop(op)?;

                let result = (self.alloc_value)();
                self.instructions.push(JoinInst::Compute(MirLikeInst::BinOp {
                    dst: result,
                    op: op_kind,
                    lhs,
                    rhs,
                }));
                Ok(result)
            }

            // Fail-Fast for unsupported expressions
            ASTNode::MethodCall { .. } => {
                Err("Unsupported init expression: method call (Phase 186 limitation)".to_string())
            }
            ASTNode::String { .. } => {
                Err("Unsupported init expression: string literal (Phase 186 limitation)".to_string())
            }
            _ => {
                Err(format!("Unsupported init expression: {:?} (Phase 186 limitation)", expr))
            }
        }
    }

    /// Convert AST BinOp to JoinIR BinOpKind
    fn convert_binop(&self, op: &str) -> Result<BinOpKind, String> {
        match op {
            "+" => Ok(BinOpKind::Add),
            "-" => Ok(BinOpKind::Sub),
            "*" => Ok(BinOpKind::Mul),
            "/" => Ok(BinOpKind::Div),
            _ => Err(format!("Unsupported binary operator in init: {}", op)),
        }
    }
}
```

## Integration Points

### Pattern2 Integration (pattern2_with_break.rs)

**Before Phase 186**:
```rust
// cf_loop_pattern2_with_break()
let ctx = build_pattern_context(...)?;
let body_locals = collect_body_local_variables(...);
let body_local_env = LoopBodyLocalEnv::from_locals(body_locals);
// ❌ body_local_env has no ValueIds yet!
```

**After Phase 186**:
```rust
// cf_loop_pattern2_with_break()
let ctx = build_pattern_context(...)?;

// 1. Collect body-local variable names (allocate placeholder ValueIds)
let body_locals = collect_body_local_variables(...);
let mut body_local_env = LoopBodyLocalEnv::from_locals(body_locals);

// 2. ⭐ Lower init expressions to JoinIR
let mut init_lowerer = LoopBodyLocalInitLowerer::new(
    &ctx.condition_env,
    &mut join_instructions,
    Box::new(|| alloc_join_value()),
);
init_lowerer.lower_inits_for_loop(body, &mut body_local_env)?;
// ✅ body_local_env now has JoinIR ValueIds!

// 3. Proceed with update analysis and emission
let updates = LoopUpdateAnalyzer::analyze_carrier_updates(...);
let update_env = UpdateEnv::new(&ctx.condition_env, &body_local_env);
for (carrier, update) in updates {
    emit_carrier_update_with_env(&carrier, &update, ..., &update_env, ...)?;
}
```

### Pattern4 Integration (pattern4_with_continue.rs)

Similar to Pattern2 - insert init lowering step after condition analysis and before update analysis.

## Error Handling

### Fail-Fast Principle (Phase 178)

Following Phase 178 design - reject unsupported features early with clear error messages:

```rust
// String operation detection
if matches!(init_expr, ASTNode::MethodCall { .. }) {
    return Err("Unsupported: string/method call in body-local init (use Rust MIR path)".to_string());
}

// Type mismatch detection
if !is_int_compatible(init_expr) {
    return Err(format!("Unsupported: body-local init must be int/arithmetic, got {:?}", init_expr));
}
```

**Error Message Format**:
```
Error: Unsupported init expression: method call (Phase 186 limitation)
Hint: Body-local init only supports int/arithmetic (BinOp, Const, Variable)
      For string operations, use Rust MIR path instead of JoinIR
```

## Test Strategy

### Unit Tests

**File**: `src/mir/join_ir/lowering/loop_body_local_init.rs` (inline tests)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_const_init() {
        // local temp = 42
        // Should emit: Const(42)
    }

    #[test]
    fn test_lower_binop_init() {
        // local digit_pos = pos - start
        // Should emit: BinOp(Sub, pos_vid, start_vid)
    }

    #[test]
    fn test_fail_fast_method_call() {
        // local ch = s.substring(0, 1)
        // Should return Err("Unsupported: method call ...")
    }

    #[test]
    fn test_fail_fast_string_concat() {
        // local msg = "Error: " + text
        // Should return Err("Unsupported: string literal ...")
    }
}
```

### Integration Tests

**New test file**: `apps/tests/phase186_p2_body_local_digit_pos_min.hako`
```nyash
static box Test {
    main(start, pos) {
        local sum = 0
        loop (pos < 10) {
            local digit_pos = pos - start  // Body-local init
            if digit_pos >= 3 { break }
            sum = sum + digit_pos          // Use body-local
            pos = pos + 1
        }
        return sum
    }
}
```

**Expected behavior**:
- `digit_pos = 0 - 0 = 0` → sum = 0
- `digit_pos = 1 - 0 = 1` → sum = 1
- `digit_pos = 2 - 0 = 2` → sum = 3
- `digit_pos = 3 - 0 = 3` → break (3 >= 3)
- Final sum: 3

**Regression tests** (ensure Phase 184/185 still work):
- `phase184_body_local_update.hako` (basic update)
- `phase184_body_local_with_break.hako` (break condition)
- `phase185_p2_body_local_int_min.hako` (JsonParser-style)

### Fail-Fast Tests

**Test file**: `apps/tests/phase186_fail_fast_string_init.hako` (expected to fail)
```nyash
static box Test {
    main() {
        local s = "hello"
        loop (true) {
            local ch = s.substring(0, 1)  // ❌ Should fail with clear error
            break
        }
        return 0
    }
}
```

**Expected error**:
```
Error: Unsupported init expression: method call (Phase 186 limitation)
```

## Validation Commands

```bash
# Build
cargo build --release

# Unit tests
cargo test --release --lib loop_body_local_init

# Integration test
NYASH_JOINIR_CORE=1 ./target/release/hakorune \
  apps/tests/phase186_p2_body_local_digit_pos_min.hako

# Regression tests
NYASH_JOINIR_CORE=1 ./target/release/hakorune \
  apps/tests/phase184_body_local_update.hako
NYASH_JOINIR_CORE=1 ./target/release/hakorune \
  apps/tests/phase185_p2_body_local_int_min.hako

# Fail-Fast test (should error)
NYASH_JOINIR_CORE=1 ./target/release/hakorune \
  apps/tests/phase186_fail_fast_string_init.hako
```

## Success Criteria

### Functional Requirements

- ✅ Body-local init expressions lower to JoinIR (int/arithmetic only)
- ✅ Init ValueIds stored in LoopBodyLocalEnv
- ✅ Body-local variables usable in update expressions
- ✅ Pattern2/4 integration complete
- ✅ Fail-Fast for unsupported expressions (string/method call)

### Quality Requirements

- ✅ Box-First design (single responsibility, clear boundaries)
- ✅ No regression in existing tests (Phase 184/185)
- ✅ Clear error messages for unsupported features
- ✅ Deterministic behavior (BTreeMap-based)

### Documentation Requirements

- ✅ Design doc (this file)
- ✅ API documentation (inline rustdoc)
- ✅ Architecture update (joinir-architecture-overview.md)
- ✅ CURRENT_TASK.md update

## Future Work (Out of Scope)

### Phase 187+: String/Method Call Init Support

```nyash
loop(...) {
    local ch = s.substring(pos, 1)  // Future: BoxCall lowering
    local msg = "Error: " + text    // Future: String concat lowering
    ...
}
```

**Requirements**:
- BoxCall lowering to JoinIR
- Type tracking for Box values
- String operation support in JoinIR

### Phase 190+: Complex Init Expressions

```nyash
loop(...) {
    local result = (a + b) * (c - d)  // Nested BinOps
    local value = calc(x, y)          // Function calls
    ...
}
```

**Requirements**:
- Recursive expression lowering
- Function call lowering to JoinIR

## References

- **Phase 184**: LoopBodyLocalEnv introduction
- **Phase 185**: Pattern2 integration with body-local variables
- **Phase 178**: Fail-Fast principle for unsupported features
- **Phase 179-B**: Pattern2 pipeline architecture
- **Box Theory**: Single responsibility, clear boundaries, determinism

## Changelog

- **2025-12-09**: Initial design document created
Status: Historical
