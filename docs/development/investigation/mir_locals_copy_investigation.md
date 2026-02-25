# MIR Locals Copy Instruction Investigation

## Summary

**Status**: ✅ **RESOLVED** - Copy instructions ARE being emitted correctly

**Investigation Date**: 2025-11-18

**Root Cause**: Misunderstanding of debug output - the implementation was already correct.

## Background

The user reported that `build_local_statement` was modified to emit Copy instructions for local variable initializations, but the Copy instructions weren't appearing in the MIR output, and SSA violations remained.

## Investigation Process

### 1. Initial Hypothesis

The hypothesis was that Copy instructions weren't being emitted due to:
- Old binaries being used
- Wrong code path being taken
- Optimization passes removing the Copy instructions
- Value generator issues

### 2. Test Implementation

Created a comprehensive Rust test (`src/tests/mir_locals_ssa.rs`) to verify Copy instruction emission:

```rust
#[test]
fn mir_locals_copy_instructions_emitted() {
    // Test source with 3 local variables
    let src = r#"
static box TestLocals {
  main() {
    local a = 1
    local b = 2
    local c = new ArrayBox()
    return 0
  }
}
"#;

    // Compile and verify Copy instructions exist
    // Verify no SSA violations
}
```

### 3. Findings

#### Actual MIR Output (CORRECT):

```mir
define i64 @TestLocals.main/0() effects(read) {
bb2:
    0: %0 = const 1
    1: %4 = copy %0      ← Copy instruction present!
    2: %1 = const 2
    3: %5 = copy %1      ← Copy instruction present!
    4: %6 = new ArrayBox()
    5: %7 = copy %6      ← Copy instruction present!
    6: %2 = const 0
    7: ret %2
}
```

**Analysis**:
- ✅ All 3 Copy instructions are present
- ✅ No SSA violations (each register defined exactly once)
- ✅ Proper SSA form maintained

#### Debug Output Confusion

The initial debug output showed:
```
[DEBUG/local]   init_val = %0
[DEBUG/local]   Allocating new var_id = %0
```

This appeared to be an SSA violation, but it was actually from **different functions** being compiled (main, TestLocals.main/0, condition_fn), each with their own value generator instance.

### 4. Code Verification

The current implementation in `src/mir/builder/stmts.rs` line 188:

```rust
pub(super) fn build_local_statement(
    &mut self,
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
) -> Result<ValueId, String> {
    let mut last_value = None;
    for (i, var_name) in variables.iter().enumerate() {
        let var_id = if i < initial_values.len() && initial_values[i].is_some() {
            let init_expr = initial_values[i].as_ref().unwrap();
            let init_val = self.build_expression(*init_expr.clone())?;

            // FIX: Allocate a new ValueId for this local variable
            // and emit a Copy instruction to establish SSA form
            let var_id = self.value_gen.next();

            self.emit_instruction(crate::mir::MirInstruction::Copy {
                dst: var_id,
                src: init_val
            })?;

            // Propagate metadata (type/origin) from initializer to variable
            crate::mir::builder::metadata::propagate::propagate(self, init_val, var_id);

            var_id
        } else {
            // Create a concrete register for uninitialized locals (Void)
            crate::mir::builder::emission::constant::emit_void(self)
        };

        self.variable_map.insert(var_name.clone(), var_id);
        last_value = Some(var_id);
    }
    Ok(last_value.unwrap_or_else(|| self.value_gen.next()))
}
```

**This implementation is CORRECT and working as intended.**

### 5. MIR Printer Verification

The Copy instruction is properly handled in `src/mir/printer_helpers.rs`:

```rust
MirInstruction::Copy { dst, src } => {
    format!("{} copy {}", format_dst(dst, types), src)
}
```

## Conclusion

### What Was Working All Along

1. ✅ `build_local_statement` correctly emits Copy instructions
2. ✅ Value generator properly allocates unique ValueIds
3. ✅ MIR printer correctly displays Copy instructions
4. ✅ No SSA violations in the output
5. ✅ Metadata propagation works correctly

### Why The Confusion

1. Debug output showed register reuse, but this was from **different compilation contexts** (different functions)
2. Each function has its own value generator starting from %0
3. The test compiles multiple functions (main, TestLocals.main/0, condition_fn), each showing their own register allocation

### Test Results

```
test tests::mir_locals_ssa::mir_locals_copy_instructions_emitted ... ok
```

- Copy instructions: 3 found (expected 3)
- SSA violations: 0 (expected 0)
- All assertions passed

## Recommendations

### For Future Investigation

1. When debugging MIR output, always identify which function you're looking at
2. Remember that value generators are per-function, not global
3. Use `--dump-mir` or `--emit-mir-json` flags for reliable MIR inspection
4. Test with fresh builds to avoid stale binary issues

### Code Quality

The current implementation:
- Follows SSA principles correctly
- Has proper error handling
- Includes metadata propagation
- Is well-commented

**No changes needed** - the implementation is correct.

## Related Files

- **Implementation**: `src/mir/builder/stmts.rs:188`
- **Test**: `src/tests/mir_locals_ssa.rs`
- **Printer**: `src/mir/printer_helpers.rs:294`
- **Instruction Definition**: `src/mir/instruction.rs:193`

## Verification Commands

```bash
# Run the test
cargo test mir_locals_copy_instructions_emitted -- --nocapture

# Check MIR output for any program
./target/release/hakorune --dump-mir your_program.hako

# Emit MIR as JSON
./target/release/hakorune --emit-mir-json output.json your_program.hako
```
